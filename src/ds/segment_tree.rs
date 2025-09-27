//! セグメント木(Segment Tree)
//!
//! 要素としてモノイドを持つ配列を管理するデータ構造．
//! 以下の操作をいずれも O(log n) で処理できる．
//! - 要素の1点変更．
//! - 任意の区間の要素の総積(和，最小値など)の取得．
//!
//! # 使用例
//! ```
//! use reprol::{ds::segment_tree::SegmentTree, ops::monoid::Monoid};
//!
//! #[derive(Default)]
//! struct Op;
//!
//! impl Monoid for Op {
//!     type Value = i64;
//!
//!     fn identity(&self) -> Self::Value {
//!         0
//!     }
//!
//!     fn op(&self, lhs: &Self::Value, rhs: &Self::Value) -> Self::Value {
//!         lhs + rhs
//!     }
//! }
//!
//! // 区間和を計算するセグメント木
//! let v = vec![1, 3, 5, 7, 9, 11];
//! let mut seg = SegmentTree::<Op>::from(v);
//! assert_eq!(seg.fold(0..3), 9); // 1 + 3 + 5 = 9
//! assert_eq!(seg.fold(1..=4), 24); // 3 + 5 + 7 + 9 = 24
//! assert_eq!(seg.fold(..), 36);
//! seg.set(2, 6);
//! assert_eq!(seg.fold(0..3), 10); // 1 + 3 + 6 = 10
//! assert_eq!(seg.get(5), &11);
//! ```

use std::{
    iter::FromIterator,
    ops::{Index, Range, RangeBounds},
};

use crate::{ops::monoid::Monoid, range::to_open_range};

/// セグメント木
pub struct SegmentTree<O: Monoid> {
    /// 列の長さ(nodesの長さではない)
    len: usize,

    /// セグ木を構成するノード
    nodes: Vec<O::Value>,

    /// 演算(モノイド)
    op: O,
}

impl<O: Monoid> SegmentTree<O> {
    /// 長さ`len`のセグメント木を単位元で初期化して生成する．
    pub fn new(len: usize) -> Self
    where
        O: Default,
    {
        Self::with_op(len, O::default())
    }

    /// 長さ`len`のセグメント木を，モノイド`op`を指定して生成する．
    pub fn with_op(len: usize, op: O) -> Self {
        let offset = len.next_power_of_two();
        Self {
            len,
            nodes: (0..2 * offset).map(|_| op.identity()).collect(),
            op,
        }
    }

    /// `index`番目の要素を返す．
    pub fn get(&self, index: usize) -> &O::Value {
        assert!(index < self.len);
        &self.nodes[index + self.nodes.len() / 2]
    }

    /// `index`番目の要素を`value`に更新する．
    pub fn set(&mut self, index: usize, value: O::Value) {
        assert!(index < self.len);
        let mut index = index + self.nodes.len() / 2;
        self.nodes[index] = value;
        while index > 1 {
            index /= 2;
            self.nodes[index] = self
                .op
                .op(&self.nodes[2 * index], &self.nodes[2 * index + 1]);
        }
    }

    /// 区間`range`の要素の総積を返す．
    pub fn fold(&self, range: impl RangeBounds<usize>) -> O::Value {
        let Range { start: l, end: r } = to_open_range(range, self.len);
        assert!(l <= r);
        assert!(r <= self.len);

        let offset = self.nodes.len() / 2;
        let mut l = l + offset;
        let mut r = r + offset;

        let mut res_l = self.op.identity();
        let mut res_r = self.op.identity();

        while l < r {
            if l % 2 == 1 {
                res_l = self.op.op(&res_l, &self.nodes[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                res_r = self.op.op(&self.nodes[r], &res_r);
            }
            l /= 2;
            r /= 2;
        }

        self.op.op(&res_l, &res_r)
    }

    /// セグメント木上の二分探索(max_right)．
    ///
    /// `g(r) = f(fold(l..r))`として，
    /// 単調な`g`に対して，`g(r) = true`となる最大の`r`を返す．
    ///
    /// # 計算量
    /// - O(log n)
    ///
    /// # 制約
    /// - `0 <= l <= len`
    /// - `f(identity()) = true`
    pub fn bisect_right(&self, l: usize, mut f: impl FnMut(&O::Value) -> bool) -> usize {
        assert!(l <= self.len);
        debug_assert!(f(&self.op.identity()));

        if l == self.len {
            return self.len;
        }

        let offset = self.nodes.len() / 2;
        let mut l = l + offset;
        let mut prod = self.op.identity();

        loop {
            while l % 2 == 0 {
                l /= 2;
            }

            let tmp = self.op.op(&prod, &self.nodes[l]);
            if !f(&tmp) {
                while l < offset {
                    l *= 2;

                    let tmp = self.op.op(&prod, &self.nodes[l]);
                    if f(&tmp) {
                        prod = tmp;
                        l += 1;
                    }
                }

                return l - offset;
            }

            prod = tmp;
            l += 1;

            if l.is_power_of_two() {
                break;
            }
        }

        self.len
    }

    /// セグメント木上の二分探索(min_left)．
    ///
    /// `g(l) = f(fold(l..r))`として，
    /// 単調な`g`に対して，`g(l) = true`となる最小の`l`を返す．
    ///
    /// # 計算量
    /// - O(log n)
    ///
    /// # 制約
    /// - `0 <= r <= len`
    /// - `f(identity()) = true`
    pub fn bisect_left(&self, r: usize, mut f: impl FnMut(&O::Value) -> bool) -> usize {
        assert!(r <= self.len);
        debug_assert!(f(&self.op.identity()));

        if r == 0 {
            return 0;
        }

        let offset = self.nodes.len() / 2;
        let mut r = r + offset;
        let mut prod = self.op.identity();

        loop {
            r -= 1;
            while r > 1 && r % 2 == 1 {
                r /= 2;
            }

            let tmp = self.op.op(&self.nodes[r], &prod);
            if !f(&tmp) {
                while r < offset {
                    r = 2 * r + 1;
                    let tmp = self.op.op(&self.nodes[r], &prod);
                    if f(&tmp) {
                        prod = tmp;
                        r -= 1;
                    }
                }

                return r + 1 - offset;
            }

            prod = tmp;

            if r.is_power_of_two() {
                break;
            }
        }

        0
    }
}

impl<O> From<(Vec<O::Value>, O)> for SegmentTree<O>
where
    O: Monoid,
    O::Value: Clone,
{
    fn from((v, op): (Vec<O::Value>, O)) -> Self {
        Self::from((&v, op))
    }
}

impl<O, const N: usize> From<([O::Value; N], O)> for SegmentTree<O>
where
    O: Monoid,
    O::Value: Clone,
{
    fn from((v, op): ([O::Value; N], O)) -> Self {
        Self::from((v.as_slice(), op))
    }
}

impl<O> From<(&Vec<O::Value>, O)> for SegmentTree<O>
where
    O: Monoid,
    O::Value: Clone,
{
    fn from((v, op): (&Vec<O::Value>, O)) -> Self {
        Self::from((v.as_slice(), op))
    }
}

impl<O> From<(&[O::Value], O)> for SegmentTree<O>
where
    O: Monoid,
    O::Value: Clone,
{
    fn from((v, op): (&[O::Value], O)) -> Self {
        let len = v.len();
        let offset = len.next_power_of_two();
        let mut nodes = vec![op.identity(); 2 * offset];

        nodes[offset..offset + len].clone_from_slice(v);

        for i in (1..offset).rev() {
            nodes[i] = op.op(&nodes[2 * i], &nodes[2 * i + 1]);
        }

        Self { len, nodes, op }
    }
}

impl<O> From<Vec<O::Value>> for SegmentTree<O>
where
    O: Monoid + Default,
    O::Value: Clone,
{
    fn from(v: Vec<O::Value>) -> Self {
        Self::from((v, O::default()))
    }
}

impl<O, const N: usize> From<[O::Value; N]> for SegmentTree<O>
where
    O: Monoid + Default,
    O::Value: Clone,
{
    fn from(v: [O::Value; N]) -> Self {
        Self::from((v, O::default()))
    }
}

impl<O> From<&Vec<O::Value>> for SegmentTree<O>
where
    O: Monoid + Default,
    O::Value: Clone,
{
    fn from(v: &Vec<O::Value>) -> Self {
        Self::from((v, O::default()))
    }
}

impl<O> From<&[O::Value]> for SegmentTree<O>
where
    O: Monoid + Default,
    O::Value: Clone,
{
    fn from(v: &[O::Value]) -> Self {
        Self::from((v, O::default()))
    }
}

impl<O> FromIterator<O::Value> for SegmentTree<O>
where
    O: Monoid + Default,
    O::Value: Clone,
{
    fn from_iter<I: IntoIterator<Item = O::Value>>(iter: I) -> Self {
        Self::from(iter.into_iter().collect::<Vec<_>>())
    }
}

impl<O> Index<usize> for SegmentTree<O>
where
    O: Monoid,
{
    type Output = O::Value;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len);
        &self.nodes[index + self.nodes.len() / 2]
    }
}

#[cfg(test)]
mod tests {
    use crate::{ops::op_add::OpAdd, ops::op_min::OpMin};

    use super::SegmentTree;

    #[test]
    fn test_add() {
        let v = vec![1, 3, 5, 7, 9, 11];
        let mut seg = SegmentTree::<OpAdd<i64>>::from(v);
        assert_eq!(seg.fold(0..3), 9);
        assert_eq!(seg.fold(1..=4), 24);
        assert_eq!(seg.fold(..), 36);
        seg.set(2, 6);
        assert_eq!(seg.fold(0..3), 10);
        assert_eq!(seg[5], 11);
    }

    #[test]
    fn test_min() {
        let v = vec![5, 2, 6, 3, 7, 1];
        let mut seg = SegmentTree::<OpMin<i32>>::from(v);
        assert_eq!(seg.fold(0..4), 2);
        assert_eq!(seg.fold(2..=5), 1);
        assert_eq!(seg.fold(..), 1);
        assert_eq!(seg.fold(..=4), 2);
        seg.set(3, 0);
        assert_eq!(seg.fold(0..4), 0);
    }
}
