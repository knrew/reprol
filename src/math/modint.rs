use std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

pub trait ModOp {
    fn add_mod(self, rhs: Self, p: Self) -> Self;
    fn sub_mod(self, rhs: Self, p: Self) -> Self;
    fn mul_mod(self, rhs: Self, p: Self) -> Self;
    fn div_mod(self, rhs: Self, p: Self) -> Self;
    fn neg_mod(self, p: Self) -> Self;
    fn pow_mod(self, exp: u64, p: Self) -> Self;
    fn inv_mod(self, p: Self) -> Self;
}

macro_rules! impl_modop {
    ($ty: ty) => {
        impl ModOp for $ty {
            fn add_mod(self, rhs: Self, p: Self) -> Self {
                assert!(p > 0);
                (self % p + rhs % p).rem_euclid(p)
            }

            fn sub_mod(self, rhs: Self, p: Self) -> Self {
                assert!(p > 0);
                (p + self % p - rhs % p).rem_euclid(p)
            }

            fn mul_mod(self, rhs: Self, p: Self) -> Self {
                assert!(p > 0);
                (self % p * (rhs % p)).rem_euclid(p)
            }

            fn div_mod(self, rhs: Self, p: Self) -> Self {
                self.mul_mod(rhs.inv_mod(p), p)
            }

            fn neg_mod(self, p: Self) -> Self {
                assert!(p > 0);
                (p - self.rem_euclid(p)) % p
            }

            fn pow_mod(self, mut exp: u64, p: Self) -> Self {
                assert!(p > 0);

                if p == 1 {
                    return 0;
                }

                let mut res = 1;
                let mut base = self.rem_euclid(p);

                while exp > 0 {
                    if exp % 2 == 1 {
                        res = res * base % p;
                    }
                    base = base * base % p;
                    exp /= 2;
                }

                res
            }

            fn inv_mod(self, p: Self) -> Self {
                assert!(p > 0);
                let mut a = self.rem_euclid(p) as i64;
                let mut b = p as i64;
                let mut u = 1;
                let mut v = 0;
                while b > 0 {
                    let q = a / b;
                    (a, b) = (b, a - q * b);
                    (u, v) = (v, u - q * v);
                }
                u.rem_euclid(p as i64) as $ty
            }
        }
    };
}

macro_rules! impl_modop_for {
    ($($ty: ty),* $(,)?) => {
        $( impl_modop!($ty); )*
    };
}

impl_modop_for! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct ModInt<const P: u64> {
    value: u64,
}

impl<const P: u64> ModInt<P> {
    pub const fn new(value: u64) -> Self {
        Self { value: value % P }
    }

    pub const fn value(&self) -> u64 {
        self.value
    }

    pub fn pow(&self, exp: u64) -> Self {
        Self {
            value: self.value.pow_mod(exp, P),
        }
    }

    pub fn inv(&self) -> Self {
        Self {
            value: self.value.inv_mod(P),
        }
    }
}

impl<const P: u64> Add for ModInt<P> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: (self.value + rhs.value) % P,
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
            value: (P + self.value - rhs.value) % P,
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
            value: self.value * rhs.value % P,
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
            value: (P - self.value) % P,
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
        self.value.hash(state)
    }
}

impl<const P: u64> Debug for ModInt<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl<const P: u64> Display for ModInt<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

macro_rules! impl_modint_from_unsigned {
    ($ty:ty) => {
        impl<const P: u64> From<$ty> for ModInt<P> {
            fn from(value: $ty) -> Self {
                Self::new(value as u64)
            }
        }
    };
}

macro_rules! impl_modint_from_signed {
    ($ty: ty) => {
        impl<const P: u64> From<$ty> for ModInt<P> {
            fn from(value: $ty) -> Self {
                Self::new(value.rem_euclid(P as $ty) as u64)
            }
        }
    };
}

macro_rules! impl_modint_from_for {
    (unsigned: [$($u:ty),* $(,)?], signed: [$($i:ty),* $(,)?]$(,)?) => {
        $( impl_modint_from_unsigned!($u); )*
        $( impl_modint_from_signed!($i); )*
    };
}

impl_modint_from_for! {
    unsigned: [u8, u16, u32, u64, u128, usize],
    signed:   [i8, i16, i32, i64, i128, isize],
}

pub type ModInt998244353 = ModInt<998244353>;
pub type ModInt1000000007 = ModInt<1000000007>;

#[cfg(test)]
mod tests {
    use std::ops::RangeInclusive;

    use crate::utils::test_utils::random::get_test_rng;
    use rand::Rng;

    const P1: u64 = 998244353;
    const P2: u64 = 1000000007;
    const P3: u64 = 2147483647;
    const P: [u64; 3] = [P1, P2, P3];

    const RANGE_I64: RangeInclusive<i64> = i64::MIN..=i64::MAX;
    const RANGE_U64: RangeInclusive<u64> = u64::MIN..=u64::MAX;

    // ランダムテストの試行回数
    const Q: usize = 10000;

    // NOTE: assertで確かめてる式が実装とほぼ一緒
    mod tests_mod_op {
        use super::{super::ModOp, *};

        #[test]
        fn test_add_mod() {
            assert_eq!(3.add_mod(5, 7), 1);
            assert_eq!((-1).add_mod(2, 5), 1);
            assert_eq!(100.add_mod(200, 17), 11);
            assert_eq!(0.add_mod(0, 3), 0);
            assert_eq!(7.add_mod(7, 7), 0);

            let mut rng = get_test_rng();
            for _ in 0..Q {
                // u64
                {
                    let lhs = rng.random_range(RANGE_U64);
                    let rhs = rng.random_range(RANGE_U64);
                    for p in P {
                        assert_eq!(lhs.add_mod(rhs, p), (lhs % p + rhs % p) % p);
                    }
                }

                // i64
                {
                    let lhs: i64 = rng.random_range(RANGE_I64);
                    let rhs: i64 = rng.random_range(RANGE_I64);
                    for p in P {
                        let p = p as i64;
                        assert_eq!(lhs.add_mod(rhs, p), (lhs % p + rhs % p).rem_euclid(p));
                    }
                }
            }
        }

        #[test]
        fn test_sub_mod() {
            assert_eq!(5.sub_mod(3, 7), 2);
            assert_eq!(3.sub_mod(5, 7), 5);
            assert_eq!((-5).sub_mod(3, 11), 3);
            assert_eq!(0.sub_mod(0, 13), 0);

            let mut rng = get_test_rng();
            for _ in 0..Q {
                // u64
                {
                    let lhs = rng.random_range(RANGE_U64);
                    let rhs = rng.random_range(RANGE_U64);
                    for p in P {
                        assert_eq!(lhs.sub_mod(rhs, p), (p + lhs % p - rhs % p) % p);
                    }
                }

                // i64
                {
                    let lhs: i64 = rng.random_range(RANGE_I64);
                    let rhs: i64 = rng.random_range(RANGE_I64);
                    for p in P {
                        let p = p as i64;
                        assert_eq!(lhs.sub_mod(rhs, p), (lhs % p - rhs % p).rem_euclid(p));
                    }
                }
            }
        }

        #[test]
        fn test_mul_mod() {
            assert_eq!(3.mul_mod(4, 5), 2);
            assert_eq!((-3).mul_mod(4, 5), 3);
            assert_eq!(0.mul_mod(12345, 7), 0);
            assert_eq!(123.mul_mod(456, 1), 0);

            let mut rng = get_test_rng();
            for _ in 0..Q {
                // u64
                {
                    let lhs = rng.random_range(RANGE_U64);
                    let rhs = rng.random_range(RANGE_U64);
                    for p in P {
                        assert_eq!(lhs.mul_mod(rhs, p), lhs % p * (rhs % p) % p);
                    }
                }

                // i64
                {
                    let lhs: i64 = rng.random_range(RANGE_I64);
                    let rhs: i64 = rng.random_range(RANGE_I64);
                    for p in P {
                        let p = p as i64;
                        assert_eq!(lhs.mul_mod(rhs, p), (lhs % p * (rhs % p)).rem_euclid(p));
                    }
                }
            }
        }

        #[test]
        fn test_div_mod() {
            assert_eq!(10.div_mod(3, 17), 9);
            assert_eq!(9.div_mod(2, 5), 2);
            assert_eq!(0.div_mod(1, 7), 0);

            let mut rng = get_test_rng();
            for _ in 0..Q {
                // u64
                {
                    let lhs = rng.random_range(RANGE_U64);
                    let rhs = rng.random_range(RANGE_U64);
                    for p in P {
                        assert_eq!(lhs.div_mod(rhs, p), lhs % p * (rhs % p).inv_mod(p) % p);
                    }
                }

                // i64
                {
                    let lhs: i64 = rng.random_range(RANGE_I64);
                    let rhs: i64 = rng.random_range(RANGE_I64);
                    for p in P {
                        let p = p as i64;

                        assert_eq!(
                            lhs.div_mod(rhs, p),
                            (lhs % p * (rhs % p).inv_mod(p)).rem_euclid(p)
                        );
                    }
                }
            }
        }

        #[test]
        fn test_neg_mod() {
            assert_eq!(3.neg_mod(5), 2);
            assert_eq!(0.neg_mod(7), 0);
            assert_eq!((-1).neg_mod(5), 1);
            assert_eq!(10.neg_mod(7), 4);

            let mut rng = get_test_rng();
            for _ in 0..Q {
                // u64
                {
                    let lhs = rng.random_range(RANGE_U64);
                    for p in P {
                        assert_eq!(lhs.add_mod(lhs.neg_mod(p), p), 0);
                    }
                }

                // i64
                {
                    let lhs: i64 = rng.random_range(RANGE_I64);
                    for p in P {
                        let p = p as i64;
                        assert_eq!(lhs.add_mod(lhs.neg_mod(p), p), 0);
                    }
                }
            }
        }

        #[test]
        fn test_pow_mod() {
            assert_eq!(2.pow_mod(10, 1000), 24);
            assert_eq!(3.pow_mod(0, 7), 1);
            assert_eq!(5.pow_mod(1, 13), 5);
            assert_eq!(7.pow_mod(2, 7), 0);
            assert_eq!(0.pow_mod(10, 13), 0);

            fn naive_pow_u64(base: u64, exp: u64, p: u64) -> u64 {
                let mut res = 1;
                for _ in 0..exp {
                    res *= base % p;
                    res %= p;
                }
                res
            }

            fn naive_pow_i64(base: i64, exp: u64, p: i64) -> i64 {
                let mut res = 1;
                for _ in 0..exp {
                    res = (res * (base % p)).rem_euclid(p);
                }
                res
            }

            let mut rng = get_test_rng();
            for _ in 0..Q {
                // u64
                {
                    let base = rng.random_range(RANGE_U64);
                    let exp = rng.random_range(0..100);
                    for p in P {
                        assert_eq!(base.pow_mod(exp, p), naive_pow_u64(base, exp, p));
                    }
                }

                // i64
                {
                    let base: i64 = rng.random_range(RANGE_I64);
                    let exp = rng.random_range(0..100);
                    for p in P {
                        let p = p as i64;
                        assert_eq!(base.pow_mod(exp, p), naive_pow_i64(base, exp, p));
                    }
                }
            }
        }

        #[test]
        fn test_inv_mod() {
            assert_eq!(3.inv_mod(11), 4);
            assert_eq!(10.inv_mod(17), 12);
            assert_eq!(1.inv_mod(5), 1);
            assert_eq!(7.inv_mod(13), 2);

            let mut rng = get_test_rng();
            for _ in 0..Q {
                // u64
                {
                    let lhs = rng.random_range(RANGE_U64);
                    for p in P {
                        assert_eq!(lhs.mul_mod(lhs.inv_mod(p), p), 1);
                    }
                }

                // i64
                {
                    let lhs = rng.random_range(RANGE_I64);
                    for p in P {
                        let p = p as i64;
                        assert_eq!(lhs.mul_mod(lhs.inv_mod(p), p), 1);
                    }
                }
            }
        }
    }

    mod tests_modint {
        use super::{super::*, *};

        #[test]
        fn test_add() {
            fn check<const P: u64>(lhs: u64, rhs: u64) {
                assert_eq!(
                    (ModInt::<P>::new(lhs) + ModInt::<P>::new(rhs)).value(),
                    (lhs % P + rhs % P) % P
                );
            }
            let mut rng = get_test_rng();
            for _ in 0..Q {
                let lhs = rng.random_range(RANGE_U64);
                let rhs = rng.random_range(RANGE_U64);
                check::<P1>(lhs, rhs);
                check::<P2>(lhs, rhs);
                check::<P3>(lhs, rhs);
            }
        }

        #[test]
        fn test_sub() {
            fn check<const P: u64>(lhs: u64, rhs: u64) {
                assert_eq!(
                    (ModInt::<P>::new(lhs) - ModInt::<P>::new(rhs)).value(),
                    (P + lhs % P - rhs % P) % P
                );
            }
            let mut rng = get_test_rng();
            for _ in 0..Q {
                let lhs = rng.random_range(RANGE_U64);
                let rhs = rng.random_range(RANGE_U64);
                check::<P1>(lhs, rhs);
                check::<P2>(lhs, rhs);
                check::<P3>(lhs, rhs);
            }
        }

        #[test]
        fn test_mul() {
            fn check<const P: u64>(lhs: u64, rhs: u64) {
                assert_eq!(
                    (ModInt::<P>::new(lhs) * ModInt::<P>::new(rhs)).value(),
                    (lhs % P * (rhs % P)) % P
                );
            }
            let mut rng = get_test_rng();
            for _ in 0..Q {
                let lhs = rng.random_range(RANGE_U64);
                let rhs = rng.random_range(RANGE_U64);
                check::<P1>(lhs, rhs);
                check::<P2>(lhs, rhs);
                check::<P3>(lhs, rhs);
            }
        }

        #[test]
        fn test_div() {
            fn check<const P: u64>(lhs: u64, rhs: u64) {
                assert_eq!(
                    (ModInt::<P>::new(lhs) / ModInt::<P>::new(rhs)).value(),
                    lhs % P * (rhs % P).inv_mod(P) % P
                );
            }
            let mut rng = get_test_rng();
            for _ in 0..Q {
                let lhs = rng.random_range(RANGE_U64);
                let rhs = rng.random_range(RANGE_U64);
                check::<P1>(lhs, rhs);
                check::<P2>(lhs, rhs);
                check::<P3>(lhs, rhs);
            }
        }

        #[test]
        fn test_neg() {
            fn check<const P: u64>(lhs: u64) {
                assert_eq!((-ModInt::<P>::new(lhs) + ModInt::<P>::new(lhs)).value(), 0);
            }
            let mut rng = get_test_rng();
            for _ in 0..Q {
                let lhs = rng.random_range(RANGE_U64);
                check::<P1>(lhs);
                check::<P2>(lhs);
                check::<P3>(lhs);
            }
            check::<P1>(0);
            check::<P2>(0);
            check::<P3>(0);
        }

        #[test]
        fn test_pow() {
            fn naive_pow(base: u64, exp: u64, p: u64) -> u64 {
                let mut res = 1;
                for _ in 0..exp {
                    res *= base % p;
                    res %= p;
                }
                res
            }
            fn check<const P: u64>(base: u64, exp: u64) {
                assert_eq!(
                    ModInt::<P>::new(base).pow(exp).value(),
                    naive_pow(base, exp, P)
                );
            }
            let mut rng = get_test_rng();
            for _ in 0..Q {
                let base = rng.random_range(RANGE_U64);
                let exp = rng.random_range(0..100);
                check::<P1>(base, exp);
                check::<P2>(base, exp);
                check::<P3>(base, exp);
            }
        }

        #[test]
        fn test_inv() {
            fn check<const P: u64>(lhs: u64) {
                assert_eq!(
                    (ModInt::<P>::new(lhs).inv() * ModInt::<P>::new(lhs)).value(),
                    1
                );
            }
            let mut rng = get_test_rng();
            for _ in 0..Q {
                let lhs = rng.random_range(RANGE_U64);
                check::<P1>(lhs);
                check::<P2>(lhs);
                check::<P3>(lhs);
            }
        }
    }
}
