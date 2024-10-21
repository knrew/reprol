/// 繰り返し二乗法による冪乗の計算
/// NOTE: stdにpowあるのでいらないかも
pub trait Pow {
    fn pow_(self, exp: Self) -> Self;
}

macro_rules! impl_integer {
    ($($ty:ident),*) => {$(
        impl Pow for $ty {
            #[allow(unused_comparisons)]
            fn pow_(self, mut exp: Self) -> Self {
                debug_assert!(exp >= 0);

                if exp == 0 {
                    return 1;
                }

                let mut base = self;
                let mut res = 1;

                while exp > 1 {
                    if (exp & 1) == 1 {
                        res = res * base;
                    }
                    exp = exp / 2;
                    base = base * base;
                }

                res * base
            }
        }
    )*};
}

impl_integer! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

#[cfg(test)]
mod tests {
    use super::Pow;

    #[test]
    fn test_pow_u64() {
        let test_cases: Vec<(u64, u64, u64)> = vec![
            (0, 0, 1),
            (2, 3, 8),
            (5, 0, 1),
            (7, 1, 7),
            (3, 4, 81),
            (0, 5, 0),
            (0, 0, 1),
            (2, 30, 1073741824),
            (10, 9, 1000000000),
            (7, 2, 49),
            (1, 1000, 1),
            (2, 10, 1024),
            (3, 20, 3486784401),
            (10, 9, 1000000000),
            (999, 2, 998001),
            (987654321, 1, 987654321),
            (1000000007, 0, 1),
            (21, 8, 37822859361),
            (37, 3, 50653),
            (50, 3, 125000),
            (18, 3, 5832),
            (50, 7, 781250000000),
            (80, 10, 10737418240000000000),
            (83, 3, 571787),
            (55, 6, 27680640625),
            (76, 3, 438976),
            (2, 9, 512),
        ];
        for &(b, e, ans) in &test_cases {
            assert_eq!(b.pow_(e), ans);
        }
    }

    #[test]
    fn test_pow_u128() {
        let test_cases: Vec<(u128, u128, u128)> =
            vec![(123456789, 4, 232305722798259244150093798251441)];
        for &(b, e, ans) in &test_cases {
            assert_eq!(b.pow_(e), ans);
        }
    }
}
