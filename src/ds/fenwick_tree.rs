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
    {
        Self::with_op(n, O::default())
    }

    /// 演算を引数で指定
    pub fn with_op(n: usize, op: O) -> Self {
        Self {
            nodes: (0..n).map(|_| op.identity()).collect(),
            op,
        }
    }

    fn from_slice(v: &[O::Value], op: O) -> Self {
        let mut res = Self::with_op(v.len(), op);
        v.into_iter()
            .enumerate()
            .for_each(|(i, rhs)| res.apply(i, rhs));
        res
    }

    /// i番目の要素にrhsを作用させる
    /// v[i] <- op(v[i], rhs)
    pub fn apply(&mut self, mut index: usize, rhs: &O::Value) {
        assert!(index < self.nodes.len());
        index += 1;
        while index <= self.nodes.len() {
            self.nodes[index - 1] = self.op.op(&self.nodes[index - 1], rhs);
            index += index & index.wrapping_neg();
        }
    }

    /// [0, r)の累積
    pub fn get(&self, mut r: usize) -> O::Value {
        assert!(r <= self.nodes.len());
        let mut res = self.op.identity();
        while r > 0 {
            res = self.op.op(&res, &self.nodes[r - 1]);
            r -= r & r.wrapping_neg();
        }
        res
    }

    /// [l, r)の区間積を取得する
    pub fn fold(&self, range: impl RangeBounds<usize>) -> O::Value
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
{
    fn from((v, op): (Vec<O::Value>, O)) -> Self {
        Self::from_slice(&v, op)
    }
}

impl<O, const N: usize> From<([O::Value; N], O)> for FenwickTree<O>
where
    O: Monoid,
{
    fn from((v, op): ([O::Value; N], O)) -> Self {
        Self::from_slice(&v, op)
    }
}

impl<O> From<Vec<O::Value>> for FenwickTree<O>
where
    O: Monoid + Default,
{
    fn from(v: Vec<O::Value>) -> Self {
        Self::from_slice(&v, O::default())
    }
}

impl<O, const N: usize> From<[O::Value; N]> for FenwickTree<O>
where
    O: Monoid + Default,
{
    fn from(v: [O::Value; N]) -> Self {
        Self::from_slice(&v, O::default())
    }
}

impl<O> Clone for FenwickTree<O>
where
    O: Monoid + Clone,
    O::Value: Clone,
{
    fn clone(&self) -> Self {
        Self {
            nodes: self.nodes.clone(),
            op: self.op.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ops::op_add::OpAdd;

    use super::FenwickTree;

    #[test]
    fn test_fenwick_tree() {
        let mut ft = FenwickTree::<OpAdd<i64>>::new(10);
        ft.apply(0, &5);
        ft.apply(2, &10);
        ft.apply(6, &20);
        assert_eq!(ft.fold(..1), 5);
        assert_eq!(ft.fold(..3), 15);
        assert_eq!(ft.fold(..7), 35);
        assert_eq!(ft.fold(..), 35);
        assert_eq!(ft.fold(0..3), 15);
        assert_eq!(ft.fold(3..=6), 20);
        ft.apply(9, &10);
        assert_eq!(ft.fold(0..10), 45);
    }
}
