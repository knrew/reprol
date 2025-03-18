use std::ops::{Range, RangeBounds};

use crate::{ops::monoid::Monoid, range::to_open_range};

pub struct DisjointSparseTable<O: Monoid> {
    len: usize,
    data: Vec<Vec<O::Value>>,
    op: O,
}

impl<O: Monoid> DisjointSparseTable<O> {
    pub fn new(v: Vec<O::Value>) -> Self
    where
        O: Default,
    {
        assert!(!v.is_empty());
        Self::with_op(v, O::default())
    }

    pub fn with_op(v: Vec<O::Value>, op: O) -> Self {
        assert!(!v.is_empty());

        let n = v.len() + 2;
        let h = n.next_power_of_two().trailing_zeros() as usize;

        let mut data = Vec::with_capacity(h);
        data.push((0..n).map(|_| op.identity()).collect());

        for w in (1..h).map(|k| 1 << k) {
            let mut datum = (0..n).map(|_| op.identity()).collect::<Vec<_>>();

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
            len: n - 2,
            data,
            op,
        }
    }

    /// i番目の要素を取得する
    pub fn get(&self, index: usize) -> O::Value {
        self.fold(index..=index)
    }

    /// `dst.fold(l..r)`で [l, r)の区間積を計算する
    pub fn fold(&self, range: impl RangeBounds<usize>) -> O::Value {
        let Range {
            start: l,
            end: mut r,
        } = to_open_range(range, self.len);
        assert!(l <= r);
        assert!(r <= self.len);
        r += 1;
        let i = ((l ^ r) + 1).next_power_of_two().trailing_zeros() - 1;
        let datum = &self.data[i as usize];
        self.op.op(&datum[l], &datum[r])
    }
}

impl<O> From<(Vec<O::Value>, O)> for DisjointSparseTable<O>
where
    O: Monoid,
{
    fn from((v, op): (Vec<O::Value>, O)) -> Self {
        Self::with_op(v, op)
    }
}

impl<O, const N: usize> From<([O::Value; N], O)> for DisjointSparseTable<O>
where
    O: Monoid,
    O::Value: Clone,
{
    fn from((v, op): ([O::Value; N], O)) -> Self {
        Self::with_op(v.to_vec(), op)
    }
}

impl<O> From<Vec<O::Value>> for DisjointSparseTable<O>
where
    O: Monoid + Default,
{
    fn from(v: Vec<O::Value>) -> Self {
        Self::new(v)
    }
}

impl<O, const N: usize> From<[O::Value; N]> for DisjointSparseTable<O>
where
    O: Monoid + Default,
    O::Value: Clone,
{
    fn from(v: [O::Value; N]) -> Self {
        Self::new(v.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use crate::ops::op_min::OpMin;

    use super::DisjointSparseTable;

    #[test]
    fn test_disjoint_sparse_table() {
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
}
