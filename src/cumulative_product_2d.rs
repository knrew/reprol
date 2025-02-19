use std::ops::{Index, Range, RangeBounds};

use crate::{group::Group, monoid::Monoid, ops::op_add::OpAdd, range::to_open_range};

pub struct CumulativeProduct2d<O: Monoid> {
    row_len: usize,
    col_len: usize,
    data: Vec<Vec<O::Value>>,
    op: O,
}

impl<O> CumulativeProduct2d<O>
where
    O: Group,
{
    /// 2次元配列から累積積を構築する
    pub fn new(v: Vec<Vec<O::Value>>) -> Self
    where
        O: Default,
        O::Value: Clone,
    {
        assert!(!v.is_empty());
        Self::with_op(v, O::default())
    }

    /// 演算を引数で指定
    pub fn with_op(v: Vec<Vec<O::Value>>, op: O) -> Self
    where
        O::Value: Clone,
    {
        assert!(!v.is_empty());
        debug_assert!(v.iter().all(|vi| vi.len() == v[0].len()));
        Self::new_by(v.len(), v[0].len(), op, |i, j| v[i][j].clone())
    }

    /// i番目の値を関数で指定
    pub fn new_by(
        row_len: usize,
        col_len: usize,
        op: O,
        mut f: impl FnMut(usize, usize) -> O::Value,
    ) -> Self
    where
        O::Value: Clone,
    {
        let mut data = vec![vec![op.identity(); col_len + 1]; row_len + 1];

        for i in 0..row_len {
            for j in 0..col_len {
                let mut res = op.op(&data[i + 1][j], &data[i][j + 1]);
                res = op.op(&res, &op.inv(&data[i][j]));
                res = op.op(&res, &f(i, j));
                data[i + 1][j + 1] = res;
            }
        }

        Self {
            row_len,
            col_len,
            data,
            op,
        }
    }

    pub fn get(&self, i: usize, j: usize) -> &O::Value {
        &self.data[i][j]
    }

    /// 区間積(区間和)を計算する
    pub fn product(
        &self,
        row_range: impl RangeBounds<usize>,
        col_range: impl RangeBounds<usize>,
    ) -> O::Value {
        let Range { start: il, end: ir } = to_open_range(row_range, self.row_len);
        let Range { start: jl, end: jr } = to_open_range(col_range, self.col_len);
        assert!(il <= ir);
        assert!(jl <= jr);
        let mut res = self.op.op(&self.data[ir][jr], &self.data[il][jl]);
        res = self.op.op(&res, &self.op.inv(&self.data[il][jr]));
        res = self.op.op(&res, &self.op.inv(&self.data[ir][jl]));
        res
    }
}

impl<O> From<Vec<Vec<O::Value>>> for CumulativeProduct2d<O>
where
    O: Group + Default,
    O::Value: Clone,
{
    fn from(v: Vec<Vec<O::Value>>) -> Self {
        CumulativeProduct2d::new(v)
    }
}

impl<O> From<&Vec<Vec<O::Value>>> for CumulativeProduct2d<O>
where
    O: Group + Default,
    O::Value: Clone,
{
    fn from(v: &Vec<Vec<O::Value>>) -> Self {
        assert!(!v.is_empty());
        debug_assert!(v.iter().all(|vi| vi.len() == v[0].len()));
        Self::new_by(v.len(), v[0].len(), O::default(), |i, j| v[i][j].clone())
    }
}

impl<O> From<&[Vec<O::Value>]> for CumulativeProduct2d<O>
where
    O: Group + Default,
    O::Value: Clone,
{
    fn from(v: &[Vec<O::Value>]) -> Self {
        assert!(!v.is_empty());
        debug_assert!(v.iter().all(|vi| vi.len() == v[0].len()));
        Self::new_by(v.len(), v[0].len(), O::default(), |i, j| v[i][j].clone())
    }
}

impl<O> Clone for CumulativeProduct2d<O>
where
    O: Monoid + Clone,
    O::Value: Clone,
{
    fn clone(&self) -> Self {
        Self {
            row_len: self.row_len,
            col_len: self.col_len,
            data: self.data.clone(),
            op: self.op.clone(),
        }
    }
}

impl<O> Index<[usize; 2]> for CumulativeProduct2d<O>
where
    O: Monoid,
{
    type Output = O::Value;
    fn index(&self, [i, j]: [usize; 2]) -> &Self::Output {
        &self.data[i][j]
    }
}

impl<O> Index<(usize, usize)> for CumulativeProduct2d<O>
where
    O: Monoid,
{
    type Output = O::Value;
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.data[i][j]
    }
}

pub type CumulativeSum2d<T> = CumulativeProduct2d<OpAdd<T>>;

#[cfg(test)]
mod tests {
    use super::CumulativeSum2d;

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
        assert_eq!(cum.product(.., ..), 45);
        for ((r1, c1, r2, c2), expected) in test_cases {
            assert_eq!(cum.product(r1..r2, c1..c2), expected);
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
            assert_eq!(cum.product(x1..x2, y1..y2), expected);
        }
    }
}
