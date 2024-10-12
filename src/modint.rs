use std::{
    fmt::{Debug, Display},
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

    pub fn pow(&self, mut exp: u64) -> Self {
        let mut res = Self::new(1);
        let mut base = *self;

        while exp > 0 {
            if exp & 1 == 1 {
                res *= base;
            }
            base *= base;
            exp >>= 1;
        }

        res
    }

    pub const fn inv(&self) -> Self {
        Self {
            value: inv_mod(self.value as i64, P as i64) as u64,
        }
    }
}

const fn inv_mod(x: i64, p: i64) -> i64 {
    debug_assert!(x > 0);
    debug_assert!(p > 0);
    if x == 1 {
        return 1;
    }
    p + (1 - p * inv_mod(p % x, x)) / x
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
        let value = if self.value < rhs.value {
            self.value + P - rhs.value
        } else {
            self.value - rhs.value
        } % P;
        Self { value }
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
    fn neg(mut self) -> Self::Output {
        if self.value > 0 {
            self.value = P - self.value
        }
        self
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

macro_rules! impl_signed {
    ($($ty:ident),*) => {$(
        impl<const P: u64> From<$ty> for ModInt<P> {
            fn from(value: $ty) -> Self {
                Self::new(value as u64)
            }
        }
    )*};
}
impl_signed! { u8, u16, u32, u64, u128, usize }

macro_rules! impl_unsigned {
    ($($ty:ident),*) => {$(
        impl<const P: u64> From<$ty> for ModInt<P> {
            fn from(value: $ty) -> Self {
                if value < 0 {
                    -Self::new((P as i64 - value as i64) as u64)
                }else{
                    Self::new(value as u64)
                }
            }
        }
    )*};
}
impl_unsigned! { i8, i16, i32, i64, i128, isize }

pub type ModInt998244353 = ModInt<998244353>;
pub type ModInt1000000007 = ModInt<1000000007>;

#[cfg(test)]
mod tests {
    use super::*;

    const P1: u64 = 998244353;
    const P2: u64 = 1000000007;

    #[test]
    fn test_inverse() {
        let test_cases = vec![
            1, 2, 123456789, 987654321, 500000000, 400000000, 99999999, 876543210, 998244352, 3,
            1000000010, 5420274012,
        ];

        let p1: i64 = P1 as i64;
        for &a in &test_cases {
            let a_inv = inv_mod(a, p1);
            assert_eq!((a % p1 * a_inv % p1), 1);
        }

        let p2 = P2 as i64;
        for &a in &test_cases {
            let a_inv = inv_mod(a, p2);
            assert_eq!((a % p2 * a_inv % p2), 1);
        }
    }

    #[test]
    fn test_add() {
        let test_cases = vec![
            (1, 2),
            (998244352, 1),
            (123456789, 987654321),
            (998244352, 998244352),
            (500000000, 500000000),
            (0, 0),
            (876543210, 123456789),
            (400000000, 600000000),
            (998244351, 1),
            (700000000, 300000000),
        ];

        for &(x, y) in &test_cases {
            let x_mod = ModInt::<P1>::new(x);
            let y_mod = ModInt::<P1>::new(y);
            assert_eq!((x_mod + y_mod).value(), (x + y) % P1);
        }

        for &(x, y) in &test_cases {
            let x_mod = ModInt::<P2>::new(x);
            let y_mod = ModInt::<P2>::new(y);
            assert_eq!((x_mod + y_mod).value(), (x + y) % P2);
        }
    }

    #[test]
    fn test_sub() {
        let test_cases = vec![
            (987654321, 123456789),
            (1, 1),
            (998244352, 998244351),
            (123456789, 987654321),
            (0, 0),
            (500000000, 500000000),
            (998244352, 0),
            (876543210, 123456789),
            (998244352, 1),
            (123456789, 987654321),
        ];

        for &(x, y) in &test_cases {
            let x_mod = ModInt::<P1>::new(x);
            let y_mod = ModInt::<P1>::new(y);
            assert_eq!((x_mod - y_mod).value(), (x % P1 + P1 - y % P1) % P1);
        }

        for &(x, y) in &test_cases {
            let x_mod = ModInt::<P2>::new(x);
            let y_mod = ModInt::<P2>::new(y);
            assert_eq!((x_mod - y_mod).value(), (x % P2 + P2 - y % P2) % P2);
        }
    }

    #[test]
    fn test_mul() {
        let test_cases = vec![
            (1, 2),
            (998244352, 1),
            (123456789, 987654321),
            (500000000, 2),
            (876543210, 123456789),
            (0, 123456789),
            (987654321, 1),
            (998244352, 998244352),
            (400000000, 600000000),
            (99999999, 123456789),
        ];

        for &(x, y) in &test_cases {
            let x_mod = ModInt::<P1>::new(x);
            let y_mod = ModInt::<P1>::new(y);
            assert_eq!((x_mod * y_mod).value(), (x * y) % P1);
        }

        for &(x, y) in &test_cases {
            let x_mod = ModInt::<P2>::new(x);
            let y_mod = ModInt::<P2>::new(y);
            assert_eq!((x_mod * y_mod).value(), (x * y) % P2);
        }
    }

    #[test]
    fn test_div() {
        let test_cases = vec![
            (987654321, 123456789),
            (123456789, 987654321),
            (876543210, 123456789),
            (998244352, 2),
            (987654321, 1),
            (876543210, 987654321),
            (1, 2),
            (500000000, 400000000),
            (123456789, 99999999),
            (998244352, 998244351),
        ];

        for &(x, y) in &test_cases {
            let answer = (x % P1 * inv_mod(y as i64, P1 as i64) as u64) % P1;
            let x_mod = ModInt::<P1>::new(x);
            let y_mod = ModInt::<P1>::new(y);
            assert_eq!((x_mod / y_mod).value(), answer);
        }

        for &(x, y) in &test_cases {
            let answer = (x % P2 * inv_mod(y as i64, P2 as i64) as u64) % P2;
            let x_mod = ModInt::<P2>::new(x);
            let y_mod = ModInt::<P2>::new(y);
            assert_eq!((x_mod / y_mod).value(), answer);
        }
    }

    #[test]
    fn test_pow() {
        let test_cases = vec![
            ((2, 10), 1024),
            ((3, 5), 243),
            ((5, 3), 125),
            ((7, 4), 2401),
            ((10, 0), 1),
            ((123, 456), 500543741),
            ((987654321, 2), 17678886),
            ((1000000006, 100), 308114436),
            ((999, 999), 117436213),
            ((500, 500), 650576768),
            ((2, 998244352), 1),
            ((2, 1000000006), 565485962),
        ];

        for &((base, exp), answer) in &test_cases {
            let result = ModInt::<P1>::new(base).pow(exp).value();
            assert_eq!(result, answer);
        }

        let test_cases = vec![
            ((2, 10), 1024),
            ((3, 5), 243),
            ((5, 3), 125),
            ((7, 4), 2401),
            ((10, 0), 1),
            ((123, 456), 565291922),
            ((987654321, 2), 961743691),
            ((1000000006, 100), 1),
            ((999, 999), 760074701),
            ((500, 500), 742761597),
            ((2, 998244352), 106733835),
            ((2, 1000000006), 1),
        ];

        for &((base, exp), answer) in &test_cases {
            let result = ModInt::<P2>::new(base).pow(exp).value();
            assert_eq!(result, answer);
        }
    }
}
