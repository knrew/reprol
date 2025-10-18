//! div_ceil
//!
//! 2個の整数x, yに対して，x/y以上の最小の整数を計算する．
//! 負の数対応．
//! $\lceil \frac{x}{y} \rceil$
//!
//!
//! ## 使用例
//! ```
//! use reprol::math::div_ceil::DivCeil;
//! assert_eq!(5i32.div_ceil_(3), 2);
//! assert_eq!((-4i32).div_ceil_(3), -1);
//! ```

pub trait DivCeil {
    fn div_ceil_(self, rhs: Self) -> Self;
}

macro_rules! impl_div_signed {
    ($ty: ty) => {
        impl DivCeil for $ty {
            fn div_ceil_(self, rhs: Self) -> Self {
                assert!(rhs != 0);
                let q = self / rhs;
                let r = self % rhs;
                if r != 0 && ((self >= 0) == (rhs >= 0)) {
                    q + 1
                } else {
                    q
                }
            }
        }
    };
}

macro_rules! impl_div_unsigned {
    ($ty: ty) => {
        impl DivCeil for $ty {
            fn div_ceil_(self, rhs: Self) -> Self {
                assert!(rhs != 0);
                let q = self / rhs;
                let r = self % rhs;
                if r != 0 { q + 1 } else { q }
            }
        }
    };
}

macro_rules! impl_div_for {
    (unsigned: [$($u:ty),* $(,)?], signed: [$($s:ty),* $(,)?]$(,)?) => {
        $( impl_div_unsigned!($u); )*
        $( impl_div_signed!($s); )*
    };
}

impl_div_for! {
    unsigned: [u8, u16, u32, u64, u128, usize],
    signed:   [i8, i16, i32, i64, i128, isize],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i32() {
        let testcases: &[(i32, i32, i32)] = &[
            //
            // 正の数どうし
            //
            (0, 1, 0),
            (1, 1, 1),
            (1, 2, 1),
            (2, 1, 2),
            (2, 2, 1),
            (3, 2, 2),
            (5, 2, 3),
            (6, 3, 2),
            (7, 3, 3),
            (10, 4, 3),
            (100, 9, 12),
            (i32::MAX, i32::MAX, 1),
            (i32::MAX - 1, i32::MAX, 1),
            //
            //負の数あり
            //
            (-4, 3, -1),
            (4, -3, -1),
            (-4, -3, 2),
            (-7, 3, -2),
            (7, -3, -2),
            (-7, -3, 3),
            (i32::MIN, 1, i32::MIN),
            (i32::MAX, -1, -i32::MAX),
            (0, -5, 0),
        ];

        for &(a, b, expected) in testcases {
            assert_eq!(a.div_ceil_(b), expected, "failed case: {a} div_ceil {b}");
        }
    }

    #[test]
    fn test_u64() {
        let cases: &[(u64, u64)] = &[
            (0, 1),
            (1, 1),
            (1, 2),
            (2, 1),
            (3, 2),
            (u64::MAX, u64::MAX),
            (u64::MAX - 1, u64::MAX),
            (u64::MAX, 2),
            (u64::MAX - 5, 3),
        ];

        for &(a, b) in cases {
            let expected = if a == 0 { 0 } else { 1 + (a - 1) / b };
            assert_eq!(a.div_ceil_(b), expected, "failed case: {a} div_ceil {b}");
        }
    }

    #[test]
    fn test_smoke_all_types() {
        assert_eq!(5u8.div_ceil_(2), 3);
        assert_eq!(5u16.div_ceil_(2), 3);
        assert_eq!(5u32.div_ceil_(2), 3);
        assert_eq!(5u64.div_ceil_(2), 3);
        assert_eq!(5u128.div_ceil_(2), 3);
        assert_eq!(5usize.div_ceil_(2), 3);
        assert_eq!(5i8.div_ceil_(2), 3);
        assert_eq!(5i16.div_ceil_(2), 3);
        assert_eq!(5i32.div_ceil_(2), 3);
        assert_eq!(5i64.div_ceil_(2), 3);
        assert_eq!(5i128.div_ceil_(2), 3);
        assert_eq!(5isize.div_ceil_(2), 3);
    }

    #[test]
    #[should_panic]
    fn test_zero_div_i32() {
        let _ = 1i32.div_ceil_(0);
    }

    #[test]
    #[should_panic]
    fn test_zero_div_u64() {
        let _ = 1u64.div_ceil_(0);
    }

    #[test]
    #[should_panic]
    fn test_i32_min_overflow_panics() {
        let _ = i32::MIN.div_ceil_(-1);
    }
}
