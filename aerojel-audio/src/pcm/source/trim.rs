use crate::pcm::{PcmSource, PcmSpec, slice::PcmSliceMut};

pub struct Trim<S: PcmSource> {
    inner: S,
    frames_left: u64,
}

impl<S: PcmSource> PcmSource for Trim<S> {
    fn read_block(&mut self, mut buf: PcmSliceMut<'_>) -> Result<usize, super::WouldBlockError> {
        let empty_frames = buf.len().min(self.frames_left as usize);

        let frames_read = self.inner.read_block(buf.slice_mut(0..empty_frames))?;

        self.frames_left -= frames_read as u64;

        Ok(frames_read)
    }

    fn spec(&self) -> PcmSpec {
        self.inner.spec()
    }

    fn frames_left(&self) -> Option<u64> {
        Some(self.frames_left)
    }
}