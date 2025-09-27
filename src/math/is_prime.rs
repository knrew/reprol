//! 素数判定
//!
//! 整数が素数かどうか判定する．
//!
//! ## 使用例
//! ```
//! use reprol::math::is_prime::IsPrime;
//! assert!(7u32.is_prime());
//! assert!(!12u32.is_prime());
//! ```

pub trait IsPrime {
    fn is_prime(&self) -> bool;
}

macro_rules! impl_is_prime {
    ($ty: ty) => {
        impl IsPrime for $ty {
            fn is_prime(&self) -> bool {
                self >= &2 && (2..).take_while(|i| i * i <= *self).all(|i| self % i != 0)
            }
        }
    };
}

macro_rules! impl_is_prime_for {
    ($($ty: ty),* $(,)?) => {
        $( impl_is_prime!($ty); )*
    };
}

impl_is_prime_for! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        let test_cases: &[(u64, bool)] = &[
            (0, false),
            (1, false),
            (2, true),
            (3, true),
            (4, false),
            (5, true),
            (6, false),
            (7, true),
            (8, false),
            (9, false),
            (10, false),
            (11, true),
            (12, false),
            (13, true),
            (14, false),
            (15, false),
            (16, false),
            (17, true),
            (18, false),
            (19, true),
            (20, false),
            (21, false),
            (22, false),
            (23, true),
            (24, false),
            (25, false),
            (26, false),
            (27, false),
            (28, false),
            (29, true),
            (30, false),
            (31, true),
            (32, false),
            (33, false),
            (34, false),
            (35, false),
            (36, false),
            (37, true),
            (38, false),
            (39, false),
            (40, false),
            (41, true),
            (42, false),
            (43, true),
            (44, false),
            (45, false),
            (46, false),
            (47, true),
            (48, false),
            (49, false),
            (50, false),
            (51, false),
            (52, false),
            (53, true),
            (54, false),
            (55, false),
            (56, false),
            (57, false),
            (58, false),
            (59, true),
            (60, false),
            (61, true),
            (62, false),
            (63, false),
            (64, false),
            (65, false),
            (66, false),
            (67, true),
            (68, false),
            (69, false),
            (70, false),
            (71, true),
            (72, false),
            (73, true),
            (74, false),
            (75, false),
            (76, false),
            (77, false),
            (78, false),
            (79, true),
            (80, false),
            (81, false),
            (82, false),
            (83, true),
            (84, false),
            (85, false),
            (86, false),
            (87, false),
            (88, false),
            (89, true),
            (90, false),
            (91, false),
            (92, false),
            (93, false),
            (94, false),
            (95, false),
            (96, false),
            (97, true),
            (98, false),
            (99, false),
            (100, false),
        ];

        for &(n, expected) in test_cases {
            assert_eq!(n.is_prime(), expected, "failed to case: {}", n);
        }
    }

    #[test]
    fn test_smoke_all_types() {
        assert!(13i8.is_prime());
        assert!(13i16.is_prime());
        assert!(13i32.is_prime());
        assert!(13i64.is_prime());
        assert!(13i128.is_prime());
        assert!(13isize.is_prime());
        assert!(13u8.is_prime());
        assert!(13u16.is_prime());
        assert!(13u32.is_prime());
        assert!(13u64.is_prime());
        assert!(13u128.is_prime());
        assert!(13usize.is_prime());
    }
}
