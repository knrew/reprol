use std::ops::{Add, Range, Sub};

pub struct CumulativeSum1D<T>(Vec<T>);

impl<T> CumulativeSum1D<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(v: &[T], zero: T) -> Self {
        debug_assert!(!v.is_empty());
        Self::construct(v.len(), zero, |i| v[i])
    }

    pub fn construct(len: usize, zero: T, mut f: impl FnMut(usize) -> T) -> Self {
        let mut cum = vec![zero; len + 1];
        for i in 0..len {
            cum[i + 1] = f(i) + cum[i];
        }
        Self(cum)
    }

    /// 半区間[l, r)の和を計算する
    /// a[l]+ ... + a[r-1]
    pub fn get_sum(&self, range: Range<usize>) -> T {
        self.0[range.end] - self.0[range.start]
    }
}

pub struct CumulativeSum2D<T>(Vec<Vec<T>>);

impl<T> CumulativeSum2D<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(v: &[Vec<T>], zero: T) -> Self {
        debug_assert!(!v.is_empty());
        debug_assert!(!v[0].is_empty());
        Self::construct(v.len(), v[0].len(), zero, |i, j| v[i][j])
    }

    pub fn construct(
        x_len: usize,
        y_len: usize,
        zero: T,
        mut f: impl FnMut(usize, usize) -> T,
    ) -> Self {
        let mut cum = vec![vec![zero; y_len + 1]; x_len + 1];

        for i in 0..x_len {
            for j in 0..y_len {
                cum[i + 1][j + 1] = f(i, j) + cum[i + 1][j] + cum[i][j + 1] - cum[i][j];
            }
        }

        Self(cum)
    }

    pub fn get_sum(&self, x_range: Range<usize>, y_range: Range<usize>) -> T {
        self.0[x_range.end][y_range.end] + self.0[x_range.start][y_range.start]
            - self.0[x_range.start][y_range.end]
            - self.0[x_range.end][y_range.start]
    }
}

pub struct CumulativeSum3D<T>(Vec<Vec<Vec<T>>>);

impl<T> CumulativeSum3D<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(v: &[Vec<Vec<T>>], zero: T) -> Self {
        debug_assert!(!v.is_empty());
        debug_assert!(!v[0].is_empty());
        debug_assert!(!v[0][0].is_empty());
        Self::construct(v.len(), v[0].len(), v[0][0].len(), zero, |i, j, k| {
            v[i][j][k]
        })
    }

    pub fn construct(
        x_len: usize,
        y_len: usize,
        z_len: usize,
        zero: T,
        mut f: impl FnMut(usize, usize, usize) -> T,
    ) -> Self {
        let mut cum = vec![vec![vec![zero; z_len + 1]; y_len + 1]; x_len + 1];

        for i in 0..x_len {
            for j in 0..y_len {
                for k in 0..z_len {
                    cum[i + 1][j + 1][k + 1] = f(i, j, k)
                        + cum[i][j + 1][k + 1]
                        + cum[i + 1][j][k + 1]
                        + cum[i + 1][j + 1][k]
                        + cum[i][j][k]
                        - cum[i][j][k + 1]
                        - cum[i][j + 1][k]
                        - cum[i + 1][j][k];
                }
            }
        }

        Self(cum)
    }

    pub fn get_sum(
        &self,
        x_range: Range<usize>,
        y_range: Range<usize>,
        z_range: Range<usize>,
    ) -> T {
        self.0[x_range.end][y_range.end][z_range.end]
            + self.0[x_range.start][y_range.start][z_range.end]
            + self.0[x_range.start][y_range.end][z_range.start]
            + self.0[x_range.end][y_range.start][z_range.start]
            - self.0[x_range.start][y_range.end][z_range.end]
            - self.0[x_range.end][y_range.start][z_range.end]
            - self.0[x_range.end][y_range.end][z_range.start]
            - self.0[x_range.start][y_range.start][z_range.start]
    }
}

#[cfg(test)]
mod tests {
    use super::{CumulativeSum1D, CumulativeSum2D};
    use rand::Rng;

    #[test]
    fn test_cumulative_sum_1d() {
        let v = vec![1, 2, 3, 4, 5];
        let cum = CumulativeSum1D::new(&v, 0);
        let test_cases = vec![
            ((0, 5), 15),
            ((0, 1), 1),
            ((1, 3), 5),
            ((3, 5), 9),
            ((2, 4), 7),
            ((2, 2), 0),
            ((4, 5), 5),
            ((0, 4), 10),
        ];
        for ((l, r), expected) in test_cases {
            assert_eq!(cum.get_sum(l..r), expected);
        }

        let cum = CumulativeSum1D::construct(5, 0, |i| i as i32 + 1);
        let test_cases = vec![
            ((0, 5), 15),
            ((0, 1), 1),
            ((1, 3), 5),
            ((3, 5), 9),
            ((2, 4), 7),
            ((2, 2), 0),
            ((4, 5), 5),
            ((0, 4), 10),
        ];

        for ((l, r), expected) in test_cases {
            assert_eq!(cum.get_sum(l..r), expected);
        }
    }

    #[test]
    fn test_cumulative_sum_2d() {
        let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let cum = CumulativeSum2D::new(&v, 0);

        let test_cases = vec![
            ((0, 0, 3, 3), 45),
            ((0, 0, 2, 2), 12),
            ((1, 1, 3, 3), 28),
            ((0, 1, 2, 3), 16),
            ((2, 0, 3, 2), 15),
            ((0, 0, 1, 1), 1),
            ((0, 0, 0, 0), 0),
        ];

        for ((r1, c1, r2, c2), expected) in test_cases {
            assert_eq!(cum.get_sum(r1..r2, c1..c2), expected);
        }

        let v = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
        ];
        let cum = CumulativeSum2D::new(&v, 0);

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

        for ((r1, c1, r2, c2), expected) in test_cases {
            assert_eq!(cum.get_sum(r1..r2, c1..c2), expected);
        }
    }

    #[test]
    fn test_cumlative_2d_random() {
        let h = 30;
        let w = 20;

        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let v = (0..h)
                .map(|_| (0..w).map(|_| rng.gen_range(-1000..1000)).collect())
                .collect::<Vec<_>>();

            let sum = CumulativeSum2D::new(&v, 0);
            for from_i in 0..h {
                for to_i in from_i + 1..=h {
                    for from_j in 0..w {
                        for to_j in from_j + 1..=w {
                            let expected = (from_i..to_i)
                                .map(|i| (from_j..to_j).map(|j| v[i][j]).sum::<i64>())
                                .sum::<i64>();
                            assert_eq!(sum.get_sum(from_i..to_i, from_j..to_j), expected);
                        }
                    }
                }
            }
        }
    }
}
