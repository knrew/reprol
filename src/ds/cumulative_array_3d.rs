//! 3次元累積積(累積和)

use std::ops::{Range, RangeBounds};

use crate::{
    ops::{group::Group, monoid::Monoid, op_add::OpAdd},
    utils::range_utils::to_half_open_index_range,
};

/// 3次元累積積を管理するデータ構造
pub struct CumulativeArray3d<O: Monoid> {
    len_i: usize,
    len_j: usize,
    len_k: usize,
    stride_i: usize,
    stride_j: usize,
    inner: Vec<O::Element>,
    op: O,
}

impl<O: Monoid> CumulativeArray3d<O> {
    /// 3次元配列の累積配列を構築する．
    pub fn new(v: Vec<Vec<Vec<O::Element>>>) -> Self
    where
        O: Group + Default,
    {
        Self::with_op(v, O::default())
    }

    /// 演算`op`を明示的に渡して3次元配列の累積配列を構築する．
    pub fn with_op(v: Vec<Vec<Vec<O::Element>>>, op: O) -> Self
    where
        O: Group,
    {
        assert!(!v.is_empty());
        assert!(!v[0].is_empty());
        assert!(!v[0][0].is_empty());
        debug_assert!(v
            .iter()
            .all(|vi| vi.len() == v[0].len() && vi.iter().all(|vij| vij.len() == v[0][0].len())));

        let len_i = v.len();
        let len_j = v[0].len();
        let len_k = v[0][0].len();
        let stride_j = len_k + 1;
        let stride_i = (len_j + 1) * stride_j;
        let len = (len_i + 1) * stride_i;

        let mut cum = Self {
            len_i,
            len_j,
            len_k,
            stride_i,
            stride_j,
            inner: (0..len).map(|_| op.id()).collect(),
            op,
        };

        for (i, vi) in v.iter().enumerate() {
            for (j, vij) in vi.iter().enumerate() {
                for (k, vijk) in vij.iter().enumerate() {
                    let mut value = cum
                        .op
                        .op(cum.prefix(i, j + 1, k + 1), cum.prefix(i + 1, j, k + 1));
                    value = cum.op.op(&value, cum.prefix(i + 1, j + 1, k));
                    value = cum.op.op(&value, cum.prefix(i, j, k));
                    value = cum.op.op(&value, &cum.op.inv(cum.prefix(i, j, k + 1)));
                    value = cum.op.op(&value, &cum.op.inv(cum.prefix(i, j + 1, k)));
                    value = cum.op.op(&value, &cum.op.inv(cum.prefix(i + 1, j, k)));
                    value = cum.op.op(&value, vijk);
                    let index = cum.idx(i + 1, j + 1, k + 1);
                    cum.inner[index] = value;
                }
            }
        }

        cum
    }

    #[inline(always)]
    fn idx(&self, i: usize, j: usize, k: usize) -> usize {
        i * self.stride_i + j * self.stride_j + k
    }

    /// `[0, i) x [0, j) x [0, k)`の累積積を返す．
    pub fn prefix(&self, i: usize, j: usize, k: usize) -> &O::Element {
        &self.inner[self.idx(i, j, k)]
    }

    pub fn get(&self, i: usize, j: usize, k: usize) -> O::Element
    where
        O: Group,
    {
        self.fold(i..=i, j..=j, k..=k)
    }

    /// 区間`[il, ir) x [jl, jr) x [kl, kr)`の累積積を返す．
    pub fn fold(
        &self,
        i_range: impl RangeBounds<usize>,
        j_range: impl RangeBounds<usize>,
        k_range: impl RangeBounds<usize>,
    ) -> O::Element
    where
        O: Group,
    {
        let Range { start: il, end: ir } = to_half_open_index_range(i_range, self.len_i);
        let Range { start: jl, end: jr } = to_half_open_index_range(j_range, self.len_j);
        let Range { start: kl, end: kr } = to_half_open_index_range(k_range, self.len_k);

        assert!(il <= ir);
        assert!(jl <= jr);
        assert!(kl <= kr);

        let mut prod = self.op.op(self.prefix(ir, jr, kr), self.prefix(il, jl, kr));
        prod = self.op.op(&prod, self.prefix(il, jr, kl));
        prod = self.op.op(&prod, self.prefix(ir, jl, kl));
        prod = self.op.op(&prod, &self.op.inv(self.prefix(il, jr, kr)));
        prod = self.op.op(&prod, &self.op.inv(self.prefix(ir, jl, kr)));
        prod = self.op.op(&prod, &self.op.inv(self.prefix(ir, jr, kl)));
        prod = self.op.op(&prod, &self.op.inv(self.prefix(il, jl, kl)));

        prod
    }
}

impl<O: Group> From<(Vec<Vec<Vec<O::Element>>>, O)> for CumulativeArray3d<O> {
    fn from((v, op): (Vec<Vec<Vec<O::Element>>>, O)) -> Self {
        CumulativeArray3d::with_op(v, op)
    }
}

impl<O: Group, const N: usize, const M: usize, const L: usize> From<([[[O::Element; L]; M]; N], O)>
    for CumulativeArray3d<O>
{
    fn from((v, op): ([[[O::Element; L]; M]; N], O)) -> Self {
        let v: Vec<Vec<Vec<O::Element>>> = v
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

impl<O: Group + Default> From<Vec<Vec<Vec<O::Element>>>> for CumulativeArray3d<O> {
    fn from(v: Vec<Vec<Vec<O::Element>>>) -> Self {
        CumulativeArray3d::new(v)
    }
}

impl<O: Group + Default, const N: usize, const M: usize, const L: usize>
    From<[[[O::Element; L]; M]; N]> for CumulativeArray3d<O>
{
    fn from(v: [[[O::Element; L]; M]; N]) -> Self {
        let v: Vec<Vec<Vec<O::Element>>> = v
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

/// 3次元累積和
pub type CumulativeSum3d<T> = CumulativeArray3d<OpAdd<T>>;

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    use crate::utils::test_utils::random::get_test_rng;

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

    macro_rules! random_sum_test {
        ($test_name: ident, $ty: ty, $num_testcases: expr, $num_elements_max: expr, $range: expr) => {
            #[test]
            fn $test_name() {
                let mut rng = get_test_rng();
                for _ in 0..$num_testcases {
                    let len_i = rng.random_range(1..=$num_elements_max);
                    let len_j = rng.random_range(1..=$num_elements_max);
                    let len_k = rng.random_range(1..=$num_elements_max);
                    let v: Vec<Vec<Vec<$ty>>> = (0..len_i)
                        .map(|_| {
                            (0..len_j)
                                .map(|_| (0..len_k).map(|_| rng.random_range($range)).collect())
                                .collect()
                        })
                        .collect();
                    let cum = CumulativeSum3d::<$ty>::new(v.clone());
                    for il in 0..len_i {
                        for ir in il + 1..=len_i {
                            for jl in 0..len_j {
                                for jr in jl + 1..=len_j {
                                    for kl in 0..len_k {
                                        for kr in kl + 1..=len_k {
                                            let naive = v[il..ir]
                                                .iter()
                                                .map(|vi| {
                                                    vi[jl..jr]
                                                        .iter()
                                                        .map(|vij| vij[kl..kr].iter().sum::<$ty>())
                                                        .sum::<$ty>()
                                                })
                                                .sum::<$ty>();
                                            assert_eq!(cum.fold(il..ir, jl..jr, kl..kr), naive);
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

    random_sum_test!(test_random_sum_i32, i32, 10, 10, -100000..=100000);
    random_sum_test!(test_random_sum_u32, u32, 10, 10, 0..=100000);
    random_sum_test!(test_random_sum_i64, i64, 10, 10, -1000000000..=1000000000);
    random_sum_test!(test_random_sum_u64, u64, 10, 10, 0..=1000000000);
    random_sum_test!(test_random_sum_usize, usize, 10, 10, 0..=1000000000);
}
