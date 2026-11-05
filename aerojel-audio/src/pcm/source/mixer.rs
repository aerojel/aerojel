use std::{sync::{Arc, atomic::{AtomicBool, Ordering}}, time::Duration};

use crossbeam_utils::atomic::AtomicCell;
use ringbuf::traits::{Consumer, Producer, Split};

use crate::pcm::{PcmSource, PcmSpec, buffer::PcmBuffer, slice::PcmSliceMut, source};

/// Arbitrary number. Ideally, shouold be small enough
/// to fit into L1 but not too small to create a lot
/// of processing overhead.
const SCRATCH_BUF_CAP: usize = 512;

#[derive(Clone, Copy)]
struct Config {
    spec: PcmSpec,
    crossfade_frames: u32,
    crossfade_step: f32,
}

struct SharedState {
    should_flush: AtomicBool,
    /// Cell for sources *without* crossfade
    next_source: AtomicCell<Option<Box<Box<dyn PcmSource>>>>,
    /// Cell for sources *with* crossfade
    next_source_cf: AtomicCell<Option<Box<Box<dyn PcmSource>>>>,
}

pub struct Controller {
    config: Config,
    is_next_cf: bool,
    shared_state: Arc<SharedState>,
    garbage_receiver: ringbuf::HeapCons<Box<Box<dyn PcmSource>>>,
    // recycled_slot: Option<Box<Box<dyn PcmSource>>>, // Possible optimization. Avoids unnecessary reallocations.
}

impl Controller {
    /// Queues a source for playback. Returns whether there was a source already queued.
    pub fn set_next(&mut self, source: Box<dyn PcmSource>) -> bool {
        self.collect_garbage();

        let is_duration_sufficient = source.frames_left().is_some_and(|frames | {
            frames >= (self.config.crossfade_frames as u64) * 2
        });

        let new_source = Some(Box::new(source));
        
        if self.is_next_cf {
            if !is_duration_sufficient {
                self.shared_state.next_source_cf.take();

                self.is_next_cf = false;
                return self.shared_state.next_source.swap(new_source).is_some();
            }

            self.is_next_cf = true;
            self.shared_state.next_source_cf.swap(new_source).is_some()
        } else {
            if is_duration_sufficient {
                self.shared_state.next_source.take();

                self.is_next_cf = true;
                return self.shared_state.next_source_cf.swap(new_source).is_some();
            }

            self.is_next_cf = false;
            self.shared_state.next_source.swap(new_source).is_some()
        }
    }

    /// Dequeues a source for playback. Returns whether there was a source already queued.
    pub fn remove_next(&mut self) -> bool {
        self.collect_garbage();

        if self.is_next_cf {
            self.shared_state.next_source_cf.take()
        } else {
            self.shared_state.next_source.take()
        }.map(|s| *s).is_some()
    }

    pub fn flush_current(&self) {
        self.shared_state.should_flush.store(true, Ordering::Release);
    }

    fn collect_garbage(&mut self) {
        for source in self.garbage_receiver.pop_iter() {
            drop(source);
        }
    }
}

#[derive(Default)]
enum State {
    #[default]
    Idle,
    Direct {
        should_crossfade: bool,
        current: Box<Box<dyn PcmSource>>,
    },
    Ramp {
        frames_left: u32,
        remainder_len: usize,
        previous: Box<Box<dyn PcmSource>>,
        current: Box<Box<dyn PcmSource>>,
    },
}

/// Low-level audio mixer that implements `PcmSource`.
pub struct Mixer {
    config: Config,
    state: State,
    shared_state: Arc<SharedState>,
    scratch: PcmBuffer,
    garbage_sender: ringbuf::HeapProd<Box<Box<dyn PcmSource>>>,
}

impl Mixer {
    pub fn new(
        crossfade_duration: Duration,
        spec: PcmSpec,
    ) -> (Self, Controller) {
        let crossfade_seconds = crossfade_duration.as_secs_f64();
        let crossfade_frames_exact = crossfade_seconds * spec.sample_rate() as f64;

        assert!(crossfade_frames_exact <= u32::MAX as f64, "crossfade duration is too large");

        let config = Config {
            spec,
            crossfade_frames: crossfade_frames_exact as u32,
            crossfade_step: (crossfade_seconds / crossfade_frames_exact) as f32,
        };

        debug_assert!(
            AtomicCell::<Box<Box<dyn PcmSource>>>::is_lock_free(),
            "atomic cell is not lock-free, audio thread might block",
        );

        let shared_state = Arc::new(SharedState {
            should_flush: AtomicBool::new(false),
            next_source: AtomicCell::new(None),
            next_source_cf: AtomicCell::new(None),
        });

        let (garbage_sender, garbage_receiver) = ringbuf::HeapRb::new(2).split();

        let mixer = Self {
            config,
            state: State::default(),
            shared_state: Arc::clone(&shared_state),
            scratch: PcmBuffer::zeroed(spec.plane_count(), SCRATCH_BUF_CAP),
            garbage_sender,
        };

        let controller = Controller {
            config,
            shared_state,
            is_next_cf: false,
            garbage_receiver,
        };

        (mixer, controller)
    }

    fn read_block_internal(&mut self, mut buf: PcmSliceMut<'_>) -> Option<Result<usize, source::WouldBlockError>> {
        // Swap state to Idle to avoid problems with borrow checker.
        match std::mem::take(&mut self.state) {
            State::Idle => {
                if let Some(source) = self.shared_state.next_source.take() {
                    self.state = State::Direct {
                        should_crossfade: false,
                        current: source,
                    };

                    return None; // Read direct.
                }

                if let Some(source) = self.shared_state.next_source_cf.take() {
                    self.state = State::Direct {
                        should_crossfade: true,
                        current: source,
                    };

                    return None; // Read direct.
                }

                Some(Ok(0)) // EoS
            }
            State::Direct {
                mut should_crossfade,
                mut current,
            } => {
                let frames_til_crit_point = if let Some(frames_til_end) = current.frames_left() {
                    if frames_til_end != 0 {
                        if should_crossfade {
                            let frames_til_crossfade = frames_til_end - self.config.crossfade_frames as u64;

                            if frames_til_crossfade == 0 {
                                if let Some(new_source) = self.shared_state.next_source_cf.take() {
                                    self.state = State::Ramp {
                                        frames_left: self.config.crossfade_frames,
                                        remainder_len: 0,
                                        previous: current,
                                        current: new_source,
                                    };

                                    return None; // Read ramp.
                                } else {
                                    // Crossfade deadline is missed.
                                    should_crossfade = false;
                                    frames_til_end
                                }
                            } else {
                                frames_til_crossfade
                            }
                        } else {
                            frames_til_end
                        }
                    } else {
                        let _ = self.garbage_sender.try_push(current);

                        return None; // Try load next source.
                    }
                } else {
                    u64::MAX
                };

                let empty_frames_avalable = frames_til_crit_point.min(buf.len() as u64) as usize;

                if let Ok(frames_read) = current.read_block(buf.slice_mut(..empty_frames_avalable)) {
                    if frames_read != 0 {
                        self.state = State::Direct {
                            should_crossfade,
                            current,
                        };

                        Some(Ok(frames_read))
                    } else {
                        let _ = self.garbage_sender.try_push(current);

                        None // Source is buggy. Try to load next.
                    }
                } else {
                    Some(Err(source::WouldBlockError))
                }
            }
            State::Ramp {
                mut frames_left,
                mut remainder_len,
                mut previous,
                mut current,
            } => {
                if frames_left == 0 {
                    let _ = self.garbage_sender.try_push(previous);

                    self.state = State::Direct {
                        // If fade-in is done, then fade-out should also be possible.
                        should_crossfade: true,
                        current,
                    };

                    return None; // Read Direct.
                }

                let lower_bound = remainder_len;
                let upper_bound = lower_bound + frames_left as usize; // Guard against misbehaving sources.

                // Read previous into scratch buffer.
                let frames_filled = match previous.read_block(
                    self.scratch.slice_mut(lower_bound..upper_bound),
                ) {
                    Ok(frames_read) => frames_read + remainder_len,
                    err @ Err(_) => return Some(err),
                };

                // Read current directly into the output buffer.
                let frames_read = match current.read_block(buf.slice_mut(..frames_filled)) {
                    Ok(frames_read) => frames_read,
                    err @ Err(_) => return Some(err),
                };

                if frames_read == 0 {
                    let _ = self.garbage_sender.try_push(previous);
                    let _ = self.garbage_sender.try_push(current);

                    return None; // One of the sources is buggy. Try load next source.
                }

                let initial_mixing_coef = (
                    frames_left as f64 / self.config.crossfade_frames as f64
                ) as f32;

                let mut output_slice = buf.slice_mut(..frames_read);
                let scratch_slice = self.scratch.slice(..frames_read);

                for (dest, src) in output_slice.iter_planes_mut().zip(scratch_slice.iter_planes()) {
                    let mut mixing_coef = initial_mixing_coef;

                    for (dest_sample, src_sample) in dest.iter_mut().zip(src.iter()) {
                        mixing_coef += self.config.crossfade_step;

                        *dest_sample = *dest_sample * (1.0 - mixing_coef) + src_sample * mixing_coef;
                    }
                }

                // Casting is completely safe since frames_read
                // can at most be frames_left which is u32.
                frames_left -= frames_read as u32;

                if frames_filled > frames_read {
                    remainder_len = frames_filled - frames_read;

                    // Compact.
                    self.scratch.as_mut_slice().copy_within(frames_read..frames_filled, 0);
                }

                self.state = State::Ramp { frames_left, remainder_len, previous, current };

                Some(Ok(frames_read))
            }
        }
    }
}

impl PcmSource for Mixer {
    fn read_block(&mut self, mut buf: PcmSliceMut<'_>) -> Result<usize, super::WouldBlockError> {
        // Avoid recursion. Run until at least one frame is written
        // or there are no sources left or WouldBlockError is returned.
        loop {
            if self.shared_state.should_flush.swap(false, Ordering::AcqRel) {
                self.state = State::Idle;
            }

            if let Some(result) = self.read_block_internal(buf.slice_mut(..)) {
                break result;
            }
            // No result, run again.
        }
    }

    fn spec(&self) -> PcmSpec {
        self.config.spec
    }

    fn frames_left(&self) -> Option<u64> {
        match self.state {
            State::Idle => Some(0),
            State::Direct {
                should_crossfade: _,
                ref current,
            } => {
                current.frames_left()
            }
            State::Ramp {
                frames_left: _,
                remainder_len: _,
                previous: _,
                ref current,
            } => {
                current.frames_left()
            }
        }
    }
}