use crate::integer::Integer;

pub fn gcd<T: Integer>(m: T, n: T) -> T {
    if n == T::ZERO {
        abs(m)
    } else {
        gcd(n, m % n)
    }
}

pub fn lcm<T: Integer>(m: T, n: T) -> T {
    abs(m) / gcd(m, n) * abs(n)
}

#[inline]
fn abs<T: Integer>(n: T) -> T {
    if n < T::ZERO {
        T::ZERO - n
    } else {
        n
    }
}

pub fn is_prime<T: Integer>(n: T) -> bool {
    if n <= T::ONE {
        return false;
    }

    let mut i = T::TWO;
    while i * i <= n {
        if n % i == T::ZERO {
            return false;
        }
        i = i + T::ONE;
    }

    true
}

/// NOTE: 出力はソートされていないので必要ならソートすること
pub fn enumerate_divisors<T: Integer>(n: T) -> Vec<T> {
    let mut divisors = vec![];

    let mut i = T::ONE;
    while i * i <= n {
        if n % i == T::ZERO {
            divisors.push(i);
            if n / i != i {
                divisors.push(n / i);
            }
        }
        i = i + T::ONE;
    }

    divisors
}

pub fn prime_factorize<T: Integer>(n: T) -> Vec<(T, usize)> {
    let mut n = n;

    let mut factors = vec![];

    let mut i = T::TWO;
    while i * i <= n {
        let mut ex = 0;
        while n % i == T::ZERO {
            ex += 1;
            n = n / i;
        }

        if ex != 0 {
            factors.push((i, ex));
        }

        i = i + T::ONE;
    }

    if n != T::ONE {
        factors.push((n, 1));
    }

    factors
}

#[cfg(test)]
mod tests {
    use super::{enumerate_divisors, gcd, is_prime, lcm, prime_factorize};

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(54, 24), 6);
        assert_eq!(gcd(101, 103), 1);
        assert_eq!(gcd(0, 10), 10);
        assert_eq!(gcd(10, 0), 10);
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(48u32, 18u32), 6);
        assert_eq!(gcd(54u64, 24u64), 6);
        assert_eq!(gcd(-48, -18), 6);
        assert_eq!(gcd(-54, 24), 6);
        assert_eq!(gcd(-101, -103), 1);
        assert_eq!(
            gcd(1_000_000_000_000_000_000u128, 500_000_000_000_000_000u128),
            500_000_000_000_000_000u128
        );
        assert_eq!(gcd(42, 42), 42);
        assert_eq!(gcd(-42, -42), 42);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 5), 20);
        assert_eq!(lcm(6, 8), 24);
        assert_eq!(lcm(7, 3), 21);
        assert_eq!(lcm(10, 15), 30);
        assert_eq!(lcm(7u32, 3u32), 21);
        assert_eq!(lcm(9u64, 6u64), 18);
        assert_eq!(lcm(-4, 5), 20);
        assert_eq!(lcm(-6, -8), 24);
        assert_eq!(lcm(-7, 3), 21);
        assert_eq!(
            lcm(1_000_000_000_000_000_000u128, 500_000_000_000_000_000u128),
            1_000_000_000_000_000_000u128
        );
        assert_eq!(lcm(42, 42), 42);
        assert_eq!(lcm(-42, -42), 42);
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
            assert_eq!(is_prime(n), ans);
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
            let mut result = enumerate_divisors(n);
            result.sort_unstable();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_prime_factorize() {
        let test_cases = [
            (1u64, vec![]),
            (2u64, vec![(2u64, 1usize)]),
            (3u64, vec![(3u64, 1usize)]),
            (4u64, vec![(2u64, 2usize)]),
            (6u64, vec![(2u64, 1usize), (3u64, 1usize)]),
            (8u64, vec![(2u64, 3usize)]),
            (12u64, vec![(2u64, 2usize), (3u64, 1usize)]),
            (100u64, vec![(2u64, 2usize), (5u64, 2usize)]),
            (
                210u64,
                vec![
                    (2u64, 1usize),
                    (3u64, 1usize),
                    (5u64, 1usize),
                    (7u64, 1usize),
                ],
            ),
            (1024u64, vec![(2u64, 10usize)]),
            (243, vec![(3u64, 5usize)]),
        ];

        for (n, expected) in test_cases {
            assert_eq!(prime_factorize(n), expected);
        }
    }
}
