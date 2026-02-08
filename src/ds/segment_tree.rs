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

use crate::{ops::monoid::Monoid, utils::range_utils::to_half_open_index_range};

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
        math::gcd::Gcd,
        ops::{op_add::OpAdd, op_gcd::OpGcd, op_max::OpMax, op_min::OpMin, op_xor::OpXor},
        utils::test_utils::{dynamic_range_query::*, random::get_test_rng, static_range_query::*},
    };

    // ============================================================
    // 基本機能テスト
    // ============================================================

    #[test]
    fn test_range_sum() {
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
    fn test_range_min() {
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

    // ============================================================
    // bisect(二分探索)の基本機能テスト
    // ============================================================

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

    // ============================================================
    // 静的クエリのランダムテスト
    // ============================================================

    macro_rules! seg_randomized_static_range_sum_exhaustive_test {
        ($test_name: ident, $ty: ty, $range: expr) => {
            randomized_static_range_sum_exhaustive_test!(
                $test_name,
                $ty,
                |v| SegmentTree::<OpAdd<$ty>>::from(v),
                |ds: &SegmentTree<_>, range| ds.fold(range),
                200,
                100,
                $range
            );
        };
    }

    macro_rules! seg_randomized_static_range_min_max_gcd_xor_exhaustive_test {
        ($min_test_name: ident, $max_test_name: ident, $gcd_test_name: ident, $xor_test_name: ident, $ty: ty) => {
            randomized_static_range_min_exhaustive_test!(
                $min_test_name,
                $ty,
                |v| SegmentTree::<OpMin<$ty>>::from(v),
                |ds: &SegmentTree<_>, range| ds.fold(range),
                200,
                100
            );

            randomized_static_range_max_exhaustive_test!(
                $max_test_name,
                $ty,
                |v| SegmentTree::<OpMax<$ty>>::from(v),
                |ds: &SegmentTree<_>, range| ds.fold(range),
                200,
                100
            );

            randomized_static_range_gcd_exhaustive_test!(
                $gcd_test_name,
                $ty,
                |v| SegmentTree::<OpGcd<$ty>>::from(v),
                |ds: &SegmentTree<_>, range| ds.fold(range),
                100,
                100
            );

            randomized_static_range_xor_exhaustive_test!(
                $xor_test_name,
                $ty,
                |v| SegmentTree::<OpXor<$ty>>::from(v),
                |ds: &SegmentTree<_>, range| ds.fold(range),
                200,
                100
            );
        };
    }

    seg_randomized_static_range_sum_exhaustive_test!(
        test_randomized_static_range_sum_exhaustive_i8,
        i8,
        -1..=1
    );
    seg_randomized_static_range_sum_exhaustive_test!(
        test_randomized_static_range_sum_exhaustive_u8,
        u8,
        0..=1
    );
    seg_randomized_static_range_sum_exhaustive_test!(
        test_randomized_static_range_sum_exhaustive_i16,
        i16,
        -300..=300
    );
    seg_randomized_static_range_sum_exhaustive_test!(
        test_randomized_static_range_sum_exhaustive_u16,
        u16,
        0..=300
    );
    seg_randomized_static_range_sum_exhaustive_test!(
        test_randomized_static_range_sum_exhaustive_i32,
        i32,
        -100000..=100000
    );
    seg_randomized_static_range_sum_exhaustive_test!(
        test_randomized_static_range_sum_exhaustive_u32,
        u32,
        0..=100000
    );
    seg_randomized_static_range_sum_exhaustive_test!(
        test_randomized_static_range_sum_exhaustive_i64,
        i64,
        -1000000000..=1000000000
    );
    seg_randomized_static_range_sum_exhaustive_test!(
        test_randomized_static_range_sum_exhaustive_u64,
        u64,
        0..=1000000000
    );
    seg_randomized_static_range_sum_exhaustive_test!(
        test_randomized_static_range_sum_exhaustive_i128,
        i128,
        -1000000000000000000..=1000000000000000000
    );
    seg_randomized_static_range_sum_exhaustive_test!(
        test_randomized_static_range_sum_exhaustive_u128,
        u128,
        0..=1000000000000000000
    );
    seg_randomized_static_range_sum_exhaustive_test!(
        test_randomized_static_range_sum_exhaustive_usize,
        usize,
        0..=1000000000
    );

    seg_randomized_static_range_min_max_gcd_xor_exhaustive_test!(
        test_randomized_static_range_min_exhaustive_i8,
        test_randomized_static_range_max_exhaustive_i8,
        test_randomized_static_range_gcd_exhaustive_i8,
        test_randomized_static_range_xor_exhaustive_i8,
        i8
    );
    seg_randomized_static_range_min_max_gcd_xor_exhaustive_test!(
        test_randomized_static_range_min_exhaustive_u8,
        test_randomized_static_range_max_exhaustive_u8,
        test_randomized_static_range_gcd_exhaustive_u8,
        test_randomized_static_range_xor_exhaustive_u8,
        u8
    );
    seg_randomized_static_range_min_max_gcd_xor_exhaustive_test!(
        test_randomized_static_range_min_exhaustive_i16,
        test_randomized_static_range_max_exhaustive_i16,
        test_randomized_static_range_gcd_exhaustive_i16,
        test_randomized_static_range_xor_exhaustive_i16,
        i16
    );
    seg_randomized_static_range_min_max_gcd_xor_exhaustive_test!(
        test_randomized_static_range_min_exhaustive_u16,
        test_randomized_static_range_max_exhaustive_u16,
        test_randomized_static_range_gcd_exhaustive_u16,
        test_randomized_static_range_xor_exhaustive_u16,
        u16
    );
    seg_randomized_static_range_min_max_gcd_xor_exhaustive_test!(
        test_randomized_static_range_min_exhaustive_i32,
        test_randomized_static_range_max_exhaustive_i32,
        test_randomized_static_range_gcd_exhaustive_i32,
        test_randomized_static_range_xor_exhaustive_i32,
        i32
    );
    seg_randomized_static_range_min_max_gcd_xor_exhaustive_test!(
        test_randomized_static_range_min_exhaustive_u32,
        test_randomized_static_range_max_exhaustive_u32,
        test_randomized_static_range_gcd_exhaustive_u32,
        test_randomized_static_range_xor_exhaustive_u32,
        u32
    );
    seg_randomized_static_range_min_max_gcd_xor_exhaustive_test!(
        test_randomized_static_range_min_exhaustive_i64,
        test_randomized_static_range_max_exhaustive_i64,
        test_randomized_static_range_gcd_exhaustive_i64,
        test_randomized_static_range_xor_exhaustive_i64,
        i64
    );
    seg_randomized_static_range_min_max_gcd_xor_exhaustive_test!(
        test_randomized_static_range_min_exhaustive_u64,
        test_randomized_static_range_max_exhaustive_u64,
        test_randomized_static_range_gcd_exhaustive_u64,
        test_randomized_static_range_xor_exhaustive_u64,
        u64
    );
    seg_randomized_static_range_min_max_gcd_xor_exhaustive_test!(
        test_randomized_static_range_min_exhaustive_i128,
        test_randomized_static_range_max_exhaustive_i128,
        test_randomized_static_range_gcd_exhaustive_i128,
        test_randomized_static_range_xor_exhaustive_i128,
        i128
    );
    seg_randomized_static_range_min_max_gcd_xor_exhaustive_test!(
        test_randomized_static_range_min_exhaustive_u128,
        test_randomized_static_range_max_exhaustive_u128,
        test_randomized_static_range_gcd_exhaustive_u128,
        test_randomized_static_range_xor_exhaustive_u128,
        u128
    );
    seg_randomized_static_range_min_max_gcd_xor_exhaustive_test!(
        test_randomized_static_range_min_exhaustive_usize,
        test_randomized_static_range_max_exhaustive_usize,
        test_randomized_static_range_gcd_exhaustive_usize,
        test_randomized_static_range_xor_exhaustive_usize,
        usize
    );

    // ============================================================
    // 1点更新と区間クエリのランダムテスト
    // ============================================================

    macro_rules! seg_randomized_point_set_range_sum_test {
        ($test_name: ident, $ty: ty, $range: expr) => {
            randomized_point_set_range_sum_test!(
                $test_name,
                $ty,
                |v| SegmentTree::<OpAdd<$ty>>::from(v),
                |ds: &SegmentTree<_>, range| ds.fold(range),
                |ds: &mut SegmentTree<_>, index, value| ds.set(index, value),
                20,     // T
                100000, //Q
                100,    // N_MAX
                $range
            );
        };
    }

    macro_rules! seg_randomized_point_set_range_min_max_gcd_xor_test {
        ($min_test_name: ident, $max_test_name: ident, $gcd_test_name: ident, $xor_test_name: ident, $ty: ty) => {
            randomized_point_set_range_min_test!(
                $min_test_name,
                $ty,
                |v| SegmentTree::<OpMin<$ty>>::from(v),
                |ds: &SegmentTree<_>, range| ds.fold(range),
                |ds: &mut SegmentTree<_>, index, value| ds.set(index, value),
                10,     // T
                100000, //Q
                100     // N_MAX
            );

            randomized_point_set_range_max_test!(
                $max_test_name,
                $ty,
                |v| SegmentTree::<OpMax<$ty>>::from(v),
                |ds: &SegmentTree<_>, range| ds.fold(range),
                |ds: &mut SegmentTree<_>, index, value| ds.set(index, value),
                10,     // T
                100000, //Q
                100     // N_MAX
            );

            randomized_point_set_range_gcd_test!(
                $gcd_test_name,
                $ty,
                |v| SegmentTree::<OpGcd<$ty>>::from(v),
                |ds: &SegmentTree<_>, range| ds.fold(range),
                |ds: &mut SegmentTree<_>, index, value| ds.set(index, value),
                10,     // T
                100000, //Q
                100     // N_MAX
            );

            randomized_point_set_range_xor_test!(
                $xor_test_name,
                $ty,
                |v| SegmentTree::<OpXor<$ty>>::from(v),
                |ds: &SegmentTree<_>, range| ds.fold(range),
                |ds: &mut SegmentTree<_>, index, value| ds.set(index, value),
                10,     // T
                100000, //Q
                100     // N_MAX
            );
        };
    }

    seg_randomized_point_set_range_sum_test!(test_randomized_point_set_range_sum_i8, i8, -1..=1);
    seg_randomized_point_set_range_sum_test!(test_randomized_point_set_range_sum_u8, u8, 0..=1);
    seg_randomized_point_set_range_sum_test!(
        test_randomized_point_set_range_sum_i16,
        i16,
        -300..=300
    );
    seg_randomized_point_set_range_sum_test!(test_randomized_point_set_range_sum_u16, u16, 0..=300);
    seg_randomized_point_set_range_sum_test!(
        test_randomized_point_set_range_sum_i32,
        i32,
        -100000..=100000
    );
    seg_randomized_point_set_range_sum_test!(
        test_randomized_point_set_range_sum_u32,
        u32,
        0..=100000
    );
    seg_randomized_point_set_range_sum_test!(
        test_randomized_point_set_range_sum_i64,
        i64,
        -1000000000..=1000000000
    );
    seg_randomized_point_set_range_sum_test!(
        test_randomized_point_set_range_sum_u64,
        u64,
        0..=1000000000
    );
    seg_randomized_point_set_range_sum_test!(
        test_randomized_point_set_range_sum_i128,
        i128,
        -1000000000000000000..=1000000000000000000
    );
    seg_randomized_point_set_range_sum_test!(
        test_randomized_point_set_range_sum_u128,
        u128,
        0..=1000000000000000000
    );
    seg_randomized_point_set_range_sum_test!(
        test_randomized_point_set_range_sum_usize,
        usize,
        0..=1000000000
    );

    seg_randomized_point_set_range_min_max_gcd_xor_test!(
        test_randomized_point_set_range_min_i8,
        test_randomized_point_set_range_max_i8,
        test_randomized_point_set_range_gcd_i8,
        test_randomized_point_set_range_xor_i8,
        i8
    );
    seg_randomized_point_set_range_min_max_gcd_xor_test!(
        test_randomized_point_set_range_min_u8,
        test_randomized_point_set_range_max_u8,
        test_randomized_point_set_range_gcd_u8,
        test_randomized_point_set_range_xor_u8,
        u8
    );
    seg_randomized_point_set_range_min_max_gcd_xor_test!(
        test_randomized_point_set_range_min_i16,
        test_randomized_point_set_range_max_i16,
        test_randomized_point_set_range_gcd_i16,
        test_randomized_point_set_range_xor_i16,
        i16
    );
    seg_randomized_point_set_range_min_max_gcd_xor_test!(
        test_randomized_point_set_range_min_u16,
        test_randomized_point_set_range_max_u16,
        test_randomized_point_set_range_gcd_u16,
        test_randomized_point_set_range_xor_u16,
        u16
    );
    seg_randomized_point_set_range_min_max_gcd_xor_test!(
        test_randomized_point_set_range_min_i32,
        test_randomized_point_set_range_max_i32,
        test_randomized_point_set_range_gcd_i32,
        test_randomized_point_set_range_xor_i32,
        i32
    );
    seg_randomized_point_set_range_min_max_gcd_xor_test!(
        test_randomized_point_set_range_min_u32,
        test_randomized_point_set_range_max_u32,
        test_randomized_point_set_range_gcd_u32,
        test_randomized_point_set_range_xor_u32,
        u32
    );
    seg_randomized_point_set_range_min_max_gcd_xor_test!(
        test_randomized_point_set_range_min_i64,
        test_randomized_point_set_range_max_i64,
        test_randomized_point_set_range_gcd_i64,
        test_randomized_point_set_range_xor_i64,
        i64
    );
    seg_randomized_point_set_range_min_max_gcd_xor_test!(
        test_randomized_point_set_range_min_u64,
        test_randomized_point_set_range_max_u64,
        test_randomized_point_set_range_gcd_u64,
        test_randomized_point_set_range_xor_u64,
        u64
    );
    seg_randomized_point_set_range_min_max_gcd_xor_test!(
        test_randomized_point_set_range_min_i128,
        test_randomized_point_set_range_max_i128,
        test_randomized_point_set_range_gcd_i128,
        test_randomized_point_set_range_xor_i128,
        i128
    );
    seg_randomized_point_set_range_min_max_gcd_xor_test!(
        test_randomized_point_set_range_min_u128,
        test_randomized_point_set_range_max_u128,
        test_randomized_point_set_range_gcd_u128,
        test_randomized_point_set_range_xor_u128,
        u128
    );
    seg_randomized_point_set_range_min_max_gcd_xor_test!(
        test_randomized_point_set_range_min_usize,
        test_randomized_point_set_range_max_usize,
        test_randomized_point_set_range_gcd_usize,
        test_randomized_point_set_range_xor_usize,
        usize
    );

    // ============================================================
    // エッジケースなど
    // ============================================================

    #[test]
    fn test_size_boundaries() {
        // 最小サイズ（1要素）
        {
            let seg = SegmentTree::<OpAdd<i32>>::from(vec![42]);
            assert_eq!(seg.fold(..), 42);
            assert_eq!(seg.fold(0..1), 42);
        }

        // 2の冪サイズ
        for size in [1, 2, 4, 8, 16, 32] {
            let v: Vec<i32> = (0..size).map(|i| i as i32).collect();
            let seg = SegmentTree::<OpAdd<i32>>::from(v.clone());
            assert_eq!(seg.fold(..), v.iter().sum());
        }

        // 非2の冪サイズ
        for size in [3, 5, 7, 9, 17, 33] {
            let v: Vec<i32> = (0..size).map(|i| i as i32).collect();
            let seg = SegmentTree::<OpAdd<i32>>::from(v.clone());
            assert_eq!(seg.fold(..), v.iter().sum());
        }

        // 境界値での操作確認
        {
            let mut seg = SegmentTree::<OpMin<i32>>::from(vec![5, 3, 8, 1, 9]);
            seg.set(0, i32::MAX);
            seg.set(4, 0);
            assert_eq!(seg.fold(..), 0);
        }
    }

    #[test]
    fn test_value_extremes() {
        // OpMin/OpMax での極値
        {
            let mut seg = SegmentTree::<OpMin<i32>>::from(vec![i32::MIN, i32::MAX, 0]);
            assert_eq!(seg.fold(..), i32::MIN);
            seg.set(0, i32::MAX);
            assert_eq!(seg.fold(..), 0);
        }

        {
            let mut seg = SegmentTree::<OpMax<u32>>::from(vec![0, u32::MAX, u32::MAX / 2]);
            assert_eq!(seg.fold(..), u32::MAX);
            seg.set(1, 0);
            assert_eq!(seg.fold(..), u32::MAX / 2);
        }

        // OpXor での全ビットセット
        {
            let seg = SegmentTree::<OpXor<u64>>::from(vec![u64::MAX, u64::MAX, u64::MAX]);
            assert_eq!(seg.fold(..), u64::MAX); // MAX ^ MAX ^ MAX = MAX
        }

        // OpGcd でのゼロ
        {
            let seg = SegmentTree::<OpGcd<i32>>::from(vec![0, 12, 0, 18]);
            assert_eq!(seg.fold(..), 6);
            assert_eq!(seg.fold(0..2), 12);
            assert_eq!(seg.fold(2..4), 18);
        }
    }

    #[test]
    fn test_range_boundaries() {
        let seg = SegmentTree::<OpAdd<i32>>::from(vec![1, 2, 3, 4, 5]);

        // 単一要素範囲
        assert_eq!(seg.fold(0..1), 1);
        assert_eq!(seg.fold(2..3), 3);
        assert_eq!(seg.fold(4..5), 5);

        // 全区間
        assert_eq!(seg.fold(..), 15);
        assert_eq!(seg.fold(0..5), 15);
        assert_eq!(seg.fold(0..=4), 15);

        // 前置範囲
        assert_eq!(seg.fold(..3), 6);
        assert_eq!(seg.fold(..=2), 6);

        // 後置範囲
        assert_eq!(seg.fold(2..), 12);
        assert_eq!(seg.fold(3..), 9);

        // 空範囲->単位元を返すことを確認
        assert_eq!(seg.fold(0..0), 0);
        assert_eq!(seg.fold(5..5), 0);
        assert_eq!(seg.fold(2..2), 0);
    }

    #[test]
    fn test_range_bounds_variants() {
        let seg = SegmentTree::<OpMin<i64>>::from(vec![8, 2, 10, 3, 4, 1, 5, 9]);

        // .. (全区間)
        assert_eq!(seg.fold(..), 1);

        // ..a
        assert_eq!(seg.fold(..3), 2);
        assert_eq!(seg.fold(..=2), 2);

        // a..
        assert_eq!(seg.fold(2..), 1);
        assert_eq!(seg.fold(5..), 1);

        // a..b
        assert_eq!(seg.fold(1..4), 2);
        assert_eq!(seg.fold(3..6), 1);

        // a..=b
        assert_eq!(seg.fold(1..=3), 2);
        assert_eq!(seg.fold(3..=5), 1);

        // 境界値
        assert_eq!(seg.fold(..0), i64::MAX);
        assert_eq!(seg.fold(0..1), 8);
        assert_eq!(seg.fold(7..), 9);
        assert_eq!(seg.fold(..=7), 1);
    }

    #[test]
    fn test_bisect_edge_cases() {
        // 単一要素配列
        {
            let seg = SegmentTree::<OpAdd<i32>>::from(vec![5]);

            // bisect_right: 累積和が条件を満たす最大位置
            assert_eq!(seg.bisect_right(0, |&x| x <= 5), 1);
            assert_eq!(seg.bisect_right(0, |&x| x <= 4), 0);

            // bisect_left: 後ろから累積和が条件を満たす最小位置
            assert_eq!(seg.bisect_left(1, |&x| x <= 5), 0);
            assert_eq!(seg.bisect_left(1, |&x| x <= 4), 1);

            // 単位元のみの場合
            assert_eq!(seg.bisect_right(0, |&x| x <= 0), 0); // 単位元のみなら0
            assert_eq!(seg.bisect_left(1, |&x| x <= 0), 1); // 後ろから見て単位元のみなら1
        }

        // 常にtrueな述語
        {
            let seg = SegmentTree::<OpAdd<i32>>::from(vec![1, 2, 3, 4, 5]);

            assert_eq!(seg.bisect_right(0, |_| true), 5);
            assert_eq!(seg.bisect_left(5, |_| true), 0);
        }

        // 境界位置
        {
            let seg = SegmentTree::<OpAdd<i32>>::from(vec![1, 2, 3, 4, 5]);

            assert_eq!(seg.bisect_right(0, |&x| x <= 15), 5); // 全部含んでも15以下
            assert_eq!(seg.bisect_right(0, |&x| x < 6), 2); // fold(0..2)=3<6, fold(0..3)=6>=6
            assert_eq!(seg.bisect_left(5, |&x| x <= 15), 0); // 全部含んでも15以下
        }

        // 全て同じ値の配列
        {
            let seg = SegmentTree::<OpAdd<i32>>::from(vec![5, 5, 5, 5]);

            assert_eq!(seg.bisect_right(0, |&x| x <= 20), 4);
            assert_eq!(seg.bisect_right(0, |&x| x <= 15), 3);
            assert_eq!(seg.bisect_right(0, |&x| x <= 5), 1);
            assert_eq!(seg.bisect_right(0, |&x| x <= 4), 0);
        }

        // 単調増加配列
        {
            let seg = SegmentTree::<OpAdd<i32>>::from(vec![1, 2, 3, 4, 5]);

            assert_eq!(seg.bisect_right(0, |&x| x <= 6), 3); // 1+2+3=6
            assert_eq!(seg.bisect_right(0, |&x| x <= 10), 4); // 1+2+3+4=10
        }

        // 2の冪サイズでの境界動作
        {
            // サイズ8（2^3）
            let seg = SegmentTree::<OpAdd<i32>>::from(vec![1; 8]); // [1,1,1,1,1,1,1,1]

            // 前半部分 [0..4)
            assert_eq!(seg.bisect_right(0, |&x| x <= 4), 4); // fold(0..4)=4
            assert_eq!(seg.bisect_right(0, |&x| x <= 3), 3);

            // 後半部分 [4..8)
            assert_eq!(seg.bisect_right(4, |&x| x <= 4), 8); // fold(4..8)=4
            assert_eq!(seg.bisect_right(4, |&x| x <= 3), 7);

            // bisect_left
            assert_eq!(seg.bisect_left(8, |&x| x <= 4), 4); // fold(4..8)=4
            assert_eq!(seg.bisect_left(8, |&x| x <= 3), 5);
        }

        // 空配列に近いケース（最小サイズ）
        {
            let seg = SegmentTree::<OpAdd<i32>>::from(vec![0]);

            // f(identity) = f(0) = true となる述語のみ使用
            // x <= 0: 0 <= 0 は true
            assert_eq!(seg.bisect_right(0, |&x| x <= 0), 1); // fold(0..1)=0 <= 0
            // x >= 0: 0 >= 0 は true
            assert_eq!(seg.bisect_left(1, |&x| x >= 0), 0); // fold(0..1)=0 >= 0
        }

        // 開始位置からの累積和がちょうど境界のケース
        {
            let seg = SegmentTree::<OpAdd<i32>>::from(vec![3, 3, 3, 3, 3]); // [3,3,3,3,3]

            // 位置2から開始
            assert_eq!(seg.bisect_right(2, |&x| x <= 6), 4); // fold(2..4)=6
            assert_eq!(seg.bisect_right(2, |&x| x <= 5), 3); // fold(2..3)=3, fold(2..4)=6

            // bisect_left
            assert_eq!(seg.bisect_left(4, |&x| x <= 6), 2); // fold(2..4)=6
            assert_eq!(seg.bisect_left(4, |&x| x <= 5), 3); // fold(3..4)=3
        }

        // Min/Maxモノイドでのbisect
        {
            let seg = SegmentTree::<OpMin<i32>>::from(vec![5, 3, 8, 2, 9, 6]);

            assert_eq!(seg.bisect_right(0, |&x| x >= 2), 6);
            assert_eq!(seg.bisect_left(6, |&x| x >= 3), 4);
        }
    }

    #[test]
    fn test_entry_mut_edge_cases() {
        // 境界インデックス
        {
            let mut seg = SegmentTree::<OpAdd<i32>>::from(vec![1, 2, 3, 4, 5]);

            {
                let mut e = seg.entry_mut(0);
                *e *= 10;
            }
            assert_eq!(seg[0], 10);

            {
                let mut e = seg.entry_mut(4);
                *e += 100;
            }
            assert_eq!(seg[4], 105);
        }

        // 境界でのin-place更新
        {
            let mut seg = SegmentTree::<OpMax<i32>>::from(vec![10, 20, 30]);

            {
                let mut e = seg.entry_mut(1);
                *e *= 2;
            }
            assert_eq!(seg[1], 40);
            assert_eq!(seg.fold(..), 40);

            {
                let mut e = seg.entry_mut(0);
                *e = 100;
            }
            assert_eq!(seg.fold(..), 100);
        }

        // 連続するentry_mut
        {
            let mut seg = SegmentTree::<OpMin<i32>>::from(vec![5, 3, 8]);

            {
                let mut e = seg.entry_mut(0);
                *e = 1;
            }
            assert_eq!(seg.fold(..), 1);

            {
                let mut e = seg.entry_mut(1);
                *e = 0;
            }
            assert_eq!(seg.fold(..), 0);

            {
                let mut e = seg.entry_mut(2);
                *e = -5;
            }
            assert_eq!(seg.fold(..), -5);
        }

        // カスタムモノイドでの使用
        {
            #[derive(Default)]
            struct OpMul;
            impl Monoid for OpMul {
                type Element = i64;
                fn id(&self) -> Self::Element {
                    1
                }
                fn op(&self, &lhs: &Self::Element, &rhs: &Self::Element) -> Self::Element {
                    lhs * rhs
                }
            }

            let mut seg = SegmentTree::from((vec![2, 3, 4], OpMul));

            {
                let mut e = seg.entry_mut(1);
                *e *= 5;
            }
            assert_eq!(seg[1], 15);
            assert_eq!(seg.fold(..), 120); // 2 * 15 * 4
        }
    }

    #[test]
    fn test_zero_and_identity_elements() {
        // 全要素が0
        {
            let seg = SegmentTree::<OpAdd<i32>>::from(vec![0, 0, 0, 0]);
            assert_eq!(seg.fold(..), 0);
            assert_eq!(seg.fold(1..3), 0);
        }

        {
            let seg = SegmentTree::<OpXor<u32>>::from(vec![0, 0, 0, 0]);
            assert_eq!(seg.fold(..), 0);
        }

        // 全要素が単位元
        {
            let seg = SegmentTree::<OpMin<i32>>::from(vec![i32::MAX; 5]);
            assert_eq!(seg.fold(..), i32::MAX);
        }

        {
            let seg = SegmentTree::<OpMax<i32>>::from(vec![i32::MIN; 5]);
            assert_eq!(seg.fold(..), i32::MIN);
        }

        // ゼロ混在
        {
            let seg = SegmentTree::<OpAdd<i32>>::from(vec![0, 5, 0, 10, 0]);
            assert_eq!(seg.fold(..), 15);
            assert_eq!(seg.fold(1..4), 15);
        }

        {
            let seg = SegmentTree::<OpGcd<i32>>::from(vec![0, 12, 0, 18, 0]);
            assert_eq!(seg.fold(..), 6);
        }
    }

    #[test]
    fn test_power_of_two_sizes() {
        // 各種2の冪サイズ
        for size in [1, 2, 4, 8, 16, 32] {
            let v: Vec<i32> = (0..size).map(|i| i as i32).collect();
            let seg = SegmentTree::<OpAdd<i32>>::from(v.clone());

            assert_eq!(seg.fold(..), v.iter().sum());
            assert_eq!(seg.fold(0..size / 2), v[..size / 2].iter().sum());
        }

        // 2の冪より少し大きいサイズ
        for size in [3, 5, 9, 17, 33] {
            let v: Vec<i32> = (0..size).map(|i| i as i32).collect();
            let seg = SegmentTree::<OpAdd<i32>>::from(v.clone());

            assert_eq!(seg.fold(..), v.iter().sum());
        }

        // 2の冪境界でのfold動作確認
        {
            let seg =
                SegmentTree::<OpMin<i32>>::from((0..16).map(|i| i as i32).collect::<Vec<_>>());
            assert_eq!(seg.fold(0..8), 0);
            assert_eq!(seg.fold(8..16), 8);
        }
    }

    #[test]
    fn test_sequential_operations() {
        // 連続set
        {
            let mut seg = SegmentTree::<OpAdd<i32>>::from(vec![0; 5]);
            for i in 0..5 {
                seg.set(i, i as i32 * 2);
            }
            assert_eq!(seg.fold(..), 20);
        }

        // setとfoldの交互実行
        {
            let mut seg = SegmentTree::<OpMax<i32>>::from(vec![1, 5, 3, 7, 2]);
            assert_eq!(seg.fold(..), 7);
            seg.set(2, 10);
            assert_eq!(seg.fold(..), 10);
            assert_eq!(seg.fold(0..3), 10);
            seg.set(4, 15);
            assert_eq!(seg.fold(..), 15);
        }

        // 連続bisect
        {
            let seg = SegmentTree::<OpAdd<i32>>::from(vec![1, 2, 3, 4, 5]);

            let pos1 = seg.bisect_right(0, |&x| x < 6);
            assert_eq!(pos1, 2);

            let pos2 = seg.bisect_right(pos1, |&x| x < 9);
            assert_eq!(pos2, 4);

            let pos3 = seg.bisect_left(5, |&x| x <= 15);
            assert_eq!(pos3, 0);
        }
    }

    #[test]
    fn test_special_monoids() {
        // OpGcd
        {
            let seg = SegmentTree::<OpGcd<i32>>::from(vec![0, 12, -18, 0, 24]);
            assert_eq!(seg.fold(..), 6);
            assert_eq!(seg.fold(1..3), 6);
            assert_eq!(seg.fold(2..4), 18);
        }

        {
            let seg = SegmentTree::<OpGcd<i32>>::from(vec![12, 18, 24]);
            assert_eq!(seg.fold(..), 6);
        }

        // OpXor
        {
            let seg = SegmentTree::<OpXor<u64>>::from(vec![u64::MAX, 0, u64::MAX]);
            assert_eq!(seg.fold(..), 0); // MAX ^ 0 ^ MAX = 0
        }

        {
            let seg = SegmentTree::<OpXor<u32>>::from(vec![0b1010, 0b1100, 0b0011]);
            assert_eq!(seg.fold(..), 0b0101);
        }

        {
            let seg = SegmentTree::<OpXor<u8>>::from(vec![255, 255, 255]);
            assert_eq!(seg.fold(..), 255); // 255 ^ 255 ^ 255 = 255
        }
    }
}
