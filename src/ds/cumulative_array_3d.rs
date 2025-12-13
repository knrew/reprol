//! 3次元累積積(累積和)

use std::{
    fmt::Debug,
    ops::{Index, Range, RangeBounds},
};

use crate::{
    ops::{group::Group, monoid::Monoid, op_add::OpAdd},
    utils::range::to_half_open_index_range,
};

/// 3次元累積積を管理するデータ構造
pub struct CumulativeArray3d<O: Monoid> {
    inner: Vec<Vec<Vec<O::Value>>>,
    op: O,
}

impl<O: Monoid> CumulativeArray3d<O> {
    /// 3次元配列の累積配列を構築する．
    pub fn new(v: Vec<Vec<Vec<O::Value>>>) -> Self
    where
        O: Group + Default,
    {
        Self::with_op(v, O::default())
    }

    /// 演算`op`を明示的に渡して3次元配列の累積配列を構築する．
    pub fn with_op(v: Vec<Vec<Vec<O::Value>>>, op: O) -> Self
    where
        O: Group,
    {
        assert!(!v.is_empty());
        assert!(!v[0].is_empty());
        assert!(!v[0][0].is_empty());
        debug_assert!(v.iter().all(|vi| vi.len() == v[0].len() && vi.iter().all(|vij|vij.len()==v[0][0].len()) ));

        let i_len = v.len();
        let j_len = v[0].len();
        let k_len = v[0][0].len();

        let mut inner: Vec<Vec<Vec<O::Value>>> = (0..i_len + 1)
            .map(|_| {
                (0..j_len + 1)
                    .map(|_| (0..k_len + 1).map(|_| op.identity()).collect())
                    .collect()
            })
            .collect();

        for i in 0..i_len {
            for j in 0..j_len {
                for k in 0..k_len {
                    let mut datum = op.op(&inner[i][j + 1][k + 1], &inner[i + 1][j][k + 1]);
                    datum = op.op(&datum, &inner[i + 1][j + 1][k]);
                    datum = op.op(&datum, &inner[i][j][k]);
                    datum = op.op(&datum, &op.inv(&inner[i][j][k + 1]));
                    datum = op.op(&datum, &op.inv(&inner[i][j + 1][k]));
                    datum = op.op(&datum, &op.inv(&inner[i + 1][j][k]));
                    datum = op.op(&datum, &v[i][j][k]);
                    inner[i + 1][j + 1][k + 1] = datum;
                }
            }
        }

        Self { inner, op }
    }

    /// `[0, i) x [0, j) x [0, k)`の累積積を返す．
    pub fn get(&self, i: usize, j: usize, k: usize) -> &O::Value {
        &self.inner[i][j][k]
    }

    /// 区間`[il, ir) x [jl, jr) x [kl, kr)`の累積積を返す．
    pub fn fold(
        &self,
        i_range: impl RangeBounds<usize>,
        j_range: impl RangeBounds<usize>,
        k_range: impl RangeBounds<usize>,
    ) -> O::Value
    where
        O: Group,
    {
        debug_assert!(!self.inner.is_empty());
        debug_assert!(!self.inner[0].is_empty());
        debug_assert!(!self.inner[0][0].is_empty());

        let Range { start: il, end: ir } = to_half_open_index_range(i_range, self.inner.len() - 1);
        let Range { start: jl, end: jr } =
            to_half_open_index_range(j_range, self.inner[0].len() - 1);
        let Range { start: kl, end: kr } =
            to_half_open_index_range(k_range, self.inner[0][0].len() - 1);

        assert!(il <= ir);
        assert!(jl <= jr);
        assert!(kl <= kr);

        let mut res = self.op.op(&self.inner[ir][jr][kr], &self.inner[il][jl][kr]);
        res = self.op.op(&res, &self.inner[il][jr][kl]);
        res = self.op.op(&res, &self.inner[ir][jl][kl]);
        res = self.op.op(&res, &self.op.inv(&self.inner[il][jr][kr]));
        res = self.op.op(&res, &self.op.inv(&self.inner[ir][jl][kr]));
        res = self.op.op(&res, &self.op.inv(&self.inner[ir][jr][kl]));
        res = self.op.op(&res, &self.op.inv(&self.inner[il][jl][kl]));

        res
    }
}

impl<O> From<(Vec<Vec<Vec<O::Value>>>, O)> for CumulativeArray3d<O>
where
    O: Group,
{
    fn from((v, op): (Vec<Vec<Vec<O::Value>>>, O)) -> Self {
        CumulativeArray3d::with_op(v, op)
    }
}

impl<O, const N: usize, const M: usize, const L: usize> From<([[[O::Value; L]; M]; N], O)>
    for CumulativeArray3d<O>
where
    O: Group,
{
    fn from((v, op): ([[[O::Value; L]; M]; N], O)) -> Self {
        let v: Vec<Vec<Vec<O::Value>>> = v
            .into_iter()
            .map(|vi| {
                vi.into_iter()
                    .map(|vij| vij.into_iter().collect())
                    .collect()
            })
            .collect();
        CumulativeArray3d::from((v, op))
    }
}

impl<O> From<Vec<Vec<Vec<O::Value>>>> for CumulativeArray3d<O>
where
    O: Group + Default,
{
    fn from(v: Vec<Vec<Vec<O::Value>>>) -> Self {
        CumulativeArray3d::new(v)
    }
}

impl<O, const N: usize, const M: usize, const L: usize> From<[[[O::Value; L]; M]; N]>
    for CumulativeArray3d<O>
where
    O: Group + Default,
{
    fn from(v: [[[O::Value; L]; M]; N]) -> Self {
        let v: Vec<Vec<Vec<O::Value>>> = v
            .into_iter()
            .map(|vi| {
                vi.into_iter()
                    .map(|vij| vij.into_iter().collect())
                    .collect()
            })
            .collect();
        CumulativeArray3d::from(v)
    }
}

impl<O> Clone for CumulativeArray3d<O>
where
    O: Monoid + Clone,
    O::Value: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            op: self.op.clone(),
        }
    }
}

impl<O> Index<[usize; 3]> for CumulativeArray3d<O>
where
    O: Monoid,
{
    type Output = O::Value;
    fn index(&self, [i, j, k]: [usize; 3]) -> &Self::Output {
        &self.inner[i][j][k]
    }
}

impl<O> Index<(usize, usize, usize)> for CumulativeArray3d<O>
where
    O: Monoid,
{
    type Output = O::Value;
    fn index(&self, (i, j, k): (usize, usize, usize)) -> &Self::Output {
        &self.inner[i][j][k]
    }
}

impl<O> Debug for CumulativeArray3d<O>
where
    O: Monoid,
    O::Value: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.inner.iter()).finish()
    }
}

/// 3次元累積和
pub type CumulativeSum3d<T> = CumulativeArray3d<OpAdd<T>>;

#[cfg(test)]
mod tests {
    use rand::{Rng, SeedableRng, rngs::StdRng};

    use super::*;

    #[test]
    fn test() {
        let v = vec![
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            vec![vec![10, 11, 12], vec![13, 14, 15], vec![16, 17, 18]],
            vec![vec![19, 20, 21], vec![22, 23, 24], vec![25, 26, 27]],
        ];
        let test_cases = vec![
            ((0, 0, 0, 3, 3, 3), 378),
            ((0, 0, 0, 2, 2, 2), 60),
            ((1, 1, 1, 3, 3, 3), 164),
            ((0, 0, 0, 1, 1, 1), 1),
            ((0, 1, 0, 3, 2, 3), 126),
            ((0, 0, 2, 2, 3, 3), 63),
            ((1, 0, 0, 3, 1, 1), 29),
            ((2, 1, 2, 3, 3, 3), 51),
            ((0, 0, 0, 0, 0, 0), 0),
        ];
        let cum = CumulativeSum3d::new(v.clone());
        assert_eq!(cum.fold(.., .., ..), 378);
        for ((x1, y1, z1, x2, y2, z2), expected) in test_cases {
            assert_eq!(cum.fold(x1..x2, y1..y2, z1..z2), expected);
        }
    }

    #[test]
    fn test_sum_random() {
        macro_rules! define_test_function {
            ($name:ident, $ty:ident) => {
                fn $name(rng: &mut StdRng, range: Range<$ty>) {
                    const T: usize = 100;
                    const N_MAX: usize = 10;

                    for _ in 0..T {
                        let n = rng.random_range(1..=N_MAX);
                        let m = rng.random_range(1..=N_MAX);
                        let l = rng.random_range(1..=N_MAX);

                        let v: Vec<Vec<Vec<$ty>>> = (0..n)
                            .map(|_| {
                                (0..m)
                                    .map(|_| {
                                        (0..l).map(|_| rng.random_range(range.clone())).collect()
                                    })
                                    .collect()
                            })
                            .collect();
                        let cum = CumulativeSum3d::new(v.clone());
                        for il in 0..v.len() {
                            for ir in il..=v.len() {
                                for jl in 0..v[0].len() {
                                    for jr in jl..=v[0].len() {
                                        for kl in 0..v[0][0].len() {
                                            for kr in kl..=v[0][0].len() {
                                                let expected = v[il..ir]
                                                    .iter()
                                                    .map(|vi| {
                                                        vi[jl..jr]
                                                            .iter()
                                                            .map(|vij| {
                                                                vij[kl..kr].iter().sum::<$ty>()
                                                            })
                                                            .sum::<$ty>()
                                                    })
                                                    .sum::<$ty>();
                                                assert_eq!(
                                                    cum.fold(il..ir, jl..jr, kl..kr),
                                                    expected
                                                );
                                            }
                                        }
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

        let mut rng = StdRng::seed_from_u64(30);
        test_i64(&mut rng, -1000000000..1000000000);
        test_u64(&mut rng, 0..1000000000);
    }
}
