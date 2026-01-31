//! 2次元累積積(累積和)
//!
//! 群の2次元配列に対する累積積を管理するデータ構造．
//! 区間積[`fold`](CumulativeArray2d::fold)も提供する．
//!
//! # 使用例
//! ## 2次元累積和
//! ```
//! use reprol::{
//!     ds::cumulative_array_2d::CumulativeSum2d,
//!     ops::op_add::OpAdd,
//! };
//! let v = vec![
//!     vec![1, 2, 3],
//!     vec![4, 5, 6],
//!     vec![7, 8, 9],
//! ];
//! let cum = CumulativeSum2d::new(v);
//! assert_eq!(cum.fold(0..3, 0..3), 45);
//! assert_eq!(cum.fold(1..3, 1..3), 28);
//! assert_eq!(cum.fold(0..2, 0..2), 12);
//! assert_eq!(*cum.prefix(2, 2), 12);
//! ```

use std::ops::{Range, RangeBounds};

use crate::{
    ops::{group::Group, monoid::Monoid, op_add::OpAdd},
    utils::range::to_half_open_index_range,
};

/// 2次元累積積を管理するデータ構造
pub struct CumulativeArray2d<O: Monoid> {
    inner: Vec<Vec<O::Element>>,
    op: O,
}

impl<O: Monoid> CumulativeArray2d<O> {
    /// 2次元配列の累積配列を構築する．
    pub fn new(v: Vec<Vec<O::Element>>) -> Self
    where
        O: Group + Default,
    {
        Self::with_op(v, O::default())
    }

    /// 演算`op`を明示的に渡して2次元配列の累積配列を構築する．
    pub fn with_op(v: Vec<Vec<O::Element>>, op: O) -> Self
    where
        O: Group,
    {
        assert!(!v.is_empty());
        assert!(!v[0].is_empty());
        debug_assert!(v.iter().all(|vi| vi.len() == v[0].len()));

        let row_len = v.len();
        let col_len = v[0].len();
        let mut inner: Vec<Vec<O::Element>> = (0..row_len + 1)
            .map(|_| (0..col_len + 1).map(|_| op.id()).collect())
            .collect();

        for i in 0..row_len {
            for j in 0..col_len {
                let mut datum = op.op(&inner[i + 1][j], &inner[i][j + 1]);
                datum = op.op(&datum, &op.inv(&inner[i][j]));
                datum = op.op(&datum, &v[i][j]);
                inner[i + 1][j + 1] = datum;
            }
        }

        Self { inner, op }
    }

    /// `[0, i) x [0, j)`の累積積を返す．
    pub fn prefix(&self, i: usize, j: usize) -> &O::Element {
        &self.inner[i][j]
    }

    pub fn get(&self, i: usize, j: usize) -> O::Element
    where
        O: Group,
    {
        self.fold(i..=i, j..=j)
    }

    /// 区間`[il, ir) x [jl, jr)`の累積積を返す．
    pub fn fold(
        &self,
        row_range: impl RangeBounds<usize>,
        col_range: impl RangeBounds<usize>,
    ) -> O::Element
    where
        O: Group,
    {
        debug_assert!(!self.inner.is_empty());
        debug_assert!(!self.inner[0].is_empty());
        let Range { start: il, end: ir } =
            to_half_open_index_range(row_range, self.inner.len() - 1);
        let Range { start: jl, end: jr } =
            to_half_open_index_range(col_range, self.inner[0].len() - 1);
        assert!(il <= ir);
        assert!(jl <= jr);
        let mut res = self.op.op(&self.inner[ir][jr], &self.inner[il][jl]);
        res = self.op.op(&res, &self.op.inv(&self.inner[il][jr]));
        res = self.op.op(&res, &self.op.inv(&self.inner[ir][jl]));
        res
    }
}

impl<O: Group> From<(Vec<Vec<O::Element>>, O)> for CumulativeArray2d<O> {
    fn from((v, op): (Vec<Vec<O::Element>>, O)) -> Self {
        CumulativeArray2d::with_op(v, op)
    }
}

impl<O: Group, const N: usize, const M: usize> From<([[O::Element; M]; N], O)>
    for CumulativeArray2d<O>
{
    fn from((v, op): ([[O::Element; M]; N], O)) -> Self {
        let v: Vec<Vec<O::Element>> = v.into_iter().map(|vi| vi.into_iter().collect()).collect();
        CumulativeArray2d::from((v, op))
    }
}

impl<O: Group + Default> From<Vec<Vec<O::Element>>> for CumulativeArray2d<O> {
    fn from(v: Vec<Vec<O::Element>>) -> Self {
        CumulativeArray2d::from((v, O::default()))
    }
}

impl<O: Group + Default, const N: usize, const M: usize> From<[[O::Element; M]; N]>
    for CumulativeArray2d<O>
{
    fn from(v: [[O::Element; M]; N]) -> Self {
        let v: Vec<Vec<O::Element>> = v.into_iter().map(|vi| vi.into_iter().collect()).collect();
        CumulativeArray2d::from(v)
    }
}

impl<O: Monoid + Clone> Clone for CumulativeArray2d<O>
where
    O::Element: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            op: self.op.clone(),
        }
    }
}

/// 2次元累積和
pub type CumulativeSum2d<T> = CumulativeArray2d<OpAdd<T>>;

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    use crate::utils::test_utils::initialize_rng;

    #[test]
    fn test_cumulative_sum_2d() {
        let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let test_cases = vec![
            ((0, 0, 3, 3), 45),
            ((0, 0, 2, 2), 12),
            ((1, 1, 3, 3), 28),
            ((0, 1, 2, 3), 16),
            ((2, 0, 3, 2), 15),
            ((0, 0, 1, 1), 1),
            ((0, 0, 0, 0), 0),
        ];
        let cum = CumulativeSum2d::new(v);
        assert_eq!(cum.fold(.., ..), 45);
        for ((r1, c1, r2, c2), expected) in test_cases {
            assert_eq!(cum.fold(r1..r2, c1..c2), expected);
        }

        let v = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
        ];
        let test_cases = vec![
            ((0, 0, 3, 5), 120),
            ((0, 0, 2, 3), 27),
            ((1, 2, 3, 5), 69),
            ((0, 1, 2, 4), 33),
            ((2, 0, 3, 2), 23),
            ((0, 0, 1, 1), 1),
            ((1, 0, 2, 5), 40),
            ((0, 4, 3, 5), 30),
            ((0, 0, 0, 0), 0),
        ];
        let cum = CumulativeSum2d::new(v);
        for ((x1, y1, x2, y2), expected) in test_cases {
            assert_eq!(cum.fold(x1..x2, y1..y2), expected);
        }
    }

    #[test]
    fn test_sum_random() {
        macro_rules! define_test_function {
            ($name:ident, $ty:ident) => {
                fn $name(rng: &mut impl Rng, range: Range<$ty>) {
                    const T: usize = 100;
                    const N_MAX: usize = 50;

                    for _ in 0..T {
                        let n = rng.random_range(1..=N_MAX);
                        let m = rng.random_range(1..=N_MAX);

                        let v: Vec<Vec<$ty>> = (0..n)
                            .map(|_| (0..m).map(|_| rng.random_range(range.clone())).collect())
                            .collect();
                        let cum = CumulativeSum2d::new(v.clone());
                        for il in 0..v.len() {
                            for ir in il..=v.len() {
                                for jl in 0..v[0].len() {
                                    for jr in jl..=v[0].len() {
                                        let expected = v[il..ir]
                                            .iter()
                                            .map(|vi| vi[jl..jr].iter().sum::<$ty>())
                                            .sum::<$ty>();
                                        assert_eq!(cum.fold(il..ir, jl..jr), expected);
                                    }
                                }
                            }
                        }
                    }
                }
            };
        }

        define_test_function!(test_i64, i64);
        define_test_function!(test_u64, u64);

        let mut rng = initialize_rng();
        test_i64(&mut rng, -1000000000..1000000000);
        test_u64(&mut rng, 0..1000000000);
    }
}
