//! 約数列挙(divisors enumerator)
//!
//! 非負整数の約数を列挙する．
//!
//! ## 使用例
//! ```
//! use reprol::math::divisors::Divisors;
//! let d = 12u64.divisors().collect::<Vec<_>>();
//! assert_eq!(d, vec![1, 2, 3, 4, 6, 12]);
//! ```
//!
//! ## NOTE
//! AtCoderジャッジのRustバージョンアップデート後にイテレータまわりを修正する．

pub trait Divisors: Sized {
    type Output: Iterator<Item = Self>;

    /// 約数を列挙する．
    /// 返り値は昇順にソート済みのイテレータ．
    fn divisors(self) -> Self::Output;

    /// 約数を列挙する．
    /// 返り値はソートされているとは限らない．
    fn divisors_unsorted(self) -> Self::Output;
}

macro_rules! impl_divisors {
    ($ty: ty) => {
        impl Divisors for $ty {
            type Output = <Vec<Self> as IntoIterator>::IntoIter;

            #[allow(unused_comparisons)]
            fn divisors_unsorted(self) -> Self::Output {
                debug_assert!(self >= 0);
                let n = self;
                (1..)
                    .take_while(|i| i * i <= n)
                    .filter(|i| n % i == 0)
                    .flat_map(|i| if n / i == i { vec![i] } else { vec![i, n / i] }.into_iter())
                    .collect::<Vec<_>>()
                    .into_iter()
            }

            fn divisors(self) -> Self::Output {
                let mut res = self.divisors_unsorted().collect::<Vec<_>>();
                res.sort_unstable();
                res.into_iter()
            }
        }
    };
}

macro_rules! impl_divisors_for {
    ($($ty: ty),* $(,)?) => {
        $( impl_divisors!($ty); )*
    };
}

impl_divisors_for! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisors() {
        let test_cases: &[(i32, Vec<i32>)] = &[
            (1i32, vec![1]),
            (2, vec![1, 2]),
            (3, vec![1, 3]),
            (4, vec![1, 2, 4]),
            (6, vec![1, 2, 3, 6]),
            (12, vec![1, 2, 3, 4, 6, 12]),
            (28, vec![1, 2, 4, 7, 14, 28]),
            (36, vec![1, 2, 3, 4, 6, 9, 12, 18, 36]),
            (47, vec![1, 47]),
            (49, vec![1, 7, 49]),
            (100, vec![1, 2, 4, 5, 10, 20, 25, 50, 100]),
        ];

        for (n, ref expected) in test_cases {
            assert!(
                n.divisors().eq(expected.iter().copied()),
                "failed case: divisors of {}",
                n
            );
        }
    }

    #[test]
    fn test_smoke_all_types() {
        assert!(6i8.divisors().eq([1, 2, 3, 6]));
        assert!(6i16.divisors().eq([1, 2, 3, 6]));
        assert!(6i32.divisors().eq([1, 2, 3, 6]));
        assert!(6i64.divisors().eq([1, 2, 3, 6]));
        assert!(6i128.divisors().eq([1, 2, 3, 6]));
        assert!(6isize.divisors().eq([1, 2, 3, 6]));
        assert!(6u8.divisors().eq([1, 2, 3, 6]));
        assert!(6u16.divisors().eq([1, 2, 3, 6]));
        assert!(6u32.divisors().eq([1, 2, 3, 6]));
        assert!(6u64.divisors().eq([1, 2, 3, 6]));
        assert!(6u128.divisors().eq([1, 2, 3, 6]));
        assert!(6usize.divisors().eq([1, 2, 3, 6]));
    }
}
