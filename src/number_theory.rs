pub trait Gcd {
    fn gcd(self, rhs: Self) -> Self;
    fn lcm(self, rhs: Self) -> Self;
}

macro_rules! impl_signed {
    ($($ty:ident),*) => {$(
        impl Gcd for $ty {
            fn gcd(self, rhs: Self) -> Self {
                if rhs == 0 {
                    self.abs()
                } else {
                    rhs.gcd(self % rhs)
                }
            }

            fn lcm(self, rhs: Self) -> Self {
                self.abs() / self.gcd(rhs) * rhs.abs()
            }
        }
    )*};
}

impl_signed! { i8, i16, i32, i64, i128, isize }

macro_rules! impl_unsigned {
    ($($ty:ident),*) => {$(
        impl Gcd for $ty {
            fn gcd(self, rhs: Self) -> Self {
                if rhs == 0 {
                    self
                } else {
                    Self::gcd(rhs, self % rhs)
                }
            }

            fn lcm(self, rhs: Self) -> Self {
                self / self.gcd(rhs) * rhs
            }
        }
    )*};
}

impl_unsigned! { u8, u16, u32, u64, u128, usize }

pub trait NumberTheory: Sized {
    fn is_prime(self) -> bool;

    /// 約数を列挙する
    /// NOTE: 出力はソートされていないので必要ならソートすること
    fn divisors(self) -> Vec<Self>;

    /// 素因数分解する
    fn factors(self) -> Vec<(Self, u64)>;
}

macro_rules! impl_integer {
    ($($ty:ident),*) => {$(
        impl NumberTheory for $ty {
            fn is_prime(self) -> bool {
                if self <= 1 {
                    return false;
                }
                (2..).take_while(|i| i * i <= self).all(|i| self % i != 0)
            }

            fn divisors(self) -> Vec<Self> {
                let n = self;
                (1..)
                    .take_while(|i| i * i <= n)
                    .filter(|i| n % i == 0)
                    .flat_map(|i| if n / i == i { vec![i] } else { vec![i, n / i] }.into_iter())
                    .collect()
            }

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
    use super::{Gcd, NumberTheory};

    #[test]
    fn test_gcd() {
        let testcases = vec![
            ((48u64, 18), 6),
            ((54, 24), 6),
            ((101, 103), 1),
            ((0, 10), 10),
            ((10, 0), 10),
            ((0, 0), 0),
            ((48, 18), 6),
            ((54, 24), 6),
            ((42, 42), 42),
        ];

        for &((m, n), answer) in &testcases {
            assert_eq!(m.gcd(n), answer);
        }

        let testcases = vec![
            ((-48, -18), 6),
            ((-54, 24), 6),
            ((-101, -103), 1),
            ((-42, -42), 42),
        ];

        for &((m, n), answer) in &testcases {
            assert_eq!(m.gcd(n), answer);
        }

        let testcases = vec![(
            (1_000_000_000_000_000_000u128, 500_000_000_000_000_000u128),
            500_000_000_000_000_000u128,
        )];

        for &((m, n), answer) in &testcases {
            assert_eq!(m.gcd(n), answer);
        }
    }

    #[test]
    fn test_lcm() {
        let testcases = vec![
            ((4, 5), 20),
            ((6, 8), 24),
            ((7, 3), 21),
            ((10, 15), 30),
            ((7, 3), 21),
            ((9, 6), 18),
            ((42, 42), 42),
        ];

        for &((m, n), answer) in &testcases {
            assert_eq!(m.lcm(n), answer);
        }

        let testcases = vec![
            ((-4, 5), 20),
            ((-6, -8), 24),
            ((-7, 3), 21),
            ((-42, -42), 42),
        ];
        for &((m, n), answer) in &testcases {
            assert_eq!(m.lcm(n), answer);
        }

        let testcases = vec![(
            (1_000_000_000_000_000_000u128, 500_000_000_000_000_000u128),
            1_000_000_000_000_000_000u128,
        )];
        for &((m, n), answer) in &testcases {
            assert_eq!(m.lcm(n), answer);
        }
    }

    #[test]
    fn test_is_prime() {
        let test_cases = [
            (1u64, false),
            (2, true),
            (3, true),
            (4, false),
            (5, true),
            (6, false),
            (7, true),
            (8, false),
            (9, false),
            (10, false),
            (11, true),
            (12, false),
        ];

        for (n, ans) in test_cases {
            assert_eq!(n.is_prime(), ans);
        }
    }

    #[test]
    fn test_enumerate_divisors() {
        let test_cases = [
            (1u64, vec![1]),
            (2u64, vec![1, 2]),
            (3u64, vec![1, 3]),
            (4u64, vec![1, 2, 4]),
            (6u64, vec![1, 2, 3, 6]),
            (12u64, vec![1, 2, 3, 4, 6, 12]),
            (28u64, vec![1, 2, 4, 7, 14, 28]),
            (36u64, vec![1, 2, 3, 4, 6, 9, 12, 18, 36]),
            (49u64, vec![1, 7, 49]),
            (100u64, vec![1, 2, 4, 5, 10, 20, 25, 50, 100]),
        ];

        for (n, expected) in test_cases {
            let mut result = n.divisors();
            result.sort_unstable();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_prime_factorize() {
        let test_cases = [
            (1u64, vec![]),
            (2u64, vec![(2u64, 1)]),
            (3u64, vec![(3u64, 1)]),
            (4u64, vec![(2u64, 2)]),
            (6u64, vec![(2u64, 1), (3u64, 1)]),
            (8u64, vec![(2u64, 3)]),
            (12u64, vec![(2u64, 2), (3u64, 1)]),
            (100u64, vec![(2u64, 2), (5u64, 2)]),
            (210u64, vec![(2u64, 1), (3u64, 1), (5u64, 1), (7u64, 1)]),
            (1024u64, vec![(2u64, 10)]),
            (243, vec![(3u64, 5)]),
        ];

        for (n, expected) in test_cases {
            assert_eq!(n.factors(), expected);
        }
    }
}
