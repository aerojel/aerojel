use crate::pcm::{PcmSpec, slice::PcmSliceMut};

pub mod dummy;

/// There are no devices currently available (headphones or speakers should be plugged in).
#[derive(Debug)]
pub struct NoDevicesError;

/// Stream was closed and cannot be used anymore.
#[derive(Debug)]
pub struct StreamClosedError;

pub trait Backend {
    type Stream: Stream;

    fn open_default_stream<D, E>(
        &self,
        data_callback: D,
        error_callback: E,
    ) -> Result<Self::Stream, NoDevicesError>
    where
        D: FnMut(Self::Stream, PcmSliceMut<'_>) + Send + 'static,
        E: FnMut() + Send + 'static;
}

pub trait Stream {
    /// Stop without draining buffered frames
    fn pause(&mut self) -> Result<(), StreamClosedError>;

    /// Continue outputting buffered frames
    fn resume(&mut self) -> Result<(), StreamClosedError>;
    
    /// Stop and drain all buffered frames.
    fn reset(&mut self) -> Result<(), StreamClosedError>;

    /// Amount of frames that were not dilivered in time.
    fn underun_count(&self) -> Result<u64, StreamClosedError>;

    fn spec(&self) -> PcmSpec;
}