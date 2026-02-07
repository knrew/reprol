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
//! assert_eq!(ft.fold(2..5), 5); // 区間[2, 5)の区間和
//! ```

use std::{
    iter::FromIterator,
    ops::{Range, RangeBounds},
};

use crate::{ops::group::Group, utils::range_utils::to_half_open_index_range};

/// Fenwick Tree
pub struct FenwickTree<O: Group> {
    nodes: Vec<O::Element>,
    op: O,
}

impl<O: Group> FenwickTree<O> {
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
            nodes: (0..n).map(|_| op.id()).collect(),
            op,
        }
    }

    /// `index`番目の要素に`rhs`を作用させる．
    /// `v[i] <- v[i] * rhs`
    pub fn op(&mut self, mut index: usize, rhs: &O::Element) {
        assert!(index < self.nodes.len());
        index += 1;
        while index <= self.nodes.len() {
            self.nodes[index - 1] = self.op.op(&self.nodes[index - 1], rhs);
            index += index & index.wrapping_neg();
        }
    }

    /// `index`番目の要素の値を`value`にする．
    pub fn set(&mut self, index: usize, value: O::Element) {
        let diff = self.op.op(&value, &self.op.inv(&self.get(index)));
        self.op(index, &diff);
    }

    /// `index`番目の要素の値を返す．
    pub fn get(&self, index: usize) -> O::Element {
        self.fold(index..=index)
    }

    /// 区間`[0, r)`の区間積を返す．
    fn prefix(&self, mut r: usize) -> O::Element {
        assert!(r <= self.nodes.len());
        let mut res = self.op.id();
        while r > 0 {
            res = self.op.op(&res, &self.nodes[r - 1]);
            r -= r & r.wrapping_neg();
        }
        res
    }

    /// 区間`[l, r)`の区間積を返す．
    pub fn fold(&self, range: impl RangeBounds<usize>) -> O::Element {
        let Range { start: l, end: r } = to_half_open_index_range(range, self.nodes.len());
        assert!(l <= r);
        let prefix_l = self.prefix(l);
        let prefix_r = self.prefix(r);
        self.op.op(&prefix_r, &self.op.inv(&prefix_l))
    }
}

impl<O: Group> From<(Vec<O::Element>, O)> for FenwickTree<O> {
    fn from((v, op): (Vec<O::Element>, O)) -> Self {
        let mut res = Self::with_op(v.len(), op);
        v.into_iter()
            .enumerate()
            .for_each(|(i, rhs)| res.op(i, &rhs));
        res
    }
}

impl<O: Group, const N: usize> From<([O::Element; N], O)> for FenwickTree<O> {
    fn from((v, op): ([O::Element; N], O)) -> Self {
        Self::from((v.into_iter().collect::<Vec<_>>(), op))
    }
}

impl<O: Group + Default> From<Vec<O::Element>> for FenwickTree<O> {
    fn from(v: Vec<O::Element>) -> Self {
        Self::from((v, O::default()))
    }
}

impl<O: Group + Default, const N: usize> From<[O::Element; N]> for FenwickTree<O> {
    fn from(v: [O::Element; N]) -> Self {
        Self::from((v, O::default()))
    }
}

impl<O: Group + Default> FromIterator<O::Element> for FenwickTree<O> {
    fn from_iter<I: IntoIterator<Item = O::Element>>(iter: I) -> Self {
        Self::from(iter.into_iter().collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    use crate::{
        ops::op_add::OpAdd,
        ops::op_xor::OpXor,
        utils::test_utils::{dynamic_range_query::*, random::get_test_rng, static_range_query::*},
    };

    #[test]
    fn test_sum() {
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
        ft.set(0, 100);
        assert_eq!(ft.get(0), 100);
        assert_eq!(ft.fold(0..1), 100);
        ft.set(3, 50);
        assert_eq!(ft.get(3), 50);
        assert_eq!(ft.fold(3..4), 50);
    }

    macro_rules! ft_randomized_static_range_sum_exhaustive_test {
        ($test_name: ident, $ty: ty, $range: expr) => {
            randomized_static_range_sum_exhaustive_test!(
                $test_name,
                $ty,
                |v| FenwickTree::<OpAdd<$ty>>::from(v),
                |ds: &FenwickTree<_>, range| ds.fold(range),
                200,
                100,
                $range
            );
        };
    }

    macro_rules! ft_randomized_static_range_xor_exhaustive_test {
        ($test_name: ident, $ty: ty) => {
            randomized_static_range_xor_exhaustive_test!(
                $test_name,
                $ty,
                |v| FenwickTree::<OpXor<$ty>>::from(v),
                |ds: &FenwickTree<_>, range| ds.fold(range),
                200,
                100
            );
        };
    }

    ft_randomized_static_range_sum_exhaustive_test!(
        test_randomize_static_range_sum_exhaustive_i8,
        i8,
        -1..=1
    );
    ft_randomized_static_range_sum_exhaustive_test!(
        test_randomize_static_range_sum_exhaustive_u8,
        u8,
        0..=1
    );
    ft_randomized_static_range_sum_exhaustive_test!(
        test_randomize_static_range_sum_exhaustive_i16,
        i16,
        -300..=300
    );
    ft_randomized_static_range_sum_exhaustive_test!(
        test_randomize_static_range_sum_exhaustive_u16,
        u16,
        0..=300
    );
    ft_randomized_static_range_sum_exhaustive_test!(
        test_randomize_static_range_sum_exhaustive_i32,
        i32,
        -100000..=100000
    );
    ft_randomized_static_range_sum_exhaustive_test!(
        test_randomize_static_range_sum_exhaustive_u32,
        u32,
        0..=100000
    );
    ft_randomized_static_range_sum_exhaustive_test!(
        test_randomize_static_range_sum_exhaustive_i64,
        i64,
        -1000000000..=1000000000
    );
    ft_randomized_static_range_sum_exhaustive_test!(
        test_randomize_static_range_sum_exhaustive_u64,
        u64,
        0..=1000000000
    );
    ft_randomized_static_range_sum_exhaustive_test!(
        test_randomize_static_range_sum_exhaustive_i128,
        i128,
        -1000000000000000000..=1000000000000000000
    );
    ft_randomized_static_range_sum_exhaustive_test!(
        test_randomize_static_range_sum_exhaustive_u128,
        u128,
        0..=1000000000000000000
    );
    ft_randomized_static_range_sum_exhaustive_test!(
        test_randomize_static_range_sum_exhaustive_usize,
        usize,
        0..=1000000000
    );

    ft_randomized_static_range_xor_exhaustive_test!(test_randomized_range_xor_exhaustive_i8, i8);
    ft_randomized_static_range_xor_exhaustive_test!(test_randomized_range_xor_exhaustive_u8, u8);
    ft_randomized_static_range_xor_exhaustive_test!(test_randomized_range_xor_exhaustive_i16, i16);
    ft_randomized_static_range_xor_exhaustive_test!(test_randomized_range_xor_exhaustive_u16, u16);
    ft_randomized_static_range_xor_exhaustive_test!(test_randomized_range_xor_exhaustive_i32, i32);
    ft_randomized_static_range_xor_exhaustive_test!(test_randomized_range_xor_exhaustive_u32, u32);
    ft_randomized_static_range_xor_exhaustive_test!(test_randomized_range_xor_exhaustive_i64, i64);
    ft_randomized_static_range_xor_exhaustive_test!(test_randomized_range_xor_exhaustive_u64, u64);
    ft_randomized_static_range_xor_exhaustive_test!(
        test_randomized_range_xor_exhaustive_i128,
        i128
    );
    ft_randomized_static_range_xor_exhaustive_test!(
        test_randomized_range_xor_exhaustive_u128,
        u128
    );
    ft_randomized_static_range_xor_exhaustive_test!(
        test_randomized_range_xor_exhaustive_usize,
        usize
    );

    macro_rules! ft_randomized_point_set_range_sum_test {
        ($test_name: ident, $ty: ty, $range: expr) => {
            randomized_point_set_range_sum_test!(
                $test_name,
                $ty,
                |v| FenwickTree::<OpAdd<$ty>>::from(v),
                |ds: &FenwickTree<_>, range| ds.fold(range),
                |ds: &mut FenwickTree<_>, index, value| ds.set(index, value),
                20,     // T
                100000, //Q
                100,    // N_MAX
                $range
            );
        };
    }

    macro_rules! ft_randomized_point_set_range_xor_test {
        ($test_name: ident, $ty: ty) => {
            randomized_point_set_range_xor_test!(
                $test_name,
                $ty,
                |v| FenwickTree::<OpXor<$ty>>::from(v),
                |ds: &FenwickTree<_>, range| ds.fold(range),
                |ds: &mut FenwickTree<_>, index, value| ds.set(index, value),
                10,     // T
                100000, //Q
                100     // N_MAX
            );
        };
    }

    ft_randomized_point_set_range_sum_test!(test_randomized_point_set_range_sum_i8, i8, -1..=1);
    ft_randomized_point_set_range_sum_test!(test_randomized_point_set_range_sum_u8, u8, 0..=1);
    ft_randomized_point_set_range_sum_test!(
        test_randomized_point_set_range_sum_i16,
        i16,
        -300..=300
    );
    ft_randomized_point_set_range_sum_test!(test_randomized_point_set_range_sum_u16, u16, 0..=300);
    ft_randomized_point_set_range_sum_test!(
        test_randomized_point_set_range_sum_i32,
        i32,
        -100000..=100000
    );
    ft_randomized_point_set_range_sum_test!(
        test_randomized_point_set_range_sum_u32,
        u32,
        0..=100000
    );
    ft_randomized_point_set_range_sum_test!(
        test_randomized_point_set_range_sum_i64,
        i64,
        -1000000000..=1000000000
    );
    ft_randomized_point_set_range_sum_test!(
        test_randomized_point_set_range_sum_u64,
        u64,
        0..=1000000000
    );
    ft_randomized_point_set_range_sum_test!(
        test_randomized_point_set_range_sum_i128,
        i128,
        -1000000000000000000..=1000000000000000000
    );
    ft_randomized_point_set_range_sum_test!(
        test_randomized_point_set_range_sum_u128,
        u128,
        0..=1000000000000000000
    );
    ft_randomized_point_set_range_sum_test!(
        test_randomized_point_set_range_sum_usize,
        usize,
        0..=1000000000
    );

    ft_randomized_point_set_range_xor_test!(test_randomized_point_set_range_xor_i8, i8);
    ft_randomized_point_set_range_xor_test!(test_randomized_point_set_range_xor_u8, u8);
    ft_randomized_point_set_range_xor_test!(test_randomized_point_set_range_xor_i16, i16);
    ft_randomized_point_set_range_xor_test!(test_randomized_point_set_range_xor_u16, u16);
    ft_randomized_point_set_range_xor_test!(test_randomized_point_set_range_xor_i32, i32);
    ft_randomized_point_set_range_xor_test!(test_randomized_point_set_range_xor_u32, u32);
    ft_randomized_point_set_range_xor_test!(test_randomized_point_set_range_xor_i64, i64);
    ft_randomized_point_set_range_xor_test!(test_randomized_point_set_range_xor_u64, u64);
    ft_randomized_point_set_range_xor_test!(test_randomized_point_set_range_xor_i128, i128);
    ft_randomized_point_set_range_xor_test!(test_randomized_point_set_range_xor_u128, u128);
    ft_randomized_point_set_range_xor_test!(test_randomized_point_set_range_xor_usize, usize);
}
