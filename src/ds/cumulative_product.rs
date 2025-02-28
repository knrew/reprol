use std::{
    fmt::Debug,
    ops::{Index, Range, RangeBounds},
};

use crate::{
    ops::{group::Group, monoid::Monoid, op_add::OpAdd},
    range::to_open_range,
};

pub struct CumulativeProduct<O: Monoid> {
    data: Vec<O::Value>,
    op: O,
}

impl<O: Monoid> CumulativeProduct<O> {
    /// 配列vの累積積を計算する
    pub fn new(v: Vec<O::Value>) -> Self
    where
        O: Default,
        O::Value: Clone,
    {
        assert!(!v.is_empty());
        Self::construct(v.len(), |i| v[i].clone())
    }

    /// i番目の要素がf(i)であるような累積積を計算する
    pub fn construct(len: usize, f: impl FnMut(usize) -> O::Value) -> Self
    where
        O: Default,
        O::Value: Clone,
    {
        Self::construct_with_op(len, O::default(), f)
    }

    /// 演算を指定して配列vの累積積を計算する
    pub fn with_op(v: Vec<O::Value>, op: O) -> Self
    where
        O::Value: Clone,
    {
        assert!(!v.is_empty());
        Self::construct_with_op(v.len(), op, |i| v[i].clone())
    }

    /// 演算を指定してi番目の要素がf(i)であるような累積積を計算する
    pub fn construct_with_op(len: usize, op: O, mut f: impl FnMut(usize) -> O::Value) -> Self
    where
        O::Value: Clone,
    {
        let mut data = vec![op.identity(); len + 1];
        for i in 0..len {
            data[i + 1] = op.op(&data[i], &f(i));
        }
        Self { data, op }
    }

    /// [0, r)の累積積を取得する
    pub fn get(&self, r: usize) -> &O::Value {
        &self.data[r]
    }

    /// `cum.product(l..r)`で [l, r)の区間積を計算する
    pub fn product(&self, range: impl RangeBounds<usize>) -> O::Value
    where
        O: Group,
    {
        let Range { start: l, end: r } = to_open_range(range, self.data.len() - 1);
        assert!(l <= r);
        self.op.op(&self.data[r], &self.op.inv(&self.data[l]))
    }
}

impl<O> From<(Vec<O::Value>, O)> for CumulativeProduct<O>
where
    O: Monoid,
    O::Value: Clone,
{
    fn from((v, op): (Vec<O::Value>, O)) -> Self {
        CumulativeProduct::with_op(v, op)
    }
}

impl<O, const N: usize> From<([O::Value; N], O)> for CumulativeProduct<O>
where
    O: Monoid,
    O::Value: Clone,
{
    fn from((v, op): ([O::Value; N], O)) -> Self {
        CumulativeProduct::construct_with_op(v.len(), op, |i| v[i].clone())
    }
}

impl<O> From<Vec<O::Value>> for CumulativeProduct<O>
where
    O: Monoid + Default,
    O::Value: Clone,
{
    fn from(v: Vec<O::Value>) -> Self {
        CumulativeProduct::new(v)
    }
}

impl<O, const N: usize> From<[O::Value; N]> for CumulativeProduct<O>
where
    O: Monoid + Default,
    O::Value: Clone,
{
    fn from(v: [O::Value; N]) -> Self {
        CumulativeProduct::construct(v.len(), |i| v[i].clone())
    }
}

impl<O> From<&Vec<O::Value>> for CumulativeProduct<O>
where
    O: Monoid + Default,
    O::Value: Clone,
{
    fn from(v: &Vec<O::Value>) -> Self {
        CumulativeProduct::construct(v.len(), |i| v[i].clone())
    }
}

impl<O> From<&[O::Value]> for CumulativeProduct<O>
where
    O: Monoid + Default,
    O::Value: Clone,
{
    fn from(v: &[O::Value]) -> Self {
        CumulativeProduct::construct(v.len(), |i| v[i].clone())
    }
}

impl<O> Clone for CumulativeProduct<O>
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

impl<O> Index<usize> for CumulativeProduct<O>
where
    O: Monoid,
{
    type Output = O::Value;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<O> Debug for CumulativeProduct<O>
where
    O: Monoid,
    O::Value: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.data.iter()).finish()
    }
}

pub type CumulativeSum<T> = CumulativeProduct<OpAdd<T>>;

#[cfg(test)]
mod tests {
    use crate::ops::op_min::OpMin;

    use super::{CumulativeProduct, CumulativeSum};

    #[test]
    fn test_cumulative_sum() {
        let v = vec![1, 2, 3, 4, 5];
        let testcases = vec![
            ((0, 5), 15),
            ((0, 1), 1),
            ((1, 3), 5),
            ((3, 5), 9),
            ((2, 4), 7),
            ((2, 2), 0),
            ((4, 5), 5),
            ((0, 4), 10),
        ];
        let cum = CumulativeSum::<i64>::new(v);
        assert_eq!(cum.product(..), 15);
        assert_eq!(cum.get(5), &15);
        for ((l, r), expected) in testcases {
            assert_eq!(cum.product(l..r), expected);
        }

        let cum = CumulativeSum::construct(5, |i| i as i64 + 1);
        let testcases = vec![
            ((0, 5), 15),
            ((0, 1), 1),
            ((1, 3), 5),
            ((3, 5), 9),
            ((2, 4), 7),
            ((2, 2), 0),
            ((4, 5), 5),
            ((0, 4), 10),
        ];
        for ((l, r), expected) in testcases {
            assert_eq!(cum.product(l..r), expected);
        }
    }

    #[test]
    fn test_cumulative_min() {
        let v = vec![8, 10, -4, 2, 11];
        let testcases = vec![(1, 8), (2, 8), (3, -4), (4, -4), (5, -4)];
        let cum = CumulativeProduct::<OpMin<i32>>::new(v);
        for (r, expected) in testcases {
            assert_eq!(cum.get(r), &expected);
        }
    }
}
