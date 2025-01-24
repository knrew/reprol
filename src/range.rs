use std::ops::{Bound, Range, RangeBounds};

/// 区間(range)を半区間[l, r)に変換する
/// 全体集合は[0, n)
pub fn to_open_range(range: impl RangeBounds<usize>, n: usize) -> Range<usize> {
    let l = match range.start_bound() {
        Bound::Unbounded => 0,
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => x + 1,
    };

    let r = match range.end_bound() {
        Bound::Unbounded => n,
        Bound::Included(&x) => x + 1,
        Bound::Excluded(&x) => x,
    };

    l..r
}
