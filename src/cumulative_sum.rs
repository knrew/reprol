use std::ops::{Add, Range, RangeBounds, Sub};

use crate::utilities::to_open_range;

pub struct CumulativeSum<T>(Vec<T>);

impl<T> CumulativeSum<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(v: &[T], zero: T) -> Self {
        assert!(!v.is_empty());
        Self::construct(v.len(), zero, |i| v[i])
    }

    pub fn construct(len: usize, zero: T, mut f: impl FnMut(usize) -> T) -> Self {
        let mut cum = vec![zero; len + 1];
        for i in 0..len {
            cum[i + 1] = f(i) + cum[i];
        }
        Self(cum)
    }

    /// 区間和を計算する
    /// a[l]+ ... + a[r-1]
    pub fn sum(&self, range: impl RangeBounds<usize>) -> T {
        let Range { start: l, end: r } = to_open_range(range, self.0.len() - 1);
        assert!(l <= r);
        self.0[r] - self.0[l]
    }
}

#[cfg(test)]
mod tests {
    use super::CumulativeSum;

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
        let cum = CumulativeSum::new(&v, 0);
        assert_eq!(cum.sum(..), 15);
        for ((l, r), expected) in test_cases {
            assert_eq!(cum.sum(l..r), expected);
        }

        let cum = CumulativeSum::construct(5, 0, |i| i as i32 + 1);
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
