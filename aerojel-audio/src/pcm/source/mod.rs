pub mod procedural;
pub mod trim;
pub mod mixer;

use crate::pcm::{PcmSpec, slice::PcmSliceMut};

/// Currently no data is available, blocking would
/// be necessary but this is a non-blocking function.
#[derive(Debug)]
pub struct WouldBlockError;

pub trait PcmSource {
    /// `Ok(0)` - EoS
    fn read_block(&mut self, buf: PcmSliceMut<'_>) -> Result<usize, WouldBlockError>;

    fn spec(&self) -> PcmSpec;
    
    fn frames_left(&self) -> Option<u64>;
}

impl<T: PcmSource + ?Sized> PcmSource for Box<T> {
    fn read_block(&mut self, buf: PcmSliceMut<'_>) -> Result<usize, WouldBlockError> {
        (**self).read_block(buf)
    }

    fn spec(&self) -> PcmSpec {
        (**self).spec()
    }

    fn frames_left(&self) -> Option<u64> {
        (**self).frames_left()
    }
}