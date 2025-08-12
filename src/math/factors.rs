//! 素因数分解(prime factorizaion)
//!
//! 正の整数を素因数分解する．
//!
//! ## 使用例
//! ```
//! use reprol::math::factors::Factors;
//! let d = 12u64.factors().collect::<Vec<_>>();
//! assert_eq!(d, vec![(2, 2), (3, 1)]);
//! ```

pub trait Factors: Sized {
    type Output: Iterator<Item = (Self, u32)>;

    /// 素因数分解する．
    /// 素因数と指数のタプルのイテレータを返す．
    fn factors(self) -> Self::Output;
}

macro_rules! impl_factors {
    ($factor_iter:ident, $ty: ty) => {
        pub struct $factor_iter {
            n: $ty,
            p: $ty,
            k: $ty,
        }

        impl Iterator for $factor_iter {
            type Item = ($ty, u32);
            fn next(&mut self) -> Option<Self::Item> {
                let $factor_iter { n, p, k } = self;

                if *n <= 1 {
                    return None;
                }

                if *p == 2 {
                    if *n % 2 == 0 {
                        let mut exp = 0;
                        while *n % 2 == 0 {
                            *n /= 2;
                            exp += 1;
                        }
                        *p = 3;
                        return Some((2, exp));
                    }
                    *p = 3;
                }

                if *p == 3 {
                    if *n % 3 == 0 {
                        let mut exp = 0;
                        while *n % 3 == 0 {
                            *n /= 3;
                            exp += 1;
                        }
                        *p = 5;
                        *k = 2;
                        return Some((3, exp));
                    }
                    *p = 5;
                    *k = 2;
                }

                while *p <= *n / *p {
                    if *n % *p == 0 {
                        let mut exp = 0;
                        while *n % *p == 0 {
                            *n /= *p;
                            exp += 1;
                        }
                        let prime = *p;
                        *p += *k;
                        *k = 6 - *k; // 2<->4
                        return Some((prime, exp));
                    }

                    *p += *k;
                    *k = 6 - *k;
                }

                let tmp = *n;
                *n = 1;
                Some((tmp, 1))
            }
        }

        impl Factors for $ty {
            type Output = $factor_iter;

            fn factors(self) -> Self::Output {
                $factor_iter {
                    n: self,
                    p: 2,
                    k: 0,
                }
            }
        }
    };
}

macro_rules! impl_factors_for {
    ($([$factor_iter: ident, $ty: ty]),* $(,)?) => {
        $( impl_factors!($factor_iter, $ty); )*
    };
}

impl_factors_for! {
    [I8FactorIter, i8],
    [I16FactorIter, i16],
    [I32FactorIter, i32],
    [I64FactorIter, i64],
    [I128FactorIter, i128],
    [IsizeFactorIter, isize],
    [U8FactorIter, u8],
    [U16FactorIter, u16],
    [U32FactorIter, u32],
    [U64FactorIter, u64],
    [U128FactorIter, u128],
    [UsizeFactorIter, usize],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factors() {
        let test_cases = &[
            (1i32, vec![]),
            (2, vec![(2, 1)]),
            (3, vec![(3, 1)]),
            (4, vec![(2, 2)]),
            (6, vec![(2, 1), (3, 1)]),
            (8, vec![(2, 3)]),
            (12, vec![(2, 2), (3, 1)]),
            (47, vec![(47, 1)]),
            (100, vec![(2, 2), (5, 2)]),
            (210, vec![(2, 1), (3, 1), (5, 1), (7, 1)]),
            (243, vec![(3, 5)]),
            (1024, vec![(2, 10)]),
        ];

        for (n, expected) in test_cases {
            assert!(
                n.factors().eq(expected.iter().copied()),
                "failed case: prime factorizaion of {}.",
                n
            );
        }
    }

    #[test]
    fn test_smoke_all_types() {
        assert!(6i8.factors().eq([(2, 1), (3, 1)]));
        assert!(6i16.factors().eq([(2, 1), (3, 1)]));
        assert!(6i32.factors().eq([(2, 1), (3, 1)]));
        assert!(6i64.factors().eq([(2, 1), (3, 1)]));
        assert!(6i128.factors().eq([(2, 1), (3, 1)]));
        assert!(6isize.factors().eq([(2, 1), (3, 1)]));
        assert!(6u8.factors().eq([(2, 1), (3, 1)]));
        assert!(6u16.factors().eq([(2, 1), (3, 1)]));
        assert!(6u32.factors().eq([(2, 1), (3, 1)]));
        assert!(6u64.factors().eq([(2, 1), (3, 1)]));
        assert!(6u128.factors().eq([(2, 1), (3, 1)]));
        assert!(6usize.factors().eq([(2, 1), (3, 1)]));
    }
}
