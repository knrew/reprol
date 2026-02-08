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
    utils::range_utils::to_half_open_index_range,
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
    use rand::Rng;

    use super::*;
    use crate::{
        math::gcd::Gcd,
        ops::{
            act_add::ActAdd, act_set::ActSet, op_add::OpAdd, op_gcd::OpGcd, op_max::OpMax,
            op_min::OpMin, op_xor::OpXor,
        },
        utils::test_utils::{dynamic_range_query::*, random::get_test_rng, static_range_query::*},
    };

    // ============================================================
    // 基本的な機能テスト
    // ============================================================

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

        {
            let mut seg = LazySegmentTree::<Op, Act>::from(vec![1, 2, 3, 4]);
            assert_eq!(seg.fold(..), 4);
            seg.set(2, 10);
            assert_eq!(seg.fold(..), 10);
            *seg.entry_mut(0) += 10;
            assert_eq!(seg.fold(..), 11);
        }
    }

    // ============================================================
    // 静的クエリの網羅的テスト（ランダム化）
    // ============================================================

    macro_rules! seg_randomized_static_range_sum_exhaustive_test {
        ($test_name: ident, $ty: ty, $range: expr) => {
            randomized_static_range_sum_exhaustive_test!(
                $test_name,
                $ty,
                |v| LazySegmentTree::<OpAdd<$ty>, ActSet<$ty>>::from(v),
                |ds: &mut LazySegmentTree<_, _>, range| ds.fold(range),
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
                |v| LazySegmentTree::<OpMin<$ty>, ActSet<$ty>>::from(v),
                |ds: &mut LazySegmentTree<_, _>, range| ds.fold(range),
                200,
                100
            );

            randomized_static_range_max_exhaustive_test!(
                $max_test_name,
                $ty,
                |v| LazySegmentTree::<OpMax<$ty>, ActAdd<_>>::from(v),
                |ds: &mut LazySegmentTree<_, _>, range| ds.fold(range),
                200,
                100
            );

            randomized_static_range_gcd_exhaustive_test!(
                $gcd_test_name,
                $ty,
                |v| LazySegmentTree::<OpGcd<$ty>, ActAdd<_>>::from(v),
                |ds: &mut LazySegmentTree<_, _>, range| ds.fold(range),
                100,
                100
            );

            randomized_static_range_xor_exhaustive_test!(
                $xor_test_name,
                $ty,
                |v| LazySegmentTree::<OpXor<$ty>, ActSet<$ty>>::from(v),
                |ds: &mut LazySegmentTree<_, _>, range| ds.fold(range),
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
    // 点更新と範囲クエリのランダム化テスト
    // ============================================================

    macro_rules! seg_randomized_point_set_range_sum_test {
        ($test_name: ident, $ty: ty, $range: expr) => {
            randomized_point_set_range_sum_test!(
                $test_name,
                $ty,
                |v| LazySegmentTree::<OpAdd<$ty>, ActSet<$ty>>::from(v),
                |ds: &mut LazySegmentTree<_, _>, range| ds.fold(range),
                |ds: &mut LazySegmentTree<_, _>, index, value| ds.set(index, value),
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
                |v| LazySegmentTree::<OpMin<$ty>, ActAdd<$ty>>::from(v),
                |ds: &mut LazySegmentTree<_, _>, range| ds.fold(range),
                |ds: &mut LazySegmentTree<_, _>, index, value| ds.set(index, value),
                10,     // T
                100000, //Q
                100     // N_MAX
            );

            randomized_point_set_range_max_test!(
                $max_test_name,
                $ty,
                |v| LazySegmentTree::<OpMax<$ty>, ActSet<$ty>>::from(v),
                |ds: &mut LazySegmentTree<_, _>, range| ds.fold(range),
                |ds: &mut LazySegmentTree<_, _>, index, value| ds.set(index, value),
                10,     // T
                100000, //Q
                100     // N_MAX
            );

            randomized_point_set_range_gcd_test!(
                $gcd_test_name,
                $ty,
                |v| LazySegmentTree::<OpGcd<$ty>, ActAdd<$ty>>::from(v),
                |ds: &mut LazySegmentTree<_, _>, range| ds.fold(range),
                |ds: &mut LazySegmentTree<_, _>, index, value| ds.set(index, value),
                10,     // T
                100000, //Q
                100     // N_MAX
            );

            randomized_point_set_range_xor_test!(
                $xor_test_name,
                $ty,
                |v| LazySegmentTree::<OpXor<$ty>, ActSet<_>>::from(v),
                |ds: &mut LazySegmentTree<_, _>, range| ds.fold(range),
                |ds: &mut LazySegmentTree<_, _>, index, value| ds.set(index, value),
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
    // カスタム実装による範囲作用テスト
    //   - 範囲加算 + 区間和取得（Node構造体使用）
    //   - 範囲代入 + 区間和取得（Node構造体使用）
    // ============================================================

    #[test]
    fn test_random_range_add_range_sum_i64() {
        #[derive(Clone, PartialEq, Eq)]
        struct Node {
            value: i64,
            len: i64,
        }

        #[derive(Default)]
        struct Op;

        impl Monoid for Op {
            type Element = Node;

            fn op(&self, lhs: &Self::Element, rhs: &Self::Element) -> Self::Element {
                Node {
                    value: lhs.value + rhs.value,
                    len: lhs.len + rhs.len,
                }
            }

            fn id(&self) -> Self::Element {
                Node { value: 0, len: 0 }
            }
        }

        #[derive(Default)]
        struct Act;

        impl Monoid for Act {
            type Element = i64;

            fn op(&self, g: &Self::Element, f: &Self::Element) -> Self::Element {
                g + f
            }

            fn id(&self) -> Self::Element {
                0
            }
        }

        impl Action<Op> for Act {
            fn act(
                &self,
                f: &Self::Element,
                x: &<Op as Monoid>::Element,
            ) -> <Op as Monoid>::Element {
                Node {
                    value: x.value + f * x.len,
                    len: x.len,
                }
            }
        }

        const NUM_TESTCASES: usize = 20;
        const NUM_QUERIES: usize = 100000;
        const NUM_ELEMENTS_MAX: usize = 100;
        const RANGE: Range<i64> = -100000..100001;

        let mut rng = get_test_rng();

        for _ in 0..NUM_TESTCASES {
            let n = rng.random_range(1..=NUM_ELEMENTS_MAX);

            let mut v_naive = (0..n).map(|_| rng.random_range(RANGE)).collect::<Vec<_>>();
            let mut seg = LazySegmentTree::<Op, Act>::from_iter(
                v_naive.iter().cloned().map(|vi| Node { value: vi, len: 1 }),
            );

            for _ in 0..NUM_QUERIES {
                match rng.random_range(0..=1) {
                    0 => {
                        let l = rng.random_range(0..n);
                        let r = rng.random_range(l + 1..=n);
                        let f = rng.random_range(RANGE);
                        for e in v_naive[l..r].iter_mut() {
                            *e = *e + f;
                        }
                        seg.act(l..r, &f);
                    }
                    1 => {
                        let l = rng.random_range(0..n);
                        let r = rng.random_range(l + 1..=n);
                        let naive = v_naive[l..r].iter().sum::<i64>();
                        assert_eq!(seg.fold(l..r).value, naive);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    #[test]
    fn test_random_range_set_range_sum_i64() {
        #[derive(Clone, PartialEq, Eq)]
        struct Node {
            value: i64,
            len: i64,
        }

        #[derive(Default)]
        struct Op;

        impl Monoid for Op {
            type Element = Node;

            fn op(&self, lhs: &Self::Element, rhs: &Self::Element) -> Self::Element {
                Node {
                    value: lhs.value + rhs.value,
                    len: lhs.len + rhs.len,
                }
            }

            fn id(&self) -> Self::Element {
                Node { value: 0, len: 0 }
            }
        }

        #[derive(Default)]
        struct Act;

        impl Monoid for Act {
            type Element = Option<i64>;

            fn op(&self, g: &Self::Element, f: &Self::Element) -> Self::Element {
                *if g.is_none() { f } else { g }
            }

            fn id(&self) -> Self::Element {
                None
            }
        }

        impl Action<Op> for Act {
            fn act(
                &self,
                f: &Self::Element,
                x: &<Op as Monoid>::Element,
            ) -> <Op as Monoid>::Element {
                if let Some(f) = f {
                    Node {
                        value: f * x.len,
                        len: x.len,
                    }
                } else {
                    x.clone()
                }
            }
        }

        const NUM_TESTCASES: usize = 20;
        const NUM_QUERIES: usize = 100000;
        const NUM_ELEMENTS_MAX: usize = 100;
        const RANGE: Range<i64> = -100000..100001;

        let mut rng = get_test_rng();

        for _ in 0..NUM_TESTCASES {
            let n = rng.random_range(1..=NUM_ELEMENTS_MAX);

            let mut v_naive = (0..n).map(|_| rng.random_range(RANGE)).collect::<Vec<_>>();
            let mut seg = LazySegmentTree::<Op, Act>::from_iter(
                v_naive.iter().cloned().map(|vi| Node { value: vi, len: 1 }),
            );

            for _ in 0..NUM_QUERIES {
                match rng.random_range(0..=1) {
                    0 => {
                        let l = rng.random_range(0..n);
                        let r = rng.random_range(l + 1..=n);
                        let f = rng.random_range(RANGE);
                        for e in v_naive[l..r].iter_mut() {
                            *e = f;
                        }
                        seg.act(l..r, &Some(f));
                    }
                    1 => {
                        let l = rng.random_range(0..n);
                        let r = rng.random_range(l + 1..=n);
                        let naive = v_naive[l..r].iter().sum::<i64>();
                        assert_eq!(seg.fold(l..r).value, naive);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    /// 範囲actと範囲foldクエリのランダムテストを生成するマクロ
    ///
    /// # パラメータ
    /// - `$test_name`: テスト関数の名前
    /// - `$ty`: 要素の型
    /// - `$naive_op`: fold演算(例: `|a: $ty, b| a + b`)
    /// - `$naive_id`: fold演算の単位元
    /// - `$naive_act`: act演算(例: `|x: $ty, f: $ty| x + f`)
    /// - `$ds_from_vec`: データ構造をVecから構築する式
    /// - `$ds_fold`: データ構造のfold操作
    /// - `$ds_act`: データ構造のact操作
    /// - `$num_testcases`: テストケースの数
    /// - `$num_queries`: 各テストケースでのクエリ数
    /// - `$num_elements_max`: 配列サイズの最大値
    /// - `$element_value_range`: 要素値の範囲
    /// - `$action_value_range`: 作用値の範囲
    macro_rules! randomized_range_act_range_fold_test {
        (
        $test_name: ident,
        $ty: ty,
        $naive_op: expr,
        $naive_id: expr,
        $naive_act: expr,
        $ds_from_vec: expr,
        $ds_fold: expr,
        $ds_act: expr,
        $num_testcases: expr,
        $num_queries: expr,
        $num_elements_max: expr,
        $element_value_range: expr,
        $action_value_range: expr
    ) => {
            #[test]
            fn $test_name() {
                let mut rng = get_test_rng();

                for _ in 0..$num_testcases {
                    let n = rng.random_range(1..=$num_elements_max);

                    let mut v_naive: Vec<$ty> = (0..n)
                        .map(|_| rng.random_range($element_value_range))
                        .collect();
                    let mut ds = $ds_from_vec(v_naive.clone());

                    for _ in 0..$num_queries {
                        match rng.random_range(0..=1) {
                            0 => {
                                let l = rng.random_range(0..n);
                                let r = rng.random_range(l + 1..=n);
                                let f = rng.random_range($action_value_range);
                                for e in v_naive[l..r].iter_mut() {
                                    *e = $naive_act(*e, f);
                                }
                                $ds_act(&mut ds, l..r, f);
                            }
                            1 => {
                                let l = rng.random_range(0..n);
                                let r = rng.random_range(l + 1..=n);
                                let naive = v_naive[l..r]
                                    .iter()
                                    .fold($naive_id, |prod, &vi| $naive_op(prod, vi));
                                assert_eq!($ds_fold(&mut ds, l..r), naive);
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
        };
    }

    macro_rules! randomized_range_add_range_min_test {
        ($test_name: ident, $ty: ty, $range: expr) => {
            randomized_range_act_range_fold_test!(
                $test_name,
                $ty,
                |a: $ty, b| a.min(b),
                <$ty>::MAX,
                |x: $ty, f| x.wrapping_add(f),
                |v| LazySegmentTree::<OpMin<$ty>, ActAdd<$ty>>::from(v),
                |ds: &mut LazySegmentTree<_, _>, range| ds.fold(range),
                |ds: &mut LazySegmentTree<_, _>, range, f| ds.act(range, &f),
                20,     // T
                100000, //Q
                100,    // N_MAX
                $range,
                $range
            );
        };
    }

    macro_rules! randomized_range_set_range_max_test {
        ($test_name: ident, $ty: ty) => {
            randomized_range_act_range_fold_test!(
                $test_name,
                $ty,
                |a: $ty, b| a.max(b),
                <$ty>::MIN,
                |_: $ty, f| f,
                |v| LazySegmentTree::<OpMax<$ty>, ActSet<$ty>>::from(v),
                |ds: &mut LazySegmentTree<_, _>, range| ds.fold(range),
                |ds: &mut LazySegmentTree<_, _>, range, f| ds.act(range, &Some(f)),
                20,     // T
                100000, //Q
                100,    // N_MAX
                <$ty>::MIN..=<$ty>::MAX,
                <$ty>::MIN..=<$ty>::MAX
            );
        };
    }

    randomized_range_add_range_min_test!(
        test_randomized_range_add_range_min_i32,
        i32,
        -100000..=100000
    );
    randomized_range_add_range_min_test!(test_randomized_range_add_range_min_u32, u32, 0..=100000);
    randomized_range_add_range_min_test!(
        test_randomized_range_add_range_min_i64,
        i64,
        -1000000000..=1000000000
    );
    randomized_range_add_range_min_test!(
        test_randomized_range_add_range_min_u64,
        u64,
        0..=1000000000
    );
    randomized_range_add_range_min_test!(
        test_randomized_range_add_range_min_usize,
        usize,
        0..=1000000000
    );
    randomized_range_set_range_max_test!(test_randomized_range_set_range_max_i32, i32);
    randomized_range_set_range_max_test!(test_randomized_range_set_range_max_u32, u32);
    randomized_range_set_range_max_test!(test_randomized_range_set_range_max_i64, i64);
    randomized_range_set_range_max_test!(test_randomized_range_set_range_max_u64, u64);
    randomized_range_set_range_max_test!(test_randomized_range_set_range_max_usize, usize);

    // ============================================================
    // エッジケースと複雑なパターンのテスト
    // ============================================================

    #[test]
    fn test_empty_range() {
        type Op = OpMin<i32>;
        type Act = ActAdd<i32>;

        let mut seg = LazySegmentTree::<Op, Act>::from(vec![1, 2, 3, 4, 5]);

        // 空区間でのactは何もしない
        let original_min = seg.fold(..);
        seg.act(0..0, &100);
        assert_eq!(seg.fold(..), original_min);

        seg.act(2..2, &100);
        assert_eq!(seg.fold(..), original_min);

        seg.act(5..5, &100);
        assert_eq!(seg.fold(..), original_min);

        // 空区間でのfoldは単位元を返す
        assert_eq!(seg.fold(0..0), i32::MAX);
        assert_eq!(seg.fold(2..2), i32::MAX);
        assert_eq!(seg.fold(5..5), i32::MAX);
    }

    #[test]
    fn test_boundary_values() {
        // i32 MIN/MAX
        {
            type Op = OpMin<i32>;
            type Act = ActAdd<i32>;

            let mut seg = LazySegmentTree::<Op, Act>::from(vec![i32::MIN, i32::MAX, 0]);
            assert_eq!(seg.fold(..), i32::MIN);
            seg.act(0..1, &1);
            assert_eq!(*seg.get(0), i32::MIN + 1);
        }

        // u32 MIN/MAX
        {
            type Op = OpMax<u32>;
            type Act = ActAdd<u32>;

            let mut seg = LazySegmentTree::<Op, Act>::from(vec![0, u32::MAX, u32::MAX / 2]);
            assert_eq!(seg.fold(..), u32::MAX);
            seg.act(0..1, &1);
            assert_eq!(*seg.get(0), 1);
        }
    }

    #[test]
    fn test_entry_mut_with_act() {
        type Op = OpMin<i32>;
        type Act = ActAdd<i32>;

        let mut seg = LazySegmentTree::<Op, Act>::from(vec![1, 2, 3, 4, 5]);

        // act → entry_mut → fold の順序
        seg.act(0..3, &10);
        {
            let mut entry = seg.entry_mut(1);
            *entry = 100;
        }
        assert_eq!(seg.fold(0..3), 11); // min(11, 100, 13) = 11

        // 遅延伝播の確認
        assert_eq!(*seg.get(0), 11);
        assert_eq!(*seg.get(1), 100);
        assert_eq!(*seg.get(2), 13);

        // entry_mut → act → fold の順序
        let mut seg2 = LazySegmentTree::<Op, Act>::from(vec![10, 20, 30]);
        {
            let mut entry = seg2.entry_mut(1);
            *entry = 50;
        }
        seg2.act(0..3, &5);
        assert_eq!(seg2.fold(..), 15); // min(15, 55, 35) = 15

        // entry_mutで遅延作用が正しく伝播されることを確認
        let mut seg3 = LazySegmentTree::<Op, Act>::from(vec![1, 2, 3, 4]);
        seg3.act(0..4, &10);
        {
            let mut entry = seg3.entry_mut(1);
            assert_eq!(*entry, 12); // 遅延伝播されて取得できる
            *entry = 50;
        }
        assert_eq!(seg3.fold(0..2), 11); // min(11, 50) = 11
    }

    #[test]
    fn test_action_composition() {
        // ActSetの順序依存性（後勝ち）の検証
        {
            type Op = OpMax<i32>;
            type Act = ActSet<i32>;

            let mut seg = LazySegmentTree::<Op, Act>::from(vec![5, 5, 5, 5, 5]);

            // 後の作用が勝つ
            seg.act(1..4, &Some(10));
            seg.act(2..5, &Some(7));

            // [5, 10, 7, 7, 7] になるはず
            assert_eq!(*seg.get(0), 5);
            assert_eq!(*seg.get(1), 10);
            assert_eq!(*seg.get(2), 7);
            assert_eq!(*seg.get(3), 7);
            assert_eq!(*seg.get(4), 7);
            assert_eq!(seg.fold(..), 10);
        }
    }

    /// より複雑な作用パターンのテスト
    #[test]
    fn test_complex_action_patterns() {
        type Op = OpMin<i32>;
        type Act = ActAdd<i32>;

        let mut seg = LazySegmentTree::<Op, Act>::from(vec![10, 20, 30, 40, 50, 60, 70, 80]);

        // 交互の区間に作用
        seg.act(0..4, &5);
        seg.act(4..8, &10);
        assert_eq!(seg.fold(0..4), 15); // min(15, 25, 35, 45)
        assert_eq!(seg.fold(4..8), 60); // min(60, 70, 80, 90)

        // 広い区間に作用で上書き
        seg.act(0..8, &100);
        assert_eq!(seg.fold(..), 115); // min(115, 125, ..., 190)

        // 狭い区間で部分更新
        seg.act(2..5, &-200);
        assert_eq!(seg.fold(2..5), -65); // min(-65, -55, -40)
    }

    #[test]
    fn test_range_affine_range_sum() {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        struct Node {
            sum: i64,
            len: i64,
        }

        #[derive(Default)]
        struct Op;

        impl Monoid for Op {
            type Element = Node;

            fn id(&self) -> Self::Element {
                Node { sum: 0, len: 0 }
            }

            fn op(&self, lhs: &Self::Element, rhs: &Self::Element) -> Self::Element {
                Node {
                    sum: lhs.sum + rhs.sum,
                    len: lhs.len + rhs.len,
                }
            }
        }

        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        struct Affine {
            a: i64,
            b: i64,
        }

        #[derive(Default)]
        struct ActAffine;

        impl Monoid for ActAffine {
            type Element = Affine;

            fn id(&self) -> Self::Element {
                Affine { a: 1, b: 0 }
            }

            fn op(&self, g: &Self::Element, f: &Self::Element) -> Self::Element {
                Affine {
                    a: g.a * f.a,
                    b: g.a * f.b + g.b,
                }
            }
        }

        impl Action<Op> for ActAffine {
            fn act(
                &self,
                f: &Self::Element,
                x: &<Op as Monoid>::Element,
            ) -> <Op as Monoid>::Element {
                // 区間和に対するアフィン変換
                // 各要素: v -> a v + b
                // sum' = a*sum + b*len
                Node {
                    sum: f.a * x.sum + f.b * x.len,
                    len: x.len,
                }
            }
        }

        let v = vec![1_i64, 2, 3, 4, 5];
        let init = v
            .into_iter()
            .map(|x| Node { sum: x, len: 1 })
            .collect::<Vec<_>>();
        let mut seg = LazySegmentTree::<Op, ActAffine>::from(init);

        assert_eq!(seg.fold(..).sum, 15);

        seg.act(1..4, &Affine { a: 2, b: 1 });

        assert_eq!(seg.get(0).sum, 1);
        assert_eq!(seg.get(1).sum, 5);
        assert_eq!(seg.get(2).sum, 7);
        assert_eq!(seg.get(3).sum, 9);
        assert_eq!(seg.get(4).sum, 5);

        assert_eq!(seg.fold(0..=2).sum, 1 + 5 + 7); // 13
        assert_eq!(seg.fold(..).sum, 1 + 5 + 7 + 9 + 5); // 27

        // 作用の合成順チェック（単点なら分かりやすい）
        let init2 = vec![Node { sum: 10, len: 1 }];
        let mut seg2 = LazySegmentTree::<Op, ActAffine>::from(init2);

        // f: 3x + 2
        seg2.act(0..1, &Affine { a: 3, b: 2 });
        // g: 2x + 5（後から適用）
        seg2.act(0..1, &Affine { a: 2, b: 5 });

        assert_eq!(seg2.get(0).sum, 69);
        assert_eq!(seg2.fold(..).sum, 69);

        let init3 = vec![0, 1, 2, 3]
            .into_iter()
            .map(|x| Node { sum: x, len: 1 })
            .collect::<Vec<_>>();
        let mut seg3 = LazySegmentTree::<Op, ActAffine>::from(init3);

        seg3.act(0..3, &Affine { a: 1, b: 10 });
        seg3.act(1..4, &Affine { a: 2, b: 0 });

        assert_eq!(seg3.get(0).sum, 10);
        assert_eq!(seg3.get(1).sum, 22);
        assert_eq!(seg3.get(2).sum, 24);
        assert_eq!(seg3.get(3).sum, 6);
        assert_eq!(seg3.fold(..).sum, 10 + 22 + 24 + 6);
    }

    // ============================================================
    // bisect（二分探索）のテスト
    // ============================================================

    #[test]
    fn test_bisect_max_with_add() {
        type Op = OpMax<i32>;
        type Act = ActAdd<i32>;

        let mut seg = LazySegmentTree::<Op, Act>::from(vec![4, 4, 4, 4, 4]);

        assert_eq!(seg.bisect_right(0, |m| *m <= 4), 5);
        assert_eq!(seg.bisect_right(0, |m| *m < 5), 5);
        assert_eq!(seg.bisect_right(0, |m| *m < 4), 0);
        assert_eq!(seg.bisect_right(2, |m| *m <= 4), 5);
        assert_eq!(seg.bisect_right(5, |m| *m <= 4), 5);

        seg.act(1..4, &2); // [4, 6, 6, 6, 4]

        assert_eq!(seg.bisect_right(0, |m| *m <= 4), 1);
        assert_eq!(seg.bisect_right(0, |m| *m <= 6), 5);
        assert_eq!(seg.bisect_right(0, |m| *m < 6), 1);

        assert_eq!(seg.bisect_left(5, |m| *m <= 6), 0);
        assert_eq!(seg.bisect_left(5, |m| *m <= 4), 4);
        assert_eq!(seg.bisect_left(5, |m| *m < 6), 4);
        assert_eq!(seg.bisect_left(1, |m| *m <= 4), 0);
        assert_eq!(seg.bisect_left(0, |m| *m <= 6), 0);

        *seg.entry_mut(2) = 10; // [4, 6, 10, 6, 4]

        assert_eq!(seg.bisect_right(0, |m| *m <= 6), 2);
        assert_eq!(seg.bisect_right(0, |m| *m <= 10), 5);
        assert_eq!(seg.bisect_right(0, |m| *m < 10), 2);

        assert_eq!(seg.bisect_left(5, |m| *m <= 6), 3);
        assert_eq!(seg.bisect_left(5, |m| *m < 10), 3);
    }

    #[test]
    fn test_bisect_min_with_add() {
        type Op = OpMin<i32>;
        type Act = ActAdd<i32>;

        let mut seg = LazySegmentTree::<Op, Act>::from(vec![5, 3, 7, 3, 9]);

        assert_eq!(seg.bisect_right(0, |m| *m >= 3), 5);
        assert_eq!(seg.bisect_right(0, |m| *m > 3), 1);
        assert_eq!(seg.bisect_right(1, |m| *m >= 3), 5);

        seg.act(1..4, &5); // [5, 8, 12, 8, 9]

        assert_eq!(seg.bisect_right(0, |m| *m >= 5), 5);
        assert_eq!(seg.bisect_right(0, |m| *m >= 8), 0);
        assert_eq!(seg.bisect_right(1, |m| *m >= 8), 5);

        assert_eq!(seg.bisect_left(5, |m| *m >= 9), 4);
        assert_eq!(seg.bisect_left(5, |m| *m > 8), 4);
        assert_eq!(seg.bisect_left(5, |m| *m >= 5), 0);

        *seg.entry_mut(2) = 3; // [5, 8, 3, 8, 9]

        assert_eq!(seg.bisect_left(5, |m| *m >= 3), 0);
        assert_eq!(seg.bisect_left(3, |m| *m >= 3), 0);
    }

    #[test]
    fn test_bisect_max_with_set() {
        type Op = OpMax<i32>;
        type Act = ActSet<i32>;

        let mut seg = LazySegmentTree::<Op, Act>::from(vec![1, 2, 3, 4, 5]);

        assert_eq!(seg.bisect_right(0, |m| *m <= 3), 3);
        assert_eq!(seg.bisect_right(0, |m| *m < 4), 3);

        seg.act(1..4, &Some(10)); // [1, 10, 10, 10, 5]

        assert_eq!(seg.bisect_right(0, |m| *m <= 5), 1);
        assert_eq!(seg.bisect_right(0, |m| *m <= 10), 5);
        assert_eq!(seg.bisect_right(0, |m| *m < 10), 1);

        assert_eq!(seg.bisect_left(5, |m| *m <= 10), 0);
        assert_eq!(seg.bisect_left(5, |m| *m < 10), 4);
    }

    #[test]
    fn test_bisect_edge_cases() {
        type Op = OpMax<i32>;
        type Act = ActAdd<i32>;

        let mut seg = LazySegmentTree::<Op, Act>::from(vec![1, 2, 3]);
        assert_eq!(seg.bisect_right(0, |m| *m <= 0), 0);
        assert_eq!(seg.bisect_right(0, |m| *m <= 3), 3);
        assert_eq!(seg.bisect_left(0, |m| *m <= 1), 0);
        assert_eq!(seg.bisect_left(3, |m| *m <= 0), 3);

        let mut seg2 = LazySegmentTree::<Op, Act>::from(vec![42]);
        assert_eq!(seg2.bisect_right(0, |m| *m <= 42), 1);
        assert_eq!(seg2.bisect_right(0, |m| *m < 42), 0);
        assert_eq!(seg2.bisect_left(1, |m| *m <= 42), 0);
        assert_eq!(seg2.bisect_left(1, |m| *m < 42), 1);

        let mut seg3 = LazySegmentTree::<Op, Act>::from(vec![7; 100]);
        assert_eq!(seg3.bisect_right(0, |m| *m <= 7), 100);
        assert_eq!(seg3.bisect_right(50, |m| *m <= 7), 100);
        assert_eq!(seg3.bisect_left(100, |m| *m <= 7), 0);
    }

    randomized_range_add_bisect_max_test!(
        test_randomized_bisect_max_with_add_i32,
        i32,
        -100000..=100000
    );
    randomized_range_add_bisect_max_test!(test_randomized_bisect_max_with_add_u32, u32, 0..=100000);
    randomized_range_add_bisect_max_test!(
        test_randomized_bisect_max_with_add_i64,
        i64,
        -1000000000..=1000000000
    );
    randomized_range_add_bisect_max_test!(
        test_randomized_bisect_max_with_add_u64,
        u64,
        0..=1000000000
    );

    randomized_range_add_bisect_min_test!(
        test_randomized_bisect_min_with_add_i32,
        i32,
        -100000..=100000
    );
    randomized_range_add_bisect_min_test!(test_randomized_bisect_min_with_add_u32, u32, 0..=100000);
    randomized_range_add_bisect_min_test!(
        test_randomized_bisect_min_with_add_i64,
        i64,
        -1000000000..=1000000000
    );
    randomized_range_add_bisect_min_test!(
        test_randomized_bisect_min_with_add_u64,
        u64,
        0..=1000000000
    );
}
