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
//!     type Element = i64;
//!
//!     fn id(&self) -> Self::Element {
//!         0
//!     }
//!
//!     fn op(&self, lhs: &Self::Element, rhs: &Self::Element) -> Self::Element {
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
    ops::{Deref, DerefMut, Index, Range, RangeBounds},
};

use crate::{ops::monoid::Monoid, utils::range::to_half_open_index_range};

/// セグメント木
pub struct SegmentTree<O: Monoid> {
    /// 列の長さ(nodesの長さではない)
    len: usize,

    offset: usize,

    /// セグ木を構成するノード
    nodes: Vec<O::Element>,

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
            offset,
            nodes: (0..2 * offset).map(|_| op.id()).collect(),
            op,
        }
    }

    /// `index`番目の要素を返す．
    pub fn get(&self, index: usize) -> &O::Element {
        assert!(index < self.len);
        &self.nodes[index + self.offset]
    }

    /// `index`番目の要素を`value`に更新する．
    #[inline(always)]
    pub fn set(&mut self, index: usize, value: O::Element) {
        *self.entry_mut(index) = value;
    }

    pub fn entry_mut(&mut self, index: usize) -> EntryMut<'_, O> {
        assert!(index < self.len);
        EntryMut { seg: self, index }
    }

    /// 区間`range`の要素の総積を返す．
    pub fn fold(&self, range: impl RangeBounds<usize>) -> O::Element {
        let Range { start: l, end: r } = to_half_open_index_range(range, self.len);
        assert!(l <= r);
        assert!(r <= self.len);

        let offset = self.offset;
        let mut l = l + offset;
        let mut r = r + offset;

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
    pub fn bisect_right(&self, l: usize, mut f: impl FnMut(&O::Element) -> bool) -> usize {
        assert!(l <= self.len);
        debug_assert!(f(&self.op.id()));

        if l == self.len {
            return self.len;
        }

        let mut l = l + self.offset;
        let mut prod = self.op.id();

        loop {
            while l.is_multiple_of(2) {
                l /= 2;
            }

            let tmp = self.op.op(&prod, &self.nodes[l]);
            if !f(&tmp) {
                while l < self.offset {
                    l *= 2;

                    let tmp = self.op.op(&prod, &self.nodes[l]);
                    if f(&tmp) {
                        prod = tmp;
                        l += 1;
                    }
                }

                return l - self.offset;
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
    pub fn bisect_left(&self, r: usize, mut f: impl FnMut(&O::Element) -> bool) -> usize {
        assert!(r <= self.len);
        debug_assert!(f(&self.op.id()));

        if r == 0 {
            return 0;
        }

        let mut r = r + self.offset;
        let mut prod = self.op.id();

        loop {
            r -= 1;
            while r > 1 && r % 2 == 1 {
                r /= 2;
            }

            let tmp = self.op.op(&self.nodes[r], &prod);
            if !f(&tmp) {
                while r < self.offset {
                    r = 2 * r + 1;
                    let tmp = self.op.op(&self.nodes[r], &prod);
                    if f(&tmp) {
                        prod = tmp;
                        r -= 1;
                    }
                }

                return r + 1 - self.offset;
            }

            prod = tmp;

            if r.is_power_of_two() {
                break;
            }
        }

        0
    }
}

impl<O: Monoid> From<(Vec<O::Element>, O)> for SegmentTree<O> {
    fn from((v, op): (Vec<O::Element>, O)) -> Self {
        assert!(!v.is_empty());

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
            offset,
            nodes,
            op,
        }
    }
}

impl<O: Monoid, const N: usize> From<([O::Element; N], O)> for SegmentTree<O> {
    fn from((v, op): ([O::Element; N], O)) -> Self {
        Self::from((v.into_iter().collect::<Vec<_>>(), op))
    }
}

impl<O: Monoid + Default> From<Vec<O::Element>> for SegmentTree<O> {
    fn from(v: Vec<O::Element>) -> Self {
        Self::from((v, O::default()))
    }
}

impl<O: Monoid + Default, const N: usize> From<[O::Element; N]> for SegmentTree<O> {
    fn from(v: [O::Element; N]) -> Self {
        Self::from((v, O::default()))
    }
}

impl<O: Monoid + Default> FromIterator<O::Element> for SegmentTree<O> {
    fn from_iter<I: IntoIterator<Item = O::Element>>(iter: I) -> Self {
        Self::from(iter.into_iter().collect::<Vec<_>>())
    }
}

impl<O: Monoid> Index<usize> for SegmentTree<O> {
    type Output = O::Element;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}

pub struct EntryMut<'a, O: Monoid> {
    seg: &'a mut SegmentTree<O>,
    index: usize,
}

impl<'a, O: Monoid> Deref for EntryMut<'a, O> {
    type Target = O::Element;
    fn deref(&self) -> &Self::Target {
        self.seg.get(self.index)
    }
}

impl<'a, O: Monoid> DerefMut for EntryMut<'a, O> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.seg.nodes[self.index + self.seg.offset]
    }
}

impl<'a, O: Monoid> Drop for EntryMut<'a, O> {
    fn drop(&mut self) {
        let mut index = self.index + self.seg.offset;
        while index > 1 {
            index /= 2;
            self.seg.nodes[index] = self
                .seg
                .op
                .op(&self.seg.nodes[2 * index], &self.seg.nodes[2 * index + 1]);
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    use crate::{
        ops::{op_add::OpAdd, op_max::OpMax, op_min::OpMin},
        utils::test_utils::initialize_rng,
    };

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

    #[test]
    fn test_entry_mut() {
        // 代入
        {
            let v = vec![1, 3, 5, 7, 9, 11];
            let mut seg = SegmentTree::<OpAdd<i64>>::from(v);
            *seg.entry_mut(2) = 6;
            assert_eq!(seg.fold(0..3), 10);
            assert_eq!(seg[2], 6);
            assert_eq!(seg.fold(..), 37);
        }

        // in-place 更新
        {
            let v = vec![1, 3, 5, 7, 9, 11];
            let mut seg = SegmentTree::<OpAdd<i64>>::from(v);
            *seg.entry_mut(4) += 100;
            assert_eq!(seg[4], 109);
            assert_eq!(seg.fold(4..=4), 109);
            assert_eq!(seg.fold(..), 136);
        }

        // 境界
        {
            let v = vec![5, 2, 6, 3, 7, 1];
            let mut seg = SegmentTree::<OpMin<i32>>::from(v);

            {
                let mut e = seg.entry_mut(0);
                *e = 10;
            } // ここでdrop

            assert_eq!(seg[0], 10);
            assert_eq!(seg.fold(..), 1);

            let mut e = seg.entry_mut(5);
            *e = 20;
            drop(e); // 明示的にdrop

            assert_eq!(seg.fold(..), 2);
        }
    }

    #[test]
    fn test_custom_monoid_mod() {
        #[derive(Clone, Copy, Debug)]
        struct OpModAdd {
            m: i64,
        }

        impl Monoid for OpModAdd {
            type Element = i64;

            fn id(&self) -> Self::Element {
                0
            }

            fn op(&self, lhs: &Self::Element, rhs: &Self::Element) -> Self::Element {
                (lhs + rhs) % self.m
            }
        }

        {
            let op = OpModAdd { m: 7 };
            let mut seg = SegmentTree::with_op(5, op);
            for (i, x) in [10, 20, 30, 40, 50].into_iter().enumerate() {
                seg.set(i, x);
            }
            assert_eq!(seg.fold(..), 3);
            assert_eq!(seg.fold(1..4), (20 + 30 + 40) % 7);
        }

        {
            let seg = SegmentTree::from((vec![1, 2, 3, 4], OpModAdd { m: 5 }));
            assert_eq!(seg.fold(..), (1 + 2 + 3 + 4) % 5);
            assert_eq!(seg.fold(2..=3), (3 + 4) % 5);
        }
    }

    #[test]
    fn test_custom_monoid_affine() {
        // アフィン変換 f(x) = a*x + b
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        struct Affine {
            a: i64,
            b: i64,
        }

        #[derive(Debug, Clone, Copy)]
        struct OpAffine;

        impl Monoid for OpAffine {
            type Element = Affine;

            fn id(&self) -> Self::Element {
                Affine { a: 1, b: 0 }
            }

            // (a1,b1) ∘ (a2,b2) = (a1*a2, a1*b2 + b1)
            fn op(&self, lhs: &Self::Element, rhs: &Self::Element) -> Self::Element {
                Affine {
                    a: lhs.a * rhs.a,
                    b: lhs.a * rhs.b + lhs.b,
                }
            }
        }

        // 変換列: f0, f1, f2
        // f0(x)=2x+1, f1(x)=3x+4, f2(x)=5x+6
        let v = vec![
            Affine { a: 2, b: 1 },
            Affine { a: 3, b: 4 },
            Affine { a: 5, b: 6 },
        ];

        let mut seg = SegmentTree::from((v, OpAffine));

        assert_eq!(seg.fold(..), Affine { a: 30, b: 45 });
        assert_eq!(seg.fold(1..3), Affine { a: 15, b: 22 });
        assert_eq!(seg.fold(1..=1), Affine { a: 3, b: 4 });
        assert_eq!(seg.fold(0..2), Affine { a: 6, b: 9 });
        seg.set(1, Affine { a: 1, b: 10 });
        assert_eq!(seg.fold(..), Affine { a: 10, b: 33 });
        *seg.entry_mut(2) = Affine { a: 7, b: 0 };
        assert_eq!(seg.fold(..), Affine { a: 14, b: 21 });

        assert_eq!(
            SegmentTree::from((vec![Affine { a: 3, b: 4 }, Affine { a: 2, b: 1 }], OpAffine))
                .fold(..),
            Affine { a: 6, b: 7 }
        );
    }

    #[test]
    fn test_bisect_add() {
        let mut seg = SegmentTree::<OpAdd<i64>>::from(vec![1, 3, 5, 7, 9, 11]);

        assert_eq!(seg.bisect_right(0, |s| *s <= 4), 2);
        assert_eq!(seg.bisect_right(2, |s| *s < 5), 2);
        assert_eq!(seg.bisect_left(6, |s| *s <= 20), 4);
        assert_eq!(seg.bisect_left(3, |s| *s < 9), 1);

        seg.set(2, 6);

        assert_eq!(seg.bisect_right(0, |s| *s <= 4), 2);
        assert_eq!(seg.bisect_right(0, |s| *s <= 9), 2);
        assert_eq!(seg.bisect_left(6, |s| *s <= 20), 4);
        assert_eq!(seg.bisect_left(3, |s| *s <= 9), 1);

        *seg.entry_mut(4) += 100;

        assert_eq!(seg.bisect_right(4, |s| *s <= 108), 4);
        assert_eq!(seg.bisect_right(4, |s| *s < 109), 4);
        assert_eq!(seg.bisect_left(6, |s| *s <= 11), 5);
    }

    #[test]
    fn test_bisect_min() {
        let mut seg = SegmentTree::<OpMin<i32>>::from(vec![5, 2, 6, 3, 7, 1]);

        assert_eq!(seg.bisect_right(0, |m| *m >= 2), 5);
        assert_eq!(seg.bisect_right(0, |m| *m > 2), 1);
        assert_eq!(seg.bisect_left(5, |m| *m >= 3), 2);

        seg.set(5, 10);

        assert_eq!(seg.bisect_right(0, |m| *m >= 2), 6);
        assert_eq!(seg.bisect_left(6, |m| *m >= 2), 0);
        assert_eq!(seg.bisect_left(6, |m| *m > 2), 2);

        *seg.entry_mut(1) = 8;

        assert_eq!(seg.bisect_right(0, |m| *m >= 3), 6);
        assert_eq!(seg.bisect_left(4, |m| *m > 5), 4);
    }

    #[test]
    fn test_add_random() {
        fn naive_bisect_right(v: &[i64], l: usize, mut f: impl FnMut(i64) -> bool) -> usize {
            let mut sum = 0;
            let mut r = l;
            while r < v.len() {
                let ns = sum + v[r];
                if f(ns) {
                    sum = ns;
                    r += 1;
                } else {
                    break;
                }
            }
            r
        }

        fn naive_bisect_left(v: &[i64], r: usize, mut f: impl FnMut(i64) -> bool) -> usize {
            let mut sum = 0;
            let mut l = r;
            while l > 0 {
                let nl = l - 1;
                let ns = v[nl] + sum;
                if f(ns) {
                    sum = ns;
                    l = nl;
                } else {
                    break;
                }
            }
            l
        }

        let mut rng = initialize_rng();

        const T: usize = 10;
        const Q: usize = 100000;
        const N_MAX: usize = 1000;
        const MIN: i64 = 0;
        const MAX: i64 = 1000000000;

        for _ in 0..T {
            let n = rng.random_range(10..=N_MAX);

            let mut v = (0..n)
                .map(|_| rng.random_range(MIN..=MAX))
                .collect::<Vec<_>>();

            let mut seg = SegmentTree::<OpAdd<_>>::from(v.clone());

            for _ in 0..Q {
                match rng.random_range(0..=4) {
                    0 => {
                        // set
                        let i = rng.random_range(0..n);
                        let vi = rng.random_range(MIN..=MAX);
                        v[i] = vi;
                        seg.set(i, vi);
                        assert_eq!(seg[i], v[i]);
                    }
                    1 => {
                        // entry_mut
                        let i = rng.random_range(0..n);
                        let d = rng.random_range(MIN..=MAX);
                        v[i] += d;
                        *seg.entry_mut(i) += d;
                        assert_eq!(seg[i], v[i]);
                    }
                    2 => {
                        // fold
                        let l = rng.random_range(0..n);
                        let r = rng.random_range(l + 1..=n);
                        assert_eq!(seg.fold(l..r), v[l..r].iter().sum::<i64>());
                    }
                    3 => {
                        // bisect_right
                        let l = rng.random_range(0..=n);
                        let t = rng.random_range(5 * MIN..=5 * MAX);
                        assert_eq!(
                            seg.bisect_right(l, |s| *s <= t),
                            naive_bisect_right(&v, l, |s| s <= t)
                        );
                    }
                    4 => {
                        // bisect_left
                        let r = rng.random_range(0..=n);
                        let t = rng.random_range(5 * MIN..=5 * MAX);
                        assert_eq!(
                            seg.bisect_left(r, |s| *s <= t),
                            naive_bisect_left(&v, r, |s| s <= t)
                        );
                    }
                    _ => unreachable!(),
                }

                if rng.random_bool(0.05) {
                    assert_eq!(seg.fold(..), v.iter().sum());
                }
            }
        }
    }

    #[test]
    fn test_max_random() {
        fn naive_bisect_right(v: &[i32], l: usize, mut f: impl FnMut(i32) -> bool) -> usize {
            let mut cur = i32::MIN;
            let mut r = l;
            while r < v.len() {
                let nm = cur.max(v[r]);
                if f(nm) {
                    cur = nm;
                    r += 1;
                } else {
                    break;
                }
            }
            r
        }

        fn naive_bisect_left(v: &[i32], r: usize, mut f: impl FnMut(i32) -> bool) -> usize {
            let mut cur = i32::MIN;
            let mut l = r;
            while l > 0 {
                let nl = l - 1;
                let nm = v[nl].max(cur);
                if f(nm) {
                    cur = nm;
                    l = nl;
                } else {
                    break;
                }
            }
            l
        }

        let mut rng = initialize_rng();

        const T: usize = 10;
        const Q: usize = 100000;
        const N_MAX: usize = 1000;
        const MIN: i32 = -1000000000;
        const MAX: i32 = 1000000000;

        for _ in 0..T {
            let n = rng.random_range(10..=N_MAX);

            let mut v = (0..n)
                .map(|_| rng.random_range(MIN..=MAX))
                .collect::<Vec<_>>();

            let mut seg = SegmentTree::<OpMax<i32>>::from(v.clone());

            for _ in 0..Q {
                match rng.random_range(0..=4) {
                    0 => {
                        // set
                        let i = rng.random_range(0..n);
                        let vi = rng.random_range(MIN..=MAX);
                        v[i] = vi;
                        seg.set(i, vi);
                        assert_eq!(seg[i], v[i]);
                    }
                    1 => {
                        // entry_mut
                        let i = rng.random_range(0..n);
                        let d = rng.random_range(-1000..=1000);
                        v[i] += d;
                        *seg.entry_mut(i) += d;
                        assert_eq!(seg[i], v[i]);
                    }
                    2 => {
                        let l = rng.random_range(0..n);
                        let r = rng.random_range(l + 1..=n);
                        let naive = *v[l..r].iter().max().unwrap();
                        assert_eq!(seg.fold(l..r), naive);
                    }
                    3 => {
                        // bisect_right
                        let l = rng.random_range(0..=n);
                        let t = rng.random_range(MIN..=MAX);
                        assert_eq!(
                            seg.bisect_right(l, |m| *m <= t),
                            naive_bisect_right(&v, l, |m| m <= t)
                        );
                    }
                    4 => {
                        // bisect_left
                        let r = rng.random_range(0..=n);
                        let t = rng.random_range(MIN..=MAX);
                        assert_eq!(
                            seg.bisect_left(r, |m| *m <= t),
                            naive_bisect_left(&v, r, |m| m <= t)
                        );
                    }
                    _ => unreachable!(),
                }

                if rng.random_bool(0.05) {
                    assert_eq!(seg.fold(..), *v.iter().max().unwrap());
                }
            }
        }
    }
}
