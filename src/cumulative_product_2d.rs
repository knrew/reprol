use std::{
    fmt::Debug,
    ops::{Index, Range, RangeBounds},
};

use crate::{group::Group, monoid::Monoid, ops::op_add::OpAdd, range::to_open_range};

pub struct CumulativeProduct2d<O: Monoid> {
    data: Vec<Vec<O::Value>>,
    op: O,
}

impl<O: Monoid> CumulativeProduct2d<O> {
    /// 2次元配列の累積積を計算する
    pub fn new(v: Vec<Vec<O::Value>>) -> Self
    where
        O: Group + Default,
        O::Value: Clone,
    {
        assert!(!v.is_empty());
        Self::with_op(v, O::default())
    }

    /// 要素(i, j)の値がf(i, j)であるような2次元累積和を計算する
    pub fn construct(
        row_len: usize,
        col_len: usize,
        f: impl FnMut(usize, usize) -> O::Value,
    ) -> Self
    where
        O: Group + Default,
        O::Value: Clone,
    {
        Self::construct_with_op(row_len, col_len, O::default(), f)
    }

    /// 演算を指定して2次元配列の累積積を計算する
    pub fn with_op(v: Vec<Vec<O::Value>>, op: O) -> Self
    where
        O: Group,
        O::Value: Clone,
    {
        assert!(!v.is_empty());
        assert!(!v[0].is_empty());
        debug_assert!(v.iter().all(|vi| vi.len() == v[0].len()));
        Self::construct_with_op(v.len(), v[0].len(), op, |i, j| v[i][j].clone())
    }

    /// 演算を指定して要素(i, j)の値がf(i, j)であるような2次元累積積を計算する
    pub fn construct_with_op(
        row_len: usize,
        col_len: usize,
        op: O,
        mut f: impl FnMut(usize, usize) -> O::Value,
    ) -> Self
    where
        O: Group,
        O::Value: Clone,
    {
        assert!(row_len > 0);
        assert!(col_len > 0);

        let mut data = vec![vec![op.identity(); col_len + 1]; row_len + 1];

        for i in 0..row_len {
            for j in 0..col_len {
                let mut res = op.op(&data[i + 1][j], &data[i][j + 1]);
                res = op.op(&res, &op.inv(&data[i][j]));
                res = op.op(&res, &f(i, j));
                data[i + 1][j + 1] = res;
            }
        }

        Self { data, op }
    }

    /// [0, i) \times [0, j)の累積積を取得
    pub fn get(&self, i: usize, j: usize) -> &O::Value {
        &self.data[i][j]
    }

    /// `cum.product(il..ir, jl..jr)`で [il, ir) \times [jl, jr)の累積積を計算する
    pub fn product(
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

impl<O> From<(Vec<Vec<O::Value>>, O)> for CumulativeProduct2d<O>
where
    O: Group,
    O::Value: Clone,
{
    fn from((v, op): (Vec<Vec<O::Value>>, O)) -> Self {
        CumulativeProduct2d::with_op(v, op)
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
        assert!(!v[0].is_empty());
        debug_assert!(v.iter().all(|vi| vi.len() == v[0].len()));
        Self::construct(v.len(), v[0].len(), |i, j| v[i][j].clone())
    }
}

impl<O> From<&[Vec<O::Value>]> for CumulativeProduct2d<O>
where
    O: Group + Default,
    O::Value: Clone,
{
    fn from(v: &[Vec<O::Value>]) -> Self {
        assert!(!v.is_empty());
        assert!(!v[0].is_empty());
        debug_assert!(v.iter().all(|vi| vi.len() == v[0].len()));
        Self::construct(v.len(), v[0].len(), |i, j| v[i][j].clone())
    }
}

impl<O> Clone for CumulativeProduct2d<O>
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

impl<O> Debug for CumulativeProduct2d<O>
where
    O: Monoid,
    O::Value: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.data.iter()).finish()
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
