use std::ops::{Range, RangeBounds};

use crate::{group::Group, range::to_open_range};

pub struct FenwickTree<G: Group> {
    len: usize,
    nodes: Vec<G::Value>,
    group: G,
}

impl<G> FenwickTree<G>
where
    G: Group,
    G::Value: Clone,
{
    pub fn new(n: usize) -> Self
    where
        G: Default,
    {
        Self::with_op(n, G::default())
    }

    /// 演算(群)を引数で指定
    pub fn with_op(n: usize, group: G) -> Self {
        Self {
            len: n,
            nodes: vec![group.identity(); n],
            group,
        }
    }

    pub fn add(&mut self, mut index: usize, value: G::Value) {
        assert!(index < self.len);
        index += 1;
        while index <= self.len {
            self.nodes[index - 1] = self.group.op(&self.nodes[index - 1], &value);
            index += index & index.wrapping_neg();
        }
    }

    pub fn product(&self, range: impl RangeBounds<usize>) -> G::Value {
        let Range { start: l, end: r } = to_open_range(range, self.len);
        assert!(l <= r);
        let cl = self.cum(l);
        let cr = self.cum(r);
        self.group.op(&cr, &self.group.inv(&cl))
    }

    fn cum(&self, mut r: usize) -> G::Value {
        let mut res = self.group.identity();
        while r > 0 {
            res = self.group.op(&res, &self.nodes[r - 1]);
            r -= r & r.wrapping_neg();
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::ops::op_add::OpAdd;

    use super::FenwickTree;

    #[test]
    fn test_fenwick_tree() {
        let mut ft = FenwickTree::<OpAdd<i64>>::new(10);
        ft.add(0, 5);
        ft.add(2, 10);
        ft.add(6, 20);
        assert_eq!(ft.product(..1), 5);
        assert_eq!(ft.product(..3), 15);
        assert_eq!(ft.product(..7), 35);
        assert_eq!(ft.product(..), 35);
        assert_eq!(ft.product(0..3), 15);
        assert_eq!(ft.product(3..=6), 20);
        ft.add(9, 10);
        assert_eq!(ft.product(0..10), 45);
    }
}
