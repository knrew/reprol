//! 累積積(累積和)
//!
//! モノイドの配列に対する累積積を管理するデータ構造．
//! 群に対しては区間積[`fold`](CumulativeArray::fold)も提供する．
//!
//! # 使用例
//! ## 累積和
//! ```
//! use reprol::ds::cumulative_array::CumulativeSum;
//! let v = vec![1, 2, 3, 4, 5];
//! let cum = CumulativeSum::new(v);
//! assert_eq!(cum[3], 6); // [0, 3)の区間和
//! assert_eq!(cum.fold(1..4), 9); // [1, 4)の区間和
//! ```

//! ## 累積最小値
//! minは群ではないので`fold`は使えない．
//! ```
//! use reprol::{ds::cumulative_array::CumulativeArray, ops::op_min::OpMin};
//! let v = vec![3, 5, 4, 1, 5];
//! let cum = CumulativeArray::<OpMin<i32>>::new(v);
//! assert_eq!(cum[3], 3); // [0, 3)の最小値
//! assert_eq!(cum[4], 1); // [0, 4)の最小値
//! ```
//!
//! ## 演算を定義する
//! 演算(モノイドや群)を自分で定義して区間積を計算できる．
//! 排他的論理和(xor)を定義した場合の例を示す．
//! ```
//! use reprol::{
//!    ds::cumulative_array::CumulativeArray,
//!    ops::{group::Group, monoid::Monoid},
//! };
//! #[derive(Default)]
//! struct Op;
//! impl Monoid for Op {
//!    type Value = u32;
//!    // 単位元
//!    fn identity(&self) -> Self::Value {
//!        0
//!    }
//!    // 演算
//!    fn op(&self, x: &Self::Value, y: &Self::Value) -> Self::Value {
//!        x ^ y
//!    }
//! }
//! impl Group for Op {
//!     // 逆元
//!    fn inv(&self, &x: &Self::Value) -> Self::Value {
//!        x
//!    }
//! }
//! let v = vec![3, 1, 4, 1, 5, 9, 2];
//! let cum = CumulativeArray::<Op>::new(v);
//! assert_eq!(cum[3], 6); // [0, 3)の区間xor
//! assert_eq!(cum.fold(1..5), 1); // [1, 5)の区間xor
//!```

use std::{
    fmt::Debug,
    iter::FromIterator,
    ops::{Index, Range, RangeBounds},
};

use crate::{
    ops::{group::Group, monoid::Monoid, op_add::OpAdd},
    range::to_open_range,
};

/// 累積積を管理するデータ構造
pub struct CumulativeArray<O: Monoid> {
    data: Vec<O::Value>,
    op: O,
}

impl<O: Monoid> CumulativeArray<O> {
    /// 配列の累積配列を構築する．
    pub fn new(v: Vec<O::Value>) -> Self
    where
        O: Default,
    {
        Self::with_op(v, O::default())
    }

    /// `i`番目の要素が`f(i)`であるような累積配列を構築する．
    pub fn construct(len: usize, f: impl FnMut(usize) -> O::Value) -> Self
    where
        O: Default,
    {
        Self::construct_with_op(len, O::default(), f)
    }

    /// 演算`op`を明示的に渡して配列の累積配列を構築する．
    pub fn with_op(v: Vec<O::Value>, op: O) -> Self {
        assert!(!v.is_empty());
        let mut data = Vec::with_capacity(v.len() + 1);
        data.push(op.identity());
        for i in 0..v.len() {
            data.push(op.op(&data[i], &v[i]));
        }
        Self { data, op }
    }

    /// 演算`op`を明示的に渡して`i`番目の要素が`f(i)`であるような累積配列を構築する．
    pub fn construct_with_op(len: usize, op: O, mut f: impl FnMut(usize) -> O::Value) -> Self {
        assert!(len > 0);
        Self::with_op((0..len).map(|i| f(i)).collect(), op)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 累積配列の`r`番目の要素を返す(区間`[0, r)`の区間積を返す)．
    pub fn get(&self, r: usize) -> &O::Value {
        &self.data[r]
    }

    /// `[l, r)`の区間積を返す．
    pub fn fold(&self, range: impl RangeBounds<usize>) -> O::Value
    where
        O: Group,
    {
        let Range { start: l, end: r } = to_open_range(range, self.data.len() - 1);
        assert!(l <= r);
        self.op.op(&self.data[r], &self.op.inv(&self.data[l]))
    }

    pub fn iter(&self) -> impl Iterator<Item = &O::Value> {
        self.data.iter()
    }
}

impl<O: Monoid> From<(Vec<O::Value>, O)> for CumulativeArray<O> {
    fn from((v, op): (Vec<O::Value>, O)) -> Self {
        CumulativeArray::with_op(v, op)
    }
}

impl<O: Monoid, const N: usize> From<([O::Value; N], O)> for CumulativeArray<O> {
    fn from((v, op): ([O::Value; N], O)) -> Self {
        CumulativeArray::with_op(v.into_iter().collect(), op)
    }
}

impl<O> From<Vec<O::Value>> for CumulativeArray<O>
where
    O: Monoid + Default,
{
    fn from(v: Vec<O::Value>) -> Self {
        CumulativeArray::from((v, O::default()))
    }
}

impl<O, const N: usize> From<[O::Value; N]> for CumulativeArray<O>
where
    O: Monoid + Default,
{
    fn from(v: [O::Value; N]) -> Self {
        CumulativeArray::from((v, O::default()))
    }
}

impl<O> FromIterator<O::Value> for CumulativeArray<O>
where
    O: Monoid + Default,
{
    fn from_iter<I: IntoIterator<Item = O::Value>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl<O> Clone for CumulativeArray<O>
where
    O: Monoid + Clone,
    O::Value: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            op: self.op.clone(),
        }
    }
}

impl<O: Monoid> Index<usize> for CumulativeArray<O> {
    type Output = O::Value;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<'a, O: Monoid> IntoIterator for &'a CumulativeArray<O> {
    type IntoIter = std::slice::Iter<'a, O::Value>;
    type Item = &'a O::Value;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<O: Monoid> IntoIterator for CumulativeArray<O> {
    type IntoIter = std::vec::IntoIter<O::Value>;
    type Item = O::Value;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<O> Debug for CumulativeArray<O>
where
    O: Monoid,
    O::Value: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.data.iter()).finish()
    }
}

/// 累積和
pub type CumulativeSum<T> = CumulativeArray<OpAdd<T>>;

#[cfg(test)]
mod tests {
    use rand::{rngs::StdRng, Rng, SeedableRng};

    use crate::ops::{op_max::OpMax, op_min::OpMin};

    use super::*;

    #[test]
    fn test_cumulative_sum() {
        let v = vec![1, 2, 3, 4, 5];
        let testcases = vec![
            ((0, 5), 15),
            ((0, 1), 1),
            ((1, 3), 5),
            ((3, 5), 9),
            ((2, 4), 7),
            ((4, 5), 5),
            ((0, 4), 10),
        ];
        let cum = CumulativeSum::<i64>::from(v);
        assert_eq!(cum.fold(..), 15);
        assert_eq!(cum.get(5), &15);
        for ((l, r), expected) in testcases {
            assert_eq!(cum.fold(l..r), expected);
        }

        let cum = CumulativeSum::construct(5, |i| i as i64 + 1);
        let testcases = vec![
            ((0, 5), 15),
            ((0, 1), 1),
            ((1, 3), 5),
            ((3, 5), 9),
            ((2, 4), 7),
            ((4, 5), 5),
            ((0, 4), 10),
        ];
        for ((l, r), expected) in testcases {
            assert_eq!(cum.fold(l..r), expected);
        }
    }

    #[test]
    fn test_cumulative_min() {
        let v = vec![8, 10, -4, 2, 11];
        let testcases = vec![(1, 8), (2, 8), (3, -4), (4, -4), (5, -4)];
        let cum = CumulativeArray::<OpMin<i32>>::new(v);
        for (r, expected) in testcases {
            assert_eq!(cum.get(r), &expected);
        }
    }

    #[test]
    fn test_sum_random() {
        macro_rules! define_test_function {
            ($name:ident, $ty:ident) => {
                fn $name(rng: &mut StdRng, mn: $ty, mx: $ty) {
                    const T: usize = 100;
                    const N: usize = 100;
                    for _ in 0..T {
                        let v = (0..N).map(|_| rng.gen_range(mn..=mx)).collect::<Vec<_>>();
                        let cum = CumulativeSum::new(v.clone());
                        for l in 0..v.len() {
                            for r in l..=v.len() {
                                assert_eq!(cum.fold(l..r), v[l..r].iter().sum());
                            }
                        }
                    }
                }
            };
        }

        define_test_function!(test_i64, i64);
        define_test_function!(test_u64, u64);

        let mut rng = StdRng::seed_from_u64(30);

        test_i64(&mut rng, -1000000000, 1000000000);
        test_u64(&mut rng, 0, 1000000000);
    }

    #[test]
    fn test_min_random() {
        macro_rules! define_test_function {
            ($name:ident, $ty:ident) => {
                fn $name(rng: &mut StdRng) {
                    const T: usize = 100;
                    const N: usize = 100;

                    for _ in 0..T {
                        let v = (0..N).map(|_| rng.gen()).collect::<Vec<_>>();
                        let cum = CumulativeArray::<OpMin<_>>::new(v.clone());
                        for r in 0..=v.len() {
                            let naive = *v[..r].iter().min().unwrap_or(&$ty::MAX);
                            assert_eq!(cum[r], naive);
                        }
                    }
                }
            };
        }

        define_test_function!(test_i64, i64);
        define_test_function!(test_u64, u64);

        let mut rng = StdRng::seed_from_u64(30);
        test_i64(&mut rng);
        test_u64(&mut rng);
    }

    #[test]
    fn test_max_random() {
        macro_rules! define_test_function {
            ($name:ident, $ty:ident) => {
                fn $name(rng: &mut StdRng) {
                    const T: usize = 100;
                    const N: usize = 100;

                    for _ in 0..T {
                        let v = (0..N).map(|_| rng.gen()).collect::<Vec<_>>();
                        let cum = CumulativeArray::<OpMax<_>>::new(v.clone());
                        for r in 0..=v.len() {
                            let naive = *v[..r].iter().max().unwrap_or(&$ty::MIN);
                            assert_eq!(cum[r], naive);
                        }
                    }
                }
            };
        }

        define_test_function!(test_i64, i64);
        define_test_function!(test_u64, u64);

        let mut rng = StdRng::seed_from_u64(30);
        test_i64(&mut rng);
        test_u64(&mut rng);
    }
}
