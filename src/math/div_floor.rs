pub trait DivFloor {
    /// $\lfloor \frac{x}{y} \rfloor$を計算する
    fn div_floor_(self, rhs: Self) -> Self;
}

macro_rules! impl_integer {
    ($($ty:ident),*) => {$(
        impl DivFloor for $ty {
            #[allow(unused_comparisons)]
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
        }
    )*};
}

impl_integer! { i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize }

#[cfg(test)]
mod tests {
    use super::DivFloor;

    #[test]
    fn test_div_floor() {
        let testcases = vec![
            ((10i32, 3), 3),
            ((9, 3), 3),
            ((8, 3), 2),
            ((-10, 3), -4),
            ((-9, 3), -3),
            ((-8, 3), -3),
            ((10, -3), -4),
            ((9, -3), -3),
            ((8, -3), -3),
            ((-10, -3), 3),
            ((-9, -3), 3),
            ((-8, -3), 2),
            ((0, 3), 0),
            ((0, -3), 0),
            ((10, 1), 10),
            ((-10, 1), -10),
            ((10, -1), -10),
            ((-10, -1), 10),
        ];

        for ((x, y), expected) in testcases {
            assert_eq!(x.div_floor_(y), expected);
        }

        let testcases = vec![
            ((10u64, 3), 3),
            ((9, 3), 3),
            ((8, 3), 2),
            ((0, 3), 0),
            ((10, 1), 10),
        ];

        for ((x, y), expected) in testcases {
            assert_eq!(x.div_floor_(y), expected);
        }
    }
}
