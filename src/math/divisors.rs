//! 約数列挙
//!
//! 正の整数の約数を昇順に列挙する．
//!
//! 計算量: O(sqrt n)
//!
//! # Examples
//!
//! ```
//! use reprol::math::divisors::Divisors;
//!
//! let divs = 12u64.divisors().collect::<Vec<_>>();
//! assert_eq!(divs, vec![1, 2, 3, 4, 6, 12]);
//! ```
//!
//! # Notes
//!
//! - 正の整数(`>= 1`)のみを受け付ける．0以下の値を渡すとpanicする．

use std::iter::FusedIterator;

/// 約数列挙トレイト．
pub trait Divisors: Sized {
    /// 約数を昇順に列挙するイテレータを返す．
    ///
    /// # Panics
    ///
    /// `self <= 0`のときpanicする．
    fn divisors(self) -> impl FusedIterator<Item = Self>;
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Phase {
    Ascending,
    Descending,
    Done,
}

macro_rules! impl_divisors_inner {
    ($divisors_iter_name: ident, $ty: ty) => {
        struct $divisors_iter_name {
            target: $ty,
            cursor: $ty,
            phase: Phase,
        }

        impl $divisors_iter_name {
            fn next_ascending(&mut self) -> Option<$ty> {
                while self.cursor <= self.target / self.cursor {
                    let d = self.cursor;
                    self.cursor += 1;
                    if self.target % d == 0 {
                        return Some(d);
                    }
                }

                self.cursor -= 1;
                self.phase = Phase::Descending;
                self.next_descending()
            }

            fn next_descending(&mut self) -> Option<$ty> {
                while self.cursor >= 1 {
                    let d = self.cursor;
                    self.cursor -= 1;

                    if self.target % d == 0 {
                        let counterpart = self.target / d;
                        if counterpart != d {
                            return Some(counterpart);
                        }
                    }
                }

                self.phase = Phase::Done;
                None
            }
        }

        impl Iterator for $divisors_iter_name {
            type Item = $ty;

            fn next(&mut self) -> Option<Self::Item> {
                match self.phase {
                    Phase::Ascending => self.next_ascending(),
                    Phase::Descending => self.next_descending(),
                    Phase::Done => None,
                }
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                match self.phase {
                    Phase::Done => (0, Some(0)),
                    Phase::Descending => (0, usize::try_from(self.cursor).ok()),
                    Phase::Ascending => {
                        let remaining = self.target / self.cursor;
                        let upper = usize::try_from(remaining).ok().map(|r| r.saturating_mul(2));
                        (0, upper)
                    }
                }
            }
        }

        impl FusedIterator for $divisors_iter_name {}

        impl Divisors for $ty {
            fn divisors(self) -> impl FusedIterator<Item = Self> {
                assert!(self > 0);
                $divisors_iter_name {
                    target: self,
                    cursor: 1,
                    phase: Phase::Ascending,
                }
            }
        }
    };
}

macro_rules! impl_divisors {
    ($([$divisors_iter_name: ident, $ty: ty]),* $(,)?) => {
        $( impl_divisors_inner!($divisors_iter_name, $ty); )*
    };
}

impl_divisors! {
    [I8DivisorsIter, i8],
    [I16DivisorsIter, i16],
    [I32DivisorsIter, i32],
    [I64DivisorsIter, i64],
    [I128DivisorsIter, i128],
    [IsizeDivisorsIter, isize],
    [U8DivisorsIter, u8],
    [U16DivisorsIter, u16],
    [U32DivisorsIter, u32],
    [U64DivisorsIter, u64],
    [U128DivisorsIter, u128],
    [UsizeDivisorsIter, usize],
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    use crate::utils::test_utils::random::get_test_rng;

    // ========== ヘルパ関数 ==========

    fn assert_divisors_matches_naive(n: u64, divisors: &[u64]) {
        assert!(n > 0, "case: n={n}");
        assert!(
            !divisors.is_empty(),
            "case: n={n}, divisors should not be empty"
        );
        assert_eq!(divisors[0], 1, "case: n={n}, first divisor should be 1");
        assert_eq!(
            divisors.last().copied(),
            Some(n),
            "case: n={n}, last divisor should be n"
        );
        assert!(
            divisors.windows(2).all(|w| w[0] < w[1]),
            "case: n={n}, divisors should be strictly ascending: {divisors:?}"
        );

        for &d in divisors {
            assert_eq!(n % d, 0, "case: n={n}, d={d} should divide n");
        }

        let mut expected = vec![];
        for d in (1..).take_while(|&d| d <= n / d).filter(|d| n % d == 0) {
            expected.push(d);
            let counterpart = n / d;
            if counterpart != d {
                expected.push(counterpart);
            }
        }
        expected.sort_unstable();

        assert_eq!(divisors, expected, "case: n={n}");
    }

    // ========== スモークテスト ==========

    #[test]
    fn test_divisors_smoke_all_types() {
        macro_rules! test {
            ($ty: ty) => {
                let x: $ty = 12;
                let expected: Vec<$ty> = vec![1, 2, 3, 4, 6, 12];
                let result: Vec<$ty> = x.divisors().collect();
                assert_eq!(result, expected, "case: type={}", stringify!($ty));
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
    fn test_divisors_one() {
        let result: Vec<u64> = 1u64.divisors().collect();
        assert_eq!(result, vec![1], "case: n=1");
    }

    #[test]
    fn test_divisors_prime() {
        for n in [2u64, 3, 5, 7, 127, 1031] {
            let result: Vec<u64> = n.divisors().collect();
            assert_eq!(result, vec![1, n], "case: n={n}");
        }
    }

    #[test]
    fn test_divisors_perfect_square() {
        let cases = [
            (4u64, vec![1, 2, 4]),
            (36, vec![1, 2, 3, 4, 6, 9, 12, 18, 36]),
            (49, vec![1, 7, 49]),
        ];

        for (n, expected) in cases {
            let result: Vec<u64> = n.divisors().collect();
            assert_eq!(result, expected, "case: n={n}");
        }
    }

    #[test]
    fn test_divisors_composite_cases() {
        let cases = [
            (12u64, vec![1, 2, 3, 4, 6, 12]),
            (28, vec![1, 2, 4, 7, 14, 28]),
            (60, vec![1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60]),
        ];

        for (n, expected) in cases {
            let result: Vec<u64> = n.divisors().collect();
            assert_eq!(result, expected, "case: n={n}");
        }
    }

    #[test]
    fn test_divisors_boundary_values() {
        let cases_i8 = [i8::MAX];
        for n in cases_i8 {
            let result: Vec<u64> = n.divisors().map(|d| u64::try_from(d).unwrap()).collect();
            assert_divisors_matches_naive(u64::try_from(n).unwrap(), &result);
        }

        let cases_u8 = [u8::MAX];
        for n in cases_u8 {
            let result: Vec<u64> = n.divisors().map(u64::from).collect();
            assert_divisors_matches_naive(u64::from(n), &result);
        }

        let cases_i16 = [i16::MAX];
        for n in cases_i16 {
            let result: Vec<u64> = n.divisors().map(|d| u64::try_from(d).unwrap()).collect();
            assert_divisors_matches_naive(u64::try_from(n).unwrap(), &result);
        }

        let cases_u16 = [u16::MAX];
        for n in cases_u16 {
            let result: Vec<u64> = n.divisors().map(u64::from).collect();
            assert_divisors_matches_naive(u64::from(n), &result);
        }

        let cases_i32 = [i32::MAX];
        for n in cases_i32 {
            let result: Vec<u64> = n.divisors().map(|d| u64::try_from(d).unwrap()).collect();
            assert_divisors_matches_naive(u64::try_from(n).unwrap(), &result);
        }

        let cases_u32 = [u32::MAX];
        for n in cases_u32 {
            let result: Vec<u64> = n.divisors().map(u64::from).collect();
            assert_divisors_matches_naive(u64::from(n), &result);
        }
    }

    // ========== 条件網羅 ==========

    #[test]
    #[should_panic]
    fn test_divisors_panic_zero_unsigned() {
        let _ = 0u64.divisors();
    }

    #[test]
    #[should_panic]
    fn test_divisors_panic_zero_signed() {
        let _ = 0i64.divisors();
    }

    #[test]
    #[should_panic]
    fn test_divisors_panic_negative() {
        let _ = i64::MIN.divisors();
    }

    // ========== 小さい入力での全探索 ==========

    #[test]
    fn test_divisors_exhaustive_u8() {
        for n in 1u8..=u8::MAX {
            let result: Vec<u64> = n.divisors().map(u64::from).collect();
            assert_divisors_matches_naive(u64::from(n), &result);
        }
    }

    #[test]
    fn test_divisors_exhaustive_small_u32() {
        for n in 1u32..=10_000 {
            let result: Vec<u64> = n.divisors().map(u64::from).collect();
            assert_divisors_matches_naive(u64::from(n), &result);
        }
    }

    // ========== FusedIteratorの挙動 ==========

    #[test]
    fn test_divisors_fused_iterator_returns_none_after_exhaustion() {
        let mut iter = 1u64.divisors();
        assert_eq!(iter.next(), Some(1), "case: n=1, first element");
        assert_eq!(iter.next(), None, "case: n=1, after exhaustion (1st)");
        assert_eq!(iter.next(), None, "case: n=1, after exhaustion (2nd)");
        assert_eq!(iter.next(), None, "case: n=1, after exhaustion (3rd)");

        let mut iter = 7u64.divisors();
        assert_eq!(iter.next(), Some(1), "case: n=7, first element");
        assert_eq!(iter.next(), Some(7), "case: n=7, second element");
        assert_eq!(iter.next(), None, "case: n=7, after exhaustion (1st)");
        assert_eq!(iter.next(), None, "case: n=7, after exhaustion (2nd)");
        assert_eq!(iter.next(), None, "case: n=7, after exhaustion (3rd)");

        let mut iter = 12u64.divisors();
        assert_eq!(iter.next(), Some(1), "case: n=12, first element");
        assert_eq!(iter.next(), Some(2), "case: n=12, second element");
        assert_eq!(iter.next(), Some(3), "case: n=12, third element");
        assert_eq!(iter.next(), Some(4), "case: n=12, fourth element");
        assert_eq!(iter.next(), Some(6), "case: n=12, fifth element");
        assert_eq!(iter.next(), Some(12), "case: n=12, sixth element");
        assert_eq!(iter.next(), None, "case: n=12, after exhaustion (1st)");
        assert_eq!(iter.next(), None, "case: n=12, after exhaustion (2nd)");
        assert_eq!(iter.next(), None, "case: n=12, after exhaustion (3rd)");
    }

    // ========== ランダムテスト ==========

    #[test]
    fn test_divisors_random_against_naive() {
        let mut rng = get_test_rng();
        for _ in 0..500 {
            let n = rng.random_range(1u64..=1_000_000);
            let result: Vec<u64> = n.divisors().collect();
            assert_divisors_matches_naive(n, &result);
        }
    }
}
