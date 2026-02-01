//! RangeUtils
//!
//! `RangeBounds`を半開区間`[l, r)`に正規化するためのユーティリティ．
//!
//! さまざまな整数型に対して，`RangeBounds`を一貫した半開区間形式に変換する機能を提供する．
//! 主にインデックス範囲の操作や範囲の正規化が必要な場面で使用する．
//!
//! # 主な機能
//!
//! - 任意の整数型に対する半開区間への正規化
//! - カスタム境界値を指定した正規化
//! - インデックス範囲のクランプ機能

use std::ops::{Bound, Range, RangeBounds};

pub trait RangeUtils: Sized {
    /// 任意の整数要素の`RangeBounds`を半開区間`[l, r)`に正規化する．
    ///
    /// # 引数
    ///
    /// * `range` - 正規化対象の範囲を表す`RangeBounds`実装
    ///
    /// # 返り値
    ///
    /// 半開区間`[l, r)`を表す`Range<Self>`
    ///
    /// # 動作
    ///
    /// 以下の規則に従って変換する:
    ///
    /// - `Bound::Unbounded` -> `Self::MIN` または `Self::MAX`
    /// - `Bound::Included(x)` -> `x`
    /// - `Bound::Excluded(x)` -> `x + 1`
    ///
    /// # パニック
    ///
    /// `Bound::Excluded(Self::MAX)` または `Bound::Included(Self::MAX)` の場合，
    /// デバッグモードでオーバーフローが発生してパニックする可能性がある
    fn to_half_open_range(range: impl RangeBounds<Self>) -> Range<Self>;

    /// 任意の整数要素の`RangeBounds`を半開区間`[l, r)`に正規化する(カスタム境界値使用)．
    ///
    /// # 引数
    ///
    /// * `range` - 正規化対象の範囲を表す`RangeBounds`実装
    /// * `min` - `Bound::Unbounded`の開始境界として使用する値
    /// * `positive_infinity` - `Bound::Unbounded`の終了境界として使用する値
    ///
    /// # 返り値
    ///
    /// 半開区間`[l, r)`を表す`Range<Self>`
    ///
    /// # 動作
    ///
    /// 以下の規則に従って変換する:
    ///
    /// - `Bound::Unbounded`（開始） -> `min`
    /// - `Bound::Unbounded`（終了） -> `positive_infinity`
    /// - `Bound::Included(x)` -> `x`
    /// - `Bound::Excluded(x)` -> `x + 1`
    ///
    /// # パニック
    ///
    /// `Bound::Excluded(Self::MAX)` または `Bound::Included(Self::MAX)` の場合、
    /// デバッグモードでオーバーフローが発生してパニックする可能性がある
    fn to_half_open_range_with_min_infinity(
        range: impl RangeBounds<Self>,
        min: Self,
        positive_infinity: Self,
    ) -> Range<Self>;
}

macro_rules! impl_rangeutils {
    ($ty: ty) => {
        impl RangeUtils for $ty {
            #[inline(always)]
            fn to_half_open_range(range: impl RangeBounds<Self>) -> Range<Self> {
                <$ty>::to_half_open_range_with_min_infinity(range, Self::MIN, Self::MAX)
            }

            fn to_half_open_range_with_min_infinity(
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

macro_rules! impl_rangeutils_for {
    ($($ty: ty),* $(,)?) => {
        $( impl_rangeutils!($ty); )*
    };
}

impl_rangeutils_for! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

/// インデックス区間を半開区間に正規化し，[0, len)の部分集合になるようにする．
///
/// # 引数
///
/// - `range` - 正規化対象のインデックス範囲を表す`RangeBounds`実装
/// - `len` - 終了境界として使用する最大インデックス（範囲は`[0, len)`）
///
/// # 返り値
///
/// `[0, len)`の範囲内にクランプされた半開区間`[l, r)`を表す`Range<usize>`
///
/// # 動作
///
/// 以下の規則に従って変換し、`[0, len)`の範囲内に制限する:
///
/// - `Bound::Unbounded`（開始） -> `0`
/// - `Bound::Unbounded`（終了） -> `len`
/// - `Bound::Included(x)` -> `x`
/// - `Bound::Excluded(x)` -> `x + 1`
pub fn to_half_open_index_range(range: impl RangeBounds<usize>, len: usize) -> Range<usize> {
    usize::to_half_open_range_with_min_infinity(range, 0, len)
}
