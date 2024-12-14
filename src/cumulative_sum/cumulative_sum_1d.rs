use std::ops::{Add, Range, Sub};

pub struct CumulativeSum1D<T>(Vec<T>);

impl<T> CumulativeSum1D<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(v: &[T], zero: T) -> Self {
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
    pub fn sum(&self, range: Range<usize>) -> T {
        assert!(range.start <= range.end);
        self.0[range.end] - self.0[range.start]
    }
}

#[cfg(test)]
mod tests {
    use super::CumulativeSum1D;

    #[test]
    fn test_cumulative_sum_1d() {
        let v = vec![1, 2, 3, 4, 5];
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
        let cum = CumulativeSum1D::new(&v, 0);
        for ((l, r), expected) in test_cases {
            assert_eq!(cum.sum(l..r), expected);
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
            assert_eq!(cum.sum(l..r), expected);
        }
    }
}
