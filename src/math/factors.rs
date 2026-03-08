//! 素因数分解(prime factorization)
//!
//! 正の整数を素因数分解し，素因数と指数のペアを昇順に列挙する．
//!
//! 計算量: O(sqrt n)
//!
//! # Examples
//!
//! ```
//! use reprol::math::factors::Factors;
//!
//! let result = 12u64.factors().collect::<Vec<_>>();
//! assert_eq!(result, vec![(2, 2), (3, 1)]);
//! ```
//!
//! # Notes
//!
//! - 正の整数(`>= 1`)のみを受け付ける．0以下の値を渡すとpanicする．

use std::iter::FusedIterator;

/// 素因数分解トレイト．
pub trait Factors: Sized {
    /// 素因数分解し，素因数と指数のペアを昇順に返すイテレータを生成する．
    ///
    /// `self == 1`の場合，空のイテレータを返す．
    ///
    /// # Panics
    ///
    /// `self <= 0`のときpanicする．
    fn factors(self) -> impl FusedIterator<Item = (Self, u32)>;
}

macro_rules! impl_factors_inner {
    ($factors_iter_name: ident, $ty: ty) => {
        struct $factors_iter_name {
            /// 未処理の残り(割り進めた商)
            n: $ty,

            /// 現在の素因数候補
            p: $ty,

            /// 2と3の倍数をスキップするための増分(2と4を交互に使用)
            k: $ty,
        }

        impl $factors_iter_name {
            /// 現在の`p`で`n`を割れるだけ割り，指数を返す．
            fn count_and_divide(&mut self) -> u32 {
                let mut exp = 0;
                while self.n % self.p == 0 {
                    self.n /= self.p;
                    exp += 1;
                }
                exp
            }

            /// 次の素因数候補に`p`を進める．
            fn advance(&mut self) {
                match self.p {
                    2 => {
                        self.p = 3;
                    }
                    3 => {
                        self.p = 5;
                        self.k = 2;
                    }
                    _ => {
                        self.p += self.k;
                        self.k = 6 - self.k; // 2 <-> 4
                    }
                }
            }
        }

        impl Iterator for $factors_iter_name {
            type Item = ($ty, u32);

            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    if self.n <= 1 {
                        return None;
                    }

                    if self.p > self.n / self.p {
                        let prime = self.n;
                        self.n = 1;
                        return Some((prime, 1));
                    }

                    let exp = self.count_and_divide();
                    let prime = self.p;
                    self.advance();
                    if exp > 0 {
                        return Some((prime, exp));
                    }
                }
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                if self.n <= 1 {
                    return (0, Some(0));
                }

                if self.p > self.n / self.p {
                    return (1, Some(1));
                }

                // floor(log_p(n)) を整数除算で近似
                let mut m = self.n;
                let mut upper = 0;
                while m >= self.p {
                    m /= self.p;
                    upper += 1;
                }

                (1, Some(upper.max(1)))
            }
        }

        impl FusedIterator for $factors_iter_name {}

        impl Factors for $ty {
            fn factors(self) -> impl FusedIterator<Item = (Self, u32)> {
                assert!(self > 0);
                $factors_iter_name {
                    n: self,
                    p: 2,
                    k: 0,
                }
            }
        }
    };
}

macro_rules! impl_factors {
    ($([$factors_iter_name: ident, $ty: ty]),* $(,)?) => {
        $( impl_factors_inner!($factors_iter_name, $ty); )*
    };
}

impl_factors! {
    [I8FactorsIter, i8],
    [I16FactorsIter, i16],
    [I32FactorsIter, i32],
    [I64FactorsIter, i64],
    [I128FactorsIter, i128],
    [IsizeFactorsIter, isize],
    [U8FactorsIter, u8],
    [U16FactorsIter, u16],
    [U32FactorsIter, u32],
    [U64FactorsIter, u64],
    [U128FactorsIter, u128],
    [UsizeFactorsIter, usize],
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    use crate::{math::is_prime::IsPrime, utils::test_utils::random::get_test_rng};

    // ========== ヘルパ関数 ==========

    /// 素因数分解の結果が正しいことを検証する
    /// 1. 各素因数が素数であること
    /// 2. 積が元の値に一致すること(n=1の場合は空)
    /// 3. 各素因数が昇順に並ぶこと
    fn assert_valid_factorization(n: u64, factors: &[(u64, u32)]) {
        if n == 1 {
            assert!(factors.is_empty(), "n=1 should produce empty result");
            return;
        }
        for &(p, e) in factors {
            assert!(p.is_prime(), "factor {p} of {n} is not prime");
            assert!(e > 0, "exponent of {p} in {n} should be positive");
        }
        let product: u64 = factors.iter().map(|&(p, e)| p.pow(e)).product();
        assert_eq!(product, n, "product reconstruction failed: n={n}");
        assert!(
            factors.windows(2).all(|w| w[0].0 < w[1].0),
            "factors not in strictly ascending order: n={n}, factors={factors:?}"
        );
    }

    // ========== スモークテスト ==========

    #[test]
    fn test_smoke_all_types() {
        macro_rules! test {
            ($ty: ty) => {
                let x: $ty = 12;
                let expected: Vec<($ty, u32)> = vec![(2, 2), (3, 1)];
                let result: Vec<($ty, u32)> = x.factors().collect();
                assert_eq!(result, expected, stringify!($ty));
            };
        }

        test!(i8);
        test!(i16);
        test!(i32);
        test!(i64);
        test!(i128);
        test!(isize);
        test!(u8);
        test!(u16);
        test!(u32);
        test!(u64);
        test!(u128);
        test!(usize);
    }

    // ========== エッジケース ==========

    #[test]
    fn test_factors_one() {
        let result: Vec<(u64, u32)> = 1u64.factors().collect();
        assert!(result.is_empty(), "n=1 should produce empty result");
    }

    #[test]
    fn test_factors_two() {
        let result: Vec<(u64, u32)> = 2u64.factors().collect();
        assert_eq!(result, vec![(2, 1)], "n=2");
    }

    #[test]
    fn test_factors_prime() {
        for p in [2u64, 3, 5, 7, 127, 1031] {
            let result: Vec<(u64, u32)> = p.factors().collect();
            assert_eq!(result, vec![(p, 1)], "n={p}");
        }
    }

    #[test]
    fn test_factors_prime_power() {
        let cases: Vec<(u64, Vec<(u64, u32)>)> = vec![
            (4, vec![(2, 2)]),
            (8, vec![(2, 3)]),
            (27, vec![(3, 3)]),
            (32, vec![(2, 5)]),
            (125, vec![(5, 3)]),
            (128, vec![(2, 7)]),
        ];
        for (n, expected) in cases {
            let result: Vec<(u64, u32)> = n.factors().collect();
            assert_eq!(result, expected, "n={n}");
        }
    }

    #[test]
    fn test_factors_boundary_values() {
        // i32::MAX = 2147483647
        let result: Vec<(i32, u32)> = i32::MAX.factors().collect();
        assert_eq!(result, vec![(2147483647, 1)], "i32::MAX");

        // u32::MAX = 4294967295 = 3 * 5 * 17 * 257 * 65537
        let result: Vec<(u32, u32)> = u32::MAX.factors().collect();
        assert_eq!(
            result,
            vec![(3, 1), (5, 1), (17, 1), (257, 1), (65537, 1)],
            "u32::MAX"
        );

        // i64::MAX = 9223372036854775807 = 7^2 * 73 * 127 * 337 * 92737 * 649657
        let result: Vec<(i64, u32)> = i64::MAX.factors().collect();
        assert_eq!(
            result,
            vec![(7, 2), (73, 1), (127, 1), (337, 1), (92737, 1), (649657, 1)],
            "i64::MAX"
        );

        // u64::MAX = 18446744073709551615 = 3 * 5 * 17 * 257 * 641 * 65537 * 6700417
        let result: Vec<(u64, u32)> = u64::MAX.factors().collect();
        assert_eq!(
            result,
            vec![
                (3, 1),
                (5, 1),
                (17, 1),
                (257, 1),
                (641, 1),
                (65537, 1),
                (6700417, 1)
            ],
            "u64::MAX"
        );
    }

    // ========== 条件網羅 ==========

    #[test]
    #[should_panic]
    fn test_factors_panic_zero_unsigned() {
        let _ = 0u64.factors();
    }

    #[test]
    #[should_panic]
    fn test_factors_panic_zero_signed() {
        let _ = 0i64.factors();
    }

    #[test]
    #[should_panic]
    fn test_factors_panic_negative() {
        let _ = (-1i64).factors();
    }

    #[test]
    fn test_factors_branch_only_small_primes() {
        // p=2,3,5のみで割り切れるケース
        // 30 = 2 * 3 * 5
        let result: Vec<(u64, u32)> = 30u64.factors().collect();
        assert_eq!(result, vec![(2, 1), (3, 1), (5, 1)], "n=30");

        // 60 = 2^2 * 3 * 5
        let result: Vec<(u64, u32)> = 60u64.factors().collect();
        assert_eq!(result, vec![(2, 2), (3, 1), (5, 1)], "n=60");
    }

    #[test]
    fn test_factors_branch_wheel_advance() {
        // 2と3の倍数スキップ(advance)を通過する
        // 7007 = 7 * 7 * 11 * 13 → p=7,11,13でスキップパターンを使用
        let result: Vec<(u64, u32)> = 7007u64.factors().collect();
        assert_eq!(result, vec![(7, 2), (11, 1), (13, 1)], "n=7007");

        // 2431 = 11 * 13 * 17
        let result: Vec<(u64, u32)> = 2431u64.factors().collect();
        assert_eq!(result, vec![(11, 1), (13, 1), (17, 1)], "n=2431");
    }

    #[test]
    fn test_factors_branch_large_remainder() {
        // p > n/p分岐: 小さい素因数で割った後に大きな素数が残る
        // 2 * 99991 = 199982
        let result: Vec<(u64, u32)> = 199982u64.factors().collect();
        assert_eq!(result, vec![(2, 1), (99991, 1)], "n=199982");

        // 3 * 10007 = 30021
        let result: Vec<(u64, u32)> = 30021u64.factors().collect();
        assert_eq!(result, vec![(3, 1), (10007, 1)], "n=30021");
    }

    #[test]
    fn test_factors_branch_exp_zero_skip() {
        // exp==0でスキップされる場合: n=49=7^2
        // p=2,3,5で割れず(exp=0でスキップ)，p=7で割れる
        let result: Vec<(u64, u32)> = 49u64.factors().collect();
        assert_eq!(result, vec![(7, 2)], "n=49");
    }

    // ========== 小さい入力での全探索 ==========

    #[test]
    fn test_factors_exhaustive_u8() {
        for n in 1u8..=255 {
            let factors: Vec<(u8, u32)> = n.factors().collect();
            let factors_u64: Vec<(u64, u32)> =
                factors.iter().map(|&(p, e)| (p as u64, e)).collect();
            assert_valid_factorization(n as u64, &factors_u64);
        }
    }

    #[test]
    fn test_factors_exhaustive_small_u32() {
        for n in 1u32..=10_000 {
            let factors: Vec<(u32, u32)> = n.factors().collect();
            let factors_u64: Vec<(u64, u32)> =
                factors.iter().map(|&(p, e)| (p as u64, e)).collect();
            assert_valid_factorization(n as u64, &factors_u64);
        }
    }

    // ========== FusedIteratorの挙動 ==========

    #[test]
    fn test_fused_iterator_returns_none_after_exhaustion() {
        // n=1: 最初からNone
        let mut iter = 1u64.factors();
        assert_eq!(iter.next(), None, "n=1: first call");
        assert_eq!(iter.next(), None, "n=1: second call");
        assert_eq!(iter.next(), None, "n=1: third call");

        // 素数: 1要素の後にNone
        let mut iter = 7u64.factors();
        assert_eq!(iter.next(), Some((7, 1)), "n=7: first element");
        assert_eq!(iter.next(), None, "n=7: after exhaustion (1st)");
        assert_eq!(iter.next(), None, "n=7: after exhaustion (2nd)");
        assert_eq!(iter.next(), None, "n=7: after exhaustion (3rd)");

        // 合成数: 複数要素の後にNone
        let mut iter = 12u64.factors();
        assert_eq!(iter.next(), Some((2, 2)), "n=12: first element");
        assert_eq!(iter.next(), Some((3, 1)), "n=12: second element");
        assert_eq!(iter.next(), None, "n=12: after exhaustion (1st)");
        assert_eq!(iter.next(), None, "n=12: after exhaustion (2nd)");
        assert_eq!(iter.next(), None, "n=12: after exhaustion (3rd)");
    }

    // ========== ランダムテスト ==========

    #[test]
    fn test_factors_random_validity() {
        let mut rng = get_test_rng();
        for _ in 0..500 {
            let n = rng.random_range(1u64..=1_000_000_000_000);
            let factors: Vec<(u64, u32)> = n.factors().collect();
            assert_valid_factorization(n, &factors);
        }
    }
}
