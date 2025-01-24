use std::ops::{Range, RangeBounds};

use crate::{monoid::Monoid, range::to_open_range, sparse_table::SparseTable};

pub struct SparseTable2d<M: Monoid> {
    h: usize,
    w: usize,
    nodes: Vec<Vec<SparseTable<M>>>,
    monoid: M,
}

impl<M> SparseTable2d<M>
where
    M: Monoid + Clone,
    M::Value: Clone,
{
    pub fn new(v: Vec<Vec<M::Value>>, monoid: M) -> Self {
        assert!(!v.is_empty());
        assert!(!v[0].is_empty());
        #[cfg(debug_assertions)]
        assert!(v.iter().all(|vi| vi.len() == v[0].len()));

        let h = v.len();
        let w = v[0].len();
        let len = v.len().next_power_of_two().trailing_zeros() as usize + 1;
        let mut nodes = Vec::with_capacity(len);
        {
            let node = v
                .into_iter()
                .map(|vi| SparseTable::new(vi, monoid.clone()))
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
                    SparseTable::new(v, monoid.clone())
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

    pub fn from_slice(v: &[Vec<M::Value>], monoid: M) -> Self {
        Self::new(v.to_vec(), monoid)
    }

    pub fn product(
        &self,
        x_range: impl RangeBounds<usize>,
        y_range: impl RangeBounds<usize>,
    ) -> M::Value {
        let Range { start: xl, end: xr } = to_open_range(x_range, self.h);
        let Range { start: yl, end: yr } = to_open_range(y_range, self.w);
        if xl >= xr {
            return self.monoid.identity();
        }
        let k = (xr - xl + 1).next_power_of_two().trailing_zeros() as usize - 1;
        self.monoid.op(
            &self.nodes[k][xl].product(yl..yr),
            &self.nodes[k][xr - (1 << k)].product(yl..yr),
        )
    }
}

impl<M> From<Vec<Vec<M::Value>>> for SparseTable2d<M>
where
    M: Monoid + Clone + Default,
    M::Value: Clone,
{
    fn from(v: Vec<Vec<M::Value>>) -> Self {
        Self::new(v, M::default())
    }
}

impl<M> From<&Vec<Vec<M::Value>>> for SparseTable2d<M>
where
    M: Monoid + Clone + Default,
    M::Value: Clone,
{
    fn from(v: &Vec<Vec<M::Value>>) -> Self {
        Self::new(v.clone(), M::default())
    }
}

impl<M> From<&[Vec<M::Value>]> for SparseTable2d<M>
where
    M: Monoid + Clone + Default,
    M::Value: Clone,
{
    fn from(v: &[Vec<M::Value>]) -> Self {
        Self::from_slice(v, M::default())
    }
}
