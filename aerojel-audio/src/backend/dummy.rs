use crate::pcm::{PcmSpec, slice::PcmSliceMut};

const SAMPLE_RATE: u32 = 48_000;
const PLANE_COUNT: usize = 2;

pub struct DummyBackend {
    _private: (),
}

impl DummyBackend {
    pub fn new() -> Self {
        Self {
            _private: (),
        }
    }
}

impl super::Backend for DummyBackend {
    type Stream = DummyStream;

    fn open_default_stream<D, E>(
        &self,
        _data_callback: D,
        _error_callback: E,
    ) -> Result<Self::Stream, super::NoDevicesError>
    where
        D: FnMut(Self::Stream, PcmSliceMut<'_>) + Send + 'static,
        E: FnMut() + Send + 'static,
    {
        Ok(DummyStream {
            _private: (),
        })
    }
}

pub struct DummyStream {
    _private: (),
}

impl super::Stream for DummyStream {
    fn pause(&mut self) -> Result<(), super::StreamClosedError> {
        Ok(())
    }

    fn resume(&mut self) -> Result<(), super::StreamClosedError> {
        Ok(())
    }

    fn reset(&mut self) -> Result<(), super::StreamClosedError> {
        Ok(())
    }

    fn underun_count(&self) -> Result<u64, super::StreamClosedError> {
        Ok(0)
    }

    fn spec(&self) -> PcmSpec {
        PcmSpec::new(SAMPLE_RATE, PLANE_COUNT)
    }
}