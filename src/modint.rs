use std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::math::{inv_mod::InvMod, pow_mod::PowMod};

#[derive(Clone, Copy, PartialEq, Eq)]
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

    use super::{InvMod, ModInt};

    const P1: u64 = 998244353;
    const P2: u64 = 1000000007;

    #[test]
    fn test_add() {
        let test_cases: Vec<(u64, u64)> = vec![
            (0, 0),
            (1, 0),
            (499122176, 499122177),
            (998244352, 998244352),
            (123456789, 987654321),
            (998244353, 1),
            (998244353, 998244353),
            (998244354, 998244354),
            (1000000006, 1),
            (500000003, 500000004),
            (123456789, 987654321),
            (1000000007, 1),
            (1000000007, 1000000007),
            (1000000008, 1000000008),
            (58256360, 365808262),
            (83667882, 299069939),
            (258232411, 272932992),
            (326601489, 171373997),
            (346847046, 51645215),
            (396883208, 298093909),
            (523216640, 514064959),
            (559863798, 635570313),
            (574376475, 712050275),
            (584669163, 679969849),
        ];

        for &(x, y) in &test_cases {
            assert_eq!(
                (ModInt::<P1>::from(x) + ModInt::<P1>::from(y)).value(),
                (x + y) % P1
            );
            assert_eq!(
                (ModInt::<P2>::from(x) + ModInt::<P2>::from(y)).value(),
                (x + y) % P2
            );
        }
    }

    #[test]
    fn test_sub() {
        let test_cases = vec![
            (0, 0),
            (1, 0),
            (998244352, 1),
            (499122176, 499122177),
            (123456789, 987654321),
            (998244353, 1),
            (998244353, 998244353),
            (998244354, 998244353),
            (1000000006, 1),
            (500000003, 500000004),
            (123456789, 987654321),
            (1000000007, 1),
            (1000000007, 1000000007),
            (1000000008, 1000000007),
            (134208286, 716894595),
            (482094914, 116671953),
            (487479340, 999809006),
            (545532469, 162945496),
            (600350847, 868494828),
            (708429456, 566729886),
            (840962678, 75211106),
            (870396913, 584869126),
            (899222911, 544210189),
            (984108954, 350145063),
        ];

        for &(x, y) in &test_cases {
            assert_eq!(
                (ModInt::<P1>::from(x) - ModInt::<P1>::from(y)).value(),
                (x % P1 + P1 - y % P1) % P1
            );
            assert_eq!(
                (ModInt::<P2>::from(x) - ModInt::<P2>::from(y)).value(),
                (x % P2 + P2 - y % P2) % P2
            );
        }
    }

    #[test]
    fn test_mul() {
        let test_cases = vec![
            (0, 0),
            (1, 0),
            (998244352, 1),
            (499122176, 2),
            (123456789, 987654321),
            (998244353, 1),
            (998244354, 998244354),
            (1000000006, 1),
            (500000003, 2),
            (123456789, 987654321),
            (1000000007, 1),
            (1000000008, 1000000008),
            (67377188, 827878966),
            (173077069, 440033898),
            (240665981, 111177377),
            (251072272, 169977166),
            (353753509, 128810916),
            (654863013, 472324155),
            (680881895, 348139943),
            (761630230, 317368077),
            (823732505, 217545776),
            (855115584, 427970193),
        ];

        for &(x, y) in &test_cases {
            assert_eq!(
                (ModInt::<P1>::from(x) * ModInt::<P1>::from(y)).value(),
                x * y % P1
            );
            assert_eq!(
                (ModInt::<P2>::from(x) * ModInt::<P2>::from(y)).value(),
                x * y % P2
            );
        }
    }

    #[test]
    fn test_div() {
        let test_cases = vec![
            (1, 1),
            (998244352, 1),
            (499122176, 2),
            (123456789, 987654321),
            (998244353, 1),
            (998244354, 998244354),
            (998244352, 998244351),
            (1000000006, 1),
            (500000003, 2),
            (1000000007, 1),
            (1000000008, 1000000008),
            (1000000006, 1000000005),
            (137691030, 306624263),
            (252282219, 677403342),
            (320667800, 309333700),
            (328600336, 324106977),
            (460037569, 420925742),
            (496839902, 500822220),
            (535384799, 574808459),
            (624967428, 824836082),
            (773513165, 917357474),
            (779736753, 685849205),
        ];

        for &(x, y) in &test_cases {
            assert_eq!(
                (ModInt::<P1>::from(x) / ModInt::<P1>::from(y)).value(),
                x % P1 * (y % P1).inv_mod(P1) % P1
            );
            assert_eq!(
                (ModInt::<P2>::from(x) / ModInt::<P2>::from(y)).value(),
                x % P2 * (y % P2).inv_mod(P2) % P2
            );
        }
    }

    #[test]
    fn test_neg() {
        let test_cases: Vec<i64> = vec![
            -1,
            -998244352,
            -998244353,
            -998244354,
            -123456789,
            -987654321,
            -999999999999999,
            -500,
            -1000000000,
            -1000000006,
            -1000000007,
            -1000000008,
            -772112994,
            -713944385,
            -525024181,
            -267636606,
            -253618839,
            -191659260,
            -157790240,
            -76104401,
            -75137182,
            -14591414,
        ];
        for &x in &test_cases {
            assert_eq!(
                ModInt::<P1>::from(x).value(),
                x.rem_euclid(P1 as i64) as u64
            );
            assert_eq!(
                (-ModInt::<P1>::from(x.abs())).value(),
                x.rem_euclid(P1 as i64) as u64
            );
            assert_eq!(
                ModInt::<P2>::from(x).value(),
                x.rem_euclid(P2 as i64) as u64
            );
            assert_eq!(
                (-ModInt::<P2>::from(x.abs())).value(),
                x.rem_euclid(P2 as i64) as u64
            );
        }
    }

    /// より詳しいテストは`inv_mod.rs`
    #[test]
    fn test_inv() {
        let test_cases = vec![1, 2, 3, 4, 5];

        for &x in &test_cases {
            {
                let x = ModInt::<P1>::from(x);
                let x_inv = x.inv();
                assert_eq!((x * x_inv).value(), 1);
            }

            let x = ModInt::<P1>::from(x);
            let x_inv = x.inv();
            assert_eq!((x * x_inv).value(), 1);
        }
    }

    /// より詳しいテストは`pow_mod.rs`
    #[test]
    fn test_pow() {
        let test_cases: Vec<(u64, u64)> = vec![(1, 2), (2, 8), (7, 4), (8, 7), (9, 3), (9, 4)];
        for &(b, e) in &test_cases {
            assert_eq!(ModInt::<P1>::from(b).pow(e).value(), b.pow(e as u32) % P1);
            assert_eq!(ModInt::<P2>::from(b).pow(e).value(), b.pow(e as u32) % P2);
        }
    }
}
