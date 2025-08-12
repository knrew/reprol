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
//!
//! ## NOTE
//! AtCoderジャッジのRustバージョンアップデート後にイテレータまわりを修正する．

pub trait Factors: Sized {
    type Output: Iterator<Item = (Self, u32)>;

    /// 素因数分解する．
    /// 素因数と指数のタプルのイテレータを返す．
    fn factors(self) -> Self::Output;
}

macro_rules! impl_factors {
    ($ty: ty) => {
        impl Factors for $ty {
            type Output = <Vec<(Self, u32)> as IntoIterator>::IntoIter;

            fn factors(self) -> Self::Output {
                assert!(self > 0);
                let mut n = self;

                let mut factors = vec![];

                for i in 2.. {
                    if i * i > n {
                        break;
                    }

                    let mut ex = 0;

                    while n % i == 0 {
                        ex += 1;
                        n = n / i;
                    }

                    if ex != 0 {
                        factors.push((i, ex));
                    }
                }

                if n != 1 {
                    factors.push((n, 1));
                }

                factors.into_iter()
            }
        }
    };
}

macro_rules! impl_factors_for {
    ($($ty: ty),* $(,)?) => {
        $( impl_factors!($ty); )*
    };
}

impl_factors_for! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
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
