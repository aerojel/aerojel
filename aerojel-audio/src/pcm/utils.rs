use std::ops::{Bound, RangeBounds};

pub fn bounds_to_range<B: RangeBounds<usize>>(b: B, len: usize) -> (usize, usize) {
    let start = match b.start_bound() {
        Bound::Included(index) => *index,
        Bound::Excluded(index) => *index + 1,
        Bound::Unbounded => 0,
    };

    let end = match b.end_bound() {
        Bound::Included(index) => *index + 1,
        Bound::Excluded(index) => *index,
        Bound::Unbounded => len,
    };

    assert!(start <= end, "invalid range");
    assert!(end <= len, "end out of bounds");

    (start, end)
}