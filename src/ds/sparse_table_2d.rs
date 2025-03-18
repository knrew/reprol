use std::ops::{Range, RangeBounds};

use crate::{ds::sparse_table::SparseTable, ops::monoid::Monoid, range::to_open_range};

pub struct SparseTable2d<M: Monoid> {
    h: usize,
    w: usize,
    nodes: Vec<Vec<SparseTable<M>>>,
    monoid: M,
}

impl<M> SparseTable2d<M>
where
    M: Monoid,
    M::Value: Clone,
{
    pub fn new(v: Vec<Vec<M::Value>>) -> Self
    where
        M: Default,
    {
        // TODO:with_opと統合する

        assert!(!v.is_empty());
        assert!(!v[0].is_empty());
        debug_assert!(v.iter().all(|vi| vi.len() == v[0].len()));

        let monoid = M::default();
        let h = v.len();
        let w = v[0].len();
        let len = v.len().next_power_of_two().trailing_zeros() as usize + 1;
        let mut nodes = Vec::with_capacity(len);
        {
            let node = v
                .into_iter()
                .map(|vi| SparseTable::new(vi))
                .collect::<Vec<_>>();
            nodes.push(node);
        }
        for i in 1..len {
            let node = (0..)
                .take_while(|j| j + (1 << i) <= h)
                .map(|j| {
                    let v = (0..w)
                        .map(|k| {
                            monoid.op(
                                &nodes[i - 1][j].raw()[0][k],
                                &nodes[i - 1][j + (1 << (i - 1))].raw()[0][k],
                            )
                        })
                        .collect();
                    SparseTable::new(v)
                })
                .collect();
            nodes.push(node);
        }

        Self {
            h,
            w,
            nodes,
            monoid,
        }
    }

    pub fn with_op(v: Vec<Vec<M::Value>>, monoid: M) -> Self
    where
        M: Clone,
    {
        assert!(!v.is_empty());
        assert!(!v[0].is_empty());
        debug_assert!(v.iter().all(|vi| vi.len() == v[0].len()));

        let h = v.len();
        let w = v[0].len();
        let len = v.len().next_power_of_two().trailing_zeros() as usize + 1;
        let mut nodes = Vec::with_capacity(len);
        {
            let node = v
                .into_iter()
                .map(|vi| SparseTable::with_op(vi, monoid.clone()))
                .collect::<Vec<_>>();
            nodes.push(node);
        }
        for i in 1..len {
            let node = (0..)
                .take_while(|j| j + (1 << i) <= h)
                .map(|j| {
                    let v = (0..w)
                        .map(|k| {
                            monoid.op(
                                &nodes[i - 1][j].raw()[0][k],
                                &nodes[i - 1][j + (1 << (i - 1))].raw()[0][k],
                            )
                        })
                        .collect();
                    SparseTable::with_op(v, monoid.clone())
                })
                .collect();
            nodes.push(node);
        }

        Self {
            h,
            w,
            nodes,
            monoid,
        }
    }

    pub fn fold(
        &self,
        row_range: impl RangeBounds<usize>,
        col_range: impl RangeBounds<usize>,
    ) -> M::Value {
        let Range { start: il, end: ir } = to_open_range(row_range, self.h);
        let Range { start: jl, end: jr } = to_open_range(col_range, self.w);
        if il >= ir {
            return self.monoid.identity();
        }
        let k = (ir - il + 1).next_power_of_two().trailing_zeros() as usize - 1;
        self.monoid.op(
            &self.nodes[k][il].fold(jl..jr),
            &self.nodes[k][ir - (1 << k)].fold(jl..jr),
        )
    }
}

impl<M> From<Vec<Vec<M::Value>>> for SparseTable2d<M>
where
    M: Monoid + Default,
    M::Value: Clone,
{
    fn from(v: Vec<Vec<M::Value>>) -> Self {
        Self::new(v)
    }
}

impl<M> From<&Vec<Vec<M::Value>>> for SparseTable2d<M>
where
    M: Monoid + Default,
    M::Value: Clone,
{
    fn from(v: &Vec<Vec<M::Value>>) -> Self {
        Self::new(v.clone())
    }
}

impl<M> From<&[Vec<M::Value>]> for SparseTable2d<M>
where
    M: Monoid + Default,
    M::Value: Clone,
{
    fn from(v: &[Vec<M::Value>]) -> Self {
        Self::new(v.to_vec())
    }
}
