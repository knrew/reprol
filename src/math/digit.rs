//! digit
//!
//! 非負整数の桁数を数える．
//!
//! ## 使用例
//! ```
//! use reprol::math::digit::Digit;
//! assert_eq!(123u64.digit(), 3);
//! ```

pub trait Digit {
    fn digit(self) -> usize;
}

macro_rules! impl_digit {
    ($ty: ty) => {
        impl Digit for $ty {
            #[allow(unused_comparisons)]
            fn digit(self) -> usize {
                assert!(self >= 0);
                if self == 0 {
                    return 1;
                }
                let mut x = self;
                let mut res = 0;
                while x > 0 {
                    x /= 10;
                    res += 1;
                }
                res
            }
        }
    };
}

macro_rules! impl_digit_for {
    ($($ty: ty),* $(,)?) => {
        $( impl_digit!($ty); )*
    };
}

impl_digit_for! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit() {
        let test_cases_u64: &[u64] = &[
            0,
            1,
            14,
            43,
            132,
            3245,
            325235,
            20872694,
            1868208004,
            2360660180,
            2393072987,
            2559907562,
            8730078281007151860,
            11962903126859318233,
            12557333292197455903,
            15287193143506470685,
            18334512744095618559,
            2332688462028828049,
            2898512312770490674,
            8448311693959430793,
            9802244488329122982,
            17061562380306305843,
            u64::MAX,
        ];

        for x in test_cases_u64 {
            assert_eq!(x.digit(), x.to_string().len());
        }

        let test_cases_u128: &[u128] = &[
            45509468001592877595755948073788932500,
            65248981567317200482825491029219331650,
            212554951304541671265400278240829527704,
            239815775772489046182975036721875234352,
            u128::MAX,
        ];

        for x in test_cases_u128 {
            assert_eq!(x.digit(), x.to_string().len());
        }
    }

    #[test]
    fn test_smoke_all_types() {
        assert_eq!(123i8.digit(), 3);
        assert_eq!(123i16.digit(), 3);
        assert_eq!(123i32.digit(), 3);
        assert_eq!(123i64.digit(), 3);
        assert_eq!(123i128.digit(), 3);
        assert_eq!(123isize.digit(), 3);
        assert_eq!(123u8.digit(), 3);
        assert_eq!(123u16.digit(), 3);
        assert_eq!(123u32.digit(), 3);
        assert_eq!(123u64.digit(), 3);
        assert_eq!(123u128.digit(), 3);
        assert_eq!(123usize.digit(), 3);
    }

    #[test]
    #[should_panic]
    fn test() {
        let _ = (-5).digit();
    }
}
