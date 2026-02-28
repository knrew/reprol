//! 区間正規化ユーティリティ
//!
//! 任意の `RangeBounds` を半開区間 `Range<T>` に正規化する．
//! Rust の範囲構文(`..`, `a..b`, `a..=b`, `..b`, `..=b`, `a..`)を
//! 統一的に扱うための変換レイヤ．
//!
//! # Examples
//!
//! ```ignore
//! use reprol::utils::normalize_range::{normalize, normalize_index};
//!
//! // 閉区間 [2, 5] → 半開区間 [2, 6)
//! assert_eq!(normalize(2..=5, 0, 10), 2..6);
//!
//! // 非有界 .. → [0, 10)
//! assert_eq!(normalize(.., 0, 10), 0..10);
//!
//! // インデックス区間の正規化(配列長 = 5)
//! assert_eq!(normalize_index(1..=3, 5), 1..4);
//! assert_eq!(normalize_index(.., 5), 0..5);
//! ```

use std::ops::{Bound, Range, RangeBounds};

/// 離散型の後続値を定義するトレイト．
///
/// `Bound::Included(x)` を `Bound::Excluded(x + 1)` に変換するために使用する．
///
/// # Notes
///
/// 整数型の最大値(`i64::MAX`, `usize::MAX` など)に対して `successor` を呼ぶと
/// オーバーフローする．閉区間の上端に最大値を使う場合は注意が必要．
pub trait Discrete: Sized + Clone + PartialOrd {
    /// 後続の値を返す．
    fn successor(&self) -> Self;
}

macro_rules! impl_discrete_inner {
    ($ty:ty) => {
        impl Discrete for $ty {
            #[inline]
            fn successor(&self) -> Self {
                self + 1
            }
        }
    };
}

macro_rules! impl_discrete {
    ($($ty:ty),* $(,)?) => {
        $( impl_discrete_inner!($ty); )*
    };
}

impl_discrete! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

/// 任意の `RangeBounds` を半開区間 `[min, sup)` に正規化する．
///
/// 非有界(`Unbounded`)の開始は `min`，終了は `sup` に置き換える．
///
/// # Panics
///
/// 正規化後の区間が `[min, sup)` の範囲外になる場合，または逆転している場合にパニックする．
pub fn normalize<T: Discrete>(range: impl RangeBounds<T>, min: T, sup: T) -> Range<T> {
    let l = match range.start_bound() {
        Bound::Unbounded => min.clone(),
        Bound::Included(x) => x.clone(),
        Bound::Excluded(x) => x.successor(),
    };

    let r = match range.end_bound() {
        Bound::Unbounded => sup.clone(),
        Bound::Excluded(x) => x.clone(),
        Bound::Included(x) => x.successor(),
    };

    assert!(min <= l && l <= r && r <= sup);

    l..r
}

/// インデックス区間を半開区間 `[0, len)` に正規化する．
///
/// `normalize(range, 0, len)` の省略形．
#[inline]
pub fn normalize_index(range: impl RangeBounds<usize>, len: usize) -> Range<usize> {
    normalize(range, 0, len)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Bound;

    // ========== Discrete::successor ==========

    /// 全整数型に対する successor の動作確認
    #[test]
    fn test_successor_all_types() {
        assert_eq!(0i8.successor(), 1i8);
        assert_eq!(0i16.successor(), 1i16);
        assert_eq!(0i32.successor(), 1i32);
        assert_eq!(0i64.successor(), 1i64);
        assert_eq!(0i128.successor(), 1i128);
        assert_eq!(0isize.successor(), 1isize);
        assert_eq!(0u8.successor(), 1u8);
        assert_eq!(0u16.successor(), 1u16);
        assert_eq!(0u32.successor(), 1u32);
        assert_eq!(0u64.successor(), 1u64);
        assert_eq!(0u128.successor(), 1u128);
        assert_eq!(0usize.successor(), 1usize);
    }

    /// 符号付き整数の負の値に対する successor
    #[test]
    fn test_successor_negative() {
        assert_eq!((-1i32).successor(), 0);
        assert_eq!((-100i64).successor(), -99);
        assert_eq!(i32::MIN.successor(), i32::MIN + 1);
    }

    // ========== normalize: 分岐網羅 ==========

    /// Range (a..b): start = Included, end = Excluded
    #[test]
    fn test_normalize_range() {
        assert_eq!(normalize(2..5, 0, 10), 2..5);
    }

    /// RangeInclusive (a..=b): start = Included, end = Included
    #[test]
    fn test_normalize_range_inclusive() {
        assert_eq!(normalize(2..=5, 0, 10), 2..6);
    }

    /// RangeFull (..): start = Unbounded, end = Unbounded
    #[test]
    fn test_normalize_range_full() {
        assert_eq!(normalize(.., 0, 10), 0..10);
    }

    /// RangeFrom (a..): start = Included, end = Unbounded
    #[test]
    fn test_normalize_range_from() {
        assert_eq!(normalize(3.., 0, 10), 3..10);
    }

    /// RangeTo (..b): start = Unbounded, end = Excluded
    #[test]
    fn test_normalize_range_to() {
        assert_eq!(normalize(..7, 0, 10), 0..7);
    }

    /// RangeToInclusive (..=b): start = Unbounded, end = Included
    #[test]
    fn test_normalize_range_to_inclusive() {
        assert_eq!(normalize(..=6, 0, 10), 0..7);
    }

    /// Excluded start (標準のRange構文では表現できないパターン)
    #[test]
    fn test_normalize_excluded_start() {
        // (Excluded, Excluded)
        assert_eq!(
            normalize((Bound::Excluded(2), Bound::Excluded(5)), 0, 10),
            3..5,
            "Excluded/Excluded"
        );
        // (Excluded, Included)
        assert_eq!(
            normalize((Bound::Excluded(2), Bound::Included(5)), 0, 10),
            3..6,
            "Excluded/Included"
        );
        // (Excluded, Unbounded)
        assert_eq!(
            normalize((Bound::Excluded(2), Bound::<i32>::Unbounded), 0, 10),
            3..10,
            "Excluded/Unbounded"
        );
    }

    // ========== normalize: エッジケース ==========

    /// 空区間 (l == r)
    #[test]
    fn test_normalize_empty_range() {
        assert_eq!(normalize(3..3, 0, 10), 3..3);
        assert_eq!(normalize(0..0, 0, 10), 0..0);
    }

    /// 単一要素区間
    #[test]
    fn test_normalize_single_element() {
        assert_eq!(normalize(3..=3, 0, 10), 3..4);
        assert_eq!(normalize(3..4, 0, 10), 3..4);
    }

    /// 全域と min..sup が一致する
    #[test]
    fn test_normalize_full_equals_min_sup() {
        assert_eq!(normalize(.., 0, 10), 0..10);
        assert_eq!(normalize(0..10, 0, 10), 0..10);
        assert_eq!(normalize(0..=9, 0, 10), 0..10);
    }

    /// 負の値を含む区間
    #[test]
    fn test_normalize_negative_values() {
        assert_eq!(normalize(-5..5, -10, 10), -5..5);
        assert_eq!(normalize(.., -10, 10), -10..10);
        assert_eq!(normalize(-3..=-1, -10, 10), -3..0);
    }

    /// min == sup (幅ゼロの値域)
    #[test]
    fn test_normalize_zero_width_universe() {
        assert_eq!(normalize(.., 0, 0), 0..0);
        assert_eq!(normalize(.., 5, 5), 5..5);
    }

    // ========== normalize_index ==========

    /// 全Range型に対する normalize_index の動作確認
    #[test]
    fn test_normalize_index_all_range_types() {
        let len = 10;
        assert_eq!(normalize_index(2..5, len), 2..5, "Range");
        assert_eq!(normalize_index(2..=5, len), 2..6, "RangeInclusive");
        assert_eq!(normalize_index(.., len), 0..10, "RangeFull");
        assert_eq!(normalize_index(3.., len), 3..10, "RangeFrom");
        assert_eq!(normalize_index(..7, len), 0..7, "RangeTo");
        assert_eq!(normalize_index(..=6, len), 0..7, "RangeToInclusive");
    }

    /// len = 0 (空配列)
    #[test]
    fn test_normalize_index_len_zero() {
        assert_eq!(normalize_index(.., 0), 0..0);
        assert_eq!(normalize_index(0..0, 0), 0..0);
    }

    /// len = 1 (単一要素配列)
    #[test]
    fn test_normalize_index_len_one() {
        assert_eq!(normalize_index(.., 1), 0..1);
        assert_eq!(normalize_index(0..=0, 1), 0..1);
        assert_eq!(normalize_index(0..1, 1), 0..1);
    }

    // ========== normalize: 範囲外 ==========

    /// start が min より小さい場合にパニックする
    #[test]
    #[should_panic]
    fn test_normalize_start_out_of_bounds() {
        let _ = normalize(0..5, 1, 10);
    }

    /// end が sup を超える場合にパニックする
    #[test]
    #[should_panic]
    fn test_normalize_end_out_of_bounds() {
        let _ = normalize(0..11, 0, 10);
    }

    /// 逆転した区間 (l > r) はパニックする
    #[test]
    #[should_panic]
    fn test_normalize_inverted_range() {
        let _ = normalize(5..3, 0, 10);
    }
}
