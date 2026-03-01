//! 剰余演算トレイト(ModOps)
//!
//! プリミティブ整数型(`u8`-`u128`, `i8`-`i128`, `usize`, `isize`)に対する
//! 剰余演算を提供する．
//!
//! # Examples
//!
//! ```
//! use reprol::math::mod_ops::ModOps;
//!
//! let p = 998244353u64;
//!
//! assert_eq!(3u64.add_mod(5, p), 8);
//! assert_eq!(3u64.mul_mod(5, p), 15);
//! assert_eq!(3u64.pow_mod(4, p), 81);
//! assert_eq!(3u64.mul_mod(3u64.inv_mod(p), p), 1);
//! ```

/// プリミティブ整数型に対する剰余演算を提供するトレイト．
pub trait ModOps {
    /// 剰余を取る．
    ///
    /// # Panics
    ///
    /// `p == 0` のとき．
    fn reduce_mod(self, p: Self) -> Self;

    /// 剰余加算．
    fn add_mod(self, rhs: Self, p: Self) -> Self;

    /// 剰余減算．
    fn sub_mod(self, rhs: Self, p: Self) -> Self;

    /// 剰余乗算．
    fn mul_mod(self, rhs: Self, p: Self) -> Self;

    /// 剰余除算．
    fn div_mod(self, rhs: Self, p: Self) -> Self;

    /// 剰余における加法逆元．
    fn neg_mod(self, p: Self) -> Self;

    /// 剰余累乗．
    fn pow_mod(self, exp: u64, p: Self) -> Self;

    /// 剰余における乗法逆元．
    ///
    /// # Panics
    ///
    /// - `self == 0` のとき．
    /// - `gcd(self, p) != 1` のとき．
    fn inv_mod(self, p: Self) -> Self;
}

/// `impl ModOps` の共通メソッド(reduce_mod, div_mod, neg_mod, inv_mod)を展開する．
macro_rules! impl_modops_common {
    ($ty:ty) => {
        fn reduce_mod(self, p: Self) -> Self {
            assert!(p > 0);
            self.rem_euclid(p)
        }

        fn div_mod(self, rhs: Self, p: Self) -> Self {
            self.mul_mod(rhs.inv_mod(p), p)
        }

        fn neg_mod(self, p: Self) -> Self {
            (p - self.reduce_mod(p)).reduce_mod(p)
        }

        fn inv_mod(self, p: Self) -> Self {
            assert!(self != 0);
            let mut a = self.reduce_mod(p);
            let mut b = p;
            let mut u = 1;
            let mut v = 0;
            while b > 0 {
                let q = a / b;
                let r = a % b;
                let nv = u.sub_mod(q.mul_mod(v, p), p); // u-qv
                (a, b, u, v) = (b, r, v, nv);
            }
            assert!(a == 1);
            u
        }
    };
}

/// upcast 方式の `impl ModOps`．
///
/// `$wide` 型にキャストして演算することでオーバーフローを回避する．
macro_rules! impl_modops_upcast {
    ($ty:ty, $wide:ty) => {
        impl ModOps for $ty {
            impl_modops_common!($ty);

            fn add_mod(self, rhs: Self, p: Self) -> Self {
                let a = self.reduce_mod(p) as $wide;
                let b = rhs.reduce_mod(p) as $wide;
                ((a + b) % p as $wide) as $ty
            }

            fn sub_mod(self, rhs: Self, p: Self) -> Self {
                let a = self.reduce_mod(p) as $wide;
                let b = rhs.reduce_mod(p) as $wide;
                let p = p as $wide;
                ((p + a - b) % p) as $ty
            }

            fn mul_mod(self, rhs: Self, p: Self) -> Self {
                let a = self.reduce_mod(p) as $wide;
                let b = rhs.reduce_mod(p) as $wide;
                ((a * b) % p as $wide) as $ty
            }

            fn pow_mod(self, mut exp: u64, p: Self) -> Self {
                if p == 1 {
                    return 0;
                }
                let mut result: $wide = 1;
                let mut base = self.reduce_mod(p) as $wide;
                let p = p as $wide;
                while exp > 0 {
                    if exp & 1 == 1 {
                        result = result * base % p;
                    }
                    base = base * base % p;
                    exp >>= 1;
                }
                result as $ty
            }
        }
    };
}

/// 最大幅型向けの `impl ModOps`．
///
/// upcast 先の型が存在しない `u128`/`i128`/`usize`/`isize` 向け．
macro_rules! impl_modops_maxwidth {
    ($ty:ty) => {
        impl ModOps for $ty {
            impl_modops_common!($ty);

            fn add_mod(self, rhs: Self, p: Self) -> Self {
                let a = self.reduce_mod(p);
                let b = rhs.reduce_mod(p);
                if b >= p - a { b - (p - a) } else { a + b }
            }

            fn sub_mod(self, rhs: Self, p: Self) -> Self {
                let a = self.reduce_mod(p);
                let b = rhs.reduce_mod(p);
                if a >= b { a - b } else { p - (b - a) }
            }

            fn mul_mod(self, rhs: Self, p: Self) -> Self {
                let mut a = self.reduce_mod(p);
                let mut b = rhs.reduce_mod(p);
                let mut result: Self = 0;
                while b > 0 {
                    if b & 1 == 1 {
                        result = result.add_mod(a, p);
                    }
                    a = a.add_mod(a, p);
                    b >>= 1;
                }
                result
            }

            fn pow_mod(self, mut exp: u64, p: Self) -> Self {
                if p == 1 {
                    return 0;
                }
                let mut result: Self = 1;
                let mut base = self.reduce_mod(p);
                while exp > 0 {
                    if exp & 1 == 1 {
                        result = result.mul_mod(base, p);
                    }
                    base = base.mul_mod(base, p);
                    exp >>= 1;
                }
                result
            }
        }
    };
}

impl_modops_upcast!(u8, u16);
impl_modops_upcast!(u16, u32);
impl_modops_upcast!(u32, u64);
impl_modops_upcast!(u64, u128);
impl_modops_upcast!(i8, i16);
impl_modops_upcast!(i16, i32);
impl_modops_upcast!(i32, i64);
impl_modops_upcast!(i64, i128);
impl_modops_maxwidth!(u128);
impl_modops_maxwidth!(i128);
impl_modops_maxwidth!(usize);
impl_modops_maxwidth!(isize);

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    use crate::utils::test_utils::random::get_test_rng;

    // ========== 定数 ==========

    const PRIMES: [u64; 3] = [998_244_353, 1_000_000_007, 2_147_483_647];
    const SMALL_PRIMES: [u64; 5] = [2, 3, 5, 7, 11];

    // ========== ヘルパ関数 ==========

    // --- unsigned (u64 -> u128) ---

    fn ref_reduce(v: u64, p: u64) -> u64 {
        (v as u128 % p as u128) as u64
    }

    fn ref_add(a: u64, b: u64, p: u64) -> u64 {
        let (a, b, p) = (a as u128, b as u128, p as u128);
        ((a % p + b % p) % p) as u64
    }

    fn ref_sub(a: u64, b: u64, p: u64) -> u64 {
        let (a, b, p) = (a as u128, b as u128, p as u128);
        ((p + a % p - b % p) % p) as u64
    }

    fn ref_mul(a: u64, b: u64, p: u64) -> u64 {
        let (a, b, p) = (a as u128, b as u128, p as u128);
        ((a % p) * (b % p) % p) as u64
    }

    fn ref_pow(base: u64, exp: u64, p: u64) -> u64 {
        if p == 1 {
            return 0;
        }
        let p = p as u128;
        let mut res: u128 = 1;
        let base = base as u128 % p;
        for _ in 0..exp {
            res = res * base % p;
        }
        res as u64
    }

    // --- signed (i64 -> i128) ---

    fn ref_reduce_signed(v: i64, p: i64) -> i64 {
        (v as i128).rem_euclid(p as i128) as i64
    }

    fn ref_add_signed(a: i64, b: i64, p: i64) -> i64 {
        ((a as i128 + b as i128).rem_euclid(p as i128)) as i64
    }

    fn ref_sub_signed(a: i64, b: i64, p: i64) -> i64 {
        ((a as i128 - b as i128).rem_euclid(p as i128)) as i64
    }

    fn ref_mul_signed(a: i64, b: i64, p: i64) -> i64 {
        ((a as i128 * b as i128).rem_euclid(p as i128)) as i64
    }

    fn ref_pow_signed(base: i64, exp: u64, p: i64) -> i64 {
        if p == 1 {
            return 0;
        }
        let p128 = p as i128;
        let base_mod = (base as i128).rem_euclid(p128);
        let mut res: i128 = 1;
        for _ in 0..exp {
            res = (res * base_mod).rem_euclid(p128);
        }
        res as i64
    }

    // ========== スモークテスト ==========

    #[test]
    fn test_reduce_mod_smoke() {
        // reduce_mod(7) == 3
        assert_eq!(10u8.reduce_mod(7), 3u8);
        assert_eq!(10u16.reduce_mod(7), 3u16);
        assert_eq!(10u32.reduce_mod(7), 3u32);
        assert_eq!(10u64.reduce_mod(7), 3u64);
        assert_eq!(10u128.reduce_mod(7), 3u128);
        assert_eq!(10usize.reduce_mod(7), 3usize);
        assert_eq!(10i8.reduce_mod(7), 3i8);
        assert_eq!(10i16.reduce_mod(7), 3i16);
        assert_eq!(10i32.reduce_mod(7), 3i32);
        assert_eq!(10i64.reduce_mod(7), 3i64);
        assert_eq!(10i128.reduce_mod(7), 3i128);
        assert_eq!(10isize.reduce_mod(7), 3isize);
    }

    #[test]
    fn test_add_mod_smoke() {
        // add_mod(5, 7) == 1
        assert_eq!(3u8.add_mod(5, 7), 1u8);
        assert_eq!(3u16.add_mod(5, 7), 1u16);
        assert_eq!(3u32.add_mod(5, 7), 1u32);
        assert_eq!(3u64.add_mod(5, 7), 1u64);
        assert_eq!(3u128.add_mod(5, 7), 1u128);
        assert_eq!(3usize.add_mod(5, 7), 1usize);
        assert_eq!(3i8.add_mod(5, 7), 1i8);
        assert_eq!(3i16.add_mod(5, 7), 1i16);
        assert_eq!(3i32.add_mod(5, 7), 1i32);
        assert_eq!(3i64.add_mod(5, 7), 1i64);
        assert_eq!(3i128.add_mod(5, 7), 1i128);
        assert_eq!(3isize.add_mod(5, 7), 1isize);
    }

    #[test]
    fn test_sub_mod_smoke() {
        // sub_mod(5, 7) == 5
        assert_eq!(3u8.sub_mod(5, 7), 5u8);
        assert_eq!(3u16.sub_mod(5, 7), 5u16);
        assert_eq!(3u32.sub_mod(5, 7), 5u32);
        assert_eq!(3u64.sub_mod(5, 7), 5u64);
        assert_eq!(3u128.sub_mod(5, 7), 5u128);
        assert_eq!(3usize.sub_mod(5, 7), 5usize);
        assert_eq!(3i8.sub_mod(5, 7), 5i8);
        assert_eq!(3i16.sub_mod(5, 7), 5i16);
        assert_eq!(3i32.sub_mod(5, 7), 5i32);
        assert_eq!(3i64.sub_mod(5, 7), 5i64);
        assert_eq!(3i128.sub_mod(5, 7), 5i128);
        assert_eq!(3isize.sub_mod(5, 7), 5isize);
    }

    #[test]
    fn test_mul_mod_smoke() {
        // mul_mod(4, 7) == 5
        assert_eq!(3u8.mul_mod(4, 7), 5u8);
        assert_eq!(3u16.mul_mod(4, 7), 5u16);
        assert_eq!(3u32.mul_mod(4, 7), 5u32);
        assert_eq!(3u64.mul_mod(4, 7), 5u64);
        assert_eq!(3u128.mul_mod(4, 7), 5u128);
        assert_eq!(3usize.mul_mod(4, 7), 5usize);
        assert_eq!(3i8.mul_mod(4, 7), 5i8);
        assert_eq!(3i16.mul_mod(4, 7), 5i16);
        assert_eq!(3i32.mul_mod(4, 7), 5i32);
        assert_eq!(3i64.mul_mod(4, 7), 5i64);
        assert_eq!(3i128.mul_mod(4, 7), 5i128);
        assert_eq!(3isize.mul_mod(4, 7), 5isize);
    }

    #[test]
    fn test_div_mod_smoke() {
        // div_mod(4, 7) == 3 (inv(4,7)=2, 5*2=10, 10%7=3)
        assert_eq!(5u8.div_mod(4, 7), 3u8);
        assert_eq!(5u16.div_mod(4, 7), 3u16);
        assert_eq!(5u32.div_mod(4, 7), 3u32);
        assert_eq!(5u64.div_mod(4, 7), 3u64);
        assert_eq!(5u128.div_mod(4, 7), 3u128);
        assert_eq!(5usize.div_mod(4, 7), 3usize);
        assert_eq!(5i8.div_mod(4, 7), 3i8);
        assert_eq!(5i16.div_mod(4, 7), 3i16);
        assert_eq!(5i32.div_mod(4, 7), 3i32);
        assert_eq!(5i64.div_mod(4, 7), 3i64);
        assert_eq!(5i128.div_mod(4, 7), 3i128);
        assert_eq!(5isize.div_mod(4, 7), 3isize);
    }

    #[test]
    fn test_neg_mod_smoke() {
        // 3.neg_mod(7) == 4
        assert_eq!(3u8.neg_mod(7), 4, "u8");
        assert_eq!(3u16.neg_mod(7), 4, "u16");
        assert_eq!(3u32.neg_mod(7), 4, "u32");
        assert_eq!(3u64.neg_mod(7), 4, "u64");
        assert_eq!(3u128.neg_mod(7), 4, "u128");
        assert_eq!(3usize.neg_mod(7), 4, "usize");
        assert_eq!(3i8.neg_mod(7), 4, "i8");
        assert_eq!(3i16.neg_mod(7), 4, "i16");
        assert_eq!(3i32.neg_mod(7), 4, "i32");
        assert_eq!(3i64.neg_mod(7), 4, "i64");
        assert_eq!(3i128.neg_mod(7), 4, "i128");
        assert_eq!(3isize.neg_mod(7), 4, "isize");
    }

    #[test]
    fn test_pow_mod_smoke() {
        // pow_mod(4, 7) == 4 (81%7=4)
        assert_eq!(3u8.pow_mod(4, 7), 4u8);
        assert_eq!(3u16.pow_mod(4, 7), 4u16);
        assert_eq!(3u32.pow_mod(4, 7), 4u32);
        assert_eq!(3u64.pow_mod(4, 7), 4u64);
        assert_eq!(3u128.pow_mod(4, 7), 4u128);
        assert_eq!(3usize.pow_mod(4, 7), 4usize);
        assert_eq!(3i8.pow_mod(4, 7), 4i8);
        assert_eq!(3i16.pow_mod(4, 7), 4i16);
        assert_eq!(3i32.pow_mod(4, 7), 4i32);
        assert_eq!(3i64.pow_mod(4, 7), 4i64);
        assert_eq!(3i128.pow_mod(4, 7), 4i128);
        assert_eq!(3isize.pow_mod(4, 7), 4isize);
    }

    #[test]
    fn test_inv_mod_smoke() {
        // inv_mod(7) == 5 (3*5=15, 15%7=1)
        assert_eq!(3u8.inv_mod(7), 5u8);
        assert_eq!(3u16.inv_mod(7), 5u16);
        assert_eq!(3u32.inv_mod(7), 5u32);
        assert_eq!(3u64.inv_mod(7), 5u64);
        assert_eq!(3u128.inv_mod(7), 5u128);
        assert_eq!(3usize.inv_mod(7), 5usize);
        assert_eq!(3i8.inv_mod(7), 5i8);
        assert_eq!(3i16.inv_mod(7), 5i16);
        assert_eq!(3i32.inv_mod(7), 5i32);
        assert_eq!(3i64.inv_mod(7), 5i64);
        assert_eq!(3i128.inv_mod(7), 5i128);
        assert_eq!(3isize.inv_mod(7), 5isize);
    }

    // ========== エッジケース ==========

    #[test]
    fn test_reduce_mod_zero() {
        for &p in &PRIMES {
            assert_eq!(0u64.reduce_mod(p), 0, "reduce_mode(0, {p})");
        }
        assert_eq!(0u64.reduce_mod(1), 0, "reduce_mod(0, 1)");
        assert_eq!(0u64.reduce_mod(2), 0, "reduce_mod(0, 2)");
    }

    #[test]
    fn test_reduce_mod_already_reduced() {
        for &p in &PRIMES {
            for v in [0, 1, p / 2, p - 1] {
                assert_eq!(v.reduce_mod(p), v, "reduce_mod({v}, {p})");
            }
        }
    }

    #[test]
    fn test_reduce_mod_multiples_of_p() {
        for &p in &PRIMES {
            assert_eq!(p.reduce_mod(p), 0, "{p} mod {p}");
            assert_eq!((2 * p).reduce_mod(p), 0, "2*{p} mod {p}");
            assert_eq!((3 * p).reduce_mod(p), 0, "3*{p} mod {p}");
        }
    }

    #[test]
    fn test_reduce_mod_negative() {
        assert_eq!((-1i64).reduce_mod(7), 6, "-1 mod 7");
        for &p in &PRIMES {
            let p_i64 = p as i64;
            let result = i64::MIN.reduce_mod(p_i64);
            let expected = ref_reduce_signed(i64::MIN, p_i64);
            assert_eq!(result, expected, "{} mod {p}", i64::MIN);
        }
    }

    #[test]
    fn test_arithmetic_identity_zero() {
        for &p in &PRIMES {
            for a in [0u64, 1, p / 2, p - 1] {
                let a_r = a.reduce_mod(p);
                assert_eq!(a.add_mod(0, p), a_r, "{a} + 0 mod {p}");
                assert_eq!(a.sub_mod(0, p), a_r, "{a} - 0 mod {p}");
                assert_eq!(a.mul_mod(0, p), 0, "{a} * 0 mod {p}");
            }
        }
    }

    #[test]
    fn test_arithmetic_identity_one() {
        for &p in &PRIMES {
            for a in [0u64, 1, p / 2, p - 1] {
                let a_r = a.reduce_mod(p);
                assert_eq!(a.mul_mod(1, p), a_r, "{a} * 1 mod {p}");
                assert_eq!(a.div_mod(1, p), a_r, "{a} / 1 mod {p}");
            }
        }
    }

    #[test]
    fn test_sub_mod_self() {
        for &p in &PRIMES {
            for a in [0u64, 1, p / 2, p - 1] {
                assert_eq!(a.sub_mod(a, p), 0, "{a} - {a} mod {p}");
            }
        }
    }

    #[test]
    fn test_pow_mod_exp_zero() {
        for &p in &PRIMES {
            for a in [0u64, 1, p / 2, p - 1] {
                assert_eq!(a.pow_mod(0, p), 1, "{a}^0 mod {p}");
            }
        }
    }

    #[test]
    fn test_pow_mod_exp_one() {
        for &p in &PRIMES {
            for a in [0u64, 1, p / 2, p - 1, p, 2 * p] {
                assert_eq!(a.pow_mod(1, p), a.reduce_mod(p), "{a}^1 mod {p}");
            }
        }
    }

    #[test]
    fn test_pow_mod_p_one() {
        for a in [0u64, 1, 1030, u64::MAX] {
            for exp in [0, 1, 10, 100] {
                assert_eq!(a.pow_mod(exp, 1), 0, "{a}^{exp} mod 1");
            }
        }
    }

    #[test]
    fn test_neg_mod_zero() {
        for &p in &PRIMES {
            assert_eq!(0u64.neg_mod(p), 0, "neg(0) mod {p}");
        }
        assert_eq!(0u64.neg_mod(1), 0, "neg(0) mod 1");
    }

    // ========== 代数的性質 ==========

    #[test]
    fn test_add_mod_commutativity() {
        let cases: &[(u64, u64)] = &[(0, 0), (0, 1), (3, 5), (1030, 7777), (u64::MAX, 1)];
        for &p in &PRIMES {
            for &(a, b) in cases {
                assert_eq!(a.add_mod(b, p), b.add_mod(a, p), "{a} + {b} mod {p}");
            }
        }
    }

    #[test]
    fn test_add_mod_associativity() {
        let cases: &[(u64, u64, u64)] = &[
            (1, 2, 3),
            (1030, 7777, 42),
            (0, 0, 0),
            (u64::MAX, u64::MAX - 1, u64::MAX - 2),
        ];
        for &p in &PRIMES {
            for &(a, b, c) in cases {
                assert_eq!(
                    a.add_mod(b, p).add_mod(c, p),
                    a.add_mod(b.add_mod(c, p), p),
                    "({a} + {b}) + {c} = {a} + ({b} + {c}) (mod {p})"
                );
            }
        }
    }

    #[test]
    fn test_mul_mod_commutativity() {
        let cases: &[(u64, u64)] = &[(0, 1), (2, 3), (1030, 7777), (u64::MAX, 42)];
        for &p in &PRIMES {
            for &(a, b) in cases {
                assert_eq!(
                    a.mul_mod(b, p),
                    b.mul_mod(a, p),
                    "{a} * {b} = {b} * {a} (mod {p})"
                );
            }
        }
    }

    #[test]
    fn test_mul_mod_associativity() {
        let cases: &[(u64, u64, u64)] = &[
            (2, 3, 5),
            (1030, 7777, 42),
            (0, 1, 2),
            (u64::MAX, u64::MAX - 1, 3),
        ];
        for &p in &PRIMES {
            for &(a, b, c) in cases {
                assert_eq!(
                    a.mul_mod(b, p).mul_mod(c, p),
                    a.mul_mod(b.mul_mod(c, p), p),
                    "({a} * {b}) * {c} = {b} * ({b} * {c}) (mod {p})"
                );
            }
        }
    }

    #[test]
    fn test_distributivity() {
        let cases: &[(u64, u64, u64)] = &[
            (2, 3, 5),
            (1030, 7777, 42),
            (0, 1, 0),
            (u64::MAX, 1, u64::MAX - 1),
        ];
        for &p in &PRIMES {
            for &(a, b, c) in cases {
                let lhs = a.mul_mod(b.add_mod(c, p), p);
                let rhs = a.mul_mod(b, p).add_mod(a.mul_mod(c, p), p);
                assert_eq!(
                    lhs, rhs,
                    "{a} * ({b} + {c}) = {a} * {b} + {a} * {c} (mod {p})"
                );
            }
        }
    }

    #[test]
    fn test_neg_mod_double() {
        for &p in &PRIMES {
            for a in [0u64, 1, p / 2, p - 1, 1030, u64::MAX] {
                let double_neg = a.neg_mod(p).neg_mod(p);
                assert_eq!(double_neg, a.reduce_mod(p), "neg(neg({a})) mod {p}");
            }
        }
    }

    #[test]
    fn test_neg_mod_additive_inverse() {
        for &p in &PRIMES {
            for a in [0u64, 1, p / 2, p - 1, 1030, u64::MAX] {
                assert_eq!(a.add_mod(a.neg_mod(p), p), 0, "{a} + neg({a}) mod {p}");
            }
        }
    }

    #[test]
    fn test_inv_mod_multiplicative_inverse() {
        for &p in &PRIMES {
            for a in [1u64, 2, p / 2, p - 1, 1030] {
                assert_eq!(a.mul_mod(a.inv_mod(p), p), 1, "{a} * inv({a}) mod {p}");
            }
        }
    }

    #[test]
    fn test_pow_mod_additive_exponents() {
        for &p in &PRIMES {
            for a in [2u64, 3, p - 1, 1030] {
                for (m, n) in [(1u64, 2u64), (3, 4), (0, 5), (10, 20)] {
                    assert_eq!(
                        a.pow_mod(m + n, p),
                        a.pow_mod(m, p).mul_mod(a.pow_mod(n, p), p),
                        "{a}^({m}+{n}) = {a}^{m} * {a}^{n} (mod {p})"
                    );
                }
            }
        }
    }

    #[test]
    fn test_pow_mod_fermats_little_theorem() {
        for &p in &PRIMES {
            for a in [1u64, 2, 3, p / 2, p - 1, 1030] {
                if a % p != 0 {
                    assert_eq!(a.pow_mod(p - 1, p), 1, "{a}^({p}-1) mod {p}");
                }
            }
        }
    }

    // ========== 条件網羅 ==========

    #[test]
    #[should_panic]
    fn test_reduce_mod_panic_p_zero() {
        let _ = 0u64.reduce_mod(0);
    }

    #[test]
    #[should_panic]
    fn test_inv_mod_panic_not_coprime() {
        // gcd(4, 6) == 2 != 1
        let _ = 4u64.inv_mod(6);
    }

    // ========== 小さい入力での全探索 ==========

    #[test]
    fn test_reduce_mod_exhaustive() {
        for &p in &SMALL_PRIMES {
            for a in 0..=3 * p {
                assert_eq!(a.reduce_mod(p), ref_reduce(a, p), "reduce_mod({a}, {p})");
            }
        }
    }

    #[test]
    fn test_add_sub_mod_exhaustive() {
        for &p in &SMALL_PRIMES {
            for a in 0..p {
                for b in 0..p {
                    assert_eq!(a.add_mod(b, p), ref_add(a, b, p), "add_mod({a}, {b}, {p})");
                    assert_eq!(a.sub_mod(b, p), ref_sub(a, b, p), "sub_mod({a}, {b}, {p})");
                }
            }
        }
    }

    #[test]
    fn test_mul_mod_exhaustive() {
        for &p in &SMALL_PRIMES {
            for a in 0..p {
                for b in 0..p {
                    assert_eq!(a.mul_mod(b, p), ref_mul(a, b, p), "mul_mod({a}, {b}, {p})");
                }
            }
        }
    }

    #[test]
    fn test_neg_mod_exhaustive() {
        for &p in &SMALL_PRIMES {
            for a in 0..p {
                let result = a.neg_mod(p);
                assert_eq!(
                    a.add_mod(result, p),
                    0,
                    "neg_mod({a}, {p}): {a} + {result} != 0"
                );
            }
        }
    }

    #[test]
    fn test_pow_inv_div_exhaustive() {
        for &p in &SMALL_PRIMES {
            // pow: a in [0,p), exp in 0..=2*p
            for a in 0..p {
                for exp in 0..=2 * p {
                    assert_eq!(
                        a.pow_mod(exp, p),
                        ref_pow(a, exp, p),
                        "pow_mod({a}, {exp}, {p})"
                    );
                }
            }

            // inv: a in [1,p), 性質 a * inv(a) ≡ 1
            for a in 1..p {
                assert_eq!(
                    a.mul_mod(a.inv_mod(p), p),
                    1,
                    "inv_mod({a}, {p}): {a} * inv({a}) != 1"
                );
            }

            // div: (a,b) in [0,p) x [1,p), 性質 result * b ≡ a
            for a in 0..p {
                for b in 1..p {
                    let result = a.div_mod(b, p);
                    assert_eq!(
                        result.mul_mod(b, p),
                        a,
                        "div_mod({a}, {b}, {p}): {result} * {b} != {a}"
                    );
                }
            }
        }
    }

    // ========== ランダムテスト ==========

    #[test]
    fn test_arithmetic_mod_random_u64() {
        let mut rng = get_test_rng();
        for _ in 0..500 {
            let a: u64 = rng.random();
            let b: u64 = rng.random();
            for &p in &PRIMES {
                assert_eq!(a.reduce_mod(p), ref_reduce(a, p), "reduce_mod({a}, {p})");
                assert_eq!(a.add_mod(b, p), ref_add(a, b, p), "add_mod({a}, {b}, {p})");
                assert_eq!(a.sub_mod(b, p), ref_sub(a, b, p), "sub_mod({a}, {b}, {p})");
                assert_eq!(a.mul_mod(b, p), ref_mul(a, b, p), "mul_mod({a}, {b}, {p})");
                assert_eq!(
                    a.add_mod(a.neg_mod(p), p),
                    0,
                    "neg_mod({a}, {p}): {a} + neg({a}) != 0"
                );
            }
        }
    }

    #[test]
    fn test_arithmetic_mod_random_i64() {
        let mut rng = get_test_rng();
        for _ in 0..500 {
            let a: i64 = rng.random();
            let b: i64 = rng.random();
            for &p in &PRIMES {
                let p_i64 = p as i64;
                assert_eq!(
                    a.reduce_mod(p_i64),
                    ref_reduce_signed(a, p_i64),
                    "reduce_mod({a}, {p})"
                );
                assert_eq!(
                    a.add_mod(b, p_i64),
                    ref_add_signed(a, b, p_i64),
                    "add_mod({a}, {b}, {p})"
                );
                assert_eq!(
                    a.sub_mod(b, p_i64),
                    ref_sub_signed(a, b, p_i64),
                    "sub_mod({a}, {b}, {p})"
                );
                assert_eq!(
                    a.mul_mod(b, p_i64),
                    ref_mul_signed(a, b, p_i64),
                    "mul_mod({a}, {b}, {p})"
                );
                assert_eq!(
                    a.add_mod(a.neg_mod(p_i64), p_i64),
                    0,
                    "neg_mod({a}, {p}): {a} + neg({a}) != 0"
                );
            }
        }
    }

    #[test]
    fn test_pow_mod_random_u64() {
        let mut rng = get_test_rng();
        for _ in 0..500 {
            let base: u64 = rng.random();
            let exp: u64 = rng.random_range(0..100);
            for &p in &PRIMES {
                assert_eq!(
                    base.pow_mod(exp, p),
                    ref_pow(base, exp, p),
                    "pow_mod({base}, {exp}, {p})"
                );
            }
        }
    }

    #[test]
    fn test_pow_mod_random_i64() {
        let mut rng = get_test_rng();
        for _ in 0..500 {
            let base: i64 = rng.random();
            let exp: u64 = rng.random_range(0..100);
            for &p in &PRIMES {
                let p_i64 = p as i64;
                assert_eq!(
                    base.pow_mod(exp, p_i64),
                    ref_pow_signed(base, exp, p_i64),
                    "pow_mod({base}, {exp}, {p})"
                );
            }
        }
    }

    #[test]
    fn test_div_inv_mod_random_u64() {
        let mut rng = get_test_rng();
        for _ in 0..500 {
            let a: u64 = rng.random();
            let b: u64 = rng.random();
            for &p in &PRIMES {
                if a % p == 0 || b % p == 0 {
                    continue;
                }
                assert_eq!(
                    a.mul_mod(a.inv_mod(p), p),
                    1,
                    "inv_mod({a}, {p}): {a} * inv({a}) != 1"
                );
                let result = a.div_mod(b, p);
                assert_eq!(
                    result.mul_mod(b, p),
                    a.reduce_mod(p),
                    "div_mod({a}, {b}, {p}): {result} * {b} != {a} mod {p}"
                );
            }
        }
    }

    #[test]
    fn test_div_inv_mod_random_i64() {
        let mut rng = get_test_rng();
        for _ in 0..500 {
            let a: i64 = rng.random();
            let b: i64 = rng.random();
            for &p in &PRIMES {
                let p_i64 = p as i64;
                if a.reduce_mod(p_i64) == 0 || b.reduce_mod(p_i64) == 0 {
                    continue;
                }
                assert_eq!(
                    a.mul_mod(a.inv_mod(p_i64), p_i64),
                    1,
                    "inv_mod({a}, {p}): {a} * inv({a}) != 1"
                );
                let result = a.div_mod(b, p_i64);
                assert_eq!(
                    result.mul_mod(b, p_i64),
                    a.reduce_mod(p_i64),
                    "div_mod({a}, {b}, {p}): {result} * {b} != {a} mod {p}"
                );
            }
        }
    }

    // ========== オーバーフロー境界テスト ==========

    #[test]
    fn test_u8_overflow_exhaustive() {
        let p: u8 = 251; // u8最大の素数
        for a in 0..p {
            for b in 0..p {
                let (a_w, b_w, p_w) = (a as u16, b as u16, p as u16);
                assert_eq!(
                    a.add_mod(b, p),
                    ((a_w + b_w) % p_w) as u8,
                    "u8: {a} + {b} mod {p}"
                );
                assert_eq!(
                    a.sub_mod(b, p),
                    ((p_w + a_w - b_w) % p_w) as u8,
                    "u8: {a} - {b} mod {p}"
                );
                assert_eq!(
                    a.mul_mod(b, p),
                    ((a_w * b_w) % p_w) as u8,
                    "u8: {a} * {b} mod {p}"
                );
            }
        }
    }

    #[test]
    fn test_u8_pow_overflow_exhaustive() {
        let p: u8 = 251;
        for base in 0..p {
            for exp in 0..=10u64 {
                let expected = ref_pow(base as u64, exp, p as u64) as u8;
                assert_eq!(base.pow_mod(exp, p), expected, "u8: {base}^{exp} mod {p}");
            }
        }
    }

    #[test]
    fn test_i8_overflow_exhaustive() {
        let p: i8 = 127; // i8最大の素数(メルセンヌ素数)
        for a in 0..p {
            for b in 0..p {
                let (a_w, b_w, p_w) = (a as i16, b as i16, p as i16);
                assert_eq!(
                    a.add_mod(b, p),
                    ((a_w + b_w) % p_w) as i8,
                    "i8: {a} + {b} mod {p}"
                );
                assert_eq!(
                    a.sub_mod(b, p),
                    ((p_w + a_w - b_w) % p_w) as i8,
                    "i8: {a} - {b} mod {p}"
                );
                assert_eq!(
                    a.mul_mod(b, p),
                    ((a_w * b_w) % p_w) as i8,
                    "i8: {a} * {b} mod {p}"
                );
            }
        }
    }

    #[test]
    fn test_i8_pow_overflow_exhaustive() {
        let p: i8 = 127;
        for base in 0..p {
            for exp in 0..=10u64 {
                let expected = ref_pow_signed(base as i64, exp, p as i64) as i8;
                assert_eq!(base.pow_mod(exp, p), expected, "i8: {base}^{exp} mod {p}");
            }
        }
    }

    #[test]
    fn test_u16_overflow_boundary() {
        let p: u16 = 65521; // u16最大の素数
        let max = p - 1;
        assert_eq!(max.add_mod(max, p), p - 2, "u16: (p-1) + (p-1) mod p");
        assert_eq!(max.sub_mod(max, p), 0, "u16: (p-1) - (p-1) mod p");
        assert_eq!(max.mul_mod(max, p), 1, "u16: (p-1)^2 mod p");
        assert_eq!(
            2u16.pow_mod(p as u64 - 1, p),
            1,
            "u16: Fermat 2^(p-1) mod p"
        );
    }

    #[test]
    fn test_u32_overflow_boundary() {
        let p: u32 = 4_294_967_291; // u32最大の素数
        let max = p - 1;
        assert_eq!(max.add_mod(max, p), p - 2, "u32: (p-1) + (p-1) mod p");
        assert_eq!(max.sub_mod(max, p), 0, "u32: (p-1) - (p-1) mod p");
        assert_eq!(max.mul_mod(max, p), 1, "u32: (p-1)^2 mod p");
        assert_eq!(
            2u32.pow_mod(p as u64 - 1, p),
            1,
            "u32: Fermat 2^(p-1) mod p"
        );
    }

    #[test]
    fn test_u64_overflow_boundary() {
        // 2^61 - 1 (メルセンヌ素数 M61)
        let p: u64 = (1u64 << 61) - 1;
        let max = p - 1;
        assert_eq!(max.add_mod(max, p), p - 2, "u64: (p-1) + (p-1) mod p");
        assert_eq!(max.sub_mod(max, p), 0, "u64: (p-1) - (p-1) mod p");
        assert_eq!(max.mul_mod(max, p), 1, "u64: (p-1)^2 mod p");
        assert_eq!(2u64.pow_mod(p - 1, p), 1, "u64: Fermat 2^(p-1) mod p");
    }

    #[test]
    fn test_i64_overflow_boundary() {
        // 2^61 - 1 (メルセンヌ素数 M61)
        let p: i64 = (1i64 << 61) - 1;
        let max = p - 1;
        assert_eq!(max.add_mod(max, p), p - 2, "i64: (p-1) + (p-1) mod p");
        assert_eq!(max.sub_mod(max, p), 0, "i64: (p-1) - (p-1) mod p");
        assert_eq!(max.mul_mod(max, p), 1, "i64: (p-1)^2 mod p");
        assert_eq!(
            2i64.pow_mod((p - 1) as u64, p),
            1,
            "i64: Fermat 2^(p-1) mod p"
        );
    }

    #[test]
    fn test_u128_overflow_boundary() {
        // 2^127 - 1 (メルセンヌ素数 M127)
        let p: u128 = (1u128 << 127) - 1;
        let max = p - 1;
        assert_eq!(max.add_mod(max, p), p - 2, "u128: (p-1) + (p-1) mod p");
        assert_eq!(max.sub_mod(max, p), 0, "u128: (p-1) - (p-1) mod p");
        // (p-1) ≡ -1 (mod p) なので (p-1)^2 ≡ 1
        assert_eq!(max.mul_mod(max, p), 1, "u128: (p-1)^2 mod p");
        // 2^127 = p + 1 ≡ 1 (mod p)
        assert_eq!(2u128.pow_mod(127, p), 1, "u128: 2^127 mod (2^127 - 1)");
    }

    #[test]
    fn test_i128_overflow_boundary() {
        // i128::MAX = 2^127 - 1 (メルセンヌ素数 M127)
        let p: i128 = i128::MAX;
        let max = p - 1;
        assert_eq!(max.add_mod(max, p), p - 2, "i128: (p-1) + (p-1) mod p");
        assert_eq!(max.sub_mod(max, p), 0, "i128: (p-1) - (p-1) mod p");
        assert_eq!(max.mul_mod(max, p), 1, "i128: (p-1)^2 mod p");
        assert_eq!(2i128.pow_mod(127, p), 1, "i128: 2^127 mod (2^127 - 1)");
    }

    #[test]
    fn test_usize_overflow_boundary() {
        // p は奇数であり，(p-1)^2 ≡ 1 (mod p) が成り立つ．
        let p: usize = usize::MAX;
        let max = p - 1;
        assert_eq!(max.add_mod(max, p), p - 2, "usize: (p-1) + (p-1) mod p");
        assert_eq!(max.sub_mod(max, p), 0, "usize: (p-1) - (p-1) mod p");
        assert_eq!(max.mul_mod(max, p), 1, "usize: (p-1)^2 mod p");
    }

    #[test]
    fn test_isize_overflow_boundary() {
        // p は奇数であり，(p-1)^2 ≡ 1 (mod p) が成り立つ．
        let p: isize = isize::MAX;
        let max = p - 1;
        assert_eq!(max.add_mod(max, p), p - 2, "isize: (p-1) + (p-1) mod p");
        assert_eq!(max.sub_mod(max, p), 0, "isize: (p-1) - (p-1) mod p");
        assert_eq!(max.mul_mod(max, p), 1, "isize: (p-1)^2 mod p");
    }

    #[test]
    fn test_inv_mod_u128_overflow_cast() {
        // p > i128::MAX のケース: 旧実装では i128 キャストでオーバーフロー
        let p: u128 = (1u128 << 127) + 1;
        let a: u128 = 1u128 << 127;
        // 2^127 ≡ -1 (mod 2^127+1) なので inv(-1) = -1 = 2^127
        assert_eq!(a.inv_mod(p), a, "inv(2^127) mod (2^127+1)");
        assert_eq!(
            a.mul_mod(a.inv_mod(p), p),
            1,
            "2^127 * inv(2^127) mod (2^127+1)"
        );

        // p - 1 のケース
        let b = p - 1;
        assert_eq!(b.inv_mod(p), b, "inv(p-1) mod p");

        // さらに大きい p
        let p2: u128 = (1u128 << 127) + 63;
        let a2: u128 = (1u128 << 127) + 1;
        assert_eq!(
            a2.mul_mod(a2.inv_mod(p2), p2),
            1,
            "u128 large: a * inv(a) ≡ 1"
        );
    }

    #[test]
    fn test_inv_mod_u128_random_large_p() {
        let mut rng = get_test_rng();
        let p: u128 = (1u128 << 127) - 1; // メルセンヌ素数 M127
        for _ in 0..100 {
            let a: u128 = rng.random::<u128>() % (p - 1) + 1;
            assert_eq!(
                a.mul_mod(a.inv_mod(p), p),
                1,
                "u128: {a} * inv({a}) mod M127"
            );
        }
    }
}
