//! 最大公約数(GCD)
//!
//! 2つの整数の最大公約数を計算するトレイト．
//! 符号付き整数型では絶対値ベースのGCDを返す．
//!
//! # Examples
//!
//! ```
//! use reprol::math::gcd::Gcd;
//!
//! assert_eq!(48u64.gcd(18), 6);
//! assert_eq!((-12i64).gcd(8), 4);
//! assert_eq!(0u64.gcd(0), 0);
//! ```

/// 最大公約数(GCD)を計算するトレイト．
///
/// `gcd(0, 0) = 0` と定義する．
/// 符号付き整数型では結果は常に非負値を返す．
pub trait Gcd {
    /// `self` と `rhs` の最大公約数を返す．
    ///
    /// # Panics
    ///
    /// 符号付き整数型において，結果の絶対値が`Self`で表現不可能な場合にpanicする:
    /// - `self == Self::MIN && rhs == 0`
    /// - `self == 0 && rhs == Self::MIN`
    /// - `self == Self::MIN && rhs == Self::MIN`
    fn gcd(self, rhs: Self) -> Self;
}

macro_rules! impl_gcd_unsigned {
    ($ty: ty) => {
        impl Gcd for $ty {
            fn gcd(self, rhs: Self) -> Self {
                let (mut a, mut b) = (self, rhs);
                while b != 0 {
                    (a, b) = (b, a % b)
                }
                a
            }
        }
    };
}

macro_rules! impl_gcd_signed {
    ($ty:ty, $unsigned_ty:ty) => {
        impl Gcd for $ty {
            fn gcd(self, rhs: Self) -> Self {
                assert_ne!((self, rhs), (<$ty>::MIN, 0));
                assert_ne!((self, rhs), (0, <$ty>::MIN));
                assert_ne!((self, rhs), (<$ty>::MIN, <$ty>::MIN));
                let ua = self.unsigned_abs();
                let ub = rhs.unsigned_abs();
                ua.gcd(ub) as $ty
            }
        }
    };
}

macro_rules! impl_gcd {
    (unsigned: [$($u:ty),* $(,)?], signed: [$(($s:ty, $su:ty)),* $(,)?]$(,)?) => {
        $( impl_gcd_unsigned!($u); )*
        $( impl_gcd_signed!($s, $su); )*
    };
}

impl_gcd! {
    unsigned: [u8, u16, u32, u64, u128, usize],
    signed:   [(i8, u8), (i16, u16), (i32, u32), (i64, u64), (i128, u128), (isize, usize)],
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    use crate::utils::test_utils::random::get_test_rng;

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

    fn naive_gcd_signed(a: i128, b: i128) -> i128 {
        naive_gcd(a.unsigned_abs(), b.unsigned_abs()) as i128
    }

    // ========== スモークテスト ==========

    #[test]
    fn test_gcd_smoke() {
        assert_eq!(6u8.gcd(8), 2u8);
        assert_eq!(6u16.gcd(8), 2u16);
        assert_eq!(6u32.gcd(8), 2u32);
        assert_eq!(6u64.gcd(8), 2u64);
        assert_eq!(6u128.gcd(8), 2u128);
        assert_eq!(6usize.gcd(8), 2usize);
        assert_eq!((-6i8).gcd(8), 2i8);
        assert_eq!((-6i16).gcd(8), 2i16);
        assert_eq!((-6i32).gcd(8), 2i32);
        assert_eq!((-6i64).gcd(8), 2i64);
        assert_eq!((-6i128).gcd(8), 2i128);
        assert_eq!((-6isize).gcd(8), 2isize);
    }

    // ========== エッジケース ==========

    #[test]
    fn test_gcd_zero_zero() {
        assert_eq!(0u64.gcd(0), 0, "u64: gcd(0, 0)");
        assert_eq!(0i64.gcd(0), 0, "i64: gcd(0, 0)");
    }

    #[test]
    fn test_gcd_unsigned_with_zero() {
        for a in [1u64, 7, 42, u64::MAX] {
            assert_eq!(a.gcd(0), a, "gcd({a}, 0)");
            assert_eq!(0u64.gcd(a), a, "gcd(0, {a})");
        }
    }

    #[test]
    fn test_gcd_signed_with_zero() {
        for a in [1i64, -1, 42, -42, i64::MAX, i64::MIN + 1] {
            let expected = a.abs();
            assert_eq!(a.gcd(0), expected, "gcd({a}, 0)");
            assert_eq!(0i64.gcd(a), expected, "gcd(0, {a})");
        }
    }

    #[test]
    fn test_gcd_unsigned_with_one() {
        for n in [0u64, 1, u64::MAX, 123456789] {
            assert_eq!(1u64.gcd(n), 1, "gcd(1, {n})");
            assert_eq!(n.gcd(1), 1, "gcd({n}, 1)");
        }
    }

    #[test]
    fn test_gcd_signed_with_one() {
        for n in [0i64, i64::MAX, i64::MIN + 1] {
            assert_eq!(1i64.gcd(n), 1, "gcd(1, {n})");
            assert_eq!((-1i64).gcd(n), 1, "gcd(-1, {n})");
            assert_eq!(n.gcd(1), 1, "gcd({n}, 1)");
            assert_eq!(n.gcd(-1), 1, "gcd({n}, -1)");
        }
    }

    #[test]
    fn test_gcd_unsigned_same_values() {
        for n in [0u64, 1, 42, u64::MAX] {
            assert_eq!(n.gcd(n), n, "gcd({n}, {n})");
        }
    }

    #[test]
    fn test_gcd_signed_same_values() {
        for n in [0i64, 1, -1, 42, -42, i64::MAX, i64::MIN + 1] {
            assert_eq!(n.gcd(n), n.abs(), "gcd({n}, {n})");
        }
    }

    #[test]
    fn test_gcd_unsigned_boundary() {
        // u8::MAX
        assert_eq!(u8::MAX.gcd(u8::MAX), u8::MAX, "gcd(255, 255)");
        assert_eq!(u8::MAX.gcd(1), 1, "gcd(255, 1)");
        assert_eq!(u8::MAX.gcd(u8::MAX - 1), 1, "gcd(255, 254)");
        assert_eq!(u8::MAX.gcd(5), 5, "gcd(255, 5)");

        // u64::MAX
        assert_eq!(
            u64::MAX.gcd(u64::MAX),
            u64::MAX,
            "gcd({}, {})",
            u64::MAX,
            u64::MAX
        );
        assert_eq!(u64::MAX.gcd(1), 1, "gcd({}, 1)", u64::MAX);
        assert_eq!(
            u64::MAX.gcd(u64::MAX - 1),
            1,
            "gcd({}, {})",
            u64::MAX,
            u64::MAX - 1
        );
        assert_eq!(u64::MAX.gcd(3), 3, "gcd({}, 3)", u64::MAX);

        // u128::MAX
        assert_eq!(
            u128::MAX.gcd(u128::MAX),
            u128::MAX,
            "gcd({}, {})",
            u128::MAX,
            u128::MAX
        );
        assert_eq!(u128::MAX.gcd(1), 1, "gcd({}, 1)", u128::MAX);
        assert_eq!(
            u128::MAX.gcd(u128::MAX - 1),
            1,
            "gcd({}, {})",
            u128::MAX,
            u128::MAX - 1
        );
        assert_eq!(u128::MAX.gcd(3), 3, "gcd({}, 3)", u128::MAX);
    }

    #[test]
    fn test_gcd_signed_boundary() {
        // i64::MAX
        assert_eq!(
            i64::MAX.gcd(i64::MAX),
            i64::MAX,
            "gcd({}, {})",
            i64::MAX,
            i64::MAX
        );
        assert_eq!(i64::MAX.gcd(1), 1, "gcd({}, 1)", i64::MAX);
        assert_eq!(
            i64::MAX.gcd(i64::MAX - 1),
            1,
            "gcd({}, {})",
            i64::MAX,
            i64::MAX - 1
        );
        assert_eq!(i64::MAX.gcd(7), 7, "gcd({}, 7)", i64::MAX);

        // i64::MIN + 1
        let min1 = i64::MIN + 1;
        assert_eq!(min1.gcd(min1), min1.abs(), "gcd({min1}, {min1})");
        assert_eq!(min1.gcd(1), 1, "gcd({min1}, 1)");
        assert_eq!(min1.gcd(-1), 1, "gcd(min1, -1)");
        assert_eq!(min1.gcd(7), 7, "gcd({min1}, 7)");
    }

    // ========== 代数的性質 ==========

    #[test]
    fn test_gcd_commutativity() {
        let unsigned_cases: &[(u64, u64)] = &[
            (0, 0),
            (0, 1),
            (1, 1),
            (6, 8),
            (12, 18),
            (100, 75),
            (u64::MAX, 1),
            (u64::MAX, u64::MAX - 1),
        ];
        for &(a, b) in unsigned_cases {
            assert_eq!(a.gcd(b), b.gcd(a), "gcd({a}, {b})");
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
            assert_eq!(a.gcd(b), b.gcd(a), "gcd({a}, {b})");
        }
    }

    #[test]
    fn test_gcd_associativity() {
        let cases: &[(u64, u64, u64)] = &[
            (12, 18, 24),
            (0, 0, 0),
            (1, 1, 1),
            (100, 75, 50),
            (36, 48, 60),
            (7, 13, 29),
            (1000000007, 999999937, 998244353),
            (u64::MAX, u64::MAX - 1, u64::MAX - 2),
        ];
        for &(a, b, c) in cases {
            assert_eq!(
                a.gcd(b).gcd(c),
                a.gcd(b.gcd(c)),
                "associativity: ({a}, {b}, {c})"
            );
        }
    }

    #[test]
    fn test_gcd_divisibility() {
        let cases: &[(u64, u64)] = &[
            (12, 18),
            (48, 36),
            (100, 75),
            (0, 5),
            (1, 1),
            (6, 8),
            (1000000007, 999999937),
            (255, 85),
            (1024, 768),
            (360, 240),
            (u64::MAX, 3),
            (u64::MAX, u64::MAX),
            (123456789, 987654321),
            (7 * 11 * 13, 7 * 13 * 17),
            (2 * 3 * 5 * 7, 3 * 5 * 11),
        ];
        for &(a, b) in cases {
            let g = a.gcd(b);
            if a != 0 {
                assert_eq!(a % g, 0, "divisibility: {g} should divide {a}");
            }
            if b != 0 {
                assert_eq!(b % g, 0, "divisibility: {g} should divide {b}");
            }
        }
    }

    #[test]
    fn test_gcd_signed_nonnegative() {
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
            (-42, -42),
            (100, -75),
        ];
        for &(a, b) in cases {
            assert!(a.gcd(b) >= 0, "gcd({a}, {b}) >= 0");
        }
    }

    #[test]
    fn test_gcd_signed_negative_combinations() {
        let cases: &[(i64, i64)] = &[(12, 18), (48, 36), (100, 75), (7, 13), (1, 999), (42, 42)];
        for &(a, b) in cases {
            let expected = a.gcd(b);
            assert_eq!(a.gcd(-b), expected, "gcd({a}, {}", -b);
            assert_eq!((-a).gcd(b), expected, "gcd({}, {b})", -a);
            assert_eq!((-a).gcd(-b), expected, "gcd({}, {})", -a, -b);
        }
    }

    #[test]
    fn test_gcd_one_divides_other() {
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
            assert_eq!(small.gcd(large), small, "gcd({small}, {large})");
            assert_eq!(large.gcd(small), small, "gcd({large}, {small})");
        }
    }

    #[test]
    fn test_gcd_coprime_pairs() {
        let cases: &[(u64, u64)] = &[
            (2, 3),
            (7, 13),
            (11, 17),
            (100, 99),
            (1000000007, 999999937),
            (998244353, 1000000007),
            (6, 35),
            (14, 15),
            (u64::MAX, 2),
        ];
        for &(a, b) in cases {
            assert_eq!(a.gcd(b), 1, "gcd({a}, {b})");
        }
    }

    #[test]
    fn test_gcd_powers_of_two() {
        for i in 0u32..=62 {
            for j in 0u32..=62 {
                let a: u64 = 1 << i;
                let b: u64 = 1 << j;
                let expected: u64 = 1 << i.min(j);
                assert_eq!(a.gcd(b), expected, "gcd(2^{i}, 2^{j})");
            }
        }
    }

    #[test]
    fn test_gcd_large_known_gcd() {
        let p = 1_000_000_007u64;
        assert_eq!((p * 3).gcd(p * 7), p, "gcd({}, {})", p * 3, p * 7);
        assert_eq!((p * 4).gcd(p * 6), p * 2, "gcd({}, {})", p * 4, p * 6);

        let x = u64::MAX as u128;
        assert_eq!((x * 3).gcd(x * 7), x, "gcd({}, {})", x * 3, x * 7);
        assert_eq!((x * 12).gcd(x * 18), x * 6, "gcd({}, {})", x * 12, x * 18);

        // i64: 大きな符号付き値
        let x = 1_000_000_007i64;
        assert_eq!((-x * 3).gcd(x * 7), x, "gcd({}, {})", -x * 3, x * 7);
        assert_eq!((x * 4).gcd(-x * 6), x * 2, "gcd({}, {})", x * 4, -x * 6);
    }

    // ========== 条件網羅 ==========

    #[test]
    #[should_panic]
    fn test_gcd_signed_panic_min_zero() {
        let _ = i64::MIN.gcd(0);
    }

    #[test]
    #[should_panic]
    fn test_gcd_signed_panic_zero_min() {
        let _ = 0i64.gcd(i64::MIN);
    }

    #[test]
    #[should_panic]
    fn test_gcd_signed_panic_min_min() {
        let _ = i64::MIN.gcd(i64::MIN);
    }

    // ========== 小さい入力での全探索 ==========

    #[test]
    fn test_gcd_exhaustive_u8() {
        for a in 0..=u8::MAX {
            for b in 0..=u8::MAX {
                let expected = naive_gcd(a as u128, b as u128) as u8;
                assert_eq!(a.gcd(b), expected, "gcd({a}, {b})");
            }
        }
    }

    #[test]
    fn test_gcd_exhaustive_i8() {
        for a in i8::MIN..=i8::MAX {
            for b in i8::MIN..=i8::MAX {
                // (MIN, 0), (0, MIN), (MIN, MIN) はpanic対象
                if (a == i8::MIN && b == 0)
                    || (a == 0 && b == i8::MIN)
                    || (a == i8::MIN && b == i8::MIN)
                {
                    continue;
                }
                let expected = naive_gcd_signed(a as i128, b as i128) as i8;
                assert_eq!(a.gcd(b), expected, "exhaustive i8: gcd({a}, {b})");
            }
        }
    }

    // ========== ランダムテスト ==========

    #[test]
    fn test_gcd_random_u64() {
        let mut rng = get_test_rng();
        for _ in 0..500 {
            let a: u64 = rng.random_range(0..=50_000);
            let b: u64 = rng.random_range(0..=50_000);
            let expected = naive_gcd(a as u128, b as u128) as u64;
            assert_eq!(a.gcd(b), expected, "gcd({a}, {b})");
        }
    }

    #[test]
    fn test_gcd_random_u128() {
        let mut rng = get_test_rng();
        for _ in 0..500 {
            let a: u128 = rng.random_range(0..=50_000u128);
            let b: u128 = rng.random_range(0..=50_000u128);
            let expected = naive_gcd(a, b);
            assert_eq!(a.gcd(b), expected, "gcd({a}, {b})");
        }
    }

    #[test]
    fn test_gcd_random_i64() {
        let mut rng = get_test_rng();
        for _ in 0..500 {
            let a: i64 = rng.random_range(-50_000..=50_000);
            let b: i64 = rng.random_range(-50_000..=50_000);
            let expected = naive_gcd_signed(a as i128, b as i128) as i64;
            assert_eq!(a.gcd(b), expected, "gcd({a}, {b})");
        }
    }

    #[test]
    fn test_gcd_random_coprime_consecutive() {
        let mut rng = get_test_rng();
        for _ in 0..500 {
            let n: u64 = rng.random_range(1..u64::MAX);
            assert_eq!(n.gcd(n + 1), 1, "gcd({n}, {})", n + 1);
        }
    }

    #[test]
    fn test_gcd_random_known_factor() {
        let mut rng = get_test_rng();
        for _ in 0..500 {
            let g: u64 = rng.random_range(1..=1_000_000);
            let m: u64 = rng.random_range(1..=1_000_000_000);
            let n: u64 = rng.random_range(1..=1_000_000_000);
            let a = g as u128 * m as u128;
            let b = g as u128 * n as u128;
            if a > u64::MAX as u128 || b > u64::MAX as u128 {
                continue;
            }
            let (a, b) = (a as u64, b as u64);
            let result = a.gcd(b);
            assert_eq!(
                result % g,
                0,
                "gcd({a}, {b}) = {result} should be multiple of {g}"
            );
            if a != 0 {
                assert_eq!(a % result, 0, "{result} should divide {a}");
            }
            if b != 0 {
                assert_eq!(b % result, 0, "{result} should divide {b}");
            }
        }
    }
}
