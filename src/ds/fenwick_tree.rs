use std::ops::{Range, RangeBounds};

use crate::{
    ops::{group::Group, monoid::Monoid},
    range::to_open_range,
};

/// 要素の1点更新と区間積(和)の取得が行えるデータ構造
pub struct FenwickTree<O: Monoid> {
    nodes: Vec<O::Value>,
    op: O,
}

impl<O: Monoid> FenwickTree<O> {
    pub fn new(n: usize) -> Self
    where
        O: Default,
        O::Value: Clone,
    {
        Self::with_op(n, O::default())
    }

    /// 演算を引数で指定
    pub fn with_op(n: usize, op: O) -> Self
    where
        O::Value: Clone,
    {
        Self {
            nodes: vec![op.identity(); n],
            op,
        }
    }

    /// i番目の要素v[i]をop(v[i], rhs)で更新する
    pub fn op(&mut self, mut index: usize, rhs: O::Value) {
        assert!(index < self.nodes.len());
        index += 1;
        while index <= self.nodes.len() {
            self.nodes[index - 1] = self.op.op(&self.nodes[index - 1], &rhs);
            index += index & index.wrapping_neg();
        }
    }

    /// [0, r)の累積
    pub fn get(&self, mut r: usize) -> O::Value {
        let mut res = self.op.identity();
        while r > 0 {
            res = self.op.op(&res, &self.nodes[r - 1]);
            r -= r & r.wrapping_neg();
        }
        res
    }

    /// [l, r)の区間積を取得する
    pub fn product(&self, range: impl RangeBounds<usize>) -> O::Value
    where
        O: Group,
    {
        let Range { start: l, end: r } = to_open_range(range, self.nodes.len());
        assert!(l <= r);
        let cl = self.get(l);
        let cr = self.get(r);
        self.op.op(&cr, &self.op.inv(&cl))
    }
}

impl<O> From<(Vec<O::Value>, O)> for FenwickTree<O>
where
    O: Monoid,
    O::Value: Clone,
{
    fn from((v, op): (Vec<O::Value>, O)) -> Self {
        let mut res = FenwickTree::with_op(v.len(), op);
        v.into_iter()
            .enumerate()
            .for_each(|(i, rhs)| res.op(i, rhs));
        res
    }
}

impl<O> From<Vec<O::Value>> for FenwickTree<O>
where
    O: Monoid + Default,
    O::Value: Clone,
{
    fn from(v: Vec<O::Value>) -> Self {
        Self::from((v, O::default()))
    }
}

#[cfg(test)]
mod tests {
    use crate::ops::op_add::OpAdd;

    use super::FenwickTree;

    #[test]
    fn test_fenwick_tree() {
        let mut ft = FenwickTree::<OpAdd<i64>>::new(10);
        ft.op(0, 5);
        ft.op(2, 10);
        ft.op(6, 20);
        assert_eq!(ft.product(..1), 5);
        assert_eq!(ft.product(..3), 15);
        assert_eq!(ft.product(..7), 35);
        assert_eq!(ft.product(..), 35);
        assert_eq!(ft.product(0..3), 15);
        assert_eq!(ft.product(3..=6), 20);
        ft.op(9, 10);
        assert_eq!(ft.product(0..10), 45);
    }
}
