use std::ops::RangeBounds;

use crate::pcm::{PcmSample, slice::{PcmSlice, PcmSliceMut}, utils};

/// Buffer containing `PcmSample`s in planar format.
pub struct PcmBuffer {
    data: Box<[PcmSample]>,
    plane_count: usize,
    plane_len: usize,
}

impl PcmBuffer {
    /// Constructs `PcmBuffer` from a boxed slice and frame count (plane length).
    /// 
    /// # Panics
    /// Panics if `data.len()` is not big enough to fit `plane_count * plane_len` samples.
    pub fn from_boxed_slice(
        data: Box<[PcmSample]>,
        plane_count: usize,
        plane_len: usize,
    ) -> Self {
        assert!(data.len() >= plane_count * plane_len, "insufficient capacity");

        Self {
            data,
            plane_count,
            plane_len,
        }
    }

    /// Constructs a zero initialized `PcmBuffer`.
    pub fn zeroed(
        plane_count: usize,
        plane_len: usize,
    ) -> Self {
        Self::from_boxed_slice(
            vec![0f32; plane_count * plane_len].into_boxed_slice(),
            plane_count,
            plane_len,
        )
    }

    pub fn slice<B: RangeBounds<usize>>(&self, bounds: B) -> PcmSlice<'_> {
        let (start, end) = utils::bounds_to_range(bounds, self.plane_len);

        PcmSlice::from_raw_parts(&self.data[start..], self.plane_count, self.plane_len, end - start)
    }

    /// Convert to slice.
    /// 
    /// Exactly the same as `self.slice(..)`.
    pub fn as_slice(&self) -> PcmSlice<'_> {
        self.slice(..)
    }

    pub fn slice_mut<B: RangeBounds<usize>>(&mut self, bounds: B) -> PcmSliceMut<'_> {
        let (start, end) = utils::bounds_to_range(bounds, self.plane_len);

        PcmSliceMut::from_raw_parts(&mut self.data[start..], self.plane_count, self.plane_len, end - start)
    }

    /// Convert to mutable slice.
    /// 
    /// Exactly the same as `self.slice_mut(..)`.
    pub fn as_mut_slice(&mut self) -> PcmSliceMut<'_> {
        self.slice_mut(..)
    }

    pub fn plane(&self, index: usize) -> &[PcmSample] {
        let (start, end) = self.plane_offsets(index);

        &self.data[start..end]
    }

    pub fn plane_mut(&mut self, index: usize) -> &mut [PcmSample] {
        let (start, end) = self.plane_offsets(index);

        &mut self.data[start..end]
    }

    fn plane_offsets(&self, index: usize) -> (usize, usize) {
        let start = index * self.plane_len;
        let end = start + self.plane_len;

        assert!(end < self.data.len(), "index out of bounds");

        (start, end)
    }

    pub fn plane_count(&self) -> usize {
        self.plane_count
    }

    pub fn len(&self) -> usize {
        self.plane_len
    }
}