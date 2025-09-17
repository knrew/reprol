//! 最大公約数(greatest common divisor)
//!
//! 最大公約数(gcd)を計算する．
//!
//! # 使用例
//! ```
//! use reprol::math::gcd::Gcd;
//! assert_eq!(48u64.gcd(18), 6);
//! ```
//!
//! # Panics
//! - `self == T::MIN && rhs == 0`の場合
//! - `self == 0 && rhs == T::MIN`の場合

pub trait Gcd {
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
    ($ty: ty) => {
        impl Gcd for $ty {
            fn gcd(self, rhs: Self) -> Self {
                assert_ne!((self, rhs), (<$ty>::MIN, 0));
                assert_ne!((self, rhs), (0, <$ty>::MIN));
                let (mut a, mut b) = (self, rhs);
                while b != 0 {
                    (a, b) = (b, a.rem_euclid(b));
                }
                a.abs()
            }
        }
    };
}

macro_rules! impl_gcd_for {
    (unsigned: [$($u:ty),* $(,)?], signed: [$($s:ty),* $(,)?]$(,)?) => {
        $( impl_gcd_unsigned!($u); )*
        $( impl_gcd_signed!($s); )*
    };
}

impl_gcd_for! {
    unsigned: [u8, u16, u32, u64, u128, usize],
    signed:   [i8, i16, i32, i64, i128, isize],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_all_types() {
        assert_eq!(6u8.gcd(8), 2);
        assert_eq!(6u16.gcd(8), 2);
        assert_eq!(6u32.gcd(8), 2);
        assert_eq!(6u64.gcd(8), 2);
        assert_eq!(6u128.gcd(8), 2);
        assert_eq!(6usize.gcd(8), 2);

        assert_eq!((-6i8).gcd(8), 2);
        assert_eq!((-6i16).gcd(8), 2);
        assert_eq!((-6i32).gcd(8), 2);
        assert_eq!((-6i64).gcd(8), 2);
        assert_eq!((-6i128).gcd(8), 2);
        assert_eq!((-6isize).gcd(8), 2);
    }

    #[test]
    fn test_gcd_i32() {
        let test_cases: &[(i32, i32, i32)] = &[
            (0, 0, 0),
            (55, 0, 55),
            (48, 18, 6),
            (54, 24, 6),
            (101, 103, 1),
            (0, 10, 10),
            (10, 0, 10),
            (42, 42, 42),
            (-48, -18, 6),
            (-54, 24, 6),
            (-101, -103, 1),
            (-42, -42, 42),
            (-553520529, -1197209986, 1),
            (540551869, 1840728616, 1),
            (1908219250, -425407391, 1),
            (509040292, 57610230, 2),
            (-1175743, 863363892, 1),
            (1030682953, 713241972, 1),
            (-1952738426, -444868397, 1),
            (-1190361003, -1401410658, 3),
            (1825167094, -100985106, 2),
            (-338164223, -1981524567, 1),
        ];

        for &(x, y, expected) in test_cases {
            assert_eq!(x.gcd(y), expected);
            assert_eq!(y.gcd(x), expected);
        }
    }

    #[test]
    fn test_gcd_u32() {
        let test_cases: &[(u32, u32, u32)] = &[
            (3106312921, 3453216366, 1),
            (3926578617, 3954465752, 1),
            (2505980391, 3406528687, 1),
            (2423778012, 3822572476, 4),
            (2917219236, 1391715291, 3),
            (536213819, 1109600266, 1),
            (2868206430, 478397803, 1),
            (1654786103, 3224039297, 1),
            (3298083185, 1690096037, 1),
            (37595066, 3306548580, 2),
        ];

        for &(x, y, expected) in test_cases {
            assert_eq!(x.gcd(y), expected);
            assert_eq!(y.gcd(x), expected);
        }
    }

    #[test]
    fn test_gcd_i64() {
        let test_cases: &[(i64, i64, i64)] = &[
            (-7359320707057902644, -6552810857364711647, 1),
            (905729105041378636, -2875869736246836725, 1),
            (8847070700217876836, 4631139478152483576, 4),
            (8016460018729025502, -2521122481601948305, 1),
            (-6227801812904253801, 4136795014664585392, 1),
            (3957933019027215470, -333365406113364589, 1),
            (-7597121186181759164, -6263631862945007953, 1),
            (-5338889392169988153, -9216016843508212313, 1),
            (-8489660990110051566, -6147875596777290523, 1),
            (-5551004925175289934, -8647799748584301441, 3),
        ];

        for &(x, y, expected) in test_cases {
            assert_eq!(x.gcd(y), expected);
            assert_eq!(y.gcd(x), expected);
        }
    }

    #[test]
    fn test_gcd_u64() {
        let test_cases: &[(u64, u64, u64)] = &[
            (17359742233317853495, 2643959224771992315, 5),
            (14386085941964695295, 1550468089565495025, 5),
            (408179090575346137, 15739249171817168003, 1),
            (8011442099917386412, 7038482920121905544, 4),
            (10931070166471004237, 3257433409529011374, 1),
            (3799476535477832801, 9811641311918688661, 1),
            (9260581597280282616, 6232070007492218880, 24),
            (17158962765808650752, 11801443467467210340, 28),
            (3145239905975089026, 8524459297996239250, 2),
            (5400548729038821887, 15221263237448695924, 1),
        ];

        for &(x, y, expected) in test_cases {
            assert_eq!(x.gcd(y), expected);
            assert_eq!(y.gcd(x), expected);
        }
    }

    #[test]
    fn test_gcd_u128() {
        let test_cases: &[(u128, u128, u128)] = &[(
            1_000_000_000_000_000_000,
            500_000_000_000_000_000,
            500_000_000_000_000_000,
        )];

        for &(x, y, expected) in test_cases {
            assert_eq!(x.gcd(y), expected);
            assert_eq!(y.gcd(x), expected);
        }
    }

    #[test]
    #[should_panic]
    fn test_gcd_min_zero() {
        let _ = i32::MIN.gcd(0);
    }

    #[test]
    #[should_panic]
    fn test_gcd_zero_min() {
        let _ = (0i32).gcd(i32::MIN);
    }
}
