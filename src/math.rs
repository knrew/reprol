/// x/yをする 小数点以下切り上げ
/// NOTE: 非負のみ対応
/// NOTE: stdのdiv_ceilはrustc1.73から
pub trait DivCeil {
    fn div_ceil_(self, rhs: Self) -> Self;
}

/// 繰り返し二乗法による冪乗の計算
pub trait Pow {
    fn pow_(self, exp: Self) -> Self;
}

macro_rules! impl_integer {
    ($($ty:ident),*) => {$(
        impl DivCeil for $ty {
            #[allow(unused_comparisons)]
            fn div_ceil_(self, rhs: Self) -> Self {
                debug_assert!(self >= 0);
                debug_assert!(rhs >= 1);
                (self + rhs - 1) / rhs
            }
        }

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
    use super::{DivCeil, Pow};

    #[test]
    fn test_div_ceil() {
        assert_eq!(10u64.div_ceil_(2), 5);
        assert_eq!(100u64.div_ceil_(5), 20);
        assert_eq!(10u64.div_ceil_(3), 4);
        assert_eq!(7u64.div_ceil_(2), 4);
        assert_eq!(15u64.div_ceil_(1), 15);
        assert_eq!(0u64.div_ceil_(1), 0);
        assert_eq!(0u64.div_ceil_(5), 0);
        assert_eq!(0u64.div_ceil_(100), 0);
    }

    #[test]
    fn test_pow() {
        assert_eq!(2u64.pow_(3), 8);
        assert_eq!(5u64.pow_(0), 1);
        assert_eq!(7u64.pow_(1), 7);
        assert_eq!(3u64.pow_(4), 81);
        assert_eq!(0u64.pow_(5), 0);
        assert_eq!(0u64.pow_(0), 1);
        assert_eq!(2u64.pow_(30), 1073741824);
        assert_eq!(10u64.pow_(9), 1000000000);
    }
}
