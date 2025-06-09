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

use crate::{ops::group::Group, range::to_open_range};

/// Fenwick Tree
pub struct FenwickTree<O: Group> {
    nodes: Vec<O::Value>,
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
            nodes: (0..n).map(|_| op.identity()).collect(),
            op,
        }
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

    /// `index`番目の要素の値を`value`にする．
    pub fn set(&mut self, index: usize, value: O::Value) {
        let diff = self.op.op(&value, &self.op.inv(&self.get(index)));
        self.op(index, &diff);
    }

    /// `index`番目の要素の値を返す．
    pub fn get(&self, index: usize) -> O::Value {
        self.fold(index..=index)
    }

    /// 区間`[0, r)`の区間積を返す．
    fn cum(&self, mut r: usize) -> O::Value {
        assert!(r <= self.nodes.len());
        let mut res = self.op.identity();
        while r > 0 {
            res = self.op.op(&res, &self.nodes[r - 1]);
            r -= r & r.wrapping_neg();
        }
        res
    }

    /// 区間`[l, r)`の区間積を返す．
    pub fn fold(&self, range: impl RangeBounds<usize>) -> O::Value {
        let Range { start: l, end: r } = to_open_range(range, self.nodes.len());
        assert!(l <= r);
        let cl = self.cum(l);
        let cr = self.cum(r);
        self.op.op(&cr, &self.op.inv(&cl))
    }
}

impl<O: Group> From<(Vec<O::Value>, O)> for FenwickTree<O> {
    fn from((v, op): (Vec<O::Value>, O)) -> Self {
        let mut res = Self::with_op(v.len(), op);
        v.into_iter()
            .enumerate()
            .for_each(|(i, rhs)| res.op(i, &rhs));
        res
    }
}

impl<O: Group, const N: usize> From<([O::Value; N], O)> for FenwickTree<O> {
    fn from((v, op): ([O::Value; N], O)) -> Self {
        Self::from((v.into_iter().collect::<Vec<_>>(), op))
    }
}

impl<O> From<Vec<O::Value>> for FenwickTree<O>
where
    O: Group + Default,
{
    fn from(v: Vec<O::Value>) -> Self {
        Self::from((v, O::default()))
    }
}

impl<O, const N: usize> From<[O::Value; N]> for FenwickTree<O>
where
    O: Group + Default,
{
    fn from(v: [O::Value; N]) -> Self {
        Self::from((v, O::default()))
    }
}

impl<O> FromIterator<O::Value> for FenwickTree<O>
where
    O: Group + Default,
{
    fn from_iter<I: IntoIterator<Item = O::Value>>(iter: I) -> Self {
        Self::from(iter.into_iter().collect::<Vec<_>>())
    }
}

impl<O> Clone for FenwickTree<O>
where
    O: Group + Clone,
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
    use rand::{rngs::StdRng, Rng, SeedableRng};

    use crate::ops::op_add::OpAdd;

    use super::*;

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
                fn $name(rng: &mut StdRng, mn: $ty, mx: $ty) {
                    const T: usize = 100;
                    const N: usize = 100;
                    const Q: usize = 1000;

                    for _ in 0..T {
                        let mut ft = FenwickTree::<OpAdd<_>>::new(N);
                        let mut naive = vec![0; N];

                        for _ in 0..Q {
                            // add
                            // v[index] += d
                            let index = rng.gen_range(0..N);
                            let d = rng.gen_range(mn..=mx);
                            ft.op(index, &d);
                            naive[index] += d;

                            // [l, r)の区間和を求める
                            let l = rng.gen_range(0..N);
                            let r = rng.gen_range(l..=N);
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

        let mut rng = StdRng::seed_from_u64(30);

        test_i64(&mut rng, -1000000000, 1000000000);
        test_u64(&mut rng, 0, 1000000000);
    }
}
