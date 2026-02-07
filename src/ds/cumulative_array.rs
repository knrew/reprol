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
//! assert_eq!(*cum.prefix(3), 6); // [0, 3)の区間和
//! assert_eq!(cum.fold(1..4), 9); // [1, 4)の区間和
//! ```

//! ## 累積最小値
//! minは群ではないので`fold`は使えない．
//! ```
//! use reprol::{ds::cumulative_array::CumulativeArray, ops::op_min::OpMin};
//! let v = vec![3, 5, 4, 1, 5];
//! let cum = CumulativeArray::<OpMin<i32>>::new(v);
//! assert_eq!(*cum.prefix(3), 3); // [0, 3)の最小値
//! assert_eq!(*cum.prefix(4), 1); // [0, 4)の最小値
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
//!    type Element = u32;
//!    // 単位元
//!    fn id(&self) -> Self::Element {
//!        0
//!    }
//!    // 演算
//!    fn op(&self, x: &Self::Element, y: &Self::Element) -> Self::Element {
//!        x ^ y
//!    }
//! }
//! impl Group for Op {
//!     // 逆元
//!    fn inv(&self, &x: &Self::Element) -> Self::Element {
//!        x
//!    }
//! }
//! let v = vec![3, 1, 4, 1, 5, 9, 2];
//! let cum = CumulativeArray::<Op>::new(v);
//! assert_eq!(*cum.prefix(3), 6); // [0, 3)の区間xor
//! assert_eq!(cum.fold(1..5), 1); // [1, 5)の区間xor
//!```

use std::{
    iter::FromIterator,
    ops::{Range, RangeBounds},
};

use crate::{
    ops::{group::Group, monoid::Monoid, op_add::OpAdd},
    utils::range_utils::to_half_open_index_range,
};

/// 累積積を管理するデータ構造
pub struct CumulativeArray<O: Monoid> {
    inner: Vec<O::Element>,
    op: O,
}

impl<O: Monoid> CumulativeArray<O> {
    /// 配列の累積配列を構築する．
    pub fn new(v: Vec<O::Element>) -> Self
    where
        O: Default,
    {
        Self::with_op(v, O::default())
    }

    /// 演算`op`を明示的に渡して配列の累積配列を構築する．
    pub fn with_op(v: Vec<O::Element>, op: O) -> Self {
        assert!(!v.is_empty());
        let mut inner = Vec::with_capacity(v.len() + 1);
        inner.push(op.id());
        for i in 0..v.len() {
            inner.push(op.op(&inner[i], &v[i]));
        }
        Self { inner, op }
    }

    /// 区間`[0, r)`の区間積を返す．
    pub fn prefix(&self, r: usize) -> &O::Element {
        &self.inner[r]
    }

    /// `index`番目の要素の値を返す．
    pub fn get(&self, index: usize) -> O::Element
    where
        O: Group,
    {
        self.fold(index..=index)
    }

    /// `[l, r)`の区間積を返す．
    pub fn fold(&self, range: impl RangeBounds<usize>) -> O::Element
    where
        O: Group,
    {
        let Range { start: l, end: r } = to_half_open_index_range(range, self.inner.len() - 1);
        assert!(l <= r);
        self.op.op(&self.inner[r], &self.op.inv(&self.inner[l]))
    }
}

impl<O: Monoid> From<(Vec<O::Element>, O)> for CumulativeArray<O> {
    fn from((v, op): (Vec<O::Element>, O)) -> Self {
        CumulativeArray::with_op(v, op)
    }
}

impl<O: Monoid, const N: usize> From<([O::Element; N], O)> for CumulativeArray<O> {
    fn from((v, op): ([O::Element; N], O)) -> Self {
        CumulativeArray::with_op(v.into_iter().collect(), op)
    }
}

impl<O: Monoid + Default> From<Vec<O::Element>> for CumulativeArray<O> {
    fn from(v: Vec<O::Element>) -> Self {
        CumulativeArray::from((v, O::default()))
    }
}

impl<O: Monoid + Default, const N: usize> From<[O::Element; N]> for CumulativeArray<O> {
    fn from(v: [O::Element; N]) -> Self {
        CumulativeArray::from((v, O::default()))
    }
}

impl<O: Monoid + Default> FromIterator<O::Element> for CumulativeArray<O> {
    fn from_iter<I: IntoIterator<Item = O::Element>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

/// 累積和
pub type CumulativeSum<T> = CumulativeArray<OpAdd<T>>;

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    use crate::{
        ops::{op_max::OpMax, op_min::OpMin, op_xor::OpXor},
        utils::test_utils::{random::get_test_rng, static_range_query::*},
    };

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
        assert_eq!(cum.prefix(5), &15);
        for ((l, r), expected) in testcases {
            assert_eq!(cum.fold(l..r), expected);
        }

        let cum = CumulativeSum::from_iter((0..5).map(|i| i as i64 + 1));
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
            assert_eq!(cum.prefix(r), &expected);
        }
    }

    macro_rules! random_sum_test {
        ($test_name: ident, $ty: ty, $range: expr) => {
            randomized_static_range_sum_exhaustive_test!(
                $test_name,
                $ty,
                |v| CumulativeSum::<$ty>::from(v),
                |ds: &CumulativeSum<_>, range| ds.fold(range),
                200,
                100,
                $range
            );
        };
    }

    macro_rules! random_xor_test {
        ($test_name: ident, $ty: ty) => {
            randomized_static_range_xor_exhaustive_test!(
                $test_name,
                $ty,
                |v| CumulativeArray::<OpXor<$ty>>::from(v),
                |ds: &CumulativeArray<_>, range| ds.fold(range),
                100,
                100
            );
        };
    }

    random_sum_test!(test_random_sum_i32, i32, -100000..=100000);
    random_sum_test!(test_random_sum_u32, u32, 0..=100000);
    random_sum_test!(test_random_sum_i64, i64, -1000000000..=1000000000);
    random_sum_test!(test_random_sum_u64, u64, 0..=1000000000);
    random_sum_test!(test_random_sum_usize, usize, 0..=1000000000);

    random_xor_test!(test_random_xor_i32, i32);
    random_xor_test!(test_random_xor_u32, u32);
    random_xor_test!(test_random_xor_i64, i64);
    random_xor_test!(test_random_xor_u64, u64);
    random_xor_test!(test_random_xor_usize, usize);

    macro_rules! random_prefix_test {
        (
            $test_name: ident,
            $ty: ty,
            $ds_op_monoid: ty,
            $fold_op: expr,
            $fold_id: expr,
            $num_testcases: expr,
            $num_elements_max: expr
        ) => {
            #[test]
            fn $test_name() {
                let mut rng = get_test_rng();
                for _ in 0..$num_testcases {
                    let n = rng.random_range(1..=$num_testcases);
                    let v = (0..n).map(|_| rng.random()).collect::<Vec<_>>();
                    let cum = CumulativeArray::<$ds_op_monoid>::new(v.clone());
                    for r in 1..=v.len() {
                        let naive = v[..r].iter().fold($fold_id, |prod, &e| $fold_op(prod, e));
                        assert_eq!(cum.prefix(r), &naive);
                    }
                }
            }
        };
    }

    macro_rules! random_prefix_min_max_test {
        ($min_test_name: ident, $max_test_name: ident, $ty: ty) => {
            random_prefix_test!(
                $min_test_name,
                $ty,
                OpMin<$ty>,
                |a: $ty, b| a.min(b),
                <$ty>::MAX,
                100,
                100
            );

            random_prefix_test!(
                $max_test_name,
                $ty,
                OpMax<$ty>,
                |a: $ty, b| a.max(b),
                <$ty>::MIN,
                100,
                100
            );
        };
    }

    random_prefix_min_max_test!(test_random_prefix_min_i32, test_random_prefix_max_i32, i32);
    random_prefix_min_max_test!(test_random_prefix_min_u32, test_random_prefix_max_u32, u32);
    random_prefix_min_max_test!(test_random_prefix_min_i64, test_random_prefix_max_i64, i64);
    random_prefix_min_max_test!(test_random_prefix_min_u64, test_random_prefix_max_u64, u64);
}
