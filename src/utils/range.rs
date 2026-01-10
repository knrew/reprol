use std::ops::{Bound, Range, RangeBounds};

pub trait RangeUtil: Sized {
    /// 任意の整数要素の`RangeBounds`を半開区間`[l, r)`に正規化する．
    fn to_half_open_range(
        range: impl RangeBounds<Self>,
        min: Self,
        positive_infinity: Self,
    ) -> Range<Self>;
}

macro_rules! impl_rangeutil {
    ($ty: ty) => {
        impl RangeUtil for $ty {
            fn to_half_open_range(
                range_bounds: impl RangeBounds<Self>,
                min: Self,
                positive_infinity: Self,
            ) -> Range<Self> {
                let l = match range_bounds.start_bound() {
                    Bound::Unbounded => min,
                    Bound::Included(&x) => x,
                    Bound::Excluded(&x) => {
                        debug_assert!(x != <$ty>::MAX);
                        x + 1
                    }
                };

                let r = match range_bounds.end_bound() {
                    Bound::Excluded(&x) => x,
                    Bound::Included(&x) => {
                        debug_assert!(x != <$ty>::MAX);
                        x + 1
                    }
                    Bound::Unbounded => positive_infinity,
                };

                l..r
            }
        }
    };
}

macro_rules! impl_rangeutil_for {
    ($($ty: ty),* $(,)?) => {
        $( impl_rangeutil!($ty); )*
    };
}

impl_rangeutil_for! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

/// インデックス区間を半開区間に正規化し，[0, len)の部分集合になるようにする．
pub fn to_half_open_index_range(range: impl RangeBounds<usize>, len: usize) -> Range<usize> {
    usize::to_half_open_range(range, 0, len)
}
