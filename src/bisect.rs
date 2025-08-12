//! 二分探索(Binary Search)
//!
//! - [`Bisect`] : 整数範囲上の二分探索
//! - [`Bounds`] : ソート済み配列上のlower_bound/upper_bound
//!
//! # 使用例
//! ## [`Bisect`]
//! ```
//! use reprol::bisect::Bisect;
//! let result = (1..=100).bisect(|&x| x * x < 30);
//! assert_eq!(result, 6);
//! ```
//!
//! ## [`Bounds`]
//! ```
//! use reprol::bisect::Bounds;
//! let v = [1, 3, 3, 5, 7];
//! assert_eq!(v.lower_bound(&0), 0);
//! assert_eq!(v.lower_bound(&3), 1);
//! assert_eq!(v.lower_bound(&4), 3);
//! assert_eq!(v.lower_bound(&8), 5);
//! ```

use std::{
    cmp::Ordering,
    ops::{Range, RangeInclusive},
};

/// 整数範囲に対して，二分探索を行うためのトレイト．
pub trait Bisect {
    /// 探索対象の整数の型．
    type Item;

    /// 単調性のある関数`f`に対して，
    /// 範囲内の整数`x`であって，`f(x)`が`false`となる最小の$x$を返す．
    /// ただし，すべての`x`に対して`f(x)`が`true`である場合は，範囲の上限を返す．
    fn bisect(&self, f: impl FnMut(&Self::Item) -> bool) -> Self::Item;
}

macro_rules! impl_bisect {
    ($ty: ty) => {
        impl Bisect for Range<$ty> {
            type Item = $ty;
            fn bisect(&self, mut f: impl FnMut(&Self::Item) -> bool) -> Self::Item {
                assert!(self.start < self.end);

                let Range {
                    start: mut ok,
                    end: mut ng,
                } = *self;

                if !f(&ok) {
                    return ok;
                }

                while ng > ok + 1 {
                    let mid = ok + (ng - ok) / 2;
                    *if f(&mid) { &mut ok } else { &mut ng } = mid;
                }

                ng
            }
        }

        impl Bisect for RangeInclusive<$ty> {
            type Item = $ty;
            fn bisect(&self, f: impl FnMut(&Self::Item) -> bool) -> Self::Item {
                let l = *self.start();
                let u = *self.end();
                assert!(l <= u);
                assert!(u < <$ty>::MAX);
                (l..u + 1).bisect(f)
            }
        }
    };
}

macro_rules! impl_bisect_for {
    ($($ty: ty),* $(,)?) => {
        $( impl_bisect!($ty); )*
    };
}

impl_bisect_for! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

/// ソート済み配列に対して，境界探索を行うためのトレイト．
pub trait Bounds {
    /// 配列の要素の型．
    type Item: Ord;

    /// `x`以上の最小の要素のインデックスを返す．
    /// なければ配列サイズを返す．
    fn lower_bound(&self, x: &Self::Item) -> usize;

    /// 条件関数`f` に対して，`f(x)`が`Ordering::Less`となる最小の要素のインデックスを返す．
    /// なければ配列サイズを返す．
    fn lower_bound_by(&self, f: impl FnMut(&Self::Item) -> Ordering) -> usize;

    /// 配列の要素`x`に対して，`f(x) < k`となる最小のインデックスを返す．
    /// なければ配列サイズを返す．
    fn lower_bound_by_key<K: Ord>(&self, k: &K, f: impl FnMut(&Self::Item) -> K) -> usize;

    /// `x`より大きい最小の要素のインデックスを返す．
    /// なければ配列サイズを返す．
    fn upper_bound(&self, x: &Self::Item) -> usize;

    /// 条件関数`f` に対して，`f(x) != Ordering::Greater`となる最小の要素のインデックスを返す．
    /// なければ配列サイズを返す．
    fn upper_bound_by(&self, f: impl FnMut(&Self::Item) -> Ordering) -> usize;

    /// 配列の要素`x`に対して，`f(x) <= k`となる最小のインデックスを返す．
    /// なければ配列サイズを返す．
    fn upper_bound_by_key<K: Ord>(&self, k: &K, f: impl FnMut(&Self::Item) -> K) -> usize;
}

impl<T: Ord> Bounds for [T] {
    type Item = T;

    fn lower_bound(&self, x: &Self::Item) -> usize {
        self.lower_bound_by(|y| y.cmp(x))
    }

    fn lower_bound_by(&self, mut f: impl FnMut(&Self::Item) -> Ordering) -> usize {
        if self.is_empty() {
            return 0;
        }
        (0..self.len()).bisect(|&i| f(&self[i]) == Ordering::Less)
    }

    fn lower_bound_by_key<K: Ord>(&self, k: &K, mut f: impl FnMut(&Self::Item) -> K) -> usize {
        self.lower_bound_by(|x| f(x).cmp(k))
    }

    fn upper_bound(&self, x: &Self::Item) -> usize {
        self.upper_bound_by(|y| y.cmp(x))
    }

    fn upper_bound_by(&self, mut f: impl FnMut(&Self::Item) -> Ordering) -> usize {
        if self.is_empty() {
            return 0;
        }
        (0..self.len()).bisect(|&i| f(&self[i]) != Ordering::Greater)
    }

    fn upper_bound_by_key<K: Ord>(&self, k: &K, mut f: impl FnMut(&Self::Item) -> K) -> usize {
        self.upper_bound_by(|x| f(x).cmp(k))
    }
}

#[cfg(test)]
mod tests {
    use rand::{rngs::StdRng, Rng, SeedableRng};

    use super::*;

    #[test]
    fn test_lower_bound() {
        let v = vec![1, 3, 3, 5, 7, 9, 9, 9, 11, 13];
        assert_eq!(v.lower_bound(&0), 0);
        assert_eq!(v.lower_bound(&3), 1);
        assert_eq!(v.lower_bound(&9), 5);
        assert_eq!(v.lower_bound(&10), 8);
        assert_eq!(v.lower_bound(&13), 9);
        assert_eq!(v.lower_bound(&14), 10);

        let v: Vec<i32> = vec![];
        assert_eq!(v.lower_bound(&5), 0);

        let v = vec![10];
        assert_eq!(v.lower_bound(&5), 0);
        assert_eq!(v.lower_bound(&10), 0);
        assert_eq!(v.lower_bound(&15), 1);

        let v = vec![4, 4, 4, 4, 4];
        assert_eq!(v.lower_bound(&4), 0);
        assert_eq!(v.lower_bound(&3), 0);
        assert_eq!(v.lower_bound(&5), 5);

        let v = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        assert_eq!(v.lower_bound(&5), 2);
        assert_eq!(v.lower_bound(&6), 3);
        assert_eq!(v.lower_bound(&1), 0);
        assert_eq!(v.lower_bound(&19), 9);
        assert_eq!(v.lower_bound(&0), 0);
        assert_eq!(v.lower_bound(&20), 10);

        let v = vec![
            2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40,
        ];
        assert_eq!(v.lower_bound(&10), 4);
        assert_eq!(v.lower_bound(&25), 12);
        assert_eq!(v.lower_bound(&0), 0);
        assert_eq!(v.lower_bound(&40), 19);
        assert_eq!(v.lower_bound(&41), 20);
        assert_eq!(v.lower_bound(&15), 7);
        assert_eq!(v.lower_bound(&5), 2);
    }

    #[test]
    fn test_upper_bound() {
        let v = vec![1, 3, 3, 5, 7, 9, 9, 9, 11, 13];
        assert_eq!(v.upper_bound(&0), 0);
        assert_eq!(v.upper_bound(&3), 3);
        assert_eq!(v.upper_bound(&9), 8);
        assert_eq!(v.upper_bound(&10), 8);
        assert_eq!(v.upper_bound(&13), 10);
        assert_eq!(v.upper_bound(&14), 10);

        let v: Vec<i32> = vec![];
        assert_eq!(v.upper_bound(&5), 0);

        let v = vec![10];
        assert_eq!(v.upper_bound(&5), 0);
        assert_eq!(v.upper_bound(&10), 1);
        assert_eq!(v.upper_bound(&15), 1);

        let v = vec![4, 4, 4, 4, 4];
        assert_eq!(v.upper_bound(&4), 5);
        assert_eq!(v.upper_bound(&3), 0);
        assert_eq!(v.upper_bound(&5), 5);
    }

    #[test]
    fn test_lower_bound_by_key() {
        let v: Vec<(i32, i32)> = vec![];
        assert_eq!(v.lower_bound_by_key(&10, |&(x, _)| x), 0);

        let v = vec![(5, 100)];
        assert_eq!(v.lower_bound_by_key(&5, |&(x, _)| x), 0);
        assert_eq!(v.lower_bound_by_key(&3, |&(x, _)| x), 0);
        assert_eq!(v.lower_bound_by_key(&10, |&(x, _)| x), 1);

        let v = vec![(1, 10), (3, 20), (5, 30), (7, 40)];
        assert_eq!(v.lower_bound_by_key(&0, |&(x, _)| x), 0);
        assert_eq!(v.lower_bound_by_key(&1, |&(x, _)| x), 0);
        assert_eq!(v.lower_bound_by_key(&2, |&(x, _)| x), 1);
        assert_eq!(v.lower_bound_by_key(&3, |&(x, _)| x), 1);
        assert_eq!(v.lower_bound_by_key(&6, |&(x, _)| x), 3);
        assert_eq!(v.lower_bound_by_key(&7, |&(x, _)| x), 3);
        assert_eq!(v.lower_bound_by_key(&8, |&(x, _)| x), 4);

        let v = vec![(2, 10), (2, 20), (2, 30), (4, 40), (6, 50)];
        assert_eq!(v.lower_bound_by_key(&2, |&(x, _)| x), 0);
        assert_eq!(v.lower_bound_by_key(&3, |&(x, _)| x), 3);
        assert_eq!(v.lower_bound_by_key(&4, |&(x, _)| x), 3);
        assert_eq!(v.lower_bound_by_key(&5, |&(x, _)| x), 4);
        assert_eq!(v.lower_bound_by_key(&20, |&(_, y)| y), 1);
        assert_eq!(v.lower_bound_by_key(&25, |&(_, y)| y), 2);
        assert_eq!(v.lower_bound_by_key(&30, |&(_, y)| y), 2);
        assert_eq!(v.lower_bound_by_key(&35, |&(_, y)| y), 3);
    }

    #[test]
    fn test_upper_bound_by_key() {
        let v: Vec<(i32, i32)> = vec![];
        assert_eq!(v.upper_bound_by_key(&10, |&(x, _)| x), 0);

        let v = vec![(5, 100)];
        assert_eq!(v.upper_bound_by_key(&5, |&(x, _)| x), 1);
        assert_eq!(v.upper_bound_by_key(&3, |&(x, _)| x), 0);
        assert_eq!(v.upper_bound_by_key(&10, |&(x, _)| x), 1);

        let v = vec![(1, 10), (3, 20), (5, 30), (7, 40)];
        assert_eq!(v.upper_bound_by_key(&0, |&(x, _)| x), 0);
        assert_eq!(v.upper_bound_by_key(&1, |&(x, _)| x), 1);
        assert_eq!(v.upper_bound_by_key(&2, |&(x, _)| x), 1);
        assert_eq!(v.upper_bound_by_key(&3, |&(x, _)| x), 2);
        assert_eq!(v.upper_bound_by_key(&6, |&(x, _)| x), 3);
        assert_eq!(v.upper_bound_by_key(&7, |&(x, _)| x), 4);
        assert_eq!(v.upper_bound_by_key(&8, |&(x, _)| x), 4);

        let v = vec![(2, 10), (2, 20), (2, 30), (4, 40), (6, 50)];
        assert_eq!(v.upper_bound_by_key(&2, |&(x, _)| x), 3);
        assert_eq!(v.upper_bound_by_key(&3, |&(x, _)| x), 3);
        assert_eq!(v.upper_bound_by_key(&4, |&(x, _)| x), 4);
        assert_eq!(v.upper_bound_by_key(&5, |&(x, _)| x), 4);
        assert_eq!(v.upper_bound_by_key(&20, |&(_, y)| y), 2);
        assert_eq!(v.upper_bound_by_key(&25, |&(_, y)| y), 2);
        assert_eq!(v.upper_bound_by_key(&30, |&(_, y)| y), 3);
        assert_eq!(v.upper_bound_by_key(&35, |&(_, y)| y), 3);
    }

    #[test]
    fn test_bounds_random() {
        fn naive_lower_bound<T: Ord>(v: &[T], x: &T) -> usize {
            for i in 0..v.len() {
                if &v[i] >= x {
                    return i;
                }
            }
            v.len()
        }

        fn naive_upper_bound<T: Ord>(v: &[T], x: &T) -> usize {
            for i in 0..v.len() {
                if &v[i] > x {
                    return i;
                }
            }
            v.len()
        }

        macro_rules! define_test_function {
            ($name:ident, $ty:ident) => {
                fn $name(rng: &mut StdRng, mn: $ty, mx: $ty) {
                    const T: usize = 100;
                    const N: usize = 1000;
                    for _ in 0..T {
                        let mut v = (0..N).map(|_| rng.gen_range(mn..=mx)).collect::<Vec<_>>();
                        v.sort_unstable();
                        let target = rng.gen_range(mn..=mx);
                        assert_eq!(v.lower_bound(&target), naive_lower_bound(&v, &target));
                        assert_eq!(v.upper_bound(&target), naive_upper_bound(&v, &target));
                    }
                }
            };
        }

        define_test_function!(test_i64, i64);
        define_test_function!(test_u64, u64);

        let mut rng = StdRng::seed_from_u64(30);

        test_i64(&mut rng, -1000, 1000);
        test_u64(&mut rng, 0, 1000);
    }
}
