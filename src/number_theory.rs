use crate::integer::Unsigned;

pub fn is_prime<T: Unsigned>(n: T) -> bool {
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
pub fn enumerate_divisors<T: Unsigned>(n: T) -> Vec<T> {
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

pub fn prime_factorize<T: Unsigned>(n: T) -> Vec<(T, usize)> {
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
    use crate::number_theory::{enumerate_divisors, is_prime, prime_factorize};

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
            (1u64, vec![1]),                                 // 1
            (2u64, vec![1, 2]),                              // 1, 2
            (3u64, vec![1, 3]),                              // 1, 3
            (4u64, vec![1, 2, 4]),                           // 1, 2, 4
            (6u64, vec![1, 2, 3, 6]),                        // 1, 2, 3, 6
            (12u64, vec![1, 2, 3, 4, 6, 12]),                // 1, 2, 3, 4, 6, 12
            (28u64, vec![1, 2, 4, 7, 14, 28]),               // 1, 2, 4, 7, 14, 28
            (36u64, vec![1, 2, 3, 4, 6, 9, 12, 18, 36]),     // 1, 2, 3, 4, 6, 9, 12, 18, 36
            (49u64, vec![1, 7, 49]),                         // 1, 7, 49
            (100u64, vec![1, 2, 4, 5, 10, 20, 25, 50, 100]), // 1, 2, 4, 5, 10, 20, 25, 50, 100
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
            (1u64, vec![]),                                 // 1 = 1
            (2u64, vec![(2u64, 1usize)]),                   // 2 = 2
            (3u64, vec![(3u64, 1usize)]),                   // 3 = 3
            (4u64, vec![(2u64, 2usize)]),                   // 4 = 2^2
            (6u64, vec![(2u64, 1usize), (3u64, 1usize)]),   // 6 = 2 * 3
            (8u64, vec![(2u64, 3usize)]),                   // 8 = 2^3
            (12u64, vec![(2u64, 2usize), (3u64, 1usize)]),  // 12 = 2^2 * 3
            (100u64, vec![(2u64, 2usize), (5u64, 2usize)]), // 100 = 2^2 * 5^2
            (
                210u64,
                vec![
                    (2u64, 1usize),
                    (3u64, 1usize),
                    (5u64, 1usize),
                    (7u64, 1usize),
                ],
            ), // 210 = 2 * 3 * 5 * 7
            (1024u64, vec![(2u64, 10usize)]),               // 1024 = 2^10
            (243, vec![(3u64, 5usize)]),                    // 243 = 3^5
        ];

        for (n, expected) in test_cases {
            assert_eq!(prime_factorize(n), expected);
        }
    }
}
