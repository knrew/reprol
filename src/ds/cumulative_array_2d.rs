//! 2次元累積積(累積和)
//!
//! 群の2次元配列に対する累積積を管理するデータ構造．
//! 区間積[`fold`](CumulativeArray2d::fold)も提供する．
//!
//! # 使用例
//! ## 2次元累積和
//! ```
//! use reprol::{
//!     ds::cumulative_array_2d::CumulativeSum2d,
//!     ops::op_add::OpAdd,
//! };
//! let v = vec![
//!     vec![1, 2, 3],
//!     vec![4, 5, 6],
//!     vec![7, 8, 9],
//! ];
//! let cum = CumulativeSum2d::new(v);
//! assert_eq!(cum.fold(0..3, 0..3), 45);
//! assert_eq!(cum.fold(1..3, 1..3), 28);
//! assert_eq!(cum.fold(0..2, 0..2), 12);
//! assert_eq!(cum[(2, 2)], 12);
//! ```

use std::{
    fmt::Debug,
    ops::{Index, Range, RangeBounds},
};

use crate::{
    ops::{group::Group, monoid::Monoid, op_add::OpAdd},
    range::to_open_range,
};

/// 2次元累積積を管理するデータ構造
pub struct CumulativeArray2d<O: Monoid> {
    data: Vec<Vec<O::Value>>,
    op: O,
}

impl<O: Monoid> CumulativeArray2d<O> {
    /// 2次元配列の累積配列を構築する．
    pub fn new(v: Vec<Vec<O::Value>>) -> Self
    where
        O: Group + Default,
    {
        Self::with_op(v, O::default())
    }

    /// 演算`op`を明示的に渡して2次元配列の累積配列を構築する．
    pub fn with_op(v: Vec<Vec<O::Value>>, op: O) -> Self
    where
        O: Group,
    {
        assert!(!v.is_empty());
        assert!(!v[0].is_empty());
        debug_assert!(v.iter().all(|vi| vi.len() == v[0].len()));

        let row_len = v.len();
        let col_len = v[0].len();
        let mut data: Vec<Vec<O::Value>> = (0..row_len + 1)
            .map(|_| (0..col_len + 1).map(|_| op.identity()).collect())
            .collect();

        for i in 0..row_len {
            for j in 0..col_len {
                let mut datum = op.op(&data[i + 1][j], &data[i][j + 1]);
                datum = op.op(&datum, &op.inv(&data[i][j]));
                datum = op.op(&datum, &v[i][j]);
                data[i + 1][j + 1] = datum;
            }
        }

        Self { data, op }
    }

    /// `[0, i) x [0, j)`の累積積を返す．
    pub fn get(&self, i: usize, j: usize) -> &O::Value {
        &self.data[i][j]
    }

    /// 区間`[il, ir) x [jl, jr)`の累積積を返す．
    pub fn fold(
        &self,
        row_range: impl RangeBounds<usize>,
        col_range: impl RangeBounds<usize>,
    ) -> O::Value
    where
        O: Group,
    {
        let Range { start: il, end: ir } = to_open_range(row_range, self.data.len() - 1);
        let Range { start: jl, end: jr } = to_open_range(col_range, self.data[0].len() - 1);
        assert!(il <= ir);
        assert!(jl <= jr);
        let mut res = self.op.op(&self.data[ir][jr], &self.data[il][jl]);
        res = self.op.op(&res, &self.op.inv(&self.data[il][jr]));
        res = self.op.op(&res, &self.op.inv(&self.data[ir][jl]));
        res
    }
}

impl<O> From<(Vec<Vec<O::Value>>, O)> for CumulativeArray2d<O>
where
    O: Group,
{
    fn from((v, op): (Vec<Vec<O::Value>>, O)) -> Self {
        CumulativeArray2d::with_op(v, op)
    }
}

impl<O> From<Vec<Vec<O::Value>>> for CumulativeArray2d<O>
where
    O: Group + Default,
{
    fn from(v: Vec<Vec<O::Value>>) -> Self {
        CumulativeArray2d::new(v)
    }
}

impl<O> Clone for CumulativeArray2d<O>
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

impl<O> Index<[usize; 2]> for CumulativeArray2d<O>
where
    O: Monoid,
{
    type Output = O::Value;
    fn index(&self, [i, j]: [usize; 2]) -> &Self::Output {
        &self.data[i][j]
    }
}

impl<O> Index<(usize, usize)> for CumulativeArray2d<O>
where
    O: Monoid,
{
    type Output = O::Value;
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.data[i][j]
    }
}

impl<O> Debug for CumulativeArray2d<O>
where
    O: Monoid,
    O::Value: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.data.iter()).finish()
    }
}

/// 2次元累積和
pub type CumulativeSum2d<T> = CumulativeArray2d<OpAdd<T>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cumulative_sum_2d() {
        let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let test_cases = vec![
            ((0, 0, 3, 3), 45),
            ((0, 0, 2, 2), 12),
            ((1, 1, 3, 3), 28),
            ((0, 1, 2, 3), 16),
            ((2, 0, 3, 2), 15),
            ((0, 0, 1, 1), 1),
            ((0, 0, 0, 0), 0),
        ];
        let cum = CumulativeSum2d::new(v);
        assert_eq!(cum.fold(.., ..), 45);
        for ((r1, c1, r2, c2), expected) in test_cases {
            assert_eq!(cum.fold(r1..r2, c1..c2), expected);
        }

        let v = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
        ];
        let test_cases = vec![
            ((0, 0, 3, 5), 120),
            ((0, 0, 2, 3), 27),
            ((1, 2, 3, 5), 69),
            ((0, 1, 2, 4), 33),
            ((2, 0, 3, 2), 23),
            ((0, 0, 1, 1), 1),
            ((1, 0, 2, 5), 40),
            ((0, 4, 3, 5), 30),
            ((0, 0, 0, 0), 0),
        ];
        let cum = CumulativeSum2d::new(v);
        for ((x1, y1, x2, y2), expected) in test_cases {
            assert_eq!(cum.fold(x1..x2, y1..y2), expected);
        }
    }
}
