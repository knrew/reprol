pub struct LinearSieve {
    lpf: Vec<usize>,
    primes: Vec<usize>,
}

impl LinearSieve {
    pub fn new(n: usize) -> Self {
        const UNASSIGNED: usize = usize::MAX;

        let mut primes = vec![];
        let mut lpf = vec![UNASSIGNED; n + 1];

        for i in 2..=n {
            if lpf[i] == UNASSIGNED {
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

        Self { lpf, primes }
    }

    pub fn lpf(&self, x: usize) -> usize {
        self.lpf[x]
    }

    pub fn primes(&self) -> impl DoubleEndedIterator<Item = usize> + '_ {
        self.primes.iter().cloned()
    }

    pub fn is_prime(&self, x: usize) -> bool {
        x >= 2 && self.lpf[x] == x
    }

    pub fn factors(&self, mut x: usize) -> impl DoubleEndedIterator<Item = (usize, u32)> + '_ {
        let mut factors = vec![];

        while x > 1 {
            let p = self.lpf[x];
            let mut exp = 0;
            while self.lpf[x] == p {
                x /= p;
                exp += 1;
            }
            factors.push((p, exp));
        }

        factors.into_iter()
    }

    #[inline]
    fn divisors_impl(&self, x: usize) -> Vec<usize> {
        let mut divisors = vec![1];
        for (p, exp) in self.factors(x) {
            for i in 0..divisors.len() {
                let mut v = 1;
                for _ in 0..exp {
                    v = v * p;
                    divisors.push(divisors[i] * v);
                }
            }
        }
        divisors
    }

    pub fn divisors_unsorted(&self, x: usize) -> impl DoubleEndedIterator<Item = usize> + '_ {
        self.divisors_impl(x).into_iter()
    }

    pub fn divisors(&self, x: usize) -> impl DoubleEndedIterator<Item = usize> + '_ {
        let mut d = self.divisors_impl(x);
        d.sort_unstable();
        d.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        let sieve = LinearSieve::new(300000);

        let test_cases = vec![
            (0, false),
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
            (13, true),
            (14, false),
            (15, false),
            (16, false),
            (17, true),
            (18, false),
            (19, true),
            (20, false),
            (21, false),
            (22, false),
            (23, true),
            (24, false),
            (25, false),
            (26, false),
            (27, false),
            (28, false),
            (29, true),
            (30, false),
            (31, true),
            (32, false),
            (33, false),
            (34, false),
            (35, false),
            (36, false),
            (37, true),
            (38, false),
            (39, false),
            (40, false),
            (41, true),
            (42, false),
            (43, true),
            (44, false),
            (45, false),
            (46, false),
            (47, true),
            (48, false),
            (49, false),
            (50, false),
            (51, false),
            (52, false),
            (53, true),
            (54, false),
            (55, false),
            (56, false),
            (57, false),
            (58, false),
            (59, true),
            (60, false),
            (61, true),
            (62, false),
            (63, false),
            (64, false),
            (65, false),
            (66, false),
            (67, true),
            (68, false),
            (69, false),
            (70, false),
            (71, true),
            (72, false),
            (73, true),
            (74, false),
            (75, false),
            (76, false),
            (77, false),
            (78, false),
            (79, true),
            (80, false),
            (81, false),
            (82, false),
            (83, true),
            (84, false),
            (85, false),
            (86, false),
            (87, false),
            (88, false),
            (89, true),
            (90, false),
            (91, false),
            (92, false),
            (93, false),
            (94, false),
            (95, false),
            (96, false),
            (97, true),
            (98, false),
            (99, false),
            (100, false),
        ];

        for (n, ans) in test_cases {
            assert_eq!(sieve.is_prime(n), ans);
        }
    }

    #[test]
    fn test_divisors() {
        let sieve = LinearSieve::new(100);

        let test_cases = [
            (1, vec![1]),
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
            let mut result = sieve.divisors_unsorted(n).collect::<Vec<_>>();
            result.sort_unstable();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_factors() {
        let sieve = LinearSieve::new(1024);

        let test_cases = [
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
            assert_eq!(sieve.factors(n).collect::<Vec<_>>(), expected);
        }
    }
}
