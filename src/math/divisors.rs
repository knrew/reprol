//! 約数列挙(divisors enumerator)
//!
//! 正の整数の約数を列挙する．
//!
//! ## 使用例
//! ```
//! use reprol::math::divisors::Divisors;
//! let d = 12u64.divisors().collect::<Vec<_>>();
//! assert_eq!(d, vec![1, 2, 3, 4, 6, 12]);
//! ```

pub trait Divisors: Sized {
    type Output: Iterator<Item = Self>;

    /// 約数を列挙する．
    /// 返り値は昇順にソート済みのイテレータ．
    fn divisors(self) -> Self::Output;
}

macro_rules! impl_divisors {
    ($divisor_iter: ident, $ty: ty) => {
        pub struct $divisor_iter {
            n: $ty,
            i: $ty,
            ascending_phase: bool,
        }

        impl Iterator for $divisor_iter {
            type Item = $ty;

            fn next(&mut self) -> Option<Self::Item> {
                let $divisor_iter {
                    n,
                    i,
                    ascending_phase,
                } = self;
                let n = *n;

                if *ascending_phase {
                    // 昇順に走査．
                    while *i * *i <= n {
                        let j = *i;
                        *i += 1;
                        if n % j == 0 {
                            return Some(j);
                        }
                    }

                    *i -= 1;
                    *ascending_phase = false;
                }

                // 降順に走査しながらn/iを収集．
                while *i >= 1 {
                    let j = *i;
                    *i -= 1;
                    if n % j == 0 {
                        let d = n / j;
                        if d != j {
                            return Some(d);
                        }
                    }
                }

                None
            }
        }

        impl Divisors for $ty {
            type Output = $divisor_iter;

            fn divisors(self) -> Self::Output {
                assert!(self > 0);
                $divisor_iter {
                    n: self,
                    i: 1,
                    ascending_phase: true,
                }
            }
        }
    };
}

macro_rules! impl_divisors_for {
    ($([$divisor_iter: ident, $ty: ty]),* $(,)?) => {
        $( impl_divisors!($divisor_iter, $ty); )*
    };
}

impl_divisors_for! {
    [I8DivisorIter, i8],
    [I16DivisorIter, i16],
    [I32DivisorIter, i32],
    [I64DivisorIter, i64],
    [I128DivisorIter, i128],
    [IsizeDivisorIter, isize],
    [U8DivisorIter, u8],
    [U16DivisorIter, u16],
    [U32DivisorIter, u32],
    [U64DivisorIter, u64],
    [U128DivisorIter, u128],
    [UsizeDivisorIter, usize],
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

        for (n, expected) in test_cases {
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
