use std::ops::{Range, RangeBounds};

use crate::{group::Group, monoid::Monoid, ops::op_add::OpAdd, range::to_open_range};

pub struct CumulativeProduct<O: Monoid> {
    len: usize,
    data: Vec<O::Value>,
    op: O,
}

impl<O> CumulativeProduct<O>
where
    O: Monoid,
    O::Value: Clone,
{
    /// 配列vの累積積(累積和)を計算する
    pub fn new(v: Vec<O::Value>) -> Self
    where
        O: Default,
    {
        assert!(!v.is_empty());
        Self::from(v)
    }

    /// 演算を引数で指定
    pub fn with_op(v: Vec<O::Value>, op: O) -> Self {
        assert!(!v.is_empty());
        Self::new_by(v.len(), op, |i| v[i].clone())
    }

    /// i番目の値を関数で指定
    pub fn new_by(len: usize, op: O, mut f: impl FnMut(usize) -> O::Value) -> Self {
        let mut cum = vec![op.identity(); len + 1];
        for i in 0..len {
            cum[i + 1] = op.op(&cum[i], &f(i));
        }
        Self { len, data: cum, op }
    }

    /// [0, r)の累積を取得する
    /// e.g. cum.get(n)で総積(総和)
    pub fn get(&self, r: usize) -> &O::Value {
        assert!(r <= self.len);
        &self.data[r]
    }
}

impl<O> CumulativeProduct<O>
where
    O: Group,
{
    /// 区間積(区間和)を計算する
    /// $a_l \cdot \ldots \cdot a_{r-1}$
    pub fn product(&self, range: impl RangeBounds<usize>) -> O::Value {
        let Range { start: l, end: r } = to_open_range(range, self.len);
        assert!(l <= r);
        self.op.op(&self.data[r], &self.op.inv(&self.data[l]))
    }
}

impl<O> From<Vec<O::Value>> for CumulativeProduct<O>
where
    O: Monoid + Default,
    O::Value: Clone,
{
    fn from(v: Vec<O::Value>) -> Self {
        CumulativeProduct::from(&v)
    }
}

impl<O> From<&Vec<O::Value>> for CumulativeProduct<O>
where
    O: Monoid + Default,
    O::Value: Clone,
{
    fn from(v: &Vec<O::Value>) -> Self {
        CumulativeProduct::new_by(v.len(), O::default(), |i| v[i].clone())
    }
}

impl<O> From<&[O::Value]> for CumulativeProduct<O>
where
    O: Monoid + Default,
    O::Value: Clone,
{
    fn from(v: &[O::Value]) -> Self {
        CumulativeProduct::new_by(v.len(), O::default(), |i| v[i].clone())
    }
}

impl<O> Clone for CumulativeProduct<O>
where
    O: Monoid + Clone,
    O::Value: Clone,
{
    fn clone(&self) -> Self {
        Self {
            len: self.len,
            data: self.data.clone(),
            op: self.op.clone(),
        }
    }
}

pub type CumulativeSum<T> = CumulativeProduct<OpAdd<T>>;

#[cfg(test)]
mod tests {
    use crate::{cumulative_product::CumulativeSum, ops::op_add::OpAdd};

    #[test]
    fn test_cumulative_sum() {
        let v = vec![1, 2, 3, 4, 5];
        let test_cases = vec![
            ((0, 5), 15),
            ((0, 1), 1),
            ((1, 3), 5),
            ((3, 5), 9),
            ((2, 4), 7),
            ((2, 2), 0),
            ((4, 5), 5),
            ((0, 4), 10),
        ];
        let cum = CumulativeSum::<i64>::from(v);
        assert_eq!(cum.product(..), 15);
        for ((l, r), expected) in test_cases {
            assert_eq!(cum.product(l..r), expected);
        }

        let cum = CumulativeSum::new_by(5, OpAdd::default(), |i| i as i64 + 1);
        let test_cases = vec![
            ((0, 5), 15),
            ((0, 1), 1),
            ((1, 3), 5),
            ((3, 5), 9),
            ((2, 4), 7),
            ((2, 2), 0),
            ((4, 5), 5),
            ((0, 4), 10),
        ];
        for ((l, r), expected) in test_cases {
            assert_eq!(cum.product(l..r), expected);
        }
    }
}
