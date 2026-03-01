//! 剰余演算(Modular Arithmetic)
//!
//! プリミティブ整数型向けの剰余演算を提供する`ModOps`トレイトと，
//! コンパイル時定数`P`を法とするmod int型`ModInt<P>`の2つを提供する．
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
            let mut u: $ty = 1;
            let mut v: $ty = 0;
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
/// `add_mod`/`sub_mod` は条件分岐で補正し，
/// `mul_mod` は double-and-add 方式で `add_mod` を利用する．
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
    use super::*;

    // ========== 共通ヘルパ関数(unsigned) ==========

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

    fn ref_neg(a: u64, p: u64) -> u64 {
        let (a, p) = (a as u128, p as u128);
        ((p - a % p) % p) as u64
    }

    mod mod_ops {
        use rand::Rng;

        use super::*;
        use crate::utils::test_utils::random::get_test_rng;

        // ========== 定数 ==========

        const PRIMES: [u64; 3] = [998_244_353, 1_000_000_007, 2_147_483_647];
        const SMALL_PRIMES: [u64; 5] = [2, 3, 5, 7, 11];

        // ========== ヘルパ関数 ==========

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

    mod modint {
        use super::*;

        use crate::utils::test_utils::random::get_test_rng;
        use rand::Rng;

        const P1: u64 = 998_244_353;
        const P2: u64 = 1_000_000_007;
        const P3: u64 = 2_147_483_647;

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
}
