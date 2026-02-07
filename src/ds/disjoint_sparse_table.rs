//! Disjoint Sparse Table
//!
//! 静的なモノイド列の区間積を計算するデータ構造．
//!
//! # 計算量
//! - 構築(前計算): `O(N log N)`
//! - 区間積の取得: `O(1)`
//!
//! # 使用例
//! ```
//! use reprol::ds::disjoint_sparse_table::DisjointSparseTable;
//! use reprol::ops::op_min::OpMin;
//! let dst = DisjointSparseTable::<OpMin<i64>>::new(vec![3, 5, 4, 100, 1]);
//! assert_eq!(dst.fold(1..4), 4); // 区間`[1, 4)`の最小値
//! assert_eq!(dst.fold(0..5), 1); // 区間`[0, 5)`の最小値
//! ```
//!
//! # Reference
//! - [Disjoint Sparse Table と セグ木に関するポエム - noshi91のメモ](https://noshi91.hatenablog.com/entry/2018/05/08/183946)

use std::{
    iter::FromIterator,
    ops::{Range, RangeBounds},
};

use crate::{ops::monoid::Monoid, utils::range_utils::to_half_open_index_range};

pub struct DisjointSparseTable<O: Monoid> {
    len: usize,
    data: Vec<Vec<O::Element>>,
    op: O,
}

impl<O: Monoid> DisjointSparseTable<O> {
    /// 配列`v`からDisjoint Sparse Tableを構築する．
    pub fn new(v: Vec<O::Element>) -> Self
    where
        O: Default,
    {
        Self::with_op(v, O::default())
    }

    /// 演算`op`を指定して，配列`v`からDisjoint Sparse Tableを構築する．
    pub fn with_op(v: Vec<O::Element>, op: O) -> Self {
        assert!(!v.is_empty());

        let n = v.len() + 2;
        let h = n.next_power_of_two().trailing_zeros() as usize;

        let mut data = Vec::with_capacity(h);
        data.push((0..n).map(|_| op.id()).collect());

        for w in (1..h).map(|k| 1 << k) {
            let mut datum = (0..n).map(|_| op.id()).collect::<Vec<_>>();

            for i in (w..n).step_by(w * 2) {
                for j in (i - w + 1..i).rev() {
                    datum[j - 1] = op.op(&v[j - 1], &datum[j]);
                }
                for j in i..(i + w).min(n) - 1 {
                    datum[j + 1] = op.op(&datum[j], &v[j - 1]);
                }
            }

            data.push(datum);
        }

        Self {
            len: v.len(),
            data,
            op,
        }
    }

    /// 指定したindexの値を返す．
    pub fn get(&self, index: usize) -> O::Element {
        self.fold(index..=index)
    }

    /// 区間`[l, r)`の区間積を返す．
    pub fn fold(&self, range: impl RangeBounds<usize>) -> O::Element {
        let Range {
            start: l,
            end: mut r,
        } = to_half_open_index_range(range, self.len);
        assert!(l <= r);
        assert!(r <= self.len);
        r += 1;
        let i = ((l ^ r) + 1).next_power_of_two().trailing_zeros() - 1;
        let datum = &self.data[i as usize];
        self.op.op(&datum[l], &datum[r])
    }
}

impl<O: Monoid> From<(Vec<O::Element>, O)> for DisjointSparseTable<O> {
    fn from((v, op): (Vec<O::Element>, O)) -> Self {
        Self::with_op(v, op)
    }
}

impl<O: Monoid, const N: usize> From<([O::Element; N], O)> for DisjointSparseTable<O> {
    fn from((v, op): ([O::Element; N], O)) -> Self {
        Self::with_op(v.into_iter().collect(), op)
    }
}

impl<O: Monoid + Default> From<Vec<O::Element>> for DisjointSparseTable<O> {
    fn from(v: Vec<O::Element>) -> Self {
        Self::new(v)
    }
}

impl<O: Monoid + Default, const N: usize> From<[O::Element; N]> for DisjointSparseTable<O> {
    fn from(v: [O::Element; N]) -> Self {
        Self::new(v.into_iter().collect())
    }
}

impl<O: Monoid + Default> FromIterator<O::Element> for DisjointSparseTable<O> {
    fn from_iter<I: IntoIterator<Item = O::Element>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    use crate::{
        ops::{op_add::OpAdd, op_max::OpMax, op_min::OpMin},
        utils::test_utils::{random::get_test_rng, static_range_query::*},
    };

    #[test]
    fn test_min() {
        let v = vec![2, 10, 1, 100];
        let test_cases = vec![
            ([0, 1], 2),
            ([0, 2], 2),
            ([0, 3], 1),
            ([0, 4], 1),
            ([1, 2], 10),
            ([1, 3], 1),
            ([1, 4], 1),
            ([2, 3], 1),
            ([2, 4], 1),
            ([3, 4], 100),
        ];

        let dst = DisjointSparseTable::<OpMin<i64>>::new(v);
        for ([l, r], expected) in test_cases {
            assert_eq!(dst.fold(l..r), expected);
        }
    }

    macro_rules! random_sum_test {
        ($test_name: ident, $ty: ty, $range: expr) => {
            randomized_static_range_sum_exhaustive_test!(
                $test_name,
                $ty,
                |v| DisjointSparseTable::<OpAdd<$ty>>::from(v),
                |ds: &DisjointSparseTable<_>, range| ds.fold(range),
                100,
                100,
                $range
            );
        };
    }

    macro_rules! random_min_max_test {
        ($min_test_name: ident, $max_test_name: ident, $ty: ty) => {
            randomized_static_range_min_exhaustive_test!(
                $min_test_name,
                $ty,
                |v| DisjointSparseTable::<OpMin<$ty>>::from(v),
                |ds: &DisjointSparseTable<_>, range| ds.fold(range),
                100,
                100
            );

            randomized_static_range_max_exhaustive_test!(
                $max_test_name,
                $ty,
                |v| DisjointSparseTable::<OpMax<$ty>>::from(v),
                |ds: &DisjointSparseTable<_>, range| ds.fold(range),
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

    random_min_max_test!(test_random_min_i32, test_random_max_i32, i32);
    random_min_max_test!(test_random_min_u32, test_random_max_u32, u32);
    random_min_max_test!(test_random_min_i64, test_random_max_i64, i64);
    random_min_max_test!(test_random_min_u64, test_random_max_u64, u64);
}
