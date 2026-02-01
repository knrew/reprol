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
    use crate::{ops::op_add::OpAdd, utils::test_utils::random::get_test_rng};

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
    }

    #[test]
    fn test_sum_random() {
        macro_rules! define_test_function {
            ($name:ident, $ty:ident) => {
                fn $name(rng: &mut impl Rng, mn: $ty, mx: $ty) {
                    const T: usize = 100;
                    const N: usize = 100;
                    const Q: usize = 1000;

                    for _ in 0..T {
                        let mut ft = FenwickTree::<OpAdd<_>>::new(N);
                        let mut naive = vec![0; N];

                        for _ in 0..Q {
                            // add
                            // v[index] += d
                            let index = rng.random_range(0..N);
                            let d = rng.random_range(mn..=mx);
                            ft.op(index, &d);
                            naive[index] += d;

                            // [l, r)の区間和を求める
                            let l = rng.random_range(0..N);
                            let r = rng.random_range(l..=N);
                            assert_eq!(ft.fold(l..r), naive[l..r].iter().sum());
                        }

                        for l in 0..N {
                            for r in l..=N {
                                assert_eq!(ft.fold(l..r), naive[l..r].iter().sum());
                            }
                        }
                    }
                }
            };
        }

        define_test_function!(test_i64, i64);
        define_test_function!(test_u64, u64);

        let mut rng = get_test_rng();

        test_i64(&mut rng, -1000000000, 1000000000);
        test_u64(&mut rng, 0, 1000000000);
    }
}
