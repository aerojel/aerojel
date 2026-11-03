use crate::pcm::{PcmSample, PcmSource, PcmSpec, slice::PcmSliceMut, source};

pub struct Sine {
    phase: f64,
    phase_step: f64,
    spec: PcmSpec,
}

impl Sine {
    pub fn new(frequency: f64, spec: PcmSpec) -> Self {
        assert!(spec.plane_count() > 0);
        assert!(spec.sample_rate() > 0);

        Self {
            phase: 0.0,
            phase_step: std::f64::consts::TAU * frequency / spec.sample_rate() as f64,
            spec,
        }
    }
}

impl PcmSource for Sine {
    fn read_block(&mut self, mut buf: PcmSliceMut<'_>) -> Result<usize, source::WouldBlockError> {
        assert_eq!(buf.plane_count(), self.spec.plane_count());

        let empty_frames = buf.len();
        let (phase, step) = (self.phase, self.phase_step);

        for (i, sample) in buf.plane_mut(0).iter_mut().enumerate() {
            *sample = (phase + i as f64 * step).sin() as PcmSample;
        }

        for plane_index in 1..buf.plane_count() {
            let (first, plane) = buf.plane_pair_mut(0, plane_index);
            plane.copy_from_slice(first);
        }

        self.phase = (phase + empty_frames as f64 * step).rem_euclid(std::f64::consts::TAU);

        Ok(empty_frames)
    }

    fn spec(&self) -> PcmSpec {
        self.spec
    }

    fn frames_left(&self) -> Option<u64> {
        None
    }
}