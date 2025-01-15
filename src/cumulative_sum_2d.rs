use std::ops::{Add, Range, Sub};

pub struct CumulativeSum2d<T>(Vec<Vec<T>>);

impl<T> CumulativeSum2d<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(v: &[Vec<T>], zero: T) -> Self {
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

    pub fn sum(&self, x_range: Range<usize>, y_range: Range<usize>) -> T {
        assert!(x_range.start <= x_range.end);
        assert!(y_range.start <= y_range.end);
        self.0[x_range.end][y_range.end] + self.0[x_range.start][y_range.start]
            - self.0[x_range.start][y_range.end]
            - self.0[x_range.end][y_range.start]
    }
}

#[cfg(test)]
mod tests {
    use super::CumulativeSum2d;

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
        let cum = CumulativeSum2d::new(&v, 0);
        for ((r1, c1, r2, c2), expected) in test_cases {
            assert_eq!(cum.sum(r1..r2, c1..c2), expected);
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
        let cum = CumulativeSum2d::new(&v, 0);
        for ((x1, y1, x2, y2), expected) in test_cases {
            assert_eq!(cum.sum(x1..x2, y1..y2), expected);
        }
    }

    // #[test]
    // fn test_cumlative_2d_random() {
    // use rand::Rng;
    //     let h = 30;
    //     let w = 20;

    //     let mut rng = rand::thread_rng();

    //     for _ in 0..10 {
    //         let v = (0..h)
    //             .map(|_| (0..w).map(|_| rng.gen_range(-1000..1000)).collect())
    //             .collect::<Vec<_>>();

    //         let sum = CumulativeSum2D::new(&v, 0);
    //         for from_i in 0..h {
    //             for to_i in from_i + 1..=h {
    //                 for from_j in 0..w {
    //                     for to_j in from_j + 1..=w {
    //                         let expected = (from_i..to_i)
    //                             .map(|i| (from_j..to_j).map(|j| v[i][j]).sum::<i64>())
    //                             .sum::<i64>();
    //                         assert_eq!(sum.get_sum(from_i..to_i, from_j..to_j), expected);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
}
