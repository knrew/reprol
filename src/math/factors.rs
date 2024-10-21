/// 素因数分解する
pub trait Factors: Sized {
    fn factors(self) -> Vec<(Self, u64)>;
}

macro_rules! impl_integer {
    ($($ty:ident),*) => {$(
        impl Factors for $ty {
            fn factors(self) -> Vec<(Self, u64)> {
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

                factors
            }
        }
    )*};
}

impl_integer! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

#[cfg(test)]
mod tests {
    use super::Factors;

    #[test]
    fn test_factors() {
        let test_cases = vec![
            (1, vec![]),
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
            assert_eq!(n.factors(), expected);
        }
    }
}
