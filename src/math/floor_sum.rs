//! FloorSum
//!
//! $\sum_{i=0}^{n-1} \lfloor \frac{a \cdot i + b}{m} \rfloor$ を計算する．
//! 符号付き整数にも対応．
//!
//! ## 使用例
//! ```
//! use reprol::math::floor_sum::FloorSum;
//! assert_eq!(u64::floor_sum(5, 3, 1, 0), 2);
//! ```
//!
//! ## Panics
//! - `n < 0` の場合
//! - `m <= 0` の場合

use std::mem::swap;

pub trait FloorSum {
    fn floor_sum(n: Self, m: Self, a: Self, b: Self) -> Self;
}

macro_rules! impl_floorsum_unsigned {
    ($ty: ty) => {
        impl FloorSum for $ty {
            fn floor_sum(mut n: Self, mut m: Self, mut a: Self, mut b: Self) -> Self {
                assert!(m != 0);

                let mut res: $ty = 0;

                loop {
                    if a >= m {
                        res = res.wrapping_add(
                            n.wrapping_mul(n.wrapping_sub(1))
                                .wrapping_div(2)
                                .wrapping_mul(a.wrapping_div(m)),
                        );
                        a %= m;
                    }

                    if b >= m {
                        res = res.wrapping_add(n.wrapping_mul(b.wrapping_div(m)));
                        b %= m;
                    }

                    let y_max = a.wrapping_mul(n).wrapping_add(b);

                    if y_max < m {
                        break;
                    }

                    n = y_max.wrapping_div(m);
                    b = y_max.wrapping_rem(m);

                    swap(&mut m, &mut a);
                }

                res
            }
        }
    };
}

macro_rules! impl_floorsum_signed {
    ($ty: ty, $uty: ty) => {
        impl FloorSum for $ty {
            fn floor_sum(n: Self, m: Self, mut a: Self, mut b: Self) -> Self {
                assert!(n >= 0);
                assert!(m > 0);

                let mut res: $ty = 0;

                if a < 0 {
                    let a2 = a.rem_euclid(m);
                    res = res.wrapping_sub(
                        n.wrapping_mul(n.wrapping_sub(1))
                            .wrapping_div(2)
                            .wrapping_mul(a2.wrapping_sub(a).wrapping_div(m)),
                    );
                    a = a2;
                }

                if b < 0 {
                    let b2 = b.rem_euclid(m);
                    res = res.wrapping_sub(n.wrapping_mul(b2.wrapping_sub(b).wrapping_div(m)));
                    b = b2;
                }

                res = res.wrapping_add(<$uty as FloorSum>::floor_sum(
                    n as $uty, m as $uty, a as $uty, b as $uty,
                ) as $ty);

                res
            }
        }
    };
}

macro_rules! impl_floorsum_for {
    (unsigned: [$($u:ty),* $(,)?], signed: [$(($s:ty, $ut:ty)),* $(,)?]$(,)?) => {
        $( impl_floorsum_unsigned!($u); )*
        $( impl_floorsum_signed!($s, $ut); )*
    };
}

impl_floorsum_for! {
    unsigned: [u8, u16, u32, u64, u128, usize],
    signed:   [(i8, u64), (i16, u64), (i32, u64), (i64, u64), (i128, u128), (isize, usize)],
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    use crate::{math::div_floor::DivFloor, utils::test_utils::random::get_test_rng};

    fn floor_sum_naive_u64(n: u64, m: u64, a: u64, b: u64) -> u64 {
        let mut res: u64 = 0;
        for i in 0..n {
            res = res.wrapping_add(
                a.wrapping_mul(i)
                    .wrapping_add(b)
                    .checked_div_floor(m)
                    .expect("overflow"),
            );
        }
        res
    }

    fn floor_sum_naive_i64(n: i64, m: i64, a: i64, b: i64) -> i64 {
        let mut res: i64 = 0;
        for i in 0..n {
            res = res.wrapping_add(
                a.wrapping_mul(i)
                    .wrapping_add(b)
                    .checked_div_floor(m)
                    .expect("overflow"),
            );
        }
        res
    }

    #[test]
    fn test_smoke_all_types() {
        assert_eq!(u64::floor_sum(5, 3, 1, 0), 2);
        assert_eq!(u32::floor_sum(5, 3, 1, 0), 2);
        assert_eq!(u16::floor_sum(5, 3, 1, 0), 2);
        assert_eq!(u8::floor_sum(5, 3, 1, 0), 2);
        assert_eq!(u128::floor_sum(5, 3, 1, 0), 2);
        assert_eq!(usize::floor_sum(5, 3, 1, 0), 2);

        assert_eq!(i64::floor_sum(5, 3, 1, 0), 2);
        assert_eq!(i32::floor_sum(5, 3, 1, 0), 2);
        assert_eq!(i16::floor_sum(5, 3, 1, 0), 2);
        assert_eq!(i8::floor_sum(5, 3, 1, 0), 2);
        assert_eq!(i128::floor_sum(5, 3, 1, 0), 2);
        assert_eq!(isize::floor_sum(5, 3, 1, 0), 2);
    }

    #[test]
    fn test_basic() {
        assert_eq!(u64::floor_sum(0, 5, 12, 12), 0);
        assert_eq!(u64::floor_sum(10, 5, 2, 0), 14);
        assert_eq!(u64::floor_sum(100, 7, 3, 2), 2107);

        assert_eq!(i64::floor_sum(0, 1, 1, 1), 0);
        assert_eq!(i64::floor_sum(10, 3, -2, 1), -30);
        assert_eq!(i64::floor_sum(20, 7, 5, -1), 124);
    }

    #[test]
    fn test_edge_cases() {
        // u64
        assert_eq!(u64::floor_sum(1, 1, 0, 0), 0);
        assert_eq!(u64::floor_sum(1, 1, 1, 0), 0);
        assert_eq!(u64::floor_sum(1, 1, 0, 1), 1);
        assert_eq!(u64::floor_sum(1, 1000000, 1, 0), 0);
        assert_eq!(u64::floor_sum(100, 1, 0, 1), 100);
        assert_eq!(u64::floor_sum(0, 1, 1, 1), 0);

        // i64
        assert_eq!(i64::floor_sum(1, 1, 0, 0), 0);
        assert_eq!(i64::floor_sum(1, 1, 1, 0), 0);
        assert_eq!(i64::floor_sum(1, 1, 0, 1), 1);
        assert_eq!(i64::floor_sum(1, 1000000, 1, 0), 0);
        assert_eq!(i64::floor_sum(100, 1, 0, 1), 100);
    }

    #[test]
    #[should_panic]
    fn test_floor_sum_m_zero_panics_u64() {
        let _ = u64::floor_sum(5, 0, 1, 0);
    }

    #[test]
    #[should_panic]
    fn test_floor_sum_m_zero_panics_i64() {
        let _ = i64::floor_sum(5, 0, 1, 0);
    }

    #[test]
    #[should_panic]
    fn test_floor_sum_negative_n_panics_i64() {
        let _ = i64::floor_sum(-1, 3, 1, 0);
    }

    #[test]
    #[should_panic]
    fn test_floor_sum_negative_m_panics_i64() {
        let _ = i64::floor_sum(5, -3, 1, 0);
    }

    #[test]
    fn test_floor_sum_large_values() {
        assert_eq!(
            u128::floor_sum(1000, 1000000007, 1000000005, 1000000000),
            499500
        );
        assert_eq!(
            i128::floor_sum(1000, 1000000007, 1000000005, 1000000000),
            499500
        );
    }

    macro_rules! random_test_unsigned {
        ($test_name:ident, $ty: ty, $num_testcases: expr, $n_max: expr, $m_max: expr, $a_min: expr, $a_max: expr, $b_min: expr, $b_max: expr) => {
            #[test]
            fn $test_name() {
                assert!($m_max > 0);
                assert!($a_min <= $a_max);
                assert!($b_min <= $b_max);

                let mut rng = get_test_rng();

                for _ in 0..$num_testcases {
                    let n = rng.random_range(0..=$n_max);
                    let m = rng.random_range(1..=$m_max);
                    let a = rng.random_range($a_min..=$a_max);
                    let b = rng.random_range($b_min..=$b_max);
                    assert_eq!(
                        <$ty>::floor_sum(n, m, a, b),
                        floor_sum_naive_u64(n as u64, m as u64, a as u64, b as u64) as $ty
                    );
                }
            }
        };
    }

    macro_rules! random_test_signed {
        ($test_name:ident, $ty: ty, $num_testcases: expr, $n_max: expr, $m_max: expr, $a_min: expr, $a_max: expr, $b_min: expr, $b_max: expr) => {
            #[test]
            fn $test_name() {
                assert!($m_max > 0);
                assert!($a_min <= $a_max);
                assert!($b_min <= $b_max);

                let mut rng = get_test_rng();

                for _ in 0..$num_testcases {
                    let n = rng.random_range(0..=$n_max);
                    let m = rng.random_range(1..=$m_max);
                    let a = rng.random_range($a_min..=$a_max);
                    let b = rng.random_range($b_min..=$b_max);
                    assert_eq!(
                        <$ty>::floor_sum(n, m, a, b),
                        floor_sum_naive_i64(n as i64, m as i64, a as i64, b as i64) as $ty
                    );
                }
            }
        };
    }

    random_test_unsigned!(
        test_random_u32,
        u32,
        100,    // T
        200,    // n_max
        10000,  // m_max
        0,      // a_min
        100000, // a_max
        0,      // b_min
        100000  // b_max
    );

    random_test_unsigned!(
        test_random_u64,
        u64,
        100,        // T
        200,        // n_max
        1000000,    // m_max
        0,          // a_min
        1000000000, // a_max
        0,          // b_min
        1000000000  // b_max
    );

    random_test_signed!(
        test_random_i32,
        i32,
        100,     // T
        200,     // n_max
        10000,   // m_max
        -100000, // a_min
        100000,  // a_max
        -100000, // b_min
        100000   // b_max
    );

    random_test_signed!(
        test_random_i64,
        i64,
        100,         // T
        200,         // n_max
        1000000,     // m_max
        -1000000000, // a_min
        1000000000,  // a_max
        -1000000000, // b_min
        1000000000   // b_max
    );
}
