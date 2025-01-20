use std::ops::{Add, Range, RangeBounds, Sub};

use crate::utilities::to_open_range;

pub struct CumulativeSum3d<T>(Vec<Vec<Vec<T>>>);

impl<T> CumulativeSum3d<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(v: &[Vec<Vec<T>>], zero: T) -> Self {
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

    pub fn sum(
        &self,
        x_range: impl RangeBounds<usize>,
        y_range: impl RangeBounds<usize>,
        z_range: impl RangeBounds<usize>,
    ) -> T {
        let Range { start: xl, end: xr } = to_open_range(x_range, self.0.len() - 1);
        let Range { start: yl, end: yr } = to_open_range(y_range, self.0[0].len() - 1);
        let Range { start: zl, end: zr } = to_open_range(z_range, self.0[0][0].len() - 1);
        assert!(xl <= xr);
        assert!(yl <= yr);
        assert!(zl <= zr);
        self.0[xr][yr][zr] + self.0[xl][yl][zr] + self.0[xl][yr][zl] + self.0[xr][yl][zl]
            - self.0[xl][yr][zr]
            - self.0[xr][yl][zr]
            - self.0[xr][yr][zl]
            - self.0[xl][yl][zl]
    }
}

#[cfg(test)]
mod tests {
    use super::CumulativeSum3d;

    #[test]
    fn test_cumultaive_sum_3d() {
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
        let cum = CumulativeSum3d::new(&v, 0);
        assert_eq!(cum.sum(.., .., ..), 378);
        for ((x1, y1, z1, x2, y2, z2), expected) in test_cases {
            assert_eq!(cum.sum(x1..x2, y1..y2, z1..z2), expected);
        }
    }
}
