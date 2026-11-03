use std::ops::RangeBounds;

use crate::pcm::{PcmSample, utils};

#[derive(Clone, Copy)]
pub struct PcmSlice<'a> {
    data: &'a [PcmSample],
    plane_count: usize,
    plane_len: usize,
    len: usize,
}

impl<'a> PcmSlice<'a> {
    pub(crate) fn from_raw_parts(
        data: &'a [PcmSample],
        plane_count: usize,
        plane_len: usize,
        len: usize,
    ) -> Self {
        Self {
            data,
            plane_count,
            plane_len,
            len,
        }
    }

    /// Get a slice of a single plane (channel).
    /// 
    /// # Panics 
    /// Panics if index is out of bounds.
    pub fn plane(&self, index: usize) -> &[PcmSample] {
        let start = index * self.plane_len;
        let end = start + self.len;

        assert_bounds(end, self.data.len());

        &self.data[start..end]
    }

    pub fn slice<B: RangeBounds<usize>>(&self, bounds: B) -> PcmSlice<'_> {
        let (start, end) = utils::bounds_to_range(bounds, self.len);

        PcmSlice {
            data: &self.data[start..],
            plane_count: self.plane_count,
            plane_len: self.plane_len,
            len: end - start,
        }
    }

    pub fn iter_planes(&self) -> Planes<'_> {
        Planes {
            data: self.data,
            plane_len: self.plane_len,
            len: self.len,
            planes_left: self.plane_count,
        }
    }

    /// Number of planes (channels).
    pub fn plane_count(&self) -> usize {
        self.plane_count
    }

    /// Number of frames (samples in each plane).
    pub fn len(&self) -> usize {
        self.len
    }
}

pub struct PcmSliceMut<'a> {
    data: &'a mut [PcmSample],
    plane_count: usize,
    plane_len: usize,
    len: usize,
}

impl<'a> PcmSliceMut<'a> {
    pub(crate) fn from_raw_parts(
        data: &'a mut [PcmSample],
        plane_count: usize,
        plane_len: usize,
        len: usize,
    ) -> Self {
        Self {
            data,
            plane_count,
            plane_len,
            len,
        }
    }

    /// Get a slice of a single plane (channel).
    /// 
    /// # Panics 
    /// Panics if index is out of bounds.
    pub fn plane(&self, index: usize) -> &[PcmSample] {
        let (start, end) = self.plane_offsets(index);

        &self.data[start..end]
    }

    /// Get a mutable slice of a single plane (channel).
    /// 
    /// # Panics 
    /// Panics if index is out of bounds.
    pub fn plane_mut(&mut self, index: usize) -> &mut [PcmSample] {
        let (start, end) = self.plane_offsets(index);

        &mut self.data[start..end]
    }

    pub fn slice<B: RangeBounds<usize>>(&self, bounds: B) -> PcmSlice<'_> {
        let (start, end) = utils::bounds_to_range(bounds, self.len);

        PcmSlice {
            data: &self.data[start..],
            plane_count: self.plane_count,
            plane_len: self.plane_len,
            len: end - start,
        }
    }

    pub fn slice_mut<B: RangeBounds<usize>>(&mut self, bounds: B) -> PcmSliceMut<'_> {
        let (start, end) = utils::bounds_to_range(bounds, self.len);

        PcmSliceMut {
            data: &mut self.data[start..],
            plane_count: self.plane_count,
            plane_len: self.plane_len,
            len: end - start,
        }
    }

    pub fn plane_pair_mut(
        &mut self,
        index0: usize,
        index1: usize,
    ) -> (&mut [PcmSample], &mut [PcmSample]) {
        let (start1, end1) = self.plane_offsets(index0);
        let (start2, end2) = self.plane_offsets(index1);

        assert_ne!(index0, index1, "cannot borrow the same plane multiple times");

        if end1 < end2 {
            let (first, second) = self.data.split_at_mut(end1);
            (&mut first[start1..], &mut second[start2 - end1..end2 - end1])
        } else {
            let (first, second) = self.data.split_at_mut(end2);
            (&mut second[start1 - end2..end1 - end2], &mut first[start2..])
        }
    }

    fn plane_offsets(&self, index: usize) -> (usize, usize) {
        let start = index * self.plane_len;
        let end = start + self.len;

        assert_bounds(end, self.data.len());

        (start, end)
    }

    pub fn iter_planes(&self) -> Planes<'_> {
        Planes {
            data: self.data,
            plane_len: self.plane_len,
            len: self.len,
            planes_left: self.plane_count,
        }
    }

    pub fn iter_planes_mut(&mut self) -> PlanesMut<'_> {
        PlanesMut {
            data: self.data,
            plane_len: self.plane_len,
            len: self.len,
            planes_left: self.plane_count,
        }
    }

    pub fn plane_count(&self) -> usize {
        self.plane_count
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

pub struct Planes<'a> {
    data: &'a [PcmSample],
    plane_len: usize,
    len: usize,
    /// Storing a separate index is required for zero-sized slices
    /// where just incrementing `data` would always produce 0 iterations.
    planes_left: usize,
}

impl<'a> Iterator for Planes<'a> {
    type Item = &'a [PcmSample];

    fn next(&mut self) -> Option<Self::Item> {
        if self.planes_left > 0 {
            let end = self.data.len() - self.plane_len * (self.planes_left - 1);
            let start = end - self.len;

            let slice = &self.data[start..end];

            self.planes_left -= 1;

            Some(slice)
        } else {
            None
        }
    }
}

pub struct PlanesMut<'a> {
    data: &'a mut [PcmSample],
    plane_len: usize,
    len: usize,
    /// Storing a separate index is required for zero-sized slices
    /// where just incrementing `data` would always produce 0 iterations.
    planes_left: usize, 
}

impl<'a> Iterator for PlanesMut<'a> {
    type Item = &'a mut [PcmSample];

    fn next(&mut self) -> Option<Self::Item> {
        if self.planes_left > 0 {
            let end = self.data.len() - self.plane_len * (self.planes_left - 1);
            let start = end - self.len;

            let (remainder, output) = std::mem::take(&mut self.data).split_at_mut(start);
            self.data = remainder;

            self.planes_left -= 1;

            Some(&mut output[..self.len])
        } else {
            None
        }
    }
}

fn assert_bounds(end: usize, avilable: usize) {
    assert!(end <= avilable, "end index is out of bounds");
}