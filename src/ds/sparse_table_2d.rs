//! 2次元Sparse Table
//!
//! 静的な冪等モノイド列の2次元配列の長方形区間積を計算するデータ構造．
//!
//! # 計算量
//! - 構築(前計算): `O(H log H × W log W)` ただし H, W は配列の高さ・幅
//! - 区間積の取得: `O(1)`
//!
//! # 使用例
//! ```
//! use reprol::ds::sparse_table_2d::SparseTable2d;
//! use reprol::ops::op_min::OpMin;
//! let v = vec![
//!     vec![2, 10, 1, 100],
//!     vec![5, 3, 8, 4],
//!     vec![7, 6, 9, 2],
//! ];
//! let st = SparseTable2d::<OpMin<i64>>::new(v);
//! assert_eq!(st.fold(0..3, 0..4), 1);
//! assert_eq!(st.fold(1..3, 1..4), 2);
//! ```

use std::ops::{Range, RangeBounds};

use crate::{
    ds::sparse_table::SparseTable, ops::monoid::IdempotentMonoid,
    utils::range_utils::to_half_open_index_range,
};

/// 2次元Sparse Table
pub struct SparseTable2d<O: IdempotentMonoid> {
    len_rows: usize,
    len_cols: usize,
    nodes: Vec<Vec<SparseTable<O>>>,
    op: O,
}

impl<O: IdempotentMonoid> SparseTable2d<O> {
    /// 2次元配列`v`からSparse Tableを構築する．
    pub fn new(v: Vec<Vec<O::Element>>) -> Self
    where
        O: Clone + Default,
    {
        Self::with_op(v, O::default())
    }

    /// 演算`op`を指定して，2次元配列`v`からSparse Tableを構築する．
    pub fn with_op(v: Vec<Vec<O::Element>>, op: O) -> Self
    where
        O: Clone,
    {
        assert!(!v.is_empty());
        assert!(!v[0].is_empty());
        debug_assert!(v.iter().all(|vi| vi.len() == v[0].len()));

        let len_rows = v.len();
        let len_cols = v[0].len();

        let len_nodes = v.len().next_power_of_two().trailing_zeros() as usize + 1;

        let mut nodes: Vec<Vec<SparseTable<O>>> = Vec::with_capacity(len_nodes);

        {
            let node = v
                .into_iter()
                .map(|vi| SparseTable::with_op(vi, op.clone()))
                .collect();
            nodes.push(node);
        }

        for i in 1..len_nodes {
            let node = (0..)
                .take_while(|j| j + (1 << i) <= len_rows)
                .map(|j| {
                    let v = (0..len_cols)
                        .map(|k| {
                            op.op(
                                nodes[i - 1][j].inner(0, k),
                                nodes[i - 1][j + (1 << (i - 1))].inner(0, k),
                            )
                        })
                        .collect();
                    SparseTable::with_op(v, op.clone())
                })
                .collect();
            nodes.push(node);
        }

        Self {
            len_rows,
            len_cols,
            nodes,
            op,
        }
    }

    /// (`i`, `j`)番目の要素を返す．
    pub fn get(&self, i: usize, j: usize) -> O::Element {
        self.fold(i..=i, j..=j)
    }

    /// 区間`[row_range] × [col_range]`の区間積を返す．
    pub fn fold(
        &self,
        row_range: impl RangeBounds<usize>,
        col_range: impl RangeBounds<usize>,
    ) -> O::Element {
        let Range { start: il, end: ir } = to_half_open_index_range(row_range, self.len_rows);
        let Range { start: jl, end: jr } = to_half_open_index_range(col_range, self.len_cols);
        assert!(il < ir && ir <= self.len_rows);
        assert!(jl < jr && jr <= self.len_cols);

        let k = (ir - il + 1).next_power_of_two().trailing_zeros() as usize - 1;
        self.op.op(
            &self.nodes[k][il].fold(jl..jr),
            &self.nodes[k][ir - (1 << k)].fold(jl..jr),
        )
    }
}

impl<O: IdempotentMonoid + Clone> From<(Vec<Vec<O::Element>>, O)> for SparseTable2d<O> {
    fn from((v, op): (Vec<Vec<O::Element>>, O)) -> Self {
        Self::with_op(v, op)
    }
}

impl<O: IdempotentMonoid + Clone, const N: usize, const M: usize> From<([[O::Element; M]; N], O)>
    for SparseTable2d<O>
{
    fn from((v, op): ([[O::Element; M]; N], O)) -> Self {
        let v: Vec<Vec<_>> = v.into_iter().map(|vi| vi.into_iter().collect()).collect();
        Self::with_op(v, op)
    }
}

impl<O: IdempotentMonoid + Clone + Default> From<Vec<Vec<O::Element>>> for SparseTable2d<O> {
    fn from(v: Vec<Vec<O::Element>>) -> Self {
        Self::from((v, O::default()))
    }
}

impl<O: IdempotentMonoid + Clone + Default, const N: usize, const M: usize>
    From<[[O::Element; M]; N]> for SparseTable2d<O>
{
    fn from(v: [[O::Element; M]; N]) -> Self {
        Self::from((v, O::default()))
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    use crate::{
        math::gcd::Gcd,
        ops::{op_gcd::OpGcd, op_max::OpMax, op_min::OpMin},
        utils::test_utils::{random::get_test_rng, static_range_query_2d::*},
    };

    #[test]
    fn test_min() {
        let a = vec![vec![2, 10, 1, 100], vec![5, 3, 8, 4], vec![7, 6, 9, 2]];
        let st = SparseTable2d::<OpMin<i64>>::new(a.clone());

        assert_eq!(st.get(0, 0), 2);
        assert_eq!(st.get(0, 2), 1);
        assert_eq!(st.get(1, 1), 3);

        assert_eq!(st.fold(0..1, 0..1), 2);
        assert_eq!(st.fold(..2, 0..2), 2);
        assert_eq!(st.fold(0..=2, 0..4), 1);
        assert_eq!(st.fold(1..3, 1..4), 2);
        assert_eq!(st.fold(.., ..), 1);
    }

    #[test]
    fn test_max() {
        let a = vec![vec![2, 10, 1, 100], vec![5, 3, 8, 4], vec![7, 6, 9, 2]];
        let st = SparseTable2d::<OpMax<i64>>::new(a.clone());

        assert_eq!(st.get(0, 0), 2);
        assert_eq!(st.get(0, 3), 100);
        assert_eq!(st.get(1, 0), 5);

        assert_eq!(st.fold(0..1, 0..1), 2);
        assert_eq!(st.fold(0..2, 0..2), 10);
        assert_eq!(st.fold(0..3, 0..4), 100);
        assert_eq!(st.fold(1..3, 1..4), 9);
        assert_eq!(st.fold(.., ..), 100);
    }

    macro_rules! random_min_max_gcd_xor_test {
        ($min_test_name: ident, $max_test_name: ident, $gcd_test_name: ident, $ty: ty) => {
            randomized_static_range_min_2d_exhaustive_test!(
                $min_test_name,
                $ty,
                |v| SparseTable2d::<OpMin<$ty>>::from(v),
                |ds: &SparseTable2d<_>, r, c| ds.fold(r, c),
                20,
                20
            );

            randomized_static_range_max_2d_exhaustive_test!(
                $max_test_name,
                $ty,
                |v| SparseTable2d::<OpMax<$ty>>::from(v),
                |ds: &SparseTable2d<_>, r, c| ds.fold(r, c),
                20,
                20
            );

            randomized_static_range_gcd_2d_exhaustive_test!(
                $gcd_test_name,
                $ty,
                |v| SparseTable2d::<OpGcd<$ty>>::from(v),
                |ds: &SparseTable2d<_>, r, c| ds.fold(r, c),
                10,
                20
            );
        };
    }

    random_min_max_gcd_xor_test!(
        test_random_min_i32,
        test_random_max_i32,
        test_random_gcd_i32,
        i32
    );
    random_min_max_gcd_xor_test!(
        test_random_min_u32,
        test_random_max_u32,
        test_random_gcd_u32,
        u32
    );
    random_min_max_gcd_xor_test!(
        test_random_min_i64,
        test_random_max_i64,
        test_random_gcd_i64,
        i64
    );
    random_min_max_gcd_xor_test!(
        test_random_min_u64,
        test_random_max_u64,
        test_random_gcd_u64,
        u64
    );
    random_min_max_gcd_xor_test!(
        test_random_min_usize,
        test_random_max_usize,
        test_random_gcd_usize,
        usize
    );
}
