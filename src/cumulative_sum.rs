use std::ops::{Add, Range, Sub};

pub struct CumulativeSum1D<T>(Vec<T>);

impl<T> CumulativeSum1D<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(v: &[T], zero: T) -> Self {
        debug_assert!(!v.is_empty());
        let mut cum = vec![zero; v.len() + 1];
        for i in 0..v.len() {
            cum[i + 1] = v[i] + cum[i];
        }
        Self(cum)
    }

    pub fn construct(n: usize, zero: T, mut f: impl FnMut(usize) -> T) -> Self {
        let mut cum = vec![zero; n + 1];
        for i in 0..n {
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

        let mut cum = vec![vec![zero; v[0].len() + 1]; v.len() + 1];

        for i in 0..v.len() {
            for j in 0..v[i].len() {
                cum[i + 1][j + 1] = v[i][j] + cum[i + 1][j] + cum[i][j + 1] - cum[i][j];
            }
        }

        Self(cum)
    }

    pub fn construct(h: usize, w: usize, zero: T, mut f: impl FnMut(usize, usize) -> T) -> Self {
        let mut cum = vec![vec![zero; w + 1]; h + 1];

        for i in 0..h {
            for j in 0..w {
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

        let mut cum = vec![vec![vec![zero; v[0][0].len() + 1]; v[0].len() + 1]; v.len() + 1];

        for i in 0..v.len() {
            for j in 0..v[i].len() {
                for k in 0..v[i][j].len() {
                    cum[i + 1][j + 1][k + 1] = v[i][j][k]
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
