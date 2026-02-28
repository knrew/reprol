//! 二分探索(Binary Search)
//!
//! - [`Bisect`] : 整数・浮動小数点数の範囲上の二分探索
//! - [`SliceBoundsExt`] : ソート済みスライス上の lower_bound / upper_bound
//!
//! # Examples
//!
//! ## [`Bisect`]
//!
//! ```
//! use reprol::bisect::Bisect;
//!
//! // 1..=100 の範囲で x*x < 30 が false となる最小の x を求める
//! let result = (1..=100).bisect(|&x| x * x < 30);
//! assert_eq!(result, 6); // 6*6 = 36 >= 30
//! ```
//!
//! ## [`SliceBoundsExt`]
//!
//! ```
//! use reprol::bisect::SliceBoundsExt;
//!
//! let v = [1, 3, 3, 5, 7];
//! assert_eq!(v.lower_bound(&3), 1); // 3 以上の最初の位置
//! assert_eq!(v.upper_bound(&3), 3); // 3 より大きい最初の位置
//! ```

use std::{
    cmp::Ordering,
    ops::{Range, RangeBounds},
};

use crate::utils::normalize_range::{Discrete, normalize};

/// 範囲上の二分探索を行うトレイト．
pub trait Bisect<T> {
    /// 単調な述語 `f` に対して，`f(x)` が `false` となる最小の `x` を返す．
    ///
    /// すべての `x` に対して `f(x)` が `true` の場合は，範囲の上限を返す．
    ///
    /// # Panics
    ///
    /// 範囲が空の場合にパニックする．
    fn bisect(self, f: impl FnMut(&T) -> bool) -> T;
}

impl<T, B> Bisect<T> for B
where
    T: BisectInteger,
    B: RangeBounds<T>,
{
    fn bisect(self, mut f: impl FnMut(&T) -> bool) -> T {
        let Range {
            start: mut ok,
            end: mut ng,
        } = normalize(self, T::MIN, T::SUP);

        assert!(ok < ng);

        if !f(&ok) {
            return ok;
        }

        while let Some(mid) = T::midpoint(&ok, &ng) {
            if f(&mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ng
    }
}

/// 浮動小数点数の二分探索．
///
/// f64 のビット表現を順序保存する全単射で u64 に変換し，
/// 整数の二分探索として実装する．
///
/// # References
///
/// - [Re: 浮動小数点数の二分探索 - えびちゃんの日記](https://rsk0315.hatenablog.com/entry/2022/04/07/004618)
impl Bisect<f64> for Range<f64> {
    fn bisect(self, mut f: impl FnMut(&f64) -> bool) -> f64 {
        #[inline]
        fn f2u(f: f64) -> u64 {
            let u = f.to_bits();
            if u >> 63 == 1 { !u } else { u ^ 1 << 63 }
        }

        #[inline]
        fn u2f(u: u64) -> f64 {
            f64::from_bits(if u >> 63 == 1 { u ^ 1 << 63 } else { !u })
        }

        let Range { start: ok, end: ng } = self;

        assert!(ok < ng);

        if !f(&ok) {
            return ok;
        }

        let mut ok = f2u(ok);
        let mut ng = f2u(ng);

        while ng > ok + 1 {
            let mid = ok + (ng - ok) / 2;
            if f(&u2f(mid)) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        u2f(ng)
    }
}

/// ソート済みスライスに対する境界探索の拡張トレイト．
pub trait SliceBoundsExt {
    /// 要素の型．
    type Item: Ord;

    /// `f(x)` が `Ordering::Less` でない最初のインデックスを返す．
    /// 該当なしの場合はスライス長を返す．
    fn lower_bound_by(&self, f: impl FnMut(&Self::Item) -> Ordering) -> usize;

    /// `f(x)` が `Ordering::Greater` である最初のインデックスを返す．
    /// 該当なしの場合はスライス長を返す．
    fn upper_bound_by(&self, f: impl FnMut(&Self::Item) -> Ordering) -> usize;

    /// `x` 以上の最初のインデックスを返す．
    /// 該当なしの場合はスライス長を返す．
    fn lower_bound(&self, x: &Self::Item) -> usize {
        self.lower_bound_by(|y| y.cmp(x))
    }

    /// `f(x) >= k` である最初のインデックスを返す．
    /// 該当なしの場合はスライス長を返す．
    fn lower_bound_by_key<K: Ord>(&self, k: &K, mut f: impl FnMut(&Self::Item) -> K) -> usize {
        self.lower_bound_by(|x| f(x).cmp(k))
    }

    /// `x` より大きい最初のインデックスを返す．
    /// 該当なしの場合はスライス長を返す．
    fn upper_bound(&self, x: &Self::Item) -> usize {
        self.upper_bound_by(|y| y.cmp(x))
    }

    /// `f(x) > k` である最初のインデックスを返す．
    /// 該当なしの場合はスライス長を返す．
    fn upper_bound_by_key<K: Ord>(&self, k: &K, mut f: impl FnMut(&Self::Item) -> K) -> usize {
        self.upper_bound_by(|x| f(x).cmp(k))
    }
}

impl<T: Ord> SliceBoundsExt for [T] {
    type Item = T;

    fn lower_bound_by(&self, mut f: impl FnMut(&Self::Item) -> Ordering) -> usize {
        if self.is_empty() {
            return 0;
        }
        (0..self.len()).bisect(|&i| f(&self[i]) == Ordering::Less)
    }

    fn upper_bound_by(&self, mut f: impl FnMut(&Self::Item) -> Ordering) -> usize {
        if self.is_empty() {
            return 0;
        }
        (0..self.len()).bisect(|&i| f(&self[i]) != Ordering::Greater)
    }
}

/// 整数型の二分探索に必要な定数と中間値計算を提供するトレイト．
trait BisectInteger: Discrete {
    /// 型の最小値．
    const MIN: Self;

    /// 探索範囲の上限．`Unbounded` 時のデフォルト上端として使用．
    const SUP: Self;

    /// `start` と `end` の中間値を返す．隣接している場合は `None`．
    fn midpoint(start: &Self, end: &Self) -> Option<Self>;
}

macro_rules! impl_bisect_integer_inner {
    ($ty: ty) => {
        impl BisectInteger for $ty {
            const MIN: Self = <$ty>::MIN;
            const SUP: Self = <$ty>::MAX;

            fn midpoint(start: &Self, end: &Self) -> Option<Self> {
                (end - start > 1).then(|| start + (end - start) / 2)
            }
        }
    };
}

macro_rules! impl_bisect_integer {
    ($($ty: ty),* $(,)?) => {
        $( impl_bisect_integer_inner!($ty); )*
    };
}

impl_bisect_integer! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

#[cfg(test)]
mod tests {
    use std::ops::Bound;

    use rand::Rng;

    use super::*;
    use crate::utils::test_utils::random::get_test_rng;

    // ========== 愚直解ヘルパ関数 ==========

    fn naive_lower_bound<T: Ord>(v: &[T], x: &T) -> usize {
        v.iter().position(|y| y >= x).unwrap_or(v.len())
    }

    fn naive_upper_bound<T: Ord>(v: &[T], x: &T) -> usize {
        v.iter().position(|y| y > x).unwrap_or(v.len())
    }

    // ========== Bisect 整数二分探索 ==========

    #[test]
    fn test_bisect_basic() {
        // x*x >= 30 となる最小の x
        let result = (1..=100).bisect(|&x: &i32| x * x < 30);
        assert_eq!(result, 6);

        // x*x >= 100 となる最小の x
        let result = (0..=100).bisect(|&x: &i32| x * x < 100);
        assert_eq!(result, 10);

        // 2^x >= 1024 となる最小の x
        let result = (0..=30).bisect(|&x: &i32| (1_i64 << x) < 1024);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_bisect_predicate_always_true() {
        // 全 true なら範囲の上限を返す
        let result = (0..10).bisect(|_| true);
        assert_eq!(result, 10);

        let result = (0..=10).bisect(|_| true);
        assert_eq!(result, 11);

        let result = (5..8).bisect(|_| true);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_bisect_predicate_false_at_start() {
        // 先頭で false なら 早期 return
        let result = (0..10).bisect(|_| false);
        assert_eq!(result, 0);

        let result = (5..10).bisect(|_| false);
        assert_eq!(result, 5);

        let result = (3..=7).bisect(|_| false);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_bisect_range_variants() {
        let f = |&x: &i32| x < 7;

        // Range
        assert_eq!((0..10).bisect(f), 7);

        // RangeInclusive
        assert_eq!((0..=10).bisect(f), 7);

        // RangeFrom
        assert_eq!((0..).bisect(f), 7);

        // RangeTo
        {
            let target = i32::MIN + 50;
            assert_eq!((..i32::MIN + 100).bisect(|&x| x < target), target);
        }

        // RangeToInclusive
        {
            let target = i32::MIN + 50;
            assert_eq!((..=i32::MIN + 100).bisect(|&x| x < target), target);
        }

        // (Included, Excluded)
        {
            let custom: (Bound<i32>, Bound<i32>) = (Bound::Included(0), Bound::Excluded(10));
            assert_eq!(custom.bisect(f), 7);
        }

        // (Excluded, Included)
        {
            let custom: (Bound<i32>, Bound<i32>) = (Bound::Excluded(0), Bound::Included(10));
            assert_eq!(custom.bisect(f), 7);
        }
    }

    #[test]
    fn test_bisect_integer_types_smoke() {
        // 全12整数型のスモークテスト: x < 5 が false となる最小の x = 5
        assert_eq!((0_i8..10).bisect(|&x| x < 5), 5_i8);
        assert_eq!((0_i16..10).bisect(|&x| x < 5), 5_i16);
        assert_eq!((0_i32..10).bisect(|&x| x < 5), 5_i32);
        assert_eq!((0_i64..10).bisect(|&x| x < 5), 5_i64);
        assert_eq!((0_i128..10).bisect(|&x| x < 5), 5_i128);
        assert_eq!((0_isize..10).bisect(|&x| x < 5), 5_isize);
        assert_eq!((0_u8..10).bisect(|&x| x < 5), 5_u8);
        assert_eq!((0_u16..10).bisect(|&x| x < 5), 5_u16);
        assert_eq!((0_u32..10).bisect(|&x| x < 5), 5_u32);
        assert_eq!((0_u64..10).bisect(|&x| x < 5), 5_u64);
        assert_eq!((0_u128..10).bisect(|&x| x < 5), 5_u128);
        assert_eq!((0_usize..10).bisect(|&x| x < 5), 5_usize);
    }

    #[test]
    fn test_bisect_single_element_range() {
        // 要素1つの範囲
        let result = (5..=5).bisect(|&x| x < 5);
        assert_eq!(result, 5);

        let result = (5..=5).bisect(|&x| x < 6);
        assert_eq!(result, 6);

        let result = (0..=0).bisect(|_| false);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_bisect_boundary_values() {
        // i32::MIN 近傍
        let result = (i32::MIN..i32::MIN + 10).bisect(|&x| x < i32::MIN + 5);
        assert_eq!(result, i32::MIN + 5);

        // 0 境界 (負->正)
        let result = (-5..5).bisect(|&x| x < 0);
        assert_eq!(result, 0);

        // i32::MAX 近傍
        let result = (i32::MAX - 10..i32::MAX).bisect(|&x| x < i32::MAX - 3);
        assert_eq!(result, i32::MAX - 3);

        // u64 の大きい値
        {
            let l = u64::MAX - 100;
            let result = (l..u64::MAX).bisect(|&x| x < l + 50);
            assert_eq!(result, l + 50);
        }
    }

    #[test]
    fn test_bisect_exhaustive_small_range() {
        // 範囲 0..=20 で全探索
        for t in 0..=22 {
            let result = (0..=20).bisect(|&x| x < t);
            let expected = t.min(21); // 範囲上限は 21 (= 20 + 1)
            assert_eq!(result, expected, "threshold t={t}");
        }
    }

    #[test]
    #[should_panic]
    fn test_bisect_empty_range_panics() {
        (5..5).bisect(|&x| x < 3);
    }

    // ========== Bisect 浮動小数点二分探索 ==========

    #[test]
    fn test_bisect_f64_sqrt() {
        // sqrt(2) の探索
        let result: f64 = (0.0..2.0).bisect(|&x| x * x < 2.0);
        let expected = std::f64::consts::SQRT_2;
        assert!((result - expected).abs() < 1e-10);

        // sqrt(3) の探索
        let result: f64 = (1.0..2.0).bisect(|&x| x * x < 3.0);
        let expected = 3.0_f64.sqrt();
        assert!((result - expected).abs() < 1e-10);
    }

    #[test]
    fn test_bisect_f64_predicate_always_true() {
        let result: f64 = (0.0..1.0).bisect(|_| true);
        assert!((result - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_bisect_f64_predicate_false_at_start() {
        let result: f64 = (0.0..1.0).bisect(|_| false);
        assert!((result - 0.0).abs() < 1e-15);

        let result: f64 = (3.0..5.0).bisect(|_| false);
        assert!((result - 3.0).abs() < 1e-15);
    }

    #[test]
    fn test_bisect_f64_negative_range() {
        // 負の範囲: x < -1.5 が false となる最小の x
        let result: f64 = (-3.0..0.0).bisect(|&x| x < -1.5);
        assert!((result - (-1.5)).abs() < 1e-10);

        // 負->正をまたぐ範囲
        let result: f64 = (-2.0..2.0).bisect(|&x| x < 0.5);
        assert!((result - 0.5).abs() < 1e-10);
    }

    #[test]
    #[should_panic]
    fn test_bisect_f64_empty_range_panics() {
        (5.0..5.0).bisect(|&x: &f64| x < 3.0);
    }

    #[test]
    #[should_panic]
    fn test_bisect_f64_nan_start_panics() {
        (f64::NAN..1.0).bisect(|&x: &f64| x < 0.5);
    }

    #[test]
    #[should_panic]
    fn test_bisect_f64_nan_end_panics() {
        (0.0..f64::NAN).bisect(|&x: &f64| x < 0.5);
    }

    #[test]
    #[should_panic]
    fn test_bisect_f64_nan_both_panics() {
        (f64::NAN..f64::NAN).bisect(|&x: &f64| x < 0.5);
    }

    // ========== SliceBoundsExt スライス境界探索 ==========

    #[test]
    fn test_lower_bound_basic() {
        let v = [1, 3, 3, 5, 7, 9, 9, 9, 11, 13];
        assert_eq!(v.lower_bound(&0), 0);
        assert_eq!(v.lower_bound(&1), 0);
        assert_eq!(v.lower_bound(&3), 1);
        assert_eq!(v.lower_bound(&4), 3);
        assert_eq!(v.lower_bound(&9), 5);
        assert_eq!(v.lower_bound(&13), 9);
        assert_eq!(v.lower_bound(&14), 10);
    }

    #[test]
    fn test_upper_bound_basic() {
        let v = [1, 3, 3, 5, 7, 9, 9, 9, 11, 13];
        assert_eq!(v.upper_bound(&0), 0);
        assert_eq!(v.upper_bound(&1), 1);
        assert_eq!(v.upper_bound(&3), 3);
        assert_eq!(v.upper_bound(&4), 3);
        assert_eq!(v.upper_bound(&9), 8);
        assert_eq!(v.upper_bound(&13), 10);
        assert_eq!(v.upper_bound(&14), 10);
    }

    #[test]
    fn test_bounds_empty_slice() {
        let v: Vec<i32> = vec![];
        assert_eq!(v.lower_bound(&0), 0);
        assert_eq!(v.upper_bound(&0), 0);
    }

    #[test]
    fn test_bounds_single_element() {
        let v = [10];
        assert_eq!(v.lower_bound(&5), 0);
        assert_eq!(v.lower_bound(&10), 0);
        assert_eq!(v.lower_bound(&15), 1);
        assert_eq!(v.upper_bound(&5), 0);
        assert_eq!(v.upper_bound(&10), 1);
        assert_eq!(v.upper_bound(&15), 1);
    }

    #[test]
    fn test_bounds_all_same() {
        let v = [4, 4, 4, 4, 4];
        assert_eq!(v.lower_bound(&3), 0);
        assert_eq!(v.lower_bound(&4), 0);
        assert_eq!(v.lower_bound(&5), 5);
        assert_eq!(v.upper_bound(&3), 0);
        assert_eq!(v.upper_bound(&4), 5);
        assert_eq!(v.upper_bound(&5), 5);
    }

    #[test]
    fn test_bounds_target_outside() {
        let v = [10, 20, 30, 40, 50];

        // 全要素未満
        assert_eq!(v.lower_bound(&5), 0);
        assert_eq!(v.upper_bound(&5), 0);

        // 全要素超過
        assert_eq!(v.lower_bound(&100), 5);
        assert_eq!(v.upper_bound(&100), 5);
    }

    #[test]
    fn test_lower_bound_by_custom() {
        // 降順ソートでのカスタム比較
        let v = [50, 40, 30, 20, 10];

        // 30 以下の最初の位置を探す (降順なので逆順比較)
        let idx = v.lower_bound_by(|x| x.cmp(&30).reverse());
        assert_eq!(idx, 2);

        // 通常のソート済み配列でカスタム比較
        let v = [1, 3, 5, 7, 9];
        let idx = v.lower_bound_by(|x| x.cmp(&6));
        assert_eq!(idx, 3);
    }

    #[test]
    fn test_upper_bound_by_custom() {
        let v = [50, 40, 30, 20, 10];
        let idx = v.upper_bound_by(|x| x.cmp(&30).reverse());
        assert_eq!(idx, 3);

        let v = [1, 3, 5, 7, 9];
        let idx = v.upper_bound_by(|x| x.cmp(&5));
        assert_eq!(idx, 3);
    }

    #[test]
    fn test_lower_bound_by_key() {
        let v = [(1, 10), (3, 20), (5, 30), (7, 40)];
        assert_eq!(v.lower_bound_by_key(&0, |&(x, _)| x), 0);
        assert_eq!(v.lower_bound_by_key(&3, |&(x, _)| x), 1);
        assert_eq!(v.lower_bound_by_key(&4, |&(x, _)| x), 2);
        assert_eq!(v.lower_bound_by_key(&8, |&(x, _)| x), 4);

        // 第2要素をキーに
        assert_eq!(v.lower_bound_by_key(&25, |&(_, y)| y), 2);
    }

    #[test]
    fn test_upper_bound_by_key() {
        let v = [(1, 10), (3, 20), (5, 30), (7, 40)];
        assert_eq!(v.upper_bound_by_key(&0, |&(x, _)| x), 0);
        assert_eq!(v.upper_bound_by_key(&3, |&(x, _)| x), 2);
        assert_eq!(v.upper_bound_by_key(&4, |&(x, _)| x), 2);
        assert_eq!(v.upper_bound_by_key(&8, |&(x, _)| x), 4);

        // 第2要素をキーに
        assert_eq!(v.upper_bound_by_key(&25, |&(_, y)| y), 2);
    }

    #[test]
    fn test_bounds_count_property() {
        // upper_bound(x) - lower_bound(x) == x の出現回数
        let v = [1, 2, 2, 3, 3, 3, 4, 5, 5];
        for target in 0..=6 {
            let count = v.iter().filter(|&&y| y == target).count();
            let lb = v.lower_bound(&target);
            let ub = v.upper_bound(&target);
            assert_eq!(
                ub - lb,
                count,
                "target={target}: upper_bound - lower_bound should equal count"
            );
        }
    }

    #[test]
    fn test_bounds_by_consistent_with_ord() {
        // by / by_key 系が通常版と一致することを検証
        let mut rng = get_test_rng();
        for _ in 0..100 {
            let n = rng.random_range(0..=100);
            let mut v: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
            v.sort_unstable();
            let target: i32 = rng.random_range(-55..=55);

            let lb = v.lower_bound(&target);
            let ub = v.upper_bound(&target);

            assert_eq!(
                v.lower_bound_by(|y| y.cmp(&target)),
                lb,
                "lower_bound_by: v.len()={n}, target={target}"
            );
            assert_eq!(
                v.upper_bound_by(|y| y.cmp(&target)),
                ub,
                "upper_bound_by: v.len()={n}, target={target}"
            );
            assert_eq!(
                v.lower_bound_by_key(&target, |&x| x),
                lb,
                "lower_bound_by_key: v.len()={n}, target={target}"
            );
            assert_eq!(
                v.upper_bound_by_key(&target, |&x| x),
                ub,
                "upper_bound_by_key: v.len()={n}, target={target}"
            );
        }
    }

    #[test]
    fn test_bounds_exhaustive_small() {
        // 長さ 0..=8, 値域 0..=4 の全ソート済み配列で全ターゲット検証
        let mut rng = get_test_rng();
        for n in 0..=8 {
            for _ in 0..100 {
                let mut v: Vec<i32> = (0..n).map(|_| rng.random_range(0..=4)).collect();
                v.sort_unstable();
                for target in 0..=5 {
                    let lb = v.lower_bound(&target);
                    let ub = v.upper_bound(&target);
                    let expected_lb = naive_lower_bound(&v, &target);
                    let expected_ub = naive_upper_bound(&v, &target);
                    assert_eq!(lb, expected_lb, "lower_bound: v={v:?}, target={target}");
                    assert_eq!(ub, expected_ub, "upper_bound: v={v:?}, target={target}");
                }
            }
        }
    }

    #[test]
    fn test_bounds_random() {
        macro_rules! test_random_bounds {
            ($name:ident, $ty:ident, $mn:expr, $mx:expr) => {
                fn $name(rng: &mut impl Rng) {
                    for _ in 0..100 {
                        let n = rng.random_range(0..=1000);
                        let mut v: Vec<$ty> = (0..n).map(|_| rng.random_range($mn..=$mx)).collect();
                        v.sort_unstable();
                        let target: $ty = rng.random_range($mn..=$mx);
                        assert_eq!(
                            v.lower_bound(&target),
                            naive_lower_bound(&v, &target),
                            "lower_bound failed: n={n}, target={target}"
                        );
                        assert_eq!(
                            v.upper_bound(&target),
                            naive_upper_bound(&v, &target),
                            "upper_bound failed: n={n}, target={target}"
                        );
                    }
                }
            };
        }

        test_random_bounds!(test_i64, i64, -1000, 1000);
        test_random_bounds!(test_u64, u64, 0, 1000);
        test_random_bounds!(test_usize, u64, 0, 1000);

        let mut rng = get_test_rng();
        test_i64(&mut rng);
        test_u64(&mut rng);
        test_usize(&mut rng);
    }
}
