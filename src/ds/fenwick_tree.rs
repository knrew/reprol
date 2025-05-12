//! Fenwick Tree(Binary Indexed Tree)
//!
//! (群をなす集合の要素からなる)配列を管理するデータ構造．
//! 要素の1点更新と区間積の取得をO(logN)で行うことができる．
//!
//! # 使用例
//! ```
//! use reprol::{ds::fenwick_tree::FenwickTree, ops::op_add::OpAdd};
//! let mut ft = FenwickTree::<OpAdd<i32>>::new(5);
//! ft.op(1, &5); // v[1] += 5
//! ft.op(2, &3); // v[2] += 3
//! ft.op(4, &2); // v[4] += 2
//! assert_eq!(ft.fold(..2), 5); // 区間[0, 2)の区間和
//! assert_eq!(ft.fold(..=2), 8); // 区間[0, 2]の区間和
//! assert_eq!(ft.get(5), 10); // 区間[0, 5)の区間和
//! assert_eq!(ft.fold(2..5), 5); // 区間[2, 5)の区間和
//! ```

use std::ops::{Range, RangeBounds};

use crate::{
    ops::{group::Group, monoid::Monoid},
    range::to_open_range,
};

/// Fenwick Tree
pub struct FenwickTree<O: Monoid> {
    nodes: Vec<O::Value>,
    op: O,
}

impl<O: Monoid> FenwickTree<O> {
    /// 長さ`n`で初期化する．
    /// 要素はすべて単位元で初期化される．
    pub fn new(n: usize) -> Self
    where
        O: Default,
    {
        Self::with_op(n, O::default())
    }

    /// 演算`op`を指定して長さ`n`で初期化する．
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
            .for_each(|(i, rhs)| res.op(i, rhs));
        res
    }

    /// `index`番目の要素に`rhs`を作用させる．
    /// `v[i] <- v[i] * rhs`
    pub fn op(&mut self, mut index: usize, rhs: &O::Value) {
        assert!(index < self.nodes.len());
        index += 1;
        while index <= self.nodes.len() {
            self.nodes[index - 1] = self.op.op(&self.nodes[index - 1], rhs);
            index += index & index.wrapping_neg();
        }
    }

    /// 区間`[0, r)`の区間積を返す．
    pub fn get(&self, mut r: usize) -> O::Value {
        assert!(r <= self.nodes.len());
        let mut res = self.op.identity();
        while r > 0 {
            res = self.op.op(&res, &self.nodes[r - 1]);
            r -= r & r.wrapping_neg();
        }
        res
    }

    /// 区間`[l, r)`の区間積を返す．
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

    use super::*;

    #[test]
    fn test_fenwick_tree() {
        let mut ft = FenwickTree::<OpAdd<i64>>::new(10);
        ft.op(0, &5);
        ft.op(2, &10);
        ft.op(6, &20);
        assert_eq!(ft.fold(..1), 5);
        assert_eq!(ft.fold(..3), 15);
        assert_eq!(ft.fold(..7), 35);
        assert_eq!(ft.fold(..), 35);
        assert_eq!(ft.fold(0..3), 15);
        assert_eq!(ft.fold(3..=6), 20);
        ft.op(9, &10);
        assert_eq!(ft.fold(0..10), 45);
    }
}
