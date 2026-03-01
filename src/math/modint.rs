//! ModInt(コンパイル時定数mod)
//!
//! コンパイル時定数`P`を法とするmod int型`ModInt<P>`を提供する．
//!
//! # Examples
//!
//! ```
//! use reprol::math::modint::ModInt998244353;
//!
//! let a = ModInt998244353::new(3);
//! let b = ModInt998244353::new(5);
//!
//! // 四則演算
//! assert_eq!((a + b).inner(), 8);
//! assert_eq!((a - b).inner(), 998244351);
//! assert_eq!((a * b).inner(), 15);
//! assert_eq!((a / b).inner(), 798595483);
//!
//! // 累乗
//! assert_eq!(a.pow(10).inner(), 59049);
//!
//! // 逆元
//! assert_eq!((a * a.inv()).inner(), 1);
//! ```

use std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// コンパイル時定数`P`を法とするmod int型．
///
/// # Panics
///
/// `P`は`0 < P <= u32::MAX`を満たす必要がある．
/// 違反した場合，生成時にpanicする．
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ModInt<const P: u64> {
    inner: u64,
}

impl<const P: u64> ModInt<P> {
    /// 値を`[0, P)`に正規化して生成する．
    pub const fn new(value: u64) -> Self {
        assert!(0 < P && P <= u32::MAX as u64);
        Self { inner: value % P }
    }

    /// 内部値を返す．
    pub const fn inner(&self) -> u64 {
        self.inner
    }

    /// 累乗．
    ///
    /// 繰り返し二乗法による実装．
    pub const fn pow(&self, mut exp: u64) -> Self {
        if P == 1 {
            return Self { inner: 0 };
        }

        let mut result = 1;
        let mut base = self.inner;

        while exp > 0 {
            if exp & 1 == 1 {
                result = result * base % P;
            }
            base = base * base % P;
            exp >>= 1;
        }

        Self { inner: result }
    }

    /// 乗法逆元．
    ///
    /// 拡張ユークリッド互除法による実装．
    ///
    /// # Panics
    ///
    /// - `self.inner == 0` のとき．
    /// - `gcd(self.inner, P) != 1` のとき．
    pub const fn inv(&self) -> Self {
        assert!(self.inner != 0);
        let mut a = self.inner;
        let mut b = P;
        let mut u = 1;
        let mut v = 0;

        while b > 0 {
            let q = a / b;
            let r = a % b;
            let nv = (P + u - q * v % P) % P; //u-qv
            (a, b, u, v) = (b, r, v, nv);
        }

        assert!(a == 1);
        Self { inner: u }
    }
}

impl<const P: u64> Default for ModInt<P> {
    fn default() -> Self {
        Self::new(0)
    }
}

impl<const P: u64> Add for ModInt<P> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            inner: (self.inner + rhs.inner) % P,
        }
    }
}

impl<const P: u64> AddAssign for ModInt<P> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const P: u64> Sub for ModInt<P> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            inner: (P + self.inner - rhs.inner) % P,
        }
    }
}

impl<const P: u64> SubAssign for ModInt<P> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const P: u64> Mul for ModInt<P> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            inner: self.inner * rhs.inner % P,
        }
    }
}

impl<const P: u64> MulAssign for ModInt<P> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const P: u64> Div for ModInt<P> {
    type Output = Self;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl<const P: u64> DivAssign for ModInt<P> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<const P: u64> Neg for ModInt<P> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            inner: (P - self.inner) % P,
        }
    }
}

impl<const P: u64> Sum for ModInt<P> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(0), |acc, x| acc + x)
    }
}

impl<'a, const P: u64> Sum<&'a Self> for ModInt<P> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl<const P: u64> Product for ModInt<P> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(1), |acc, x| acc * x)
    }
}

impl<'a, const P: u64> Product<&'a Self> for ModInt<P> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}

impl<const P: u64> Hash for ModInt<P> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state)
    }
}

impl<const P: u64> Debug for ModInt<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<const P: u64> Display for ModInt<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

macro_rules! impl_modint_from_unsigned {
    ($ty:ty) => {
        impl<const P: u64> From<$ty> for ModInt<P> {
            fn from(value: $ty) -> Self {
                assert!(0 < P && P <= u32::MAX as u64);
                Self {
                    inner: (value as u128).rem_euclid(P as u128) as u64,
                }
            }
        }
    };
}

macro_rules! impl_modint_from_signed {
    ($ty: ty) => {
        impl<const P: u64> From<$ty> for ModInt<P> {
            fn from(value: $ty) -> Self {
                assert!(0 < P && P <= u32::MAX as u64);
                Self {
                    inner: (value as i128).rem_euclid(P as i128) as u64,
                }
            }
        }
    };
}

macro_rules! impl_modint_from {
    (unsigned: [$($u:ty),* $(,)?], signed: [$($i:ty),* $(,)?]$(,)?) => {
        $( impl_modint_from_unsigned!($u); )*
        $( impl_modint_from_signed!($i); )*
    };
}

impl_modint_from! {
    unsigned: [u8, u16, u32, u64, u128, usize],
    signed:   [i8, i16, i32, i64, i128, isize],
}

/// `P = 998244353`の`ModInt`型エイリアス．
pub type ModInt998244353 = ModInt<998244353>;

/// `P = 1000000007`の`ModInt`型エイリアス．
pub type ModInt1000000007 = ModInt<1000000007>;

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    use crate::utils::test_utils::random::get_test_rng;

    // ========== 定数 ==========

    const P1: u64 = 998_244_353;
    const P2: u64 = 1_000_000_007;
    const P3: u64 = 2_147_483_647;

    // ========== ヘルパ関数 ==========

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

    fn ref_neg(a: u64, p: u64) -> u64 {
        let (a, p) = (a as u128, p as u128);
        ((p - a % p) % p) as u64
    }

    // ========== コンストラクタ・アクセサ ==========

    #[test]
    fn test_new_inner() {
        fn check<const P: u64>() {
            for v in [0, 1, P / 2, P - 1] {
                assert_eq!(ModInt::<P>::new(v).inner(), v, "P={P}, v={v}");
            }
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_new_normalization() {
        fn check<const P: u64>() {
            assert_eq!(ModInt::<P>::new(P).inner(), 0, "P={P}, v=P");
            assert_eq!(ModInt::<P>::new(2 * P).inner(), 0, "P={P}, v=2P");
            assert_eq!(ModInt::<P>::new(3 * P + 1).inner(), 1, "P={P}, v=3P+1");
            let max = u64::MAX;
            assert_eq!(ModInt::<P>::new(max).inner(), max % P, "P={P}, v=u64::MAX");
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_default() {
        fn check<const P: u64>() {
            assert_eq!(ModInt::<P>::default().inner(), 0, "P={P}");
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_default_boundary() {
        assert_eq!(ModInt::<1>::default().inner(), 0, "P=1");
        assert_eq!(ModInt::<2>::default().inner(), 0, "P=2");
        assert_eq!(
            ModInt::<{ u32::MAX as u64 }>::default().inner(),
            0,
            "P=u32::MAX"
        );
    }

    #[test]
    #[should_panic]
    fn test_default_panic_p_zero() {
        let _ = ModInt::<0>::default();
    }

    #[test]
    #[should_panic]
    fn test_default_panic_p_exceeds_u32_max() {
        let _ = ModInt::<{ u32::MAX as u64 + 1 }>::default();
    }

    // ========== From変換 ==========

    #[test]
    fn test_from_unsigned_smoke() {
        fn check<const P: u64>() {
            assert_eq!(ModInt::<P>::from(10u8).inner(), 10 % P);
            assert_eq!(ModInt::<P>::from(10u16).inner(), 10 % P);
            assert_eq!(ModInt::<P>::from(10u32).inner(), 10 % P);
            assert_eq!(ModInt::<P>::from(10u64).inner(), 10 % P);
            assert_eq!(ModInt::<P>::from(10u128).inner(), 10 % P);
            assert_eq!(ModInt::<P>::from(10usize).inner(), 10 % P);
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_from_signed_smoke() {
        fn check<const P: u64>() {
            assert_eq!(ModInt::<P>::from(10i8).inner(), 10 % P);
            assert_eq!(ModInt::<P>::from(10i16).inner(), 10 % P);
            assert_eq!(ModInt::<P>::from(10i32).inner(), 10 % P);
            assert_eq!(ModInt::<P>::from(10i64).inner(), 10 % P);
            assert_eq!(ModInt::<P>::from(10i128).inner(), 10 % P);
            assert_eq!(ModInt::<P>::from(10isize).inner(), 10 % P);
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_from_negative() {
        fn check<const P: u64>() {
            let p128 = P as i128;
            assert_eq!(
                ModInt::<P>::from(-1i64).inner(),
                (-1i128).rem_euclid(p128) as u64
            );
            assert_eq!(
                ModInt::<P>::from(-(P as i64)).inner(),
                (-(P as i128)).rem_euclid(p128) as u64
            );
            assert_eq!(
                ModInt::<P>::from(i64::MIN).inner(),
                (i64::MIN as i128).rem_euclid(p128) as u64
            );
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_from_large() {
        fn check<const P: u64>() {
            assert_eq!(
                ModInt::<P>::from(u128::MAX).inner(),
                (u128::MAX % P as u128) as u64
            );
            assert_eq!(ModInt::<P>::from(u64::MAX).inner(), u64::MAX % P);
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    // ========== 四則演算(基本) ==========

    #[test]
    fn test_add_basic() {
        fn check<const P: u64>() {
            let a = ModInt::<P>::new(3);
            let b = ModInt::<P>::new(5);
            assert_eq!((a + b).inner(), 8, "{a} + {b} mod {P}");

            let c = ModInt::<P>::new(P - 1);
            let d = ModInt::<P>::new(2);
            assert_eq!((c + d).inner(), 1, "{c} + {d} mod {P}");
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_sub_basic() {
        fn check<const P: u64>() {
            let a = ModInt::<P>::new(5);
            let b = ModInt::<P>::new(3);
            assert_eq!((a - b).inner(), 2, "{a} - {b} mod {P}");

            let c = ModInt::<P>::new(3);
            let d = ModInt::<P>::new(5);
            assert_eq!((c - d).inner(), P - 2, "{c} - {d} mod {P}");
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_mul_basic() {
        fn check<const P: u64>() {
            let a = ModInt::<P>::new(3);
            let b = ModInt::<P>::new(5);
            assert_eq!((a * b).inner(), 15, "{a} * {b} mod {P}");

            let c = ModInt::<P>::new(P - 1);
            let d = ModInt::<P>::new(P - 1);
            assert_eq!(
                (c * d).inner(),
                ref_mul(P - 1, P - 1, P),
                "{c} * {d} mod {P}"
            );
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_div_basic() {
        fn check<const P: u64>() {
            let a = ModInt::<P>::new(15);
            let b = ModInt::<P>::new(5);
            let result = a / b;
            assert_eq!((result * b).inner(), a.inner(), "{a} / {b} mod {P}");

            let c = ModInt::<P>::new(1);
            let d = ModInt::<P>::new(3);
            let result2 = c / d;
            assert_eq!((result2 * d).inner(), c.inner(), "{c} / {d} mod {P}");
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    // ========== 複合代入演算 ==========

    #[test]
    fn test_add_assign() {
        fn check<const P: u64>(a: u64, b: u64) {
            let mut x = ModInt::<P>::new(a);
            let y = ModInt::<P>::new(b);
            let expected = x + y;
            x += y;
            assert_eq!(x, expected);
        }
        for (a, b) in [(0, 0), (1, 2), (1030, 7777)] {
            check::<P1>(a, b);
            check::<P2>(a, b);
            check::<P3>(a, b);
        }
    }

    #[test]
    fn test_sub_assign() {
        fn check<const P: u64>(a: u64, b: u64) {
            let mut x = ModInt::<P>::new(a);
            let y = ModInt::<P>::new(b);
            let expected = x - y;
            x -= y;
            assert_eq!(x, expected);
        }
        for (a, b) in [(0, 0), (5, 3), (3, 5), (1030, 7777)] {
            check::<P1>(a, b);
            check::<P2>(a, b);
            check::<P3>(a, b);
        }
    }

    #[test]
    fn test_mul_assign() {
        fn check<const P: u64>(a: u64, b: u64) {
            let mut x = ModInt::<P>::new(a);
            let y = ModInt::<P>::new(b);
            let expected = x * y;
            x *= y;
            assert_eq!(x, expected);
        }
        for (a, b) in [(0, 1), (3, 5), (1030, 7777)] {
            check::<P1>(a, b);
            check::<P2>(a, b);
            check::<P3>(a, b);
        }
    }

    #[test]
    fn test_div_assign() {
        fn check<const P: u64>(a: u64, b: u64) {
            let mut x = ModInt::<P>::new(a);
            let y = ModInt::<P>::new(b);
            let expected = x / y;
            x /= y;
            assert_eq!(x, expected);
        }
        for (a, b) in [(0, 1), (15, 5), (1030, 7777)] {
            check::<P1>(a, b);
            check::<P2>(a, b);
            check::<P3>(a, b);
        }
    }

    // ========== 単項演算・累乗・逆元(基本) ==========

    #[test]
    fn test_neg_basic() {
        fn check<const P: u64>() {
            let a = ModInt::<P>::new(3);
            assert_eq!((-a).inner(), P - 3);
            assert_eq!((-a + a).inner(), 0);
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_pow_basic() {
        fn check<const P: u64>() {
            assert_eq!(
                ModInt::<P>::new(2).pow(10).inner(),
                1024 % P,
                "2^10 mod {P}"
            );
            assert_eq!(ModInt::<P>::new(3).pow(4).inner(), 81 % P, "3^4 mod {P}");
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_inv_basic() {
        fn check<const P: u64>() {
            for a in [1u64, 2, 3, P / 2, P - 1] {
                let m = ModInt::<P>::new(a);
                assert_eq!((m * m.inv()).inner(), 1, "{a}^-1 mod {P}");
            }
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_inv_composite_mod() {
        // P=8: gcd(a, 8)=1 となるのは a in {1,3,5,7}
        for a in [1u64, 3, 5, 7] {
            let m = ModInt::<8>::new(a);
            assert_eq!((m * m.inv()).inner(), 1, "{a}^-1 mod 8");
        }

        // P=15: gcd(a, 15)=1 となるのは a in {1,2,4,7,8,11,13,14}
        for a in [1u64, 2, 4, 7, 8, 11, 13, 14] {
            let m = ModInt::<15>::new(a);
            assert_eq!((m * m.inv()).inner(), 1, "{a}^-1 mod 15");
        }
    }

    #[test]
    fn test_div_composite_mod() {
        // P=8: 分母 b は gcd(b, 8)=1 のもののみ
        for a in 0u64..8 {
            for b in [1u64, 3, 5, 7] {
                let result = ModInt::<8>::new(a) / ModInt::<8>::new(b);
                assert_eq!(
                    (result * ModInt::<8>::new(b)).inner(),
                    a % 8,
                    "{a} / {b} mod 8"
                );
            }
        }

        // P=15: 分母 b は gcd(b, 15)=1 のもののみ
        for a in 0u64..15 {
            for b in [1u64, 2, 4, 7, 8, 11, 13, 14] {
                let result = ModInt::<15>::new(a) / ModInt::<15>::new(b);
                assert_eq!(
                    (result * ModInt::<15>::new(b)).inner(),
                    a % 15,
                    "{a} / {b} mod 15"
                );
            }
        }
    }

    // ========== エッジケース ==========

    #[test]
    fn test_add_wraparound() {
        fn check<const P: u64>() {
            let a = ModInt::<P>::new(P - 1);
            for k in [1u64, 2, P / 2, P - 1] {
                let b = ModInt::<P>::new(k);
                assert_eq!(
                    (a + b).inner(),
                    ref_add(P - 1, k, P),
                    "{} + {k} mod {P}",
                    P - 1
                );
            }
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_sub_wraparound() {
        fn check<const P: u64>() {
            for (a, b) in [(0u64, 1u64), (1, P - 1), (0, P - 1)] {
                assert_eq!(
                    (ModInt::<P>::new(a) - ModInt::<P>::new(b)).inner(),
                    ref_sub(a, b, P),
                    "{a} - {b} mod {P}"
                );
            }
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_neg_zero() {
        fn check<const P: u64>() {
            assert_eq!((-ModInt::<P>::new(0)).inner(), 0);
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_pow_exp_zero() {
        fn check<const P: u64>() {
            for a in [0u64, 1, 3, P / 2, P - 1] {
                assert_eq!(ModInt::<P>::new(a).pow(0).inner(), 1);
            }
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_pow_exp_one() {
        fn check<const P: u64>() {
            for a in [0u64, 1, 3, P / 2, P - 1] {
                assert_eq!(ModInt::<P>::new(a).pow(1).inner(), a % P);
            }
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_pow_base_zero() {
        fn check<const P: u64>() {
            assert_eq!(ModInt::<P>::new(0).pow(0).inner(), 1);
            for n in [1u64, 2, 10, 100] {
                assert_eq!(ModInt::<P>::new(0).pow(n).inner(), 0);
            }
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_pow_base_one() {
        fn check<const P: u64>() {
            for n in [0u64, 1, 10, 100, 1_000_000] {
                assert_eq!(ModInt::<P>::new(1).pow(n).inner(), 1);
            }
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_pow_p_one() {
        let a = ModInt::<1>::new(0);
        let b = ModInt::<1>::new(0);
        assert_eq!(a.inner(), 0);
        assert_eq!((a + b).inner(), 0);
        assert_eq!((a - b).inner(), 0);
        assert_eq!((a * b).inner(), 0);
        assert_eq!((-a).inner(), 0);
        assert_eq!(a.pow(0).inner(), 0);
        assert_eq!(a.pow(10).inner(), 0);
    }

    #[test]
    fn test_inv_one() {
        fn check<const P: u64>() {
            assert_eq!(ModInt::<P>::new(1).inv().inner(), 1);
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_arithmetic_identity() {
        fn check<const P: u64>() {
            for v in [0u64, 1, P / 2, P - 1] {
                let a = ModInt::<P>::new(v);
                let zero = ModInt::<P>::new(0);
                let one = ModInt::<P>::new(1);
                assert_eq!((a + zero).inner(), a.inner());
                assert_eq!((a * one).inner(), a.inner());
                assert_eq!((a - a).inner(), 0);
            }
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    // ========== 代数的性質 ==========

    #[test]
    fn test_add_commutativity() {
        fn check<const P: u64>(a: u64, b: u64) {
            let ma = ModInt::<P>::new(a);
            let mb = ModInt::<P>::new(b);
            assert_eq!(ma + mb, mb + ma);
        }
        for (a, b) in [(0, 0), (0, 1), (3, 5), (1030, 7777)] {
            check::<P1>(a, b);
            check::<P2>(a, b);
            check::<P3>(a, b);
        }
    }

    #[test]
    fn test_add_associativity() {
        fn check<const P: u64>(a: u64, b: u64, c: u64) {
            let (ma, mb, mc) = (
                ModInt::<P>::new(a),
                ModInt::<P>::new(b),
                ModInt::<P>::new(c),
            );
            assert_eq!((ma + mb) + mc, ma + (mb + mc));
        }
        for (a, b, c) in [(1, 2, 3), (1030, 7777, 42), (0, 0, 0)] {
            check::<P1>(a, b, c);
            check::<P2>(a, b, c);
            check::<P3>(a, b, c);
        }
    }

    #[test]
    fn test_mul_commutativity() {
        fn check<const P: u64>(a: u64, b: u64) {
            let ma = ModInt::<P>::new(a);
            let mb = ModInt::<P>::new(b);
            assert_eq!(ma * mb, mb * ma);
        }
        for (a, b) in [(0, 1), (2, 3), (1030, 7777)] {
            check::<P1>(a, b);
            check::<P2>(a, b);
            check::<P3>(a, b);
        }
    }

    #[test]
    fn test_mul_associativity() {
        fn check<const P: u64>(a: u64, b: u64, c: u64) {
            let (ma, mb, mc) = (
                ModInt::<P>::new(a),
                ModInt::<P>::new(b),
                ModInt::<P>::new(c),
            );
            assert_eq!((ma * mb) * mc, ma * (mb * mc));
        }
        for (a, b, c) in [(2, 3, 5), (1030, 7777, 42), (0, 1, 2)] {
            check::<P1>(a, b, c);
            check::<P2>(a, b, c);
            check::<P3>(a, b, c);
        }
    }

    #[test]
    fn test_distributivity() {
        fn check<const P: u64>(a: u64, b: u64, c: u64) {
            let (ma, mb, mc) = (
                ModInt::<P>::new(a),
                ModInt::<P>::new(b),
                ModInt::<P>::new(c),
            );
            assert_eq!(ma * (mb + mc), ma * mb + ma * mc);
        }
        for (a, b, c) in [(2, 3, 5), (1030, 7777, 42), (0, 1, 0)] {
            check::<P1>(a, b, c);
            check::<P2>(a, b, c);
            check::<P3>(a, b, c);
        }
    }

    #[test]
    fn test_neg_double_negation() {
        fn check<const P: u64>() {
            for v in [0u64, 1, P / 2, P - 1, 1030] {
                let a = ModInt::<P>::new(v);
                assert_eq!(-(-a), a);
            }
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_neg_additive_inverse() {
        fn check<const P: u64>() {
            for v in [0u64, 1, P / 2, P - 1, 1030] {
                let a = ModInt::<P>::new(v);
                assert_eq!((a + (-a)).inner(), 0);
            }
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_inv_multiplicative_inverse() {
        fn check<const P: u64>() {
            for v in [1u64, 2, P / 2, P - 1, 1030] {
                let a = ModInt::<P>::new(v);
                assert_eq!((a * a.inv()).inner(), 1);
            }
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_pow_additive_exponents() {
        fn check<const P: u64>() {
            for a in [2u64, 3, P - 1, 1030] {
                for (m, n) in [(1u64, 2u64), (3, 4), (0, 5), (10, 20)] {
                    let ma = ModInt::<P>::new(a);
                    assert_eq!(ma.pow(m + n), ma.pow(m) * ma.pow(n));
                }
            }
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_pow_fermats_little_theorem() {
        fn check<const P: u64>() {
            for a in [1u64, 2, 3, P / 2, P - 1, 1030] {
                if a % P != 0 {
                    assert_eq!(ModInt::<P>::new(a).pow(P - 1).inner(), 1);
                }
            }
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    // ========== イテレータ (Sum / Product) ==========

    #[test]
    fn test_sum() {
        fn check<const P: u64>() {
            let vals: Vec<ModInt<P>> = (1..=10).map(ModInt::<P>::new).collect();
            let expected = ModInt::<P>::new(55);

            let sum_owned: ModInt<P> = vals.clone().into_iter().sum();
            assert_eq!(sum_owned, expected);

            let sum_ref: ModInt<P> = vals.iter().sum();
            assert_eq!(sum_ref, expected);
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_sum_empty() {
        fn check<const P: u64>() {
            let sum: ModInt<P> = std::iter::empty::<ModInt<P>>().sum();
            assert_eq!(sum.inner(), 0);
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_product() {
        fn check<const P: u64>() {
            let vals: Vec<ModInt<P>> = (1..=6).map(ModInt::<P>::new).collect();
            let expected = ModInt::<P>::new(720);

            let prod_owned: ModInt<P> = vals.clone().into_iter().product();
            assert_eq!(prod_owned, expected);

            let prod_ref: ModInt<P> = vals.iter().product();
            assert_eq!(prod_ref, expected);
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_product_empty() {
        fn check<const P: u64>() {
            let prod: ModInt<P> = std::iter::empty::<ModInt<P>>().product();
            assert_eq!(prod.inner(), 1);
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    // ========== ハッシュ ==========

    #[test]
    fn test_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        fn compute_hash<T: Hash>(val: &T) -> u64 {
            let mut hasher = DefaultHasher::new();
            val.hash(&mut hasher);
            hasher.finish()
        }

        fn check<const P: u64>() {
            // 等しい値は同じハッシュを持つ
            let a = ModInt::<P>::new(1030);
            let b = ModInt::<P>::new(1030);
            assert_eq!(compute_hash(&a), compute_hash(&b));

            // P + v と v は正規化後等しい
            let c = ModInt::<P>::new(P + 42);
            let d = ModInt::<P>::new(42);
            assert_eq!(compute_hash(&c), compute_hash(&d));
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    #[test]
    fn test_eq() {
        fn check<const P: u64>() {
            assert_eq!(ModInt::<P>::new(1030), ModInt::<P>::new(1030));
            assert_eq!(ModInt::<P>::new(P + 42), ModInt::<P>::new(42));
            assert_ne!(ModInt::<P>::new(1), ModInt::<P>::new(2));
        }
        check::<P1>();
        check::<P2>();
        check::<P3>();
    }

    // ========== 条件網羅(panic) ==========

    #[test]
    #[should_panic]
    fn test_new_panic_p_zero() {
        let _ = ModInt::<0>::new(0);
    }

    #[test]
    #[should_panic]
    fn test_new_panic_p_exceeds_u32_max() {
        let _ = ModInt::<4_294_967_296>::new(0);
    }

    #[test]
    #[should_panic]
    fn test_inv_panic_zero() {
        let _ = ModInt::<P1>::new(0).inv();
    }

    #[test]
    #[should_panic]
    fn test_inv_panic_not_coprime() {
        // gcd(4, 6) == 2 != 1
        let _ = ModInt::<6>::new(4).inv();
    }

    #[test]
    #[should_panic]
    fn test_inv_panic_p_one() {
        let _ = ModInt::<1>::new(0).inv();
    }

    #[test]
    #[should_panic]
    fn test_div_panic_p_one() {
        let _ = ModInt::<1>::new(0) / ModInt::<1>::new(0);
    }

    // ========== 小さい入力での全探索 ==========

    #[test]
    fn test_arithmetic_exhaustive_small_p() {
        fn check<const P: u64>() {
            for a in 0..P {
                for b in 0..P {
                    assert_eq!(
                        (ModInt::<P>::new(a) + ModInt::<P>::new(b)).inner(),
                        ref_add(a, b, P),
                        "{a} + {b} mod {P}"
                    );
                    assert_eq!(
                        (ModInt::<P>::new(a) - ModInt::<P>::new(b)).inner(),
                        ref_sub(a, b, P),
                        "{a} - {b} mod {P}"
                    );
                    assert_eq!(
                        (ModInt::<P>::new(a) * ModInt::<P>::new(b)).inner(),
                        ref_mul(a, b, P),
                        "{a} * {b} mod {P}"
                    );
                }
            }
        }
        check::<2>();
        check::<3>();
        check::<5>();
        check::<7>();
    }

    #[test]
    fn test_pow_inv_div_exhaustive_small_p() {
        fn check<const P: u64>() {
            // pow: 全base × exp[0, 2P]
            for base in 0..P {
                for exp in 0..=2 * P {
                    assert_eq!(
                        ModInt::<P>::new(base).pow(exp).inner(),
                        ref_pow(base, exp, P),
                        "{base}^{exp} mod {P}"
                    );
                }
            }

            // inv: [1, P)
            for a in 1..P {
                let m = ModInt::<P>::new(a);
                assert_eq!((m * m.inv()).inner(), 1, "{a} * inv({a}) mod {P}");
            }

            // div: 全ペア (a, b) where b != 0
            for a in 0..P {
                for b in 1..P {
                    let result = ModInt::<P>::new(a) / ModInt::<P>::new(b);
                    assert_eq!(
                        (result * ModInt::<P>::new(b)).inner(),
                        a,
                        "{a} / {b} mod {P}"
                    );
                }
            }
        }
        check::<2>();
        check::<3>();
        check::<5>();
        check::<7>();
    }

    #[test]
    fn test_from_exhaustive_small_p() {
        fn check<const P: u64>() {
            for v in i8::MIN..=i8::MAX {
                let result = ModInt::<P>::from(v).inner();
                let expected = (v as i128).rem_euclid(P as i128) as u64;
                assert_eq!(result, expected, "from_i8({v}) mod {P}");
            }
        }
        check::<2>();
        check::<3>();
        check::<5>();
        check::<7>();
    }

    // ========== ランダムテスト ==========

    #[test]
    fn test_arithmetic_random() {
        fn check<const P: u64>(a: u64, b: u64) {
            assert_eq!(
                (ModInt::<P>::new(a) + ModInt::<P>::new(b)).inner(),
                ref_add(a, b, P),
                "{a} + {b} mod {P}"
            );
            assert_eq!(
                (ModInt::<P>::new(a) - ModInt::<P>::new(b)).inner(),
                ref_sub(a, b, P),
                "{a} - {b} mod {P}"
            );
            assert_eq!(
                (ModInt::<P>::new(a) * ModInt::<P>::new(b)).inner(),
                ref_mul(a, b, P),
                "{a} * {b} mod {P}"
            );
            assert_eq!(
                (-ModInt::<P>::new(a)).inner(),
                ref_neg(a, P),
                "-{a} mod {P}"
            );
        }
        let mut rng = get_test_rng();
        for _ in 0..1000 {
            let a: u64 = rng.random();
            let b: u64 = rng.random();
            check::<P1>(a, b);
            check::<P2>(a, b);
            check::<P3>(a, b);
        }
    }

    #[test]
    fn test_pow_random() {
        fn check<const P: u64>(base: u64, exp: u64) {
            assert_eq!(
                ModInt::<P>::new(base).pow(exp).inner(),
                ref_pow(base, exp, P),
                "{base}^{exp} mod {P}"
            );
        }
        let mut rng = get_test_rng();
        for _ in 0..1000 {
            let base: u64 = rng.random();
            let exp: u64 = rng.random_range(0..100);
            check::<P1>(base, exp);
            check::<P2>(base, exp);
            check::<P3>(base, exp);
        }
    }

    // ========== オーバーフロー境界テスト ==========

    #[test]
    fn test_new_boundary_u32_max() {
        // P = u32::MAX (非素数: 3 × 5 × 17 × 257 × 65537)
        const P: u64 = u32::MAX as u64;
        assert_eq!(ModInt::<P>::new(0).inner(), 0);
        assert_eq!(ModInt::<P>::new(1).inner(), 1);
        assert_eq!(ModInt::<P>::new(P - 1).inner(), P - 1);
        assert_eq!(ModInt::<P>::new(P).inner(), 0, "P mod P");
        assert_eq!(ModInt::<P>::new(P + 1).inner(), 1, "P+1 mod P");
        assert_eq!(
            ModInt::<P>::new(u64::MAX).inner(),
            u64::MAX % P,
            "u64::MAX mod P"
        );
    }

    #[test]
    fn test_add_boundary_u32_max() {
        const P: u64 = u32::MAX as u64;
        let max = ModInt::<P>::new(P - 1);
        let one = ModInt::<P>::new(1);
        let zero = ModInt::<P>::new(0);
        assert_eq!((max + one).inner(), 0, "(P-1) + 1 mod P");
        assert_eq!((max + max).inner(), P - 2, "(P-1) + (P-1) mod P");
        assert_eq!((max + zero).inner(), P - 1, "(P-1) + 0 mod P");
    }

    #[test]
    fn test_sub_boundary_u32_max() {
        const P: u64 = u32::MAX as u64;
        let max = ModInt::<P>::new(P - 1);
        let one = ModInt::<P>::new(1);
        let zero = ModInt::<P>::new(0);
        assert_eq!((zero - one).inner(), P - 1, "0 - 1 mod P");
        assert_eq!((zero - max).inner(), 1, "0 - (P-1) mod P");
        assert_eq!((max - max).inner(), 0, "(P-1) - (P-1) mod P");
        assert_eq!((one - max).inner(), 2, "1 - (P-1) mod P");
    }

    #[test]
    fn test_mul_boundary_u32_max() {
        const P: u64 = u32::MAX as u64;
        let max = ModInt::<P>::new(P - 1);
        // (P-1)^2 mod P = 1
        assert_eq!(
            (max * max).inner(),
            ref_mul(P - 1, P - 1, P),
            "(P-1)^2 mod P"
        );
        assert_eq!((max * max).inner(), 1, "(P-1)^2 ≡ 1 mod P");

        let half = ModInt::<P>::new(P / 2);
        assert_eq!(
            (half * ModInt::<P>::new(2)).inner(),
            ref_mul(P / 2, 2, P),
            "(P/2) * 2 mod P"
        );
    }

    #[test]
    fn test_neg_boundary_u32_max() {
        const P: u64 = u32::MAX as u64;
        assert_eq!((-ModInt::<P>::new(0)).inner(), 0, "-0 mod P");
        assert_eq!((-ModInt::<P>::new(1)).inner(), P - 1, "-1 mod P");
        assert_eq!((-ModInt::<P>::new(P - 1)).inner(), 1, "-(P-1) mod P");
    }

    #[test]
    fn test_pow_boundary_u32_max() {
        const P: u64 = u32::MAX as u64;
        let base = ModInt::<P>::new(2);
        assert_eq!(base.pow(32).inner(), ref_pow(2, 32, P), "2^32 mod P");

        let max = ModInt::<P>::new(P - 1);
        assert_eq!(max.pow(2).inner(), 1, "(P-1)^2 mod P");
        assert_eq!(max.pow(3).inner(), P - 1, "(P-1)^3 mod P");

        assert_eq!(ModInt::<P>::new(0).pow(0).inner(), 1, "0^0 mod P");
        assert_eq!(ModInt::<P>::new(0).pow(1).inner(), 0, "0^1 mod P");
    }

    #[test]
    fn test_from_boundary_u32_max() {
        const P: u64 = u32::MAX as u64;
        assert_eq!(ModInt::<P>::from(u32::MAX).inner(), 0, "from(u32::MAX)");
        assert_eq!(
            ModInt::<P>::from(u64::MAX).inner(),
            u64::MAX % P,
            "from(u64::MAX)"
        );
        assert_eq!(
            ModInt::<P>::from(u128::MAX).inner(),
            (u128::MAX % P as u128) as u64,
            "from(u128::MAX)"
        );
        assert_eq!(
            ModInt::<P>::from(-1i64).inner(),
            (-1i128).rem_euclid(P as i128) as u64,
            "from(-1i64)"
        );
        assert_eq!(
            ModInt::<P>::from(i64::MIN).inner(),
            (i64::MIN as i128).rem_euclid(P as i128) as u64,
            "from(i64::MIN)"
        );
    }

    #[test]
    fn test_arithmetic_boundary_largest_u32_prime() {
        // 4294967291: u32に収まる最大の素数
        const P: u64 = 4_294_967_291;
        let max = ModInt::<P>::new(P - 1);
        let one = ModInt::<P>::new(1);

        // 加算
        assert_eq!((max + max).inner(), P - 2, "(P-1) + (P-1) mod P");
        assert_eq!((max + one).inner(), 0, "(P-1) + 1 mod P");

        // 減算
        assert_eq!((ModInt::<P>::new(0) - one).inner(), P - 1, "0 - 1 mod P");

        // 乗算
        assert_eq!((max * max).inner(), 1, "(P-1)^2 mod P");

        // 累乗: フェルマーの小定理 2^(P-1) ≡ 1 (mod P)
        assert_eq!(
            ModInt::<P>::new(2).pow(P - 1).inner(),
            1,
            "Fermat: 2^(P-1) mod P"
        );

        // 逆元
        for a in [1u64, 2, P / 2, P - 1, 1030] {
            let m = ModInt::<P>::new(a);
            assert_eq!((m * m.inv()).inner(), 1, "{a} * inv({a}) mod P={P}");
        }

        // 除算
        let a = ModInt::<P>::new(P - 1);
        let b = ModInt::<P>::new(P - 2);
        let result = a / b;
        assert_eq!((result * b).inner(), a.inner(), "(P-1) / (P-2) mod P");
    }

    #[test]
    fn test_pow_boundary_largest_u32_prime() {
        const P: u64 = 4_294_967_291;
        // 大きな指数
        assert_eq!(
            ModInt::<P>::new(2).pow(64).inner(),
            ref_pow(2, 64, P),
            "2^64 mod P"
        );
        assert_eq!(
            ModInt::<P>::new(P - 1).pow(P - 1).inner(),
            1,
            "(P-1)^(P-1) mod P (Fermat)"
        );
    }

    // ========== ランダムテスト ==========

    #[test]
    fn test_div_inv_random() {
        fn check<const P: u64>(a: u64, b: u64) {
            let ma = ModInt::<P>::new(a);
            let mb = ModInt::<P>::new(b);
            let result = ma / mb;
            assert_eq!((result * mb).inner(), ma.inner(), "{a} / {b} mod {P}");
            let m = ModInt::<P>::new(a);
            assert_eq!((m * m.inv()).inner(), 1, "{a} * {a}^-1 mod {P}");
        }
        let mut rng = get_test_rng();
        for _ in 0..1000 {
            let a: u64 = rng.random();
            let b: u64 = rng.random();
            if a % P1 == 0
                || a % P2 == 0
                || a % P3 == 0
                || b % P1 == 0
                || b % P2 == 0
                || b % P3 == 0
            {
                continue;
            }
            check::<P1>(a, b);
            check::<P2>(a, b);
            check::<P3>(a, b);
        }
    }
}
