use std::ops::{Range, RangeBounds};

use crate::{monoid::Monoid, range::to_open_range};

/// なんか遅い？
pub struct SparseTable<M: Monoid> {
    n: usize,
    nodes: Vec<Vec<M::Value>>,
    monoid: M,
}

impl<M> SparseTable<M>
where
    M: Monoid,
    M::Value: Clone,
{
    pub fn new(v: Vec<M::Value>) -> Self
    where
        M: Default,
    {
        Self::with_op(v, M::default())
    }

    /// 演算(モノイド)を引数で指定
    pub fn with_op(v: Vec<M::Value>, monoid: M) -> Self {
        assert!(!v.is_empty());
        let n = v.len();
        let len = v.len().next_power_of_two().trailing_zeros() as usize + 1;
        let mut nodes = Vec::with_capacity(len);
        nodes.push(v);
        for i in 1..len {
            let node = (0..)
                .take_while(|j| j + (1 << i) <= n)
                .map(|j| monoid.op(&nodes[i - 1][j], &nodes[i - 1][j + (1 << (i - 1))]))
                .collect::<Vec<_>>();
            nodes.push(node);
        }
        Self { n, nodes, monoid }
    }

    pub fn product(&self, range: impl RangeBounds<usize>) -> M::Value {
        let Range { start: l, end: r } = to_open_range(range, self.n);
        if l >= r {
            return self.monoid.identity();
        }
        let k = (r - l + 1).next_power_of_two().trailing_zeros() as usize - 1;
        self.monoid
            .op(&self.nodes[k][l], &self.nodes[k][r - (1 << k)])
    }

    pub fn raw(&self) -> &Vec<Vec<M::Value>> {
        &self.nodes
    }
}

impl<M> From<Vec<M::Value>> for SparseTable<M>
where
    M: Monoid + Default,
    M::Value: Clone,
{
    fn from(v: Vec<M::Value>) -> Self {
        Self::new(v)
    }
}

impl<M> From<&Vec<M::Value>> for SparseTable<M>
where
    M: Monoid + Default,
    M::Value: Clone,
{
    fn from(v: &Vec<M::Value>) -> Self {
        Self::new(v.clone())
    }
}

impl<M> From<&[M::Value]> for SparseTable<M>
where
    M: Monoid + Default,
    M::Value: Clone,
{
    fn from(v: &[M::Value]) -> Self {
        Self::new(v.to_vec())
    }
}

impl<M> FromIterator<M::Value> for SparseTable<M>
where
    M: Monoid + Default,
    M::Value: Clone,
{
    fn from_iter<T: IntoIterator<Item = M::Value>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod tests {
    use crate::ops::op_min::OpMin;

    use super::SparseTable;

    #[test]
    fn test_sparse_table() {
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

        let sp = SparseTable::<OpMin<i64>>::new(v);
        for ([l, r], expected) in test_cases {
            assert_eq!(sp.product(l..r), expected);
        }
    }
}
