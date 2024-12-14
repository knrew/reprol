use std::ops::{Add, Range, Sub};

pub struct CumulativeSum3D<T>(Vec<Vec<Vec<T>>>);

impl<T> CumulativeSum3D<T>
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

    pub fn sum(&self, x_range: Range<usize>, y_range: Range<usize>, z_range: Range<usize>) -> T {
        assert!(x_range.start <= x_range.end);
        assert!(y_range.start <= y_range.end);
        assert!(z_range.start <= z_range.end);
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
    use super::CumulativeSum3D;

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
        let cum = CumulativeSum3D::new(&v, 0);
        for ((x1, y1, z1, x2, y2, z2), expected) in test_cases {
            assert_eq!(cum.sum(x1..x2, y1..y2, z1..z2), expected);
        }
    }
}
