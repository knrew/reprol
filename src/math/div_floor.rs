//! div_floor
//!
//! 2個の整数x, yに対して，x/y以下の最大の整数を計算する．
//! 負の数対応．
//! $\lfloor \frac{x}{y} \rfloor$
//!
//! ## 使用例
//! ```
//! use reprol::math::div_floor::DivFloor;
//! assert_eq!(7i32.div_floor_(3), 2);
//! assert_eq!((-4i32).div_floor_(3), -2);
//! ```

pub trait DivFloor {
    fn div_floor_(self, rhs: Self) -> Self;
    fn checked_div_floor(self, rhs: Self) -> Option<Self>
    where
        Self: Sized;
}

macro_rules! impl_div_unsigned {
    ($ty: ty) => {
        impl DivFloor for $ty {
            fn div_floor_(self, rhs: Self) -> Self {
                assert!(rhs != 0);
                self / rhs
            }

            fn checked_div_floor(self, rhs: Self) -> Option<Self> {
                self.checked_div(rhs)
            }
        }
    };
}

macro_rules! impl_div_signed {
    ($ty: ty) => {
        impl DivFloor for $ty {
            fn div_floor_(self, rhs: Self) -> Self {
                assert!(rhs != 0);
                let q = self / rhs;
                let r = self % rhs;
                if r != 0 && (self < 0) != (rhs < 0) {
                    q - 1
                } else {
                    q
                }
            }

            fn checked_div_floor(self, rhs: Self) -> Option<Self> {
                let q = self.checked_div(rhs)?;
                let r = self.checked_rem(rhs)?;
                if r != 0 && (self < 0) != (rhs < 0) {
                    q.checked_sub(1)
                } else {
                    Some(q)
                }
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
        let cases: &[(i32, i32, i32)] = &[
            //
            // 正の数どうし
            //
            (0, 1, 0),
            (1, 1, 1),
            (1, 2, 0),
            (2, 1, 2),
            (2, 2, 1),
            (3, 2, 1),
            (5, 2, 2),
            (6, 3, 2),
            (7, 3, 2),
            (10, 4, 2),
            (100, 9, 11),
            (i32::MAX, i32::MAX, 1),
            (i32::MAX - 1, i32::MAX, 0),
            //
            // 負の数あり
            //
            (-4, 3, -2), // floor(-1.333..) = -2
            (4, -3, -2), // floor(-1.333..) = -2
            (-4, -3, 1), // floor( 1.333..) =  1
            (-7, 3, -3),
            (7, -3, -3),
            (-7, -3, 2),
            (i32::MIN, 1, i32::MIN),
            (i32::MAX, -1, -i32::MAX),
            (0, -5, 0),
            (0, 5, 0),
            (-6, 3, -2),
            (6, -3, -2),
            (-6, -3, 2),
        ];

        for &(a, b, exp) in cases {
            assert_eq!(a.div_floor_(b), exp, "failed case: {a} div_floor {b}");
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
            let expected = a / b;
            assert_eq!(a.div_floor_(b), expected, "failed case: {a} div_floor {b}");
        }
    }

    #[test]
    fn test_smoke_all_types() {
        assert_eq!(5u8.div_floor_(2), 2);
        assert_eq!(5u16.div_floor_(2), 2);
        assert_eq!(5u32.div_floor_(2), 2);
        assert_eq!(5u64.div_floor_(2), 2);
        assert_eq!(5u128.div_floor_(2), 2);
        assert_eq!(5usize.div_floor_(2), 2);

        assert_eq!(5i8.div_floor_(2), 2);
        assert_eq!(5i16.div_floor_(2), 2);
        assert_eq!(5i32.div_floor_(2), 2);
        assert_eq!(5i64.div_floor_(2), 2);
        assert_eq!(5i128.div_floor_(2), 2);
        assert_eq!(5isize.div_floor_(2), 2);
    }

    #[test]
    #[should_panic]
    fn u64_rhs_zero_panics() {
        let _ = 1u64.div_floor_(0);
    }

    #[test]
    #[should_panic]
    fn i32_rhs_zero_panics() {
        let _ = 1i32.div_floor_(0);
    }

    #[test]
    #[should_panic]
    fn i32_min_overflow_panics() {
        let _ = i32::MIN.div_floor_(-1);
    }
}
