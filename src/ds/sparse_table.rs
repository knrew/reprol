//! Sparse Table
//!
//! 静的な冪等モノイド列の区間積を計算するデータ構造．
//!
//! # 計算量
//! - 構築(前計算): `O(N log N)`
//! - 区間積の取得: `O(1)`
//!
//! # 使用例
//! ```
//! use reprol::ds::sparse_table::SparseTable;
//! use reprol::ops::op_min::OpMin;
//! let st = SparseTable::<OpMin<i64>>::new(vec![3, 5, 4, 100, 1]);
//! assert_eq!(st.fold(1..4), 4); // 区間`[1, 4)`の最小値
//! assert_eq!(st.fold(0..5), 1); // 区間`[0, 5)`の最小値
//! ```

use std::{
    iter::FromIterator,
    ops::{Range, RangeBounds},
};

use crate::{ops::monoid::IdempotentMonoid, utils::range_utils::to_half_open_index_range};

pub struct SparseTable<O: IdempotentMonoid> {
    len: usize,
    nodes: Vec<Vec<O::Element>>,
    op: O,
}

impl<O: IdempotentMonoid> SparseTable<O> {
    /// 配列`v`からSparse Tableを構築する．
    pub fn new(v: Vec<O::Element>) -> Self
    where
        O: Default,
    {
        Self::with_op(v, O::default())
    }

    /// 演算`op`を指定して，配列`v`からSparse Tableを構築する．
    pub fn with_op(v: Vec<O::Element>, op: O) -> Self {
        assert!(!v.is_empty());

        let len = v.len();
        let len_nodes = v.len().next_power_of_two().trailing_zeros() as usize + 1;

        let mut nodes = Vec::with_capacity(len_nodes);
        nodes.push(v);

        for i in 1..len_nodes {
            let node = (0..)
                .take_while(|j| j + (1 << i) <= len)
                .map(|j| op.op(&nodes[i - 1][j], &nodes[i - 1][j + (1 << (i - 1))]))
                .collect();
            nodes.push(node);
        }

        Self { len, nodes, op }
    }

    /// 指定したindexの値を返す．
    pub fn get(&self, index: usize) -> O::Element {
        self.fold(index..=index)
    }

    /// 区間`[l, r)`の区間積を返す．
    pub fn fold(&self, range: impl RangeBounds<usize>) -> O::Element {
        let Range { start: l, end: r } = to_half_open_index_range(range, self.len);
        assert!(l < r);
        let k = (r - l + 1).next_power_of_two().trailing_zeros() as usize - 1;
        self.op.op(&self.nodes[k][l], &self.nodes[k][r - (1 << k)])
    }

    pub fn inner(&self, i: usize, j: usize) -> &O::Element {
        &self.nodes[i][j]
    }
}

impl<O: IdempotentMonoid> From<(Vec<O::Element>, O)> for SparseTable<O> {
    fn from((v, op): (Vec<O::Element>, O)) -> Self {
        Self::with_op(v, op)
    }
}

impl<O: IdempotentMonoid, const N: usize> From<([O::Element; N], O)> for SparseTable<O> {
    fn from((v, op): ([O::Element; N], O)) -> Self {
        Self::with_op(v.into_iter().collect(), op)
    }
}

impl<O: IdempotentMonoid + Default> From<Vec<O::Element>> for SparseTable<O> {
    fn from(v: Vec<O::Element>) -> Self {
        Self::new(v)
    }
}

impl<O: IdempotentMonoid + Default, const N: usize> From<[O::Element; N]> for SparseTable<O> {
    fn from(v: [O::Element; N]) -> Self {
        Self::from((v, O::default()))
    }
}

impl<O: IdempotentMonoid + Default> FromIterator<O::Element> for SparseTable<O> {
    fn from_iter<T: IntoIterator<Item = O::Element>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::{
        math::gcd::Gcd,
        ops::{op_gcd::OpGcd, op_max::OpMax, op_min::OpMin},
        utils::test_utils::{random::get_test_rng, static_range_query::*},
    };

    use super::*;

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

        let st = SparseTable::<OpMin<i64>>::new(v);
        for ([l, r], expected) in test_cases {
            assert_eq!(st.fold(l..r), expected);
        }
    }

    macro_rules! random_min_max_gcd_test {
        ($min_test_name: ident, $max_test_name: ident, $gcd_test_name: ident, $ty: ty) => {
            randomized_static_range_min_exhaustive_test!(
                $min_test_name,
                $ty,
                |v| SparseTable::<OpMin<$ty>>::from(v),
                |ds: &SparseTable<_>, range| ds.fold(range),
                100,
                100
            );

            randomized_static_range_max_exhaustive_test!(
                $max_test_name,
                $ty,
                |v| SparseTable::<OpMax<$ty>>::from(v),
                |ds: &SparseTable<_>, range| ds.fold(range),
                100,
                100
            );

            randomized_static_range_gcd_exhaustive_test!(
                $gcd_test_name,
                $ty,
                |v| SparseTable::<OpGcd<$ty>>::from(v),
                |ds: &SparseTable<_>, range| ds.fold(range),
                100,
                100
            );
        };
    }

    random_min_max_gcd_test!(
        test_random_min_i32,
        test_random_max_i32,
        test_random_gcd_i32,
        i32
    );
    random_min_max_gcd_test!(
        test_random_min_u32,
        test_random_max_u32,
        test_random_gcd_u32,
        u32
    );
    random_min_max_gcd_test!(
        test_random_min_i64,
        test_random_max_i64,
        test_random_gcd_i64,
        i64
    );
    random_min_max_gcd_test!(
        test_random_min_u64,
        test_random_max_u64,
        test_random_gcd_u64,
        u64
    );
    random_min_max_gcd_test!(
        test_random_min_usize,
        test_random_max_usize,
        test_random_gcd_usize,
        usize
    );
}
