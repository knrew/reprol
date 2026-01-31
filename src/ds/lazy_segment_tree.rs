//! 遅延評価付きセグメント木(Lazy Segment Tree)
//!
//! 要素としてモノイドを持つ配列を管理するデータ構造．
//! 以下の操作をいずれも O(log n) で処理できる．
//! - 任意区間の要素に対する作用を一括適用．
//! - 任意の区間の要素の総積(和，最小値など)の取得．
//!
//! # 使用例
//!
//! ## 区間加算・区間最大値取得
//! ```
//! use reprol::{
//!     ds::lazy_segment_tree::LazySegmentTree,
//!     ops::{action::Action, monoid::Monoid},
//! };
//!
//! #[derive(Default)]
//! struct Op;
//!
//! impl Monoid for Op {
//!     type Element = i64;
//!
//!     fn id(&self) -> Self::Element {
//!         0
//!     }
//!
//!     fn op(&self, lhs: &Self::Element, rhs: &Self::Element) -> Self::Element {
//!         *lhs.max(rhs)
//!     }
//! }
//!
//! #[derive(Default)]
//! struct Act;
//!
//! impl Monoid for Act {
//!     type Element = i64;
//!
//!     fn id(&self) -> Self::Element {
//!         0
//!     }
//!
//!     fn op(&self, g: &Self::Element, f: &Self::Element) -> Self::Element {
//!         f + g
//!     }
//! }
//!
//! impl Action<Op> for Act {
//!     fn act(&self, f: &Self::Element, x: &<Op as Monoid>::Element) -> <Op as Monoid>::Element {
//!         x + f
//!     }
//! }
//!
//! let v = vec![4, 4, 4, 4, 4];
//! let mut seg = LazySegmentTree::<Op, Act>::from(v);
//!
//! seg.act(1..4, &2);
//! assert_eq!(seg.get(0), &4);
//! assert_eq!(seg.get(1), &6);
//! assert_eq!(seg.get(2), &6);
//! assert_eq!(seg.get(3), &6);
//! assert_eq!(seg.get(4), &4);
//!
//! assert_eq!(seg.fold(0..=2), 6);
//!
//! seg.act(.., &(-1));
//! assert_eq!(seg.get(0), &3);
//! assert_eq!(seg.get(1), &5);
//! assert_eq!(seg.get(2), &5);
//! assert_eq!(seg.get(3), &5);
//! assert_eq!(seg.get(4), &3);
//!
//! assert_eq!(seg.fold(..), 5);
//! ```

use std::{
    iter::FromIterator,
    mem::replace,
    ops::{Deref, DerefMut, Range, RangeBounds},
};

use crate::{
    ops::{action::Action, monoid::Monoid},
    utils::range::to_half_open_index_range,
};

/// 遅延評価付きセグメント木
pub struct LazySegmentTree<O: Monoid, A: Action<O>> {
    /// 列の長さ(nodesの長さではない)
    len: usize,

    /// セグ木を構成するノード
    nodes: Vec<O::Element>,

    /// 作用の遅延配列
    lazies: Vec<A::Element>,

    log: u32,

    /// 演算(モノイド)
    op: O,

    /// モノイドに対する作用
    action: A,
}

impl<O: Monoid, A: Action<O>> LazySegmentTree<O, A> {
    /// 長さ`len`のセグメント木を単位元で初期化して生成する．
    pub fn new(len: usize) -> Self
    where
        O: Default,
        A: Default,
    {
        Self::with_op(len, O::default(), A::default())
    }

    /// 長さ`len`のセグメント木を、モノイド`op`と作用`action`を指定して生成する．
    pub fn with_op(len: usize, op: O, action: A) -> Self {
        let offset = len.next_power_of_two();
        Self {
            len,
            nodes: (0..2 * offset).map(|_| op.id()).collect(),
            lazies: (0..offset).map(|_| action.id()).collect(),
            log: offset.trailing_zeros(),
            op,
            action,
        }
    }

    /// `index`番目の要素を返す．
    pub fn get(&mut self, index: usize) -> &O::Element
    where
        A::Element: PartialEq,
    {
        assert!(index < self.len);
        let index = index + self.nodes.len() / 2;
        for i in (1..=self.log).rev() {
            self.propagate(index >> i);
        }
        &self.nodes[index]
    }

    /// `index`番目の要素を`value`に更新する．
    pub fn set(&mut self, index: usize, value: O::Element)
    where
        A::Element: PartialEq,
    {
        *self.entry_mut(index) = value;
    }

    pub fn entry_mut(&mut self, index: usize) -> EntryMut<'_, O, A>
    where
        A::Element: PartialEq,
    {
        assert!(index < self.len);
        let leaf = index + self.nodes.len() / 2;

        for i in (1..=self.log).rev() {
            self.propagate(leaf >> i);
        }

        EntryMut {
            seg: self,
            node_index: leaf,
        }
    }

    /// 区間`range`の要素に作用`f`を適用する．
    pub fn act(&mut self, range: impl RangeBounds<usize>, f: &A::Element)
    where
        A::Element: PartialEq,
    {
        let Range { start: l, end: r } = to_half_open_index_range(range, self.len);
        assert!(r <= self.len);
        if l >= r {
            return;
        }

        let offset = self.nodes.len() / 2;
        let l = l + offset;
        let r = r + offset;

        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.propagate(l >> i);
            }
            if ((r >> i) << i) != r {
                self.propagate((r - 1) >> i);
            }
        }

        {
            let mut l = l;
            let mut r = r;

            while l < r {
                if l % 2 == 1 {
                    self.apply(l, f);
                    l += 1;
                }
                if r % 2 == 1 {
                    r -= 1;
                    self.apply(r, f);
                }
                l /= 2;
                r /= 2;
            }
        }

        for i in 1..=self.log {
            if ((l >> i) << i) != l {
                let k = l >> i;
                self.nodes[k] = self.op.op(&self.nodes[2 * k], &self.nodes[2 * k + 1]);
            }
            if ((r >> i) << i) != r {
                let k = (r - 1) >> i;
                self.nodes[k] = self.op.op(&self.nodes[2 * k], &self.nodes[2 * k + 1]);
            }
        }
    }

    /// 区間`range`の要素の総積を返す．
    pub fn fold(&mut self, range: impl RangeBounds<usize>) -> O::Element
    where
        A::Element: PartialEq,
    {
        let Range { start: l, end: r } = to_half_open_index_range(range, self.len);
        assert!(l <= r);
        assert!(r <= self.len);

        let offset = self.nodes.len() / 2;
        let mut l = l + offset;
        let mut r = r + offset;

        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.propagate(l >> i);
            }
            if ((r >> i) << i) != r {
                self.propagate((r - 1) >> i);
            }
        }

        let mut res_l = self.op.id();
        let mut res_r = self.op.id();

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
    pub fn bisect_right(&mut self, l: usize, mut f: impl FnMut(&O::Element) -> bool) -> usize
    where
        A::Element: PartialEq,
    {
        assert!(l <= self.len);
        debug_assert!(f(&self.op.id()));

        if l == self.len {
            return self.len;
        }

        let offset = self.nodes.len() / 2;
        let mut l = l + offset;

        for i in (1..=self.log).rev() {
            self.propagate(l >> i);
        }

        let mut prod = self.op.id();

        loop {
            while l.is_multiple_of(2) {
                l /= 2;
            }

            let tmp = self.op.op(&prod, &self.nodes[l]);
            if !f(&tmp) {
                while l < offset {
                    self.propagate(l);
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
    pub fn bisect_left(&mut self, r: usize, mut f: impl FnMut(&O::Element) -> bool) -> usize
    where
        A::Element: PartialEq,
    {
        assert!(r <= self.len);
        debug_assert!(f(&self.op.id()));

        if r == 0 {
            return 0;
        }

        let offset = self.nodes.len() / 2;
        let mut r = r + offset;
        for i in (1..=self.log).rev() {
            self.propagate((r - 1) >> i);
        }

        let mut prod = self.op.id();

        loop {
            r -= 1;
            while r > 1 && r % 2 == 1 {
                r /= 2;
            }

            let tmp = self.op.op(&self.nodes[r], &prod);
            if !f(&tmp) {
                while r < offset {
                    self.propagate(r);
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

    fn propagate(&mut self, k: usize)
    where
        A::Element: PartialEq,
    {
        if self.lazies[k] == self.action.id() {
            return;
        }
        let lz = replace(&mut self.lazies[k], self.action.id());
        self.apply(2 * k, &lz);
        self.apply(2 * k + 1, &lz);
    }

    /// ノードkにfを作用させる
    fn apply(&mut self, k: usize, f: &A::Element) {
        self.nodes[k] = self.action.act(f, &self.nodes[k]);
        if k < self.nodes.len() / 2 {
            self.lazies[k] = self.action.op(f, &self.lazies[k]);
        }
    }
}

impl<O: Monoid, A: Action<O>> From<(Vec<O::Element>, O, A)> for LazySegmentTree<O, A> {
    fn from((v, op, action): (Vec<O::Element>, O, A)) -> Self {
        let len = v.len();
        let offset = len.next_power_of_two();

        let mut nodes = (0..2 * offset).map(|_| op.id()).collect::<Vec<_>>();

        for (node_i, vi) in nodes[offset..offset + len].iter_mut().zip(v) {
            *node_i = vi;
        }

        for i in (1..offset).rev() {
            nodes[i] = op.op(&nodes[2 * i], &nodes[2 * i + 1]);
        }

        Self {
            len,
            nodes,
            lazies: (0..offset).map(|_| action.id()).collect(),
            log: offset.trailing_zeros(),
            op,
            action,
        }
    }
}

impl<O: Monoid, A: Action<O>, const N: usize> From<([O::Element; N], O, A)>
    for LazySegmentTree<O, A>
{
    fn from((v, op, action): ([O::Element; N], O, A)) -> Self {
        Self::from((v.into_iter().collect::<Vec<_>>(), op, action))
    }
}

impl<O: Monoid + Default, A: Action<O> + Default> From<Vec<O::Element>> for LazySegmentTree<O, A> {
    fn from(v: Vec<O::Element>) -> Self {
        Self::from((v, O::default(), A::default()))
    }
}

impl<O: Monoid + Default, A: Action<O> + Default, const N: usize> From<[O::Element; N]>
    for LazySegmentTree<O, A>
{
    fn from(v: [O::Element; N]) -> Self {
        Self::from((v, O::default(), A::default()))
    }
}

impl<O: Monoid + Default, A: Action<O> + Default> FromIterator<O::Element>
    for LazySegmentTree<O, A>
{
    fn from_iter<I: IntoIterator<Item = O::Element>>(iter: I) -> Self {
        Self::from(iter.into_iter().collect::<Vec<_>>())
    }
}

pub struct EntryMut<'a, O: Monoid, A: Action<O>>
where
    A::Element: PartialEq,
{
    seg: &'a mut LazySegmentTree<O, A>,
    node_index: usize,
}

impl<'a, O: Monoid, A: Action<O>> Deref for EntryMut<'a, O, A>
where
    A::Element: PartialEq,
{
    type Target = O::Element;
    fn deref(&self) -> &Self::Target {
        &self.seg.nodes[self.node_index]
    }
}

impl<'a, O: Monoid, A: Action<O>> DerefMut for EntryMut<'a, O, A>
where
    A::Element: PartialEq,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.seg.nodes[self.node_index]
    }
}

impl<'a, O: Monoid, A: Action<O>> Drop for EntryMut<'a, O, A>
where
    A::Element: PartialEq,
{
    fn drop(&mut self) {
        for i in 1..=self.seg.log {
            let k = self.node_index >> i;
            self.seg.nodes[k] = self
                .seg
                .op
                .op(&self.seg.nodes[2 * k], &self.seg.nodes[2 * k + 1]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::{act_add::ActAdd, op_max::OpMax};

    #[test]
    fn test_opmax_actadd() {
        type Op = OpMax<i64>;
        type Act = ActAdd<i64>;

        {
            let v = vec![4, 4, 4, 4, 4];
            let mut seg = LazySegmentTree::<Op, Act>::from(v);
            seg.act(1..4, &2);
            assert_eq!(
                (0..5).map(|i| *seg.get(i)).collect::<Vec<_>>(),
                vec![4, 6, 6, 6, 4]
            );
            assert_eq!(seg.fold(0..=2), 6);
            seg.act(0..5, &(-1));
            assert_eq!(
                (0..5).map(|i| *seg.get(i)).collect::<Vec<_>>(),
                vec![3, 5, 5, 5, 3]
            );
            assert_eq!(seg.fold(..), 5);
        }

        {
            let mut seg = LazySegmentTree::<OpMax<i64>, ActAdd<i64>>::from(vec![0; 4]);
            seg.act(0..4, &5);
            assert_eq!(*seg.get(0), 5);
        }

        // test set
        {
            let mut seg = LazySegmentTree::<Op, Act>::from(vec![1, 2, 3, 4]);
            assert_eq!(seg.fold(..), 4);
            seg.set(2, 10);
            assert_eq!(seg.fold(..), 10);
        }
    }
}
