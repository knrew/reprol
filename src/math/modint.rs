use std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

pub trait PowMod {
    /// 法pのもとで冪乗を計算する
    fn pow_mod(self, exp: Self, p: Self) -> Self;
}

macro_rules! impl_integer {
    ($($ty:ident),*) => {$(
        impl PowMod for $ty {
            #[allow(unused_comparisons)]
            fn pow_mod(self, mut exp: Self, p: Self) -> Self {
                assert!(self >= 0);
                assert!(exp >= 0);
                assert!(p > 0);

                if p == 1 {
                    return 0;
                }

                let mut res = 1;
                let mut base = self % p;

                while exp > 0 {
                    if exp & 1 == 1 {
                        res = res * base % p;
                    }
                    base = base * base % p;
                    exp >>= 1;
                }

                res
            }
        }
    )*};
}

impl_integer! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

pub trait InvMod {
    /// 法pにおける逆元を計算する
    fn inv_mod(self, p: Self) -> Self;
}

macro_rules! impl_signed {
    ($($ty:ident),*) => {$(
        impl InvMod for $ty {
            fn inv_mod(self, p: Self) -> Self {
                assert!(self > 0);
                assert!(p > 0);
                if self == 1 {
                    return 1;
                }
                p + (1 - p * (p % self).inv_mod(self)) / self
            }
        }
    )*};
}

impl_signed! { i8, i16, i32, i64, i128, isize }

macro_rules! impl_unsigned {
    ($($ty:ident),*) => {$(
        impl InvMod for $ty {
            fn inv_mod(self, p: Self) -> Self {
                (self as i64).inv_mod(p as i64) as $ty
            }
        }
    )*};
}

impl_unsigned! { u8, u16, u32, u64, usize }

impl InvMod for u128 {
    fn inv_mod(self, p: Self) -> Self {
        (self as i128).inv_mod(p as i128) as u128
    }
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
impl_signed! { u8, u16, u32, u64, usize }

impl<const P: u64> From<u128> for ModInt<P> {
    fn from(value: u128) -> Self {
        Self::new(value.rem_euclid(P as u128) as u64)
    }
}

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

impl_unsigned! { i8, i16, i32, i64, isize }

impl<const P: u64> From<i128> for ModInt<P> {
    fn from(value: i128) -> Self {
        Self::new(value.rem_euclid(P as i128) as u64)
    }
}

pub type ModInt998244353 = ModInt<998244353>;
pub type ModInt1000000007 = ModInt<1000000007>;

#[cfg(test)]
mod tests {
    use crate::math::modint::InvMod;

    use super::PowMod;

    #[test]
    fn test_pow_mod() {
        const P1: u64 = 998244353;
        const P2: u64 = 1000000007;

        // (base, exp, ans1, ans2)の順で並んでいる
        // base^expを計算する
        // ans1, ans2はそれぞれ法P1, P2での答え
        let test_cases = vec![
            (0, 0, 1, 1),
            (10, 0, 1, 1),
            (0, 12, 0, 0),
            (2, 10, 1024, 1024),
            (3, 5, 243, 243),
            (5, 3, 125, 125),
            (7, 4, 2401, 2401),
            (123, 456, 500543741, 565291922),
            (987654321, 2, 17678886, 961743691),
            (1000000006, 100, 308114436, 1),
            (999, 999, 117436213, 760074701),
            (500, 500, 650576768, 742761597),
            (2, 998244352, 1, 106733835),
            (2, 1000000006, 565485962, 1),
            (35159992, 853659348, 171826619, 73025258),
            (173744080, 972168833, 562413643, 338142216),
            (258912740, 518302010, 763696358, 868359857),
            (561083107, 110854587, 592288248, 136419826),
            (578612337, 331137309, 165640937, 170496686),
            (595763466, 176515871, 635087261, 802111797),
            (633335045, 18529847, 929415341, 539935827),
            (723091847, 451729607, 531431947, 242080099),
            (775348050, 914965051, 833671373, 960043753),
            (947772619, 548149867, 577212826, 184934494),
            (930769844, 4294967295, 517902255, 190677013),
        ];

        for &(base, exp, ans1, ans2) in &test_cases {
            assert_eq!(base.pow_mod(exp, P1), ans1);
            assert_eq!(base.pow_mod(exp, P2), ans2);
        }
    }

    #[test]
    fn test_inv_mod() {
        const P1: u64 = 998244353;
        const P2: u64 = 1000000007;

        let test_cases = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 123456789, 987654321, 500000000, 400000000, 99999999,
            876543210, 998244352, 1000000010, 5420274012, 51307341, 177174101, 272154126,
            285120554, 310046136, 512696315, 537364739, 606810056, 703996446, 808398679, 93762712,
            126607016, 126882966, 169157861, 431575151, 489724038, 667652900, 735396744, 931229540,
            966373973, 1000000006,
        ];

        for &x in &test_cases {
            let x_inv = x.inv_mod(P1);
            assert_eq!((x % P1 * x_inv % P1), 1);

            let x_inv = x.inv_mod(P2);
            assert_eq!((x % P2 * x_inv % P2), 1);
        }
    }

    mod modint {
        use crate::math::modint::InvMod;

        use super::super::ModInt;

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

        #[test]
        fn test_pow() {
            let test_cases: Vec<(u64, u64)> = vec![(1, 2), (2, 8), (7, 4), (8, 7), (9, 3), (9, 4)];
            for &(b, e) in &test_cases {
                assert_eq!(ModInt::<P1>::from(b).pow(e).value(), b.pow(e as u32) % P1);
                assert_eq!(ModInt::<P2>::from(b).pow(e).value(), b.pow(e as u32) % P2);
            }
        }
    }
}
