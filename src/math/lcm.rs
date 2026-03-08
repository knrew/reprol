//! 最小公倍数(LCM)
//!
//! 2つの整数の最小公倍数を計算するトレイト．
//! 符号付き整数型では絶対値ベースのLCMを返す．
//! ただし`Self::MIN`を含む入力は前提条件違反として扱う．
//!
//! # Examples
//!
//! ```
//! use reprol::math::lcm::Lcm;
//!
//! assert_eq!(6u64.lcm(8), 24);
//! assert_eq!((-12i64).lcm(8), 24);
//! assert_eq!(0u64.lcm(5), 0);
//! ```

use crate::math::gcd::Gcd;

/// 最小公倍数(LCM)を計算するトレイト．
///
/// `lcm(0, n) = lcm(n, 0) = 0` と定義する．
/// 符号付き整数型では結果は常に非負値を返す．
/// ただし符号付き整数型において，`self == Self::MIN` または `rhs == Self::MIN` は前提条件違反とする．
pub trait Lcm: Gcd + Sized {
    /// `self` と `rhs` の最小公倍数を返す．
    ///
    /// # Panics
    ///
    /// 符号付き整数型において，`self == Self::MIN` または `rhs == Self::MIN` の場合にpanicする．
    fn lcm(self, rhs: Self) -> Self;

    /// `self` と `rhs` の最小公倍数を返す．
    ///
    /// オーバーフローまたは前提条件違反(`self == Self::MIN` または `rhs == Self::MIN`)の場合は`None`を返す．
    fn checked_lcm(self, rhs: Self) -> Option<Self>;
}

macro_rules! impl_lcm_unsigned {
    ($ty: ty) => {
        impl Lcm for $ty {
            fn lcm(self, rhs: Self) -> Self {
                if self == 0 || rhs == 0 {
                    0
                } else {
                    self / self.gcd(rhs) * rhs
                }
            }

            fn checked_lcm(self, rhs: Self) -> Option<Self> {
                if self == 0 || rhs == 0 {
                    Some(0)
                } else {
                    (self / self.gcd(rhs)).checked_mul(rhs)
                }
            }
        }
    };
}

macro_rules! impl_lcm_signed {
    ($ty: ty) => {
        impl Lcm for $ty {
            fn lcm(self, rhs: Self) -> Self {
                assert_ne!(self, <$ty>::MIN);
                assert_ne!(rhs, <$ty>::MIN);

                if self == 0 || rhs == 0 {
                    0
                } else {
                    let m = self.abs();
                    let n = rhs.abs();
                    m / m.gcd(n) * n
                }
            }

            fn checked_lcm(self, rhs: Self) -> Option<Self> {
                if self == <$ty>::MIN || rhs == <$ty>::MIN {
                    None
                } else if self == 0 || rhs == 0 {
                    Some(0)
                } else {
                    let m = self.abs();
                    let n = rhs.abs();
                    (m / m.gcd(n)).checked_mul(n)
                }
            }
        }
    };
}

macro_rules! impl_lcm {
    (unsigned: [$($u:ty),* $(,)?], signed: [$($s:ty),* $(,)?]$(,)?) => {
        $( impl_lcm_unsigned!($u); )*
        $( impl_lcm_signed!($s); )*
    };
}

impl_lcm! {
    unsigned: [u8, u16, u32, u64, u128, usize],
    signed:   [i8, i16, i32, i64, i128, isize],
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    use crate::{math::gcd::Gcd, utils::test_utils::random::get_test_rng};

    // ========== ヘルパ関数 ==========

    fn naive_gcd(a: u128, b: u128) -> u128 {
        if a == 0 {
            return b;
        }
        if b == 0 {
            return a;
        }
        for d in (1..=a.min(b)).rev() {
            if a % d == 0 && b % d == 0 {
                return d;
            }
        }
        unreachable!()
    }

    fn naive_lcm(a: u128, b: u128) -> u128 {
        if a == 0 || b == 0 {
            0
        } else {
            a / naive_gcd(a, b) * b
        }
    }

    fn naive_lcm_signed(a: i128, b: i128) -> i128 {
        naive_lcm(a.unsigned_abs(), b.unsigned_abs()) as i128
    }

    // ========== スモークテスト ==========

    #[test]
    fn test_lcm_smoke() {
        assert_eq!(6u8.lcm(8), 24);
        assert_eq!(6u16.lcm(8), 24);
        assert_eq!(6u32.lcm(8), 24);
        assert_eq!(6u64.lcm(8), 24);
        assert_eq!(6u128.lcm(8), 24);
        assert_eq!(6usize.lcm(8), 24);
        assert_eq!((-6i8).lcm(8), 24);
        assert_eq!((-6i16).lcm(8), 24);
        assert_eq!((-6i32).lcm(8), 24);
        assert_eq!((-6i64).lcm(8), 24);
        assert_eq!((-6i128).lcm(8), 24);
        assert_eq!((-6isize).lcm(8), 24);
    }

    #[test]
    fn test_checked_lcm_smoke() {
        assert_eq!(6u8.checked_lcm(8), Some(24));
        assert_eq!(6u16.checked_lcm(8), Some(24));
        assert_eq!(6u32.checked_lcm(8), Some(24));
        assert_eq!(6u64.checked_lcm(8), Some(24));
        assert_eq!(6u128.checked_lcm(8), Some(24));
        assert_eq!(6usize.checked_lcm(8), Some(24));
        assert_eq!((-6i8).checked_lcm(8), Some(24));
        assert_eq!((-6i16).checked_lcm(8), Some(24));
        assert_eq!((-6i32).checked_lcm(8), Some(24));
        assert_eq!((-6i64).checked_lcm(8), Some(24));
        assert_eq!((-6i128).checked_lcm(8), Some(24));
        assert_eq!((-6isize).checked_lcm(8), Some(24));
    }

    // ========== エッジケース ==========

    #[test]
    fn test_lcm_zero_zero() {
        assert_eq!(0u64.lcm(0), 0, "u64: lcm(0, 0)");
        assert_eq!(0i64.lcm(0), 0, "i64: lcm(0, 0)");
    }

    #[test]
    fn test_lcm_with_zero() {
        for n in [1u64, 7, 1030, u64::MAX] {
            assert_eq!(n.lcm(0), 0, "lcm({n}, 0)");
            assert_eq!(0u64.lcm(n), 0, "lcm(0, {n})");
        }
        for n in [1i64, -1, 1030, -1030, i64::MAX, i64::MIN + 1] {
            assert_eq!(n.lcm(0), 0, "lcm({n}, 0)");
            assert_eq!(0i64.lcm(n), 0, "lcm(0, {n})");
        }
    }

    #[test]
    fn test_lcm_with_one() {
        for n in [0u64, 1, 1030, u64::MAX] {
            assert_eq!(1u64.lcm(n), n, "lcm(1, {n})");
            assert_eq!(n.lcm(1), n, "lcm({n}, 1)");
        }
        for n in [0i64, 1, -1, 1030, -1030, i64::MAX, i64::MIN + 1] {
            let expected = n.abs();
            assert_eq!(1i64.lcm(n), expected, "lcm(1, {n})");
            assert_eq!(n.lcm(1), expected, "lcm({n}, 1)");
            assert_eq!((-1i64).lcm(n), expected, "lcm(-1, {n})");
            assert_eq!(n.lcm(-1), expected, "lcm({n}, -1)");
        }
    }

    #[test]
    fn test_lcm_same_values() {
        for n in [0u64, 1, 1030, u64::MAX] {
            assert_eq!(n.lcm(n), n, "lcm({n}, {n})");
        }
        for n in [0i64, 1, -1, 1030, -1030, i64::MAX, i64::MIN + 1] {
            assert_eq!(n.lcm(n), n.abs(), "lcm({n}, {n})");
        }
    }

    #[test]
    fn test_lcm_unsigned_boundary() {
        // u8::MAX = 255 = 3 * 5 * 17
        assert_eq!(u8::MAX.lcm(1), u8::MAX, "lcm(255, 1)");
        assert_eq!(u8::MAX.lcm(u8::MAX), u8::MAX, "lcm(255, 255)");
        assert_eq!(u8::MAX.lcm(5), u8::MAX, "lcm(255, 5)");
        assert_eq!(u8::MAX.lcm(3), u8::MAX, "lcm(255, 3)");

        // u64::MAX
        assert_eq!(u64::MAX.lcm(1), u64::MAX, "lcm(u64::MAX, 1)");
        assert_eq!(u64::MAX.lcm(u64::MAX), u64::MAX, "lcm(u64::MAX, u64::MAX)");
        // u64::MAX = 3 * 5 * 17 * 257 * 641 * 65537 * 6700417
        assert_eq!(u64::MAX.lcm(3), u64::MAX, "lcm(u64::MAX, 3)");

        // u128::MAX
        assert_eq!(u128::MAX.lcm(1), u128::MAX, "lcm(u128::MAX, 1)");
        assert_eq!(
            u128::MAX.lcm(u128::MAX),
            u128::MAX,
            "lcm(u128::MAX, u128::MAX)"
        );
        assert_eq!(u128::MAX.lcm(3), u128::MAX, "lcm(u128::MAX, 3)");
    }

    #[test]
    fn test_lcm_signed_boundary() {
        // i64::MAX
        assert_eq!(i64::MAX.lcm(1), i64::MAX, "lcm(i64::MAX, 1)");
        assert_eq!(i64::MAX.lcm(i64::MAX), i64::MAX, "lcm(i64::MAX, i64::MAX)");
        assert_eq!(i64::MAX.lcm(7), i64::MAX, "lcm(i64::MAX, 7)");

        // i64::MIN + 1
        let min1 = i64::MIN + 1;
        assert_eq!(min1.lcm(1), min1.abs(), "lcm(MIN+1, 1)");
        assert_eq!(min1.lcm(-1), min1.abs(), "lcm(MIN+1, -1)");
        assert_eq!(min1.lcm(min1), min1.abs(), "lcm(MIN+1, MIN+1)");
        assert_eq!(min1.lcm(7), min1.abs(), "lcm(MIN+1, 7)");
    }

    // ========== 代数的性質 ==========

    #[test]
    fn test_lcm_commutativity() {
        let unsigned_cases: &[(u64, u64)] = &[
            (0, 0),
            (0, 1),
            (1, 1),
            (6, 8),
            (12, 18),
            (100, 75),
            (u64::MAX, 1),
            (u64::MAX, u64::MAX),
        ];
        for &(a, b) in unsigned_cases {
            assert_eq!(a.lcm(b), b.lcm(a), "lcm({a}, {b})");
        }
        let signed_cases: &[(i64, i64)] = &[
            (0, 0),
            (1, -1),
            (-6, 8),
            (-12, -18),
            (100, -75),
            (i64::MAX, 1),
            (i64::MIN + 1, 7),
        ];
        for &(a, b) in signed_cases {
            assert_eq!(a.lcm(b), b.lcm(a), "lcm({a}, {b})");
        }
    }

    #[test]
    fn test_lcm_associativity() {
        let cases: &[(u64, u64, u64)] = &[
            (12, 18, 24),
            (0, 0, 0),
            (1, 1, 1),
            (100, 75, 50),
            (36, 48, 60),
            (7, 13, 29),
        ];
        for &(a, b, c) in cases {
            assert_eq!(
                a.lcm(b).lcm(c),
                a.lcm(b.lcm(c)),
                "associativity: ({a}, {b}, {c})"
            );
        }
    }

    #[test]
    fn test_lcm_signed_nonnegative() {
        let cases: &[(i64, i64)] = &[
            (6, 8),
            (6, -8),
            (-6, 8),
            (-6, -8),
            (0, 5),
            (0, -5),
            (1, -1),
            (i64::MAX, -1),
            (i64::MIN + 1, 1),
            (i64::MIN + 1, i64::MAX),
            (-1030, -1030),
            (100, -75),
        ];
        for &(a, b) in cases {
            assert!(a.lcm(b) >= 0, "lcm({a}, {b}) should be >= 0");
        }
    }

    #[test]
    fn test_lcm_signed_sign_independence() {
        let cases: &[(i64, i64)] = &[
            (12, 18),
            (48, 36),
            (100, 75),
            (7, 13),
            (1, 999),
            (1030, 1030),
        ];
        for &(a, b) in cases {
            let expected = a.lcm(b);
            assert_eq!(a.lcm(-b), expected, "lcm({a}, {})", -b);
            assert_eq!((-a).lcm(b), expected, "lcm({}, {b})", -a);
            assert_eq!((-a).lcm(-b), expected, "lcm({}, {})", -a, -b);
        }
    }

    #[test]
    fn test_lcm_gcd_product_identity() {
        // lcm(a, b) * gcd(a, b) = a * b (unsigned, 小さい値でオーバーフローを避ける)
        let cases: &[(u64, u64)] = &[
            (1, 1),
            (6, 8),
            (12, 18),
            (100, 75),
            (36, 48),
            (7, 13),
            (1030, 15),
            (255, 85),
            (1024, 768),
            (360, 240),
        ];
        for &(a, b) in cases {
            let l = a.lcm(b) as u128;
            let g = a.gcd(b) as u128;
            let product = a as u128 * b as u128;
            assert_eq!(l * g, product, "lcm({a}, {b}) * gcd({a}, {b}) = {a} * {b}");
        }
    }

    #[test]
    fn test_lcm_divisibility() {
        let cases: &[(u64, u64)] = &[
            (1, 1),
            (6, 8),
            (12, 18),
            (100, 75),
            (0, 5),
            (36, 48),
            (7, 13),
            (255, 85),
            (1024, 768),
            (360, 240),
            (u64::MAX, 3),
            (u64::MAX, u64::MAX),
        ];
        for &(a, b) in cases {
            let l = a.lcm(b);
            if l != 0 {
                assert_eq!(l % a, 0, "lcm({a}, {b}) = {l} should be divisible by {a}");
                assert_eq!(l % b, 0, "lcm({a}, {b}) = {l} should be divisible by {b}");
            }
        }
    }

    #[test]
    fn test_lcm_one_divides_other() {
        // a | b ⇒ lcm(a, b) = b
        let cases: &[(u64, u64)] = &[
            (1, 1),
            (1, 100),
            (3, 9),
            (7, 49),
            (12, 60),
            (100, 1000),
            (13, 13 * 17),
            (256, 256 * 1024),
        ];
        for &(small, large) in cases {
            assert_eq!(small.lcm(large), large, "lcm({small}, {large})");
            assert_eq!(large.lcm(small), large, "lcm({large}, {small})");
        }
    }

    // ========== 条件網羅 ==========

    #[test]
    #[should_panic]
    fn test_lcm_signed_panic_min_self() {
        let _ = i64::MIN.lcm(1);
    }

    #[test]
    #[should_panic]
    fn test_lcm_signed_panic_min_rhs() {
        let _ = 1i64.lcm(i64::MIN);
    }

    #[test]
    fn test_checked_lcm_overflow() {
        assert_eq!(u8::MAX.checked_lcm(2), None, "u8: MAX * 2");
        assert_eq!(u16::MAX.checked_lcm(2), None, "u16: MAX * 2");
        assert_eq!(u32::MAX.checked_lcm(2), None, "u32: MAX * 2");
        assert_eq!(u64::MAX.checked_lcm(2), None, "u64: MAX * 2");
        assert_eq!(u128::MAX.checked_lcm(2), None, "u128: MAX * 2");
        assert_eq!(usize::MAX.checked_lcm(2), None, "usize: MAX * 2");
        assert_eq!(i8::MAX.checked_lcm(2), None, "i8: MAX * 2");
        assert_eq!(i16::MAX.checked_lcm(2), None, "i16: MAX * 2");
        assert_eq!(i32::MAX.checked_lcm(2), None, "i32: MAX * 2");
        assert_eq!(i64::MAX.checked_lcm(2), None, "i64: MAX * 2");
        assert_eq!(i128::MAX.checked_lcm(2), None, "i128: MAX * 2");
        assert_eq!(isize::MAX.checked_lcm(2), None, "isize: MAX * 2");
    }

    #[test]
    fn test_checked_lcm_signed_min() {
        assert_eq!(i8::MIN.checked_lcm(1), None, "i8: MIN as self");
        assert_eq!(1i8.checked_lcm(i8::MIN), None, "i8: MIN as rhs");
        assert_eq!(i16::MIN.checked_lcm(1), None, "i16: MIN as self");
        assert_eq!(1i16.checked_lcm(i16::MIN), None, "i16: MIN as rhs");
        assert_eq!(i32::MIN.checked_lcm(1), None, "i32: MIN as self");
        assert_eq!(1i32.checked_lcm(i32::MIN), None, "i32: MIN as rhs");
        assert_eq!(i64::MIN.checked_lcm(1), None, "i64: MIN as self");
        assert_eq!(1i64.checked_lcm(i64::MIN), None, "i64: MIN as rhs");
        assert_eq!(i128::MIN.checked_lcm(1), None, "i128: MIN as self");
        assert_eq!(1i128.checked_lcm(i128::MIN), None, "i128: MIN as rhs");
        assert_eq!(isize::MIN.checked_lcm(1), None, "isize: MIN as self");
        assert_eq!(1isize.checked_lcm(isize::MIN), None, "isize: MIN as rhs");
    }

    #[test]
    fn test_checked_lcm_signed_min_with_zero() {
        assert_eq!(i8::MIN.checked_lcm(0), None, "i8: lcm(MIN, 0)");
        assert_eq!(0i8.checked_lcm(i8::MIN), None, "i8: lcm(0, MIN)");
        assert_eq!(i16::MIN.checked_lcm(0), None, "i16: lcm(MIN, 0)");
        assert_eq!(0i16.checked_lcm(i16::MIN), None, "i16: lcm(0, MIN)");
        assert_eq!(i32::MIN.checked_lcm(0), None, "i32: lcm(MIN, 0)");
        assert_eq!(0i32.checked_lcm(i32::MIN), None, "i32: lcm(0, MIN)");
        assert_eq!(i64::MIN.checked_lcm(0), None, "i64: lcm(MIN, 0)");
        assert_eq!(0i64.checked_lcm(i64::MIN), None, "i64: lcm(0, MIN)");
        assert_eq!(i128::MIN.checked_lcm(0), None, "i128: lcm(MIN, 0)");
        assert_eq!(0i128.checked_lcm(i128::MIN), None, "i128: lcm(0, MIN)");
        assert_eq!(isize::MIN.checked_lcm(0), None, "isize: lcm(MIN, 0)");
        assert_eq!(0isize.checked_lcm(isize::MIN), None, "isize: lcm(0, MIN)");
    }

    // ========== 小さい入力での全探索 ==========

    #[test]
    fn test_lcm_exhaustive_u8() {
        for a in 0..=u8::MAX {
            for b in 0..=u8::MAX {
                let expected = naive_lcm(a as u128, b as u128);
                if expected <= u8::MAX as u128 {
                    assert_eq!(a.lcm(b), expected as u8, "lcm({a}, {b})");
                }
                let expected_checked = if expected <= u8::MAX as u128 {
                    Some(expected as u8)
                } else {
                    None
                };
                assert_eq!(a.checked_lcm(b), expected_checked, "checked_lcm({a}, {b})");
            }
        }
    }

    #[test]
    fn test_lcm_exhaustive_i8() {
        for a in i8::MIN..=i8::MAX {
            for b in i8::MIN..=i8::MAX {
                let expected_checked = if a == i8::MIN || b == i8::MIN {
                    None
                } else {
                    let l = naive_lcm_signed(a as i128, b as i128);
                    if l <= i8::MAX as i128 {
                        Some(l as i8)
                    } else {
                        None
                    }
                };
                assert_eq!(a.checked_lcm(b), expected_checked, "checked_lcm({a}, {b})");
                if let Some(expected) = expected_checked {
                    assert_eq!(a.lcm(b), expected, "lcm({a}, {b})");
                }
            }
        }
    }

    // ========== ランダムテスト ==========

    #[test]
    fn test_lcm_random_u64() {
        let mut rng = get_test_rng();
        for _ in 0..500 {
            let a: u64 = rng.random_range(0..=50_000);
            let b: u64 = rng.random_range(0..=50_000);
            let expected = naive_lcm(a as u128, b as u128) as u64;
            assert_eq!(a.lcm(b), expected, "lcm({a}, {b})");
        }
    }

    #[test]
    fn test_lcm_random_i64() {
        let mut rng = get_test_rng();
        for _ in 0..500 {
            let a: i64 = rng.random_range(-50_000..=50_000);
            let b: i64 = rng.random_range(-50_000..=50_000);
            let expected = naive_lcm_signed(a as i128, b as i128) as i64;
            assert_eq!(a.lcm(b), expected, "lcm({a}, {b})");
        }
    }

    #[test]
    fn test_lcm_random_gcd_product() {
        let mut rng = get_test_rng();
        for _ in 0..500 {
            let a: u64 = rng.random_range(1..=50_000);
            let b: u64 = rng.random_range(1..=50_000);
            let l = a.lcm(b) as u128;
            let g = a.gcd(b) as u128;
            let product = a as u128 * b as u128;
            assert_eq!(l * g, product, "lcm({a}, {b}) * gcd({a}, {b}) = {a} * {b}");
        }
    }
}
