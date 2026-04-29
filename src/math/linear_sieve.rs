//! 線形篩(Linear Sieve)
//!
//! `1..=n` の正の整数に対して，最小素因数(lpf)や素数列挙をO(n)で前計算する．
//! `lpf` は `2..=n` を対象とし，素因数分解，約数列挙，素数個数，
//! オイラーのトーシェント関数，メビウス関数も高速に求められる．
//!
//! # 使用例
//! ```
//! use reprol::math::linear_sieve::LinearSieve;
//!
//! let sieve = LinearSieve::new(100);
//! assert_eq!(sieve.limit(), 100);
//!
//! // 素数判定
//! assert!(sieve.is_prime(97));
//! assert!(!sieve.is_prime(100));
//!
//! // 最小素因数
//! assert_eq!(sieve.lpf(100), 2);
//! assert_eq!(sieve.lpf(99), 3);
//!
//! // 素因数分解
//! let factors = sieve.factors(100).collect::<Vec<_>>();
//! assert_eq!(factors, vec![(2, 2), (5, 2)]);
//!
//! // 約数列挙
//! let mut divisors = sieve.divisors(36).collect::<Vec<_>>();
//! divisors.sort_unstable();
//! assert_eq!(divisors, vec![1, 2, 3, 4, 6, 9, 12, 18, 36]);
//!
//! // 数論関数
//! assert_eq!(sieve.prime_pi(30), 10);
//! assert_eq!(sieve.divisors_count(36), 9);
//! assert_eq!(sieve.divisors_sum(36), 91);
//! assert_eq!(sieve.euler_phi(36), 12);
//! assert_eq!(sieve.mobius(30), -1);
//! ```
//!
//! # Notes
//!
//! - `LinearSieve::new(n)` は `n >= 1` を要求する．
//! - 問い合わせメソッドは `1..=n` の正の整数を対象とし，`lpf` のみ `2..=n` を対象とする．

use std::iter::FusedIterator;

use crate::bisect::SliceBoundsExt;

/// 線形篩．
pub struct LinearSieve {
    lpf: Vec<usize>,
    primes: Vec<usize>,
}

impl LinearSieve {
    /// `n`までの線形篩を構築する．
    ///
    /// # Panics
    ///
    /// `n == 0` のときpanicする．
    pub fn new(n: usize) -> Self {
        const UNASSIGNED: usize = usize::MAX;

        assert!(n >= 1, "n must be positive, but got {n}");

        let mut primes = vec![];
        let mut lpf = vec![UNASSIGNED; n + 1];

        for i in 2..=n {
            if lpf[i] == UNASSIGNED {
                lpf[i] = i;
                primes.push(i);
            }

            for &p in &primes {
                if i > n / p || p > lpf[i] {
                    break;
                }
                lpf[p * i] = p;
            }
        }

        Self { lpf, primes }
    }

    /// 構築済みの上限を返す．
    #[inline]
    pub fn limit(&self) -> usize {
        self.lpf.len() - 1
    }

    /// `x`の最小素因数(lpf)を返す．
    ///
    /// # Panics
    ///
    /// `x < 2` または `x > self.limit()` のときpanicする．
    #[inline]
    pub fn lpf(&self, x: usize) -> usize {
        self.assert_lpf_in_range(x);
        self.lpf[x]
    }

    /// 素数のイテレータを返す．
    #[inline]
    pub fn primes(
        &self,
    ) -> impl DoubleEndedIterator<Item = usize> + ExactSizeIterator + FusedIterator {
        self.primes.iter().copied()
    }

    /// `x`が素数かどうかを判定する．
    ///
    /// `x == 1` のとき `false` を返す．
    ///
    /// # Panics
    ///
    /// `x == 0` または `x > self.limit()` のときpanicする．
    #[inline]
    pub fn is_prime(&self, x: usize) -> bool {
        self.assert_query_in_range(x);
        x >= 2 && self.lpf[x] == x
    }

    /// `x`を素因数分解する．
    /// (素数, 指数)の形で列挙するイテレータを返す．
    ///
    /// `x == 1` のとき空のイテレータを返す．
    ///
    /// # Panics
    ///
    /// `x == 0` または `x > self.limit()` のときpanicする．
    #[inline]
    pub fn factors(&self, x: usize) -> impl Iterator<Item = (usize, u32)> {
        self.assert_query_in_range(x);
        FactorsIter { sieve: self, x }
    }

    /// `x`の約数を列挙するイテレータを返す(未ソート)．
    ///
    /// `x == 1` のとき `1` のみを返す．
    ///
    /// # Panics
    ///
    /// `x == 0` または `x > self.limit()` のときpanicする．
    pub fn divisors(&self, x: usize) -> impl Iterator<Item = usize> {
        let mut factors = vec![];
        let mut remaining = 1;
        for (p, exp) in self.factors(x) {
            factors.push(FactorState {
                prime: p,
                current_pow: 1,
                max_pow: p.pow(exp),
            });
            remaining *= exp as usize + 1;
        }
        DivisorsIter {
            factors,
            current: 1,
            remaining,
        }
    }

    /// `x`以下の素数の個数を返す．
    ///
    /// # Panics
    ///
    /// `x == 0` または `x > self.limit()` のときpanicする．
    #[inline]
    pub fn prime_pi(&self, x: usize) -> usize {
        assert!((1..=self.limit()).contains(&x));
        self.primes.upper_bound(&x)
    }

    /// `x`の正の約数の個数を返す．
    ///
    /// # Panics
    ///
    /// `x == 0` または `x > self.limit()` のときpanicする．
    #[inline]
    pub fn divisors_count(&self, x: usize) -> usize {
        self.factors(x).map(|(_, e)| e as usize + 1).product()
    }

    /// `x`の正の約数の総和を返す．
    ///
    /// # Panics
    ///
    /// `x == 0` または `x > self.limit()` のときpanicする．
    /// 約数和が`usize`に収まらないときpanicする．
    #[inline]
    pub fn divisors_sum(&self, x: usize) -> usize {
        self.factors(x).fold(1usize, |acc, (p, exp)| {
            let mut pow = 1usize;
            let mut sum = 1usize;
            for _ in 0..exp {
                pow = pow
                    .checked_mul(p)
                    .expect("divisors_sum overflow while computing prime power");
                sum = sum
                    .checked_add(pow)
                    .expect("divisors_sum overflow while summing geometric series");
            }
            acc.checked_mul(sum)
                .expect("divisors_sum overflow while multiplying factor sums")
        })
    }

    /// オイラーのトーシェント関数 `phi(x)` を返す．
    ///
    /// # Panics
    ///
    /// `x == 0` または `x > self.limit()` のときpanicする．
    #[inline]
    pub fn euler_phi(&self, x: usize) -> usize {
        self.factors(x).fold(x, |acc, (p, _)| acc - acc / p)
    }

    /// メビウス関数 `mu(x)` を返す．
    ///
    /// # Panics
    ///
    /// `x == 0` または `x > self.limit()` のときpanicする．
    #[inline]
    pub fn mobius(&self, x: usize) -> i8 {
        let mut res = 1;
        for (_, exp) in self.factors(x) {
            if exp >= 2 {
                return 0;
            }
            res = -res;
        }
        res
    }

    #[inline]
    fn assert_query_in_range(&self, x: usize) {
        let limit = self.limit();
        assert!(
            (1..=limit).contains(&x),
            "x must satisfy 1 <= x <= {limit}, but got {x}"
        );
    }

    #[inline]
    fn assert_lpf_in_range(&self, x: usize) {
        let limit = self.limit();
        assert!(
            (2..=limit).contains(&x),
            "x must satisfy 2 <= x <= {limit}, but got {x}"
        );
    }
}

/// 素因数分解の結果を列挙するためのイテレータ．
struct FactorsIter<'a> {
    sieve: &'a LinearSieve,
    x: usize,
}

impl<'a> Iterator for FactorsIter<'a> {
    type Item = (usize, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let Self { sieve, x } = self;

        if *x <= 1 {
            return None;
        }

        let p = sieve.lpf[*x];
        let mut exp = 0;

        while *x > 1 && sieve.lpf[*x] == p {
            *x /= p;
            exp += 1;
        }

        Some((p, exp))
    }
}

impl<'a> FusedIterator for FactorsIter<'a> {}

struct FactorState {
    prime: usize,
    current_pow: usize,
    max_pow: usize,
}

struct DivisorsIter {
    factors: Vec<FactorState>,
    current: usize,
    remaining: usize,
}

impl DivisorsIter {
    fn advance(&mut self) {
        for factor in &mut self.factors {
            if factor.current_pow < factor.max_pow {
                factor.current_pow *= factor.prime;
                self.current *= factor.prime;
                return;
            }

            self.current /= factor.current_pow;
            factor.current_pow = 1;
        }

        unreachable!("advance should be called only when another divisor exists");
    }
}

impl Iterator for DivisorsIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let res = self.current;

        self.remaining -= 1;

        if self.remaining > 0 {
            self.advance();
        }

        Some(res)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for DivisorsIter {}

impl FusedIterator for DivisorsIter {}

#[cfg(test)]
mod tests {
    use std::panic::{AssertUnwindSafe, catch_unwind};

    use super::*;

    fn assert_panics(case: &str, f: impl FnOnce()) {
        assert!(
            catch_unwind(AssertUnwindSafe(f)).is_err(),
            "case: {case} should panic"
        );
    }

    #[test]
    fn test_new_zero_panics() {
        assert_panics("new(0)", || {
            let _ = LinearSieve::new(0);
        });
    }

    #[test]
    fn test_limit() {
        assert_eq!(LinearSieve::new(1).limit(), 1, "case: n=1");
        assert_eq!(LinearSieve::new(100).limit(), 100, "case: n=100");
    }

    #[test]
    fn test_lpf() {
        let sieve = LinearSieve::new(100);

        let test_cases = [(2, 2), (3, 3), (4, 2), (6, 2), (49, 7), (99, 3), (100, 2)];

        for (n, expected) in test_cases {
            assert_eq!(sieve.lpf(n), expected, "case: n={n}");
        }
    }

    #[test]
    fn test_lpf_invalid_queries_panic() {
        let sieve = LinearSieve::new(100);

        assert_panics("lpf(0)", || {
            let _ = sieve.lpf(0);
        });
        assert_panics("lpf(1)", || {
            let _ = sieve.lpf(1);
        });
        assert_panics("lpf(101)", || {
            let _ = sieve.lpf(101);
        });
    }

    #[test]
    fn test_is_prime() {
        let sieve = LinearSieve::new(100);

        let test_cases = vec![
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
            assert_eq!(sieve.is_prime(n), ans, "case: n={n}");
        }
    }

    #[test]
    fn test_query_methods_zero_panic() {
        let sieve = LinearSieve::new(100);

        assert_panics("is_prime(0)", || {
            let _ = sieve.is_prime(0);
        });
        assert_panics("factors(0)", || {
            let _ = sieve.factors(0);
        });
        assert_panics("divisors(0)", || {
            let _ = sieve.divisors(0);
        });
        assert_panics("prime_pi(0)", || {
            let _ = sieve.prime_pi(0);
        });
        assert_panics("divisor_count(0)", || {
            let _ = sieve.divisors_count(0);
        });
        assert_panics("divisors_sum(0)", || {
            let _ = sieve.divisors_sum(0);
        });
        assert_panics("euler_phi(0)", || {
            let _ = sieve.euler_phi(0);
        });
        assert_panics("mobius(0)", || {
            let _ = sieve.mobius(0);
        });
    }

    #[test]
    fn test_query_methods_out_of_range_panic() {
        let sieve = LinearSieve::new(100);

        assert_panics("is_prime(101)", || {
            let _ = sieve.is_prime(101);
        });
        assert_panics("factors(101)", || {
            let _ = sieve.factors(101);
        });
        assert_panics("divisors(101)", || {
            let _ = sieve.divisors(101);
        });
        assert_panics("prime_pi(101)", || {
            let _ = sieve.prime_pi(101);
        });
        assert_panics("divisor_count(101)", || {
            let _ = sieve.divisors_count(101);
        });
        assert_panics("divisors_sum(101)", || {
            let _ = sieve.divisors_sum(101);
        });
        assert_panics("euler_phi(101)", || {
            let _ = sieve.euler_phi(101);
        });
        assert_panics("mobius(101)", || {
            let _ = sieve.mobius(101);
        });
    }

    #[test]
    fn test_prime_pi() {
        let sieve = LinearSieve::new(100);

        let test_cases = [(1, 0), (2, 1), (3, 2), (10, 4), (30, 10), (100, 25)];

        for (n, expected) in test_cases {
            assert_eq!(sieve.prime_pi(n), expected, "case: n={n}");
        }
    }

    #[test]
    fn test_divisor_count() {
        let sieve = LinearSieve::new(1024);

        let test_cases = [
            (1, 1),
            (2, 2),
            (12, 6),
            (36, 9),
            (72, 12),
            (210, 16),
            (243, 6),
            (1024, 11),
        ];

        for (n, expected) in test_cases {
            assert_eq!(sieve.divisors_count(n), expected, "case: n={n}");
        }
    }

    #[test]
    fn test_divisors_sum() {
        let sieve = LinearSieve::new(1024);

        let test_cases = [
            (1, 1),
            (2, 3),
            (12, 28),
            (36, 91),
            (72, 195),
            (210, 576),
            (243, 364),
            (1024, 2047),
        ];

        for (n, expected) in test_cases {
            assert_eq!(sieve.divisors_sum(n), expected, "case: n={n}");
        }
    }

    #[test]
    fn test_euler_phi() {
        let sieve = LinearSieve::new(210);

        let test_cases = [
            (1, 1),
            (2, 1),
            (5, 4),
            (12, 4),
            (36, 12),
            (49, 42),
            (210, 48),
        ];

        for (n, expected) in test_cases {
            assert_eq!(sieve.euler_phi(n), expected, "case: n={n}");
        }
    }

    #[test]
    fn test_mobius() {
        let sieve = LinearSieve::new(1024);

        let test_cases = [
            (1, 1),
            (2, -1),
            (6, 1),
            (12, 0),
            (30, -1),
            (36, 0),
            (210, 1),
        ];

        for (n, expected) in test_cases {
            assert_eq!(sieve.mobius(n), expected, "case: n={n}");
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
            let mut result = sieve.divisors(n).collect::<Vec<_>>();
            result.sort_unstable();
            assert_eq!(result, expected, "case: n={n}");
        }
    }

    #[test]
    fn test_divisors_iterator_exhaustion() {
        let sieve = LinearSieve::new(100);

        let mut iter = sieve.divisors(1);
        assert_eq!(iter.next(), Some(1), "case: n=1, first divisor");
        assert_eq!(iter.next(), None, "case: n=1, exhausted once");
        assert_eq!(iter.next(), None, "case: n=1, exhausted twice");

        let mut iter = sieve.divisors(12);
        assert_eq!(iter.by_ref().count(), 6, "case: n=12, divisor count");
        assert_eq!(iter.next(), None, "case: n=12, exhausted once");
        assert_eq!(iter.next(), None, "case: n=12, exhausted twice");
    }

    #[test]
    fn test_divisors_size_hint() {
        let sieve = LinearSieve::new(100);

        let mut iter = sieve.divisors(36);
        let mut remaining = 9;
        assert_eq!(
            iter.size_hint(),
            (remaining, Some(remaining)),
            "case: n=36, initial size_hint"
        );

        while iter.next().is_some() {
            remaining -= 1;
            assert_eq!(
                iter.size_hint(),
                (remaining, Some(remaining)),
                "case: n=36, remaining={remaining}"
            );
        }

        assert_eq!(
            iter.size_hint(),
            (0, Some(0)),
            "case: n=36, size_hint after exhaustion"
        );
    }

    #[test]
    fn test_divisors_count_matches_product_of_exponents() {
        let sieve = LinearSieve::new(1024);

        for n in [1, 2, 12, 36, 72, 210, 243, 1024] {
            let expected = sieve
                .factors(n)
                .fold(1usize, |acc, (_, exp)| acc * (exp as usize + 1));
            let result = sieve.divisors(n).collect::<Vec<_>>();
            assert_eq!(
                result.len(),
                expected,
                "case: n={n}, divisor count should match the product of exponents"
            );
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
