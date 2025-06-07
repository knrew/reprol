//! Disjoint Sparse Table
//!
//! 静的なモノイド列の区間積を計算するデータ構造．
//!
//! # 計算量
//! - 構築(前計算): `O(N log N)`
//! - 区間積の取得: `O(1)`
//!
//! # 使用例
//! ```
//! use reprol::ds::disjoint_sparse_table::DisjointSparseTable;
//! use reprol::ops::op_min::OpMin;
//! let dst = DisjointSparseTable::<OpMin<i64>>::new(vec![3, 5, 4, 100, 1]);
//! assert_eq!(dst.fold(1..4), 4); // 区間`[1, 4)`の最小値
//! assert_eq!(dst.fold(0..5), 1); // 区間`[0, 5)`の最小値
//! ```
//!
//! # Reference
//! - [Disjoint Sparse Table と セグ木に関するポエム - noshi91のメモ](https://noshi91.hatenablog.com/entry/2018/05/08/183946)

use std::{
    iter::FromIterator,
    ops::{Range, RangeBounds},
};

use crate::{ops::monoid::Monoid, range::to_open_range};

pub struct DisjointSparseTable<O: Monoid> {
    len: usize,
    data: Vec<Vec<O::Value>>,
    op: O,
}

impl<O: Monoid> DisjointSparseTable<O> {
    /// 配列`v`からDisjoint Sparse Tableを構築する．
    pub fn new(v: Vec<O::Value>) -> Self
    where
        O: Default,
    {
        Self::with_op(v, O::default())
    }

    /// 演算`op`を指定して，配列`v`からDisjoint Sparse Tableを構築する．
    pub fn with_op(v: Vec<O::Value>, op: O) -> Self {
        assert!(!v.is_empty());

        let n = v.len() + 2;
        let h = n.next_power_of_two().trailing_zeros() as usize;

        let mut data = Vec::with_capacity(h);
        data.push((0..n).map(|_| op.identity()).collect());

        for w in (1..h).map(|k| 1 << k) {
            let mut datum = (0..n).map(|_| op.identity()).collect::<Vec<_>>();

            for i in (w..n).step_by(w * 2) {
                for j in (i - w + 1..i).rev() {
                    datum[j - 1] = op.op(&v[j - 1], &datum[j]);
                }
                for j in i..(i + w).min(n) - 1 {
                    datum[j + 1] = op.op(&datum[j], &v[j - 1]);
                }
            }

            data.push(datum);
        }

        Self {
            len: v.len(),
            data,
            op,
        }
    }

    /// 指定したindexの値を返す．
    pub fn get(&self, index: usize) -> O::Value {
        self.fold(index..=index)
    }

    /// 区間`[l, r)`の区間積を返す．
    pub fn fold(&self, range: impl RangeBounds<usize>) -> O::Value {
        let Range {
            start: l,
            end: mut r,
        } = to_open_range(range, self.len);
        assert!(l <= r);
        assert!(r <= self.len);
        r += 1;
        let i = ((l ^ r) + 1).next_power_of_two().trailing_zeros() - 1;
        let datum = &self.data[i as usize];
        self.op.op(&datum[l], &datum[r])
    }
}

impl<O> From<(Vec<O::Value>, O)> for DisjointSparseTable<O>
where
    O: Monoid,
{
    fn from((v, op): (Vec<O::Value>, O)) -> Self {
        Self::with_op(v, op)
    }
}

impl<O, const N: usize> From<([O::Value; N], O)> for DisjointSparseTable<O>
where
    O: Monoid,
{
    fn from((v, op): ([O::Value; N], O)) -> Self {
        Self::with_op(v.into_iter().collect(), op)
    }
}

impl<O> From<Vec<O::Value>> for DisjointSparseTable<O>
where
    O: Monoid + Default,
{
    fn from(v: Vec<O::Value>) -> Self {
        Self::new(v)
    }
}

impl<O, const N: usize> From<[O::Value; N]> for DisjointSparseTable<O>
where
    O: Monoid + Default,
{
    fn from(v: [O::Value; N]) -> Self {
        Self::new(v.into_iter().collect())
    }
}

impl<O> FromIterator<O::Value> for DisjointSparseTable<O>
where
    O: Monoid + Default,
{
    fn from_iter<I: IntoIterator<Item = O::Value>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use rand::{rngs::StdRng, Rng, SeedableRng};

    use crate::ops::{op_add::OpAdd, op_max::OpMax, op_min::OpMin};

    use super::*;

    #[test]
    fn test_min() {
        let v = vec![2, 10, 1, 100];
        let test_cases = vec![
            ([0, 1], 2),
            ([0, 2], 2),
            ([0, 3], 1),
            ([0, 4], 1),
            ([1, 2], 10),
            ([1, 3], 1),
            ([1, 4], 1),
            ([2, 3], 1),
            ([2, 4], 1),
            ([3, 4], 100),
        ];

        let dst = DisjointSparseTable::<OpMin<i64>>::new(v);
        for ([l, r], expected) in test_cases {
            assert_eq!(dst.fold(l..r), expected);
        }
    }

    #[test]
    fn test_sum_random() {
        let mut rng = StdRng::seed_from_u64(30);

        for _ in 0..100 {
            let v = (0..100)
                .map(|_| rng.gen_range(-1000000000..=1000000000))
                .collect::<Vec<i64>>();
            let dst = DisjointSparseTable::<OpAdd<_>>::from(v.clone());
            for l in 0..v.len() {
                for r in l..=v.len() {
                    let naive = (l..r).map(|i| v[i]).sum::<i64>();
                    assert_eq!(dst.fold(l..r), naive);
                }
            }
        }

        for _ in 0..100 {
            let v = (0..100)
                .map(|_| rng.gen_range(0..=1000000000))
                .collect::<Vec<u64>>();
            let dst = DisjointSparseTable::<OpAdd<_>>::from(v.clone());
            for l in 0..v.len() {
                for r in l..=v.len() {
                    let naive = (l..r).map(|i| v[i]).sum::<u64>();
                    assert_eq!(dst.fold(l..r), naive);
                }
            }
        }
    }

    #[test]
    fn test_min_random() {
        let mut rng = StdRng::seed_from_u64(30);

        for _ in 0..100 {
            let v = (0..100).map(|_| rng.gen()).collect::<Vec<i64>>();
            let dst = DisjointSparseTable::<OpMin<_>>::from(v.clone());
            for l in 0..v.len() {
                for r in l..=v.len() {
                    let naive = (l..r).map(|i| v[i]).min().unwrap_or(i64::MAX);
                    assert_eq!(dst.fold(l..r), naive);
                }
            }
        }

        for _ in 0..100 {
            let v = (0..100).map(|_| rng.gen()).collect::<Vec<u64>>();
            let dst = DisjointSparseTable::<OpMin<_>>::from(v.clone());
            for l in 0..v.len() {
                for r in l..=v.len() {
                    let naive = (l..r).map(|i| v[i]).min().unwrap_or(u64::MAX);
                    assert_eq!(dst.fold(l..r), naive);
                }
            }
        }
    }

    #[test]
    fn test_max_random() {
        let mut rng = StdRng::seed_from_u64(30);

        for _ in 0..100 {
            let v = (0..100).map(|_| rng.gen()).collect::<Vec<i64>>();
            let dst = DisjointSparseTable::<OpMax<_>>::from(v.clone());
            for l in 0..v.len() {
                for r in l..=v.len() {
                    let naive = (l..r).map(|i| v[i]).max().unwrap_or(i64::MIN);
                    assert_eq!(dst.fold(l..r), naive);
                }
            }
        }

        for _ in 0..100 {
            let v = (0..100).map(|_| rng.gen()).collect::<Vec<u64>>();
            let dst = DisjointSparseTable::<OpMax<_>>::from(v.clone());
            for l in 0..v.len() {
                for r in l..=v.len() {
                    let naive = (l..r).map(|i| v[i]).max().unwrap_or(u64::MIN);
                    assert_eq!(dst.fold(l..r), naive);
                }
            }
        }
    }
}
