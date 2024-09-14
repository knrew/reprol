use crate::integer::Integer;

pub struct LinearSieve {
    lpf: Vec<usize>,
}

impl LinearSieve {
    const UNASSIGNED: usize = usize::MAX;

    pub fn new<T: Integer>(n: T) -> Self {
        let n = n.as_usize();

        let mut primes = vec![];
        let mut lpf = vec![Self::UNASSIGNED; n + 1];

        for i in 2..=n {
            if lpf[i] == Self::UNASSIGNED {
                lpf[i] = i;
                primes.push(i);
            }

            for &p in &primes {
                if p * i > n || p > lpf[i] {
                    break;
                }
                lpf[p * i] = p;
            }
        }

        Self { lpf }
    }

    pub fn is_prime<T: Integer>(&self, x: T) -> bool {
        let x = x.as_usize();
        self.lpf[x] == x
    }

    pub fn factorize<T: Integer>(&self, x: T) -> Vec<(T, usize)> {
        let mut x = x.as_usize();

        let mut factors = vec![];

        while x > 1 {
            let p = self.lpf[x];
            let mut ex = 0;
            while self.lpf[x] == p {
                x /= p;
                ex += 1;
            }
            factors.push((T::from_usize(p), ex));
        }

        factors
    }

    pub fn enumerate_divisors<T: Integer>(&self, x: T) -> Vec<T> {
        let mut divisors = vec![T::ONE];
        let factors = self.factorize(x);

        for &(factor, ex) in &factors {
            for i in 0..divisors.len() {
                let mut v = T::ONE;
                for _ in 0..ex {
                    v *= factor;
                    divisors.push(divisors[i] * v);
                }
            }
        }

        divisors
    }
}

#[cfg(test)]
mod tests {
    use crate::linear_sieve::LinearSieve;

    #[test]
    fn test_is_prime() {
        let sieve = LinearSieve::new(12);

        let test_cases = [
            (1, false),
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
            assert_eq!(sieve.is_prime(n), ans);
        }
    }

    #[test]
    fn test_enumerate_divisors() {
        let sieve = LinearSieve::new(100);

        let test_cases = [
            (1, vec![1]),                                 // 1
            (2, vec![1, 2]),                              // 1, 2
            (3, vec![1, 3]),                              // 1, 3
            (4, vec![1, 2, 4]),                           // 1, 2, 4
            (6, vec![1, 2, 3, 6]),                        // 1, 2, 3, 6
            (12, vec![1, 2, 3, 4, 6, 12]),                // 1, 2, 3, 4, 6, 12
            (28, vec![1, 2, 4, 7, 14, 28]),               // 1, 2, 4, 7, 14, 28
            (36, vec![1, 2, 3, 4, 6, 9, 12, 18, 36]),     // 1, 2, 3, 4, 6, 9, 12, 18, 36
            (49, vec![1, 7, 49]),                         // 1, 7, 49
            (100, vec![1, 2, 4, 5, 10, 20, 25, 50, 100]), // 1, 2, 4, 5, 10, 20, 25, 50, 100
        ];

        for (n, expected) in test_cases {
            let mut result = sieve.enumerate_divisors(n);
            result.sort_unstable();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_prime_factorize() {
        let sieve = LinearSieve::new(1024);

        let test_cases = [
            (1usize, vec![]),                            // 1 = 1
            (2, vec![(2, 1)]),                           // 2 = 2
            (3, vec![(3, 1)]),                           // 3 = 3
            (4, vec![(2, 2)]),                           // 4 = 2^2
            (6, vec![(2, 1), (3, 1)]),                   // 6 = 2 * 3
            (8, vec![(2, 3)]),                           // 8 = 2^3
            (12, vec![(2, 2), (3, 1)]),                  // 12 = 2^2 * 3
            (100, vec![(2, 2), (5, 2)]),                 // 100 = 2^2 * 5^2
            (210, vec![(2, 1), (3, 1), (5, 1), (7, 1)]), // 210 = 2 * 3 * 5 * 7
            (1024, vec![(2, 10)]),                       // 1024 = 2^10
            (243, vec![(3, 5)]),                         // 243 = 3^5
        ];

        for (n, expected) in test_cases {
            assert_eq!(sieve.factorize(n), expected);
        }
    }
}
