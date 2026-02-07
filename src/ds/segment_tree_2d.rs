//! 2次元セグメント木(2D Segment Tree)
//!
//! モノイドの2次元配列を管理するデータ構造．
//! 以下の操作をいずれも O(log h × log w) で処理できる．
//! - 要素の1点変更．
//! - 任意の長方形区間の要素の総積(和，最小値など)の取得．
//!
//! # 使用例
//! ```
//! use reprol::{ds::segment_tree_2d::SegmentTree2d, ops::op_add::OpAdd};
//!
//! let v = vec![
//!     vec![1, 2, 3],
//!     vec![4, 5, 6],
//!     vec![7, 8, 9],
//! ];
//! let mut seg = SegmentTree2d::<OpAdd<i64>>::from(v);
//! assert_eq!(seg.fold(.., ..), 45);
//! assert_eq!(seg.fold(1..3, 1..3), 28);
//! seg.set(0, 0, 10);
//! assert_eq!(seg.fold(0..2, 0..2), 21);
//! ```

use std::ops::{Deref, DerefMut, Index, Range, RangeBounds};

use crate::{ops::monoid::Monoid, utils::range_utils::to_half_open_index_range};

/// 2次元セグメント木
pub struct SegmentTree2d<O: Monoid> {
    /// 行の長さ
    len_rows: usize,

    /// 列の長さ
    len_cols: usize,

    /// 行方向のオフセット
    offset_row: usize,

    /// 列方向のオフセット
    offset_col: usize,

    /// セグ木を構成するノード
    nodes: Vec<O::Element>,

    /// nodesの列数
    nodes_len_cols: usize,

    /// 演算(モノイド)
    op: O,
}

impl<O: Monoid> SegmentTree2d<O> {
    /// 高さ`h`，幅`w`の2次元セグメント木を単位元で初期化して生成する．
    pub fn new(h: usize, w: usize) -> Self
    where
        O: Default,
    {
        Self::with_op(h, w, O::default())
    }

    /// 高さ`h`、幅`w`の2次元セグメント木を，モノイド`op`を指定して生成する．
    pub fn with_op(h: usize, w: usize, op: O) -> Self {
        assert!(h > 0 && w > 0);

        let offset_row = h.next_power_of_two();
        let offset_col = w.next_power_of_two();

        let nodes_len_rows = 2 * offset_row;
        let nodes_len_cols = 2 * offset_col;
        let nodes_len = nodes_len_rows * nodes_len_cols;

        let nodes = (0..nodes_len).map(|_| op.id()).collect();

        Self {
            len_rows: h,
            len_cols: w,
            offset_row,
            offset_col,
            nodes,
            nodes_len_cols,
            op,
        }
    }

    #[inline(always)]
    fn idx(&self, i: usize, j: usize) -> usize {
        i * self.nodes_len_cols + j
    }

    /// (`i`, `j`)番目の要素を返す．
    pub fn get(&self, i: usize, j: usize) -> &O::Element {
        assert!(i < self.len_rows && j < self.len_cols);
        &self.nodes[self.idx(i + self.offset_row, j + self.offset_col)]
    }

    /// (`i`, `j`)番目の要素を`value`に更新する．
    #[inline(always)]
    pub fn set(&mut self, i: usize, j: usize, value: O::Element) {
        *self.entry_mut(i, j) = value;
    }

    /// (`i`, `j`)番目の要素への可変参照を返す．
    pub fn entry_mut(&mut self, i: usize, j: usize) -> EntryMut<'_, O> {
        assert!(i < self.len_rows && j < self.len_cols);
        EntryMut {
            seg: self,
            index_row: i,
            index_col: j,
        }
    }

    /// 区間`[row_range] x [col_range]`の要素の総積を返す．
    pub fn fold(
        &self,
        row_range: impl RangeBounds<usize>,
        col_range: impl RangeBounds<usize>,
    ) -> O::Element {
        let Range { start: il, end: ir } = to_half_open_index_range(row_range, self.len_rows);
        let Range { start: jl, end: jr } = to_half_open_index_range(col_range, self.len_cols);
        assert!(il <= ir && ir <= self.len_rows);
        assert!(jl <= jr && jr <= self.len_cols);

        let mut l = il + self.offset_row;
        let mut r = ir + self.offset_row;

        let mut prod_l = self.op.id();
        let mut prod_r = self.op.id();

        while l < r {
            if l % 2 == 1 {
                let tmp = self.fold_col(l, jl..jr);
                prod_l = self.op.op(&prod_l, &tmp);
                l += 1;
            }

            if r % 2 == 1 {
                r -= 1;
                let tmp = self.fold_col(r, jl..jr);
                prod_r = self.op.op(&tmp, &prod_r);
            }

            l /= 2;
            r /= 2;
        }

        self.op.op(&prod_l, &prod_r)
    }

    fn rebuild_col(&mut self, node_index_i: usize, j: usize) {
        let mut node_index_j = j + self.offset_col;
        while node_index_j > 1 {
            node_index_j /= 2;
            let index = self.idx(node_index_i, node_index_j);
            let index_l = self.idx(node_index_i, 2 * node_index_j);
            let index_r = self.idx(node_index_i, 2 * node_index_j + 1);
            self.nodes[index] = self.op.op(&self.nodes[index_l], &self.nodes[index_r]);
        }
    }

    fn fold_col(&self, node_index_i: usize, col_range: Range<usize>) -> O::Element {
        let Range { start: jl, end: jr } = col_range;

        let mut l = jl + self.offset_col;
        let mut r = jr + self.offset_col;

        let mut prod_l = self.op.id();
        let mut prod_r = self.op.id();

        while l < r {
            if l % 2 == 1 {
                prod_l = self.op.op(&prod_l, &self.nodes[self.idx(node_index_i, l)]);
                l += 1;
            }

            if r % 2 == 1 {
                r -= 1;
                prod_r = self.op.op(&self.nodes[self.idx(node_index_i, r)], &prod_r);
            }

            l /= 2;
            r /= 2;
        }

        self.op.op(&prod_l, &prod_r)
    }
}

impl<O: Monoid> From<(Vec<Vec<O::Element>>, O)> for SegmentTree2d<O> {
    fn from((v, op): (Vec<Vec<O::Element>>, O)) -> Self {
        assert!(!v.is_empty());
        assert!(!v[0].is_empty());
        debug_assert!(v.iter().all(|vi| vi.len() == v[0].len()));

        let h = v.len();
        let w = v[0].len();

        let mut seg = Self::with_op(h, w, op);

        for (i, vi) in v.into_iter().enumerate() {
            let node_index_i = i + seg.offset_row;

            for (j, vij) in vi.into_iter().enumerate() {
                let index = seg.idx(node_index_i, j + seg.offset_col);
                seg.nodes[index] = vij;
            }

            for j in (1..seg.offset_col).rev() {
                let index = seg.idx(node_index_i, j);
                let index_l = seg.idx(node_index_i, 2 * j);
                let index_r = seg.idx(node_index_i, 2 * j + 1);
                seg.nodes[index] = seg.op.op(&seg.nodes[index_l], &seg.nodes[index_r]);
            }
        }

        for i in (1..seg.offset_row).rev() {
            for j in 1..2 * seg.offset_col {
                let index = seg.idx(i, j);
                let index_l = seg.idx(2 * i, j);
                let index_r = seg.idx(2 * i + 1, j);
                seg.nodes[index] = seg.op.op(&seg.nodes[index_l], &seg.nodes[index_r]);
            }
        }

        seg
    }
}

impl<O: Monoid, const N: usize, const M: usize> From<([[O::Element; M]; N], O)>
    for SegmentTree2d<O>
{
    fn from((v, op): ([[O::Element; M]; N], O)) -> Self {
        let v: Vec<Vec<_>> = v.into_iter().map(|vi| vi.into_iter().collect()).collect();
        Self::from((v, op))
    }
}

impl<O: Monoid + Default> From<Vec<Vec<O::Element>>> for SegmentTree2d<O> {
    fn from(v: Vec<Vec<O::Element>>) -> Self {
        Self::from((v, O::default()))
    }
}

impl<O: Monoid + Default, const N: usize, const M: usize> From<[[O::Element; M]; N]>
    for SegmentTree2d<O>
{
    fn from(v: [[O::Element; M]; N]) -> Self {
        Self::from((v, O::default()))
    }
}

impl<O: Monoid> Index<(usize, usize)> for SegmentTree2d<O> {
    type Output = O::Element;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index.0, index.1)
    }
}

impl<O: Monoid> Index<[usize; 2]> for SegmentTree2d<O> {
    type Output = O::Element;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        self.get(index[0], index[1])
    }
}

/// 2次元セグメント木の要素への可変参照
pub struct EntryMut<'a, O: Monoid> {
    seg: &'a mut SegmentTree2d<O>,
    index_row: usize,
    index_col: usize,
}

impl<'a, O: Monoid> Deref for EntryMut<'a, O> {
    type Target = O::Element;
    fn deref(&self) -> &Self::Target {
        self.seg.get(self.index_row, self.index_col)
    }
}

impl<'a, O: Monoid> DerefMut for EntryMut<'a, O> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let node_index_i = self.index_row + self.seg.offset_row;
        let node_index_j = self.index_col + self.seg.offset_col;
        let index = self.seg.idx(node_index_i, node_index_j);
        &mut self.seg.nodes[index]
    }
}

impl<'a, O: Monoid> Drop for EntryMut<'a, O> {
    fn drop(&mut self) {
        let mut node_index_i = self.index_row + self.seg.offset_row;

        self.seg.rebuild_col(node_index_i, self.index_col);

        while node_index_i > 1 {
            node_index_i /= 2;

            let node_index_j = self.index_col + self.seg.offset_col;

            let index = self.seg.idx(node_index_i, node_index_j);
            let index_l = self.seg.idx(2 * node_index_i, node_index_j);
            let index_r = self.seg.idx(2 * node_index_i + 1, node_index_j);
            self.seg.nodes[index] = self
                .seg
                .op
                .op(&self.seg.nodes[index_l], &self.seg.nodes[index_r]);

            self.seg.rebuild_col(node_index_i, self.index_col);
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
        utils::test_utils::{
            dynamic_range_query_2d::*, random::get_test_rng, static_range_query_2d::*,
        },
    };

    #[test]
    fn test_2d_add() {
        let a = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];

        let mut seg = SegmentTree2d::<OpAdd<i64>>::from(a);

        assert_eq!(seg.fold(.., ..), 78);

        assert_eq!(seg.fold(1..3, 1..4), 54);
        assert_eq!(seg.fold(0..1, 0..2), 3);

        seg.set(0, 0, 100);
        assert_eq!(*seg.get(0, 0), 100);
        assert_eq!(seg.fold(0..1, 0..2), 102);
        assert_eq!(seg[(0, 0)], 100);
    }

    #[test]
    fn test_2d_min() {
        let a = vec![vec![5, 2, 6, 3], vec![7, 1, 4, 8], vec![9, 3, 2, 5]];
        let mut seg = SegmentTree2d::<OpMin<i32>>::from(a);

        assert_eq!(seg.fold(0..3, 0..4), 1);
        assert_eq!(seg.fold(1..3, 1..4), 1);
        assert_eq!(seg.fold(0..2, 1..3), 1);

        seg.set(1, 1, 10);
        assert_eq!(seg.fold(0..3, 0..4), 2);
        assert_eq!(seg[(1, 2)], 4);
    }

    #[test]
    fn test_2d_max() {
        let a = vec![vec![5, 2, 6, 3], vec![7, 1, 4, 8], vec![9, 3, 2, 5]];
        let mut seg = SegmentTree2d::<OpMax<i32>>::from(a);

        assert_eq!(seg.fold(0..3, 0..4), 9);
        assert_eq!(seg.fold(1..3, 1..4), 8);
        assert_eq!(seg.fold(0..2, 1..3), 6);

        seg.set(2, 0, 100);
        assert_eq!(seg.fold(.., ..), 100);
    }

    #[test]
    fn test_entry_mut() {
        // 代入
        {
            let a = vec![vec![1, 2, 3], vec![4, 5, 6]];
            let mut seg = SegmentTree2d::<OpAdd<i64>>::from(a);
            *seg.entry_mut(0, 0) = 10;
            assert_eq!(seg[(0, 0)], 10);
            assert_eq!(seg.fold(0..1, 0..1), 10);
        }

        // in-place 更新
        {
            let a = vec![vec![1, 2, 3], vec![4, 5, 6]];
            let mut seg = SegmentTree2d::<OpAdd<i64>>::from(a);
            *seg.entry_mut(1, 2) += 100;
            assert_eq!(seg[(1, 2)], 106);
            assert_eq!(seg.fold(1..2, 2..3), 106);
        }

        // 境界
        {
            let a = vec![vec![5, 2, 6], vec![3, 7, 1]];
            let mut seg = SegmentTree2d::<OpMin<i32>>::from(a);

            {
                let mut e = seg.entry_mut(0, 0);
                *e = 10;
            }

            assert_eq!(seg[(0, 0)], 10);
            assert_eq!(seg.fold(.., ..), 1);

            let mut e = seg.entry_mut(1, 2);
            *e = 20;
            drop(e);

            assert_eq!(seg.fold(.., ..), 2);
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
            let mut seg = SegmentTree2d::with_op(2, 3, op);
            let v = vec![vec![10, 20, 30], vec![40, 50, 60]];
            for i in 0..2 {
                for (j, &x) in v[i].iter().enumerate() {
                    seg.set(i, j, x);
                }
            }
            assert_eq!(seg.fold(.., ..), (10 + 20 + 30 + 40 + 50 + 60) % 7);
            assert_eq!(seg.fold(0..1, 0..3), (10 + 20 + 30) % 7);
        }

        {
            let v = vec![vec![1, 2, 3], vec![4, 5, 6]];
            let seg = SegmentTree2d::from((v, OpModAdd { m: 5 }));
            assert_eq!(seg.fold(.., ..), (1 + 2 + 3 + 4 + 5 + 6) % 5);
            assert_eq!(seg.fold(1..2, 1..3), (5 + 6) % 5);
        }
    }

    macro_rules! seg2d_randomized_static_range_sum_exhaustive_test {
        ($test_name: ident, $ty: ty, $range: expr) => {
            randomized_static_range_sum_2d_exhaustive_test!(
                $test_name,
                $ty,
                |v| SegmentTree2d::<OpAdd<$ty>>::from(v),
                |ds: &SegmentTree2d<_>, r, c| ds.fold(r, c),
                50,
                20,
                $range
            );
        };
    }

    macro_rules! seg2d_randomized_static_range_min_max_gcd_xor_exhaustive_test {
        ($min_test_name: ident, $max_test_name: ident, $gcd_test_name: ident, $xor_test_name: ident, $ty: ty) => {
            randomized_static_range_min_2d_exhaustive_test!(
                $min_test_name,
                $ty,
                |v| SegmentTree2d::<OpMin<$ty>>::from(v),
                |ds: &SegmentTree2d<_>, r, c| ds.fold(r, c),
                50,
                20
            );

            randomized_static_range_max_2d_exhaustive_test!(
                $max_test_name,
                $ty,
                |v| SegmentTree2d::<OpMax<$ty>>::from(v),
                |ds: &SegmentTree2d<_>, r, c| ds.fold(r, c),
                50,
                20
            );

            randomized_static_range_gcd_2d_exhaustive_test!(
                $gcd_test_name,
                $ty,
                |v| SegmentTree2d::<OpGcd<$ty>>::from(v),
                |ds: &SegmentTree2d<_>, r, c| ds.fold(r, c),
                10,
                20
            );

            randomized_static_range_xor_2d_exhaustive_test!(
                $xor_test_name,
                $ty,
                |v| SegmentTree2d::<OpXor<$ty>>::from(v),
                |ds: &SegmentTree2d<_>, r, c| ds.fold(r, c),
                10,
                20
            );
        };
    }

    seg2d_randomized_static_range_sum_exhaustive_test!(
        test_randomized_static_range_sum_exhaustive_i32,
        i32,
        -100000..=100000
    );
    seg2d_randomized_static_range_sum_exhaustive_test!(
        test_randomized_static_range_sum_exhaustive_u32,
        u32,
        0..=100000
    );
    seg2d_randomized_static_range_sum_exhaustive_test!(
        test_randomized_static_range_sum_exhaustive_i64,
        i64,
        -1000000000..=1000000000
    );
    seg2d_randomized_static_range_sum_exhaustive_test!(
        test_randomized_static_range_sum_exhaustive_u64,
        u64,
        0..=1000000000
    );
    seg2d_randomized_static_range_sum_exhaustive_test!(
        test_randomized_static_range_sum_exhaustive_usize,
        usize,
        0..=1000000000
    );

    seg2d_randomized_static_range_min_max_gcd_xor_exhaustive_test!(
        test_randomized_static_range_min_exhaustive_i32,
        test_randomized_static_range_max_exhaustive_i32,
        test_randomized_static_range_gcd_exhaustive_i32,
        test_randomized_static_range_xor_exhaustive_i32,
        i32
    );
    seg2d_randomized_static_range_min_max_gcd_xor_exhaustive_test!(
        test_randomized_static_range_min_exhaustive_u32,
        test_randomized_static_range_max_exhaustive_u32,
        test_randomized_static_range_gcd_exhaustive_u32,
        test_randomized_static_range_xor_exhaustive_u32,
        u32
    );
    seg2d_randomized_static_range_min_max_gcd_xor_exhaustive_test!(
        test_randomized_static_range_min_exhaustive_i64,
        test_randomized_static_range_max_exhaustive_i64,
        test_randomized_static_range_gcd_exhaustive_i64,
        test_randomized_static_range_xor_exhaustive_i64,
        i64
    );
    seg2d_randomized_static_range_min_max_gcd_xor_exhaustive_test!(
        test_randomized_static_range_min_exhaustive_u64,
        test_randomized_static_range_max_exhaustive_u64,
        test_randomized_static_range_gcd_exhaustive_u64,
        test_randomized_static_range_xor_exhaustive_u64,
        u64
    );
    seg2d_randomized_static_range_min_max_gcd_xor_exhaustive_test!(
        test_randomized_static_range_min_exhaustive_usize,
        test_randomized_static_range_max_exhaustive_usize,
        test_randomized_static_range_gcd_exhaustive_usize,
        test_randomized_static_range_xor_exhaustive_usize,
        usize
    );

    macro_rules! seg2d_randomized_point_set_range_sum_test {
        ($test_name: ident, $ty: ty, $range: expr) => {
            randomized_point_set_range_sum_2d_test!(
                $test_name,
                $ty,
                |v| SegmentTree2d::<OpAdd<$ty>>::from(v),
                |ds: &SegmentTree2d<_>, r, c| ds.fold(r, c),
                |ds: &mut SegmentTree2d<_>, i, j, val| ds.set(i, j, val),
                50,
                1000,
                20,
                $range
            );
        };
    }

    macro_rules! seg2d_randomized_point_set_range_min_max_gcd_xor_test {
        ($min_test_name: ident, $max_test_name: ident, $gcd_test_name: ident, $xor_test_name: ident, $ty: ty) => {
            randomized_point_set_range_min_2d_test!(
                $min_test_name,
                $ty,
                |v| SegmentTree2d::<OpMin<$ty>>::from(v),
                |ds: &SegmentTree2d<_>, r, c| ds.fold(r, c),
                |ds: &mut SegmentTree2d<_>, i, j, val| ds.set(i, j, val),
                50,
                1000,
                20
            );

            randomized_point_set_range_max_2d_test!(
                $max_test_name,
                $ty,
                |v| SegmentTree2d::<OpMax<$ty>>::from(v),
                |ds: &SegmentTree2d<_>, r, c| ds.fold(r, c),
                |ds: &mut SegmentTree2d<_>, i, j, val| ds.set(i, j, val),
                50,
                1000,
                20
            );

            randomized_point_set_range_gcd_2d_test!(
                $gcd_test_name,
                $ty,
                |v| SegmentTree2d::<OpGcd<$ty>>::from(v),
                |ds: &SegmentTree2d<_>, r, c| ds.fold(r, c),
                |ds: &mut SegmentTree2d<_>, i, j, val| ds.set(i, j, val),
                10,
                1000,
                20
            );

            randomized_point_set_range_xor_2d_test!(
                $xor_test_name,
                $ty,
                |v| SegmentTree2d::<OpXor<$ty>>::from(v),
                |ds: &SegmentTree2d<_>, r, c| ds.fold(r, c),
                |ds: &mut SegmentTree2d<_>, i, j, val| ds.set(i, j, val),
                10,
                1000,
                20
            );
        };
    }

    seg2d_randomized_point_set_range_sum_test!(
        test_randomized_point_set_range_sum_i32,
        i32,
        -100000..=100000
    );
    seg2d_randomized_point_set_range_sum_test!(
        test_randomized_point_set_range_sum_u32,
        u32,
        0..=100000
    );
    seg2d_randomized_point_set_range_sum_test!(
        test_randomized_point_set_range_sum_i64,
        i64,
        -1000000000..=1000000000
    );
    seg2d_randomized_point_set_range_sum_test!(
        test_randomized_point_set_range_sum_u64,
        u64,
        0..=1000000000
    );
    seg2d_randomized_point_set_range_sum_test!(
        test_randomized_point_set_range_sum_usize,
        usize,
        0..=1000000000
    );

    seg2d_randomized_point_set_range_min_max_gcd_xor_test!(
        test_randomized_point_set_range_min_i32,
        test_randomized_point_set_range_max_i32,
        test_randomized_point_set_range_gcd_i32,
        test_randomized_point_set_range_xor_i32,
        i32
    );
    seg2d_randomized_point_set_range_min_max_gcd_xor_test!(
        test_randomized_point_set_range_min_u32,
        test_randomized_point_set_range_max_u32,
        test_randomized_point_set_range_gcd_u32,
        test_randomized_point_set_range_xor_u32,
        u32
    );
    seg2d_randomized_point_set_range_min_max_gcd_xor_test!(
        test_randomized_point_set_range_min_i64,
        test_randomized_point_set_range_max_i64,
        test_randomized_point_set_range_gcd_i64,
        test_randomized_point_set_range_xor_i64,
        i64
    );
    seg2d_randomized_point_set_range_min_max_gcd_xor_test!(
        test_randomized_point_set_range_min_u64,
        test_randomized_point_set_range_max_u64,
        test_randomized_point_set_range_gcd_u64,
        test_randomized_point_set_range_xor_u64,
        u64
    );
    seg2d_randomized_point_set_range_min_max_gcd_xor_test!(
        test_randomized_point_set_range_min_usize,
        test_randomized_point_set_range_max_usize,
        test_randomized_point_set_range_gcd_usize,
        test_randomized_point_set_range_xor_usize,
        usize
    );
}
