//! 素数判定
//!
//! 整数が素数であるかを判定する．
//!
//! 計算量: O(sqrt n)
//!
//! # Examples
//!
//! ```
//! use reprol::math::is_prime::IsPrime;
//!
//! assert!(7u64.is_prime());
//! assert!(!1u64.is_prime());
//! ```
//!
//! # Notes
//!
//! - 符号付き整数型では，負の値に対して常に`false`を返す．

/// 素数判定トレイト．
pub trait IsPrime {
    /// 素数であれば`true`を返す．
    fn is_prime(&self) -> bool;
}

macro_rules! impl_is_prime_inner {
    ($ty: ty) => {
        impl IsPrime for $ty {
            fn is_prime(&self) -> bool {
                let n = *self;

                if n < 2 {
                    return false;
                }
                if n <= 3 {
                    return true;
                }
                if n % 2 == 0 || n % 3 == 0 {
                    return false;
                }

                let mut i = 5;
                while i <= n / i {
                    if n % i == 0 || n % (i + 2) == 0 {
                        return false;
                    }

                    i += 6;
                }

                true
            }
        }
    };
}

macro_rules! impl_is_prime {
    ($($ty:ty),* $(,)?) => {
        $( impl_is_prime_inner!($ty); )*
    };
}

impl_is_prime! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive_is_prime_u128(n: u128) -> bool {
        if n < 2 {
            return false;
        }
        let mut i = 2u128;
        while i * i <= n {
            if n % i == 0 {
                return false;
            }
            i += 1;
        }
        true
    }

    #[test]
    fn test_smoke_all_types() {
        macro_rules! test {
            ($ty: ty) => {
                assert!((13 as $ty).is_prime(), stringify!($ty));
                assert!(!(30 as $ty).is_prime(), stringify!($ty));
            };
        }

        test!(i8);
        test!(i16);
        test!(i32);
        test!(i64);
        test!(i128);
        test!(isize);
        test!(u8);
        test!(u16);
        test!(u32);
        test!(u64);
        test!(u128);
        test!(usize);
    }

    #[test]
    fn test_is_prime_signed_boundary_and_small_values() {
        // 負値
        assert!(!(-1_i32).is_prime(), "n=-1");
        assert!(!i8::MIN.is_prime(), "n={}", i8::MIN);
        assert!(!isize::MIN.is_prime(), "n={}", isize::MIN);

        assert!(!0_i32.is_prime(), "n=0");
        assert!(!1_i32.is_prime(), "n=1");
        assert!(2_i32.is_prime(), "n=2");
        assert!(3_i32.is_prime(), "n=3");
    }

    #[test]
    fn test_is_prime_unsigned_boundary_and_small_values() {
        assert!(!0_u32.is_prime(), "n=0");
        assert!(!1_u32.is_prime(), "n=1");
        assert!(2_u32.is_prime(), "n=2");
        assert!(3_u32.is_prime(), "n=3");

        assert!(!u8::MAX.is_prime(), "n=u8::MAX({})", u8::MAX);
        assert!(!u128::MAX.is_prime(), "n=u128::MAX({})", u128::MAX);
        assert!(!usize::MAX.is_prime(), "n=usize::MAX({})", usize::MAX);
    }

    #[test]
    fn test_is_prime_branch_divisible_by_2_or_3() {
        // 偶数
        for n in [4_u32, 6, 8, 100] {
            assert!(!n.is_prime(), "n={n}");
        }

        // 3の倍数
        for n in [9_u32, 15, 21, 27] {
            assert!(!n.is_prime(), "n={n}");
        }
    }

    #[test]
    fn test_is_prime_branch_6k_minus_1_and_6k_plus_1_composites() {
        // 6k±1形式の合成数
        assert!(!25_u32.is_prime(), "n=25");
        assert!(!49_u32.is_prime(), "n=49");
        assert!(!35_u32.is_prime(), "n=35");

        // n%(i+2)==0 補足
        assert!(!77_u32.is_prime(), "n=77");

        // 素数
        assert!(29_u32.is_prime(), "n=29");
        assert!(31_u32.is_prime(), "n=31");
        assert!(37_u32.is_prime(), "n=37");
    }

    #[test]
    fn test_is_prime_exhaustive_small_range_against_naive() {
        for n in 0..=10_000_u32 {
            let expected = naive_is_prime_u128(n as u128);
            assert_eq!(n.is_prime(), expected, "n={n}");
        }

        for n in -1000..=10_000_i32 {
            let expected = n >= 0 && naive_is_prime_u128(n as u128);
            assert_eq!(n.is_prime(), expected, "n={n}");
        }
    }
}
