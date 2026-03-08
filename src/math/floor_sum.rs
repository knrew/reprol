//! floor sum
//!
//! `sum_{i=0}^{n-1} floor((a*i + b) / m)` を計算するトレイト．
//!
//! 計算量: O(log(|a| + |m|))
//!
//! # Examples
//!
//! ```
//! use reprol::math::floor_sum::FloorSum;
//!
//! assert_eq!(i64::floor_sum(4, 3, 2, 1), 4);
//! assert_eq!(i64::checked_floor_sum(4, 3, 2, 1), Some(4));
//! assert_eq!(i64::checked_floor_sum(4, 0, 2, 1), None);
//! ```
//!
//! # Notes
//!
//! 内部計算は`i128`で行い，結果を元の型に変換する．
//! 入力は`[i64::MIN, i64::MAX]`の範囲を前提とする．
//! `floor_sum`は不正入力(制約外，`m == 0`，符号付き整数型で`n < 0`)やオーバーフローでpanicし，`checked_floor_sum`は`None`を返す．
//! また，`i128`実装では中間項を`i128`で逐次加算しているため，入力が`[i64::MIN, i64::MAX]`内でも，中間計算でオーバーフロー検知されて`None`/panicとなる場合がある(最終的な厳密値は`i128`に収まる場合を含む)．

use std::{mem::swap, ops::RangeInclusive};

fn checked_floor_sum_i128_inner(
    mut n: i128,
    mut m: i128,
    mut a: i128,
    mut b: i128,
) -> Option<i128> {
    // `n * (n - 1) / 2`
    fn checked_n_choose_2_i128(n: i128) -> Option<i128> {
        if n <= 1 {
            return Some(0);
        }
        let n_minus_one = n.checked_sub(1)?;
        if n % 2 == 0 {
            n.checked_div(2)?.checked_mul(n_minus_one)
        } else {
            n.checked_mul(n_minus_one.checked_div(2)?)
        }
    }

    debug_assert!(n >= 0, "precondition violation: n must be non-negative");
    debug_assert!(m > 0, "precondition violation: m must be positive");

    let mut res = 0i128;

    loop {
        let a_quot = a.div_euclid(m);
        if a_quot != 0 {
            let term = checked_n_choose_2_i128(n)?.checked_mul(a_quot)?;
            res = res.checked_add(term)?;
        }
        a = a.rem_euclid(m);

        let b_quot = b.div_euclid(m);
        if b_quot != 0 {
            let term = b_quot.checked_mul(n)?;
            res = res.checked_add(term)?;
        }
        b = b.rem_euclid(m);

        let y_max = a.checked_mul(n)?.checked_add(b)?;
        if y_max < m {
            return Some(res);
        }

        n = y_max.div_euclid(m);
        b = y_max.rem_euclid(m);
        swap(&mut m, &mut a);
    }
}

fn checked_floor_sum_i128(n: i128, mut m: i128, mut a: i128, mut b: i128) -> Option<i128> {
    const RANGE: RangeInclusive<i128> = i64::MIN as i128..=i64::MAX as i128;
    if !(RANGE.contains(&n) && RANGE.contains(&m) && RANGE.contains(&a) && RANGE.contains(&b)) {
        return None;
    }

    if n < 0 || m == 0 {
        return None;
    }

    if m < 0 {
        m = m.checked_neg()?;
        a = a.checked_neg()?;
        b = b.checked_neg()?;
    }

    checked_floor_sum_i128_inner(n, m, a, b)
}

/// floor sumを計算するトレイト．
pub trait FloorSum: Sized {
    /// `sum_{i=0}^{n-1} floor((a*i + b) / m)` を返す．
    ///
    /// # Panics
    ///
    /// - 入力が`[i64::MIN, i64::MAX]`の範囲外の場合
    /// - `m == 0` の場合
    /// - 符号付き整数型で `n < 0` の場合
    /// - 結果がオーバーフローする場合
    fn floor_sum(n: Self, m: Self, a: Self, b: Self) -> Self {
        FloorSum::checked_floor_sum(n, m, a, b)
            .expect("floor_sum failed: invalid input or overflow")
    }

    /// `sum_{i=0}^{n-1} floor((a*i + b) / m)` を返す．
    ///
    /// オーバーフローまたは不正な引数の場合は`None`を返す．
    /// 入力制約`[i64::MIN, i64::MAX]`の範囲外も`None`となる．
    /// `i128`実装では，中間計算でのオーバーフロー検知により，最終結果が`i128`範囲内でも`None`となる場合がある．
    fn checked_floor_sum(n: Self, m: Self, a: Self, b: Self) -> Option<Self>;
}

impl FloorSum for i128 {
    fn checked_floor_sum(n: Self, m: Self, a: Self, b: Self) -> Option<Self> {
        checked_floor_sum_i128(n, m, a, b)
    }
}

macro_rules! impl_floor_sum_via_as_inner {
    ($ty: ty) => {
        impl FloorSum for $ty {
            fn checked_floor_sum(n: Self, m: Self, a: Self, b: Self) -> Option<Self> {
                checked_floor_sum_i128(n as i128, m as i128, a as i128, b as i128)
                    .and_then(|res| Self::try_from(res).ok())
            }
        }
    };
}

macro_rules! impl_floor_sum_via_try_from_inner {
    ($ty: ty) => {
        impl FloorSum for $ty {
            fn checked_floor_sum(n: Self, m: Self, a: Self, b: Self) -> Option<Self> {
                let n = i128::try_from(n).ok()?;
                let m = i128::try_from(m).ok()?;
                let a = i128::try_from(a).ok()?;
                let b = i128::try_from(b).ok()?;
                checked_floor_sum_i128(n, m, a, b).and_then(|res| Self::try_from(res).ok())
            }
        }
    };
}

macro_rules! impl_floor_sum {
    (as: [$($ty_as:ty),* $(,)?], try_from: [$($ty_try:ty),* $(,)?] $(,)?) => {
        $( impl_floor_sum_via_as_inner!($ty_as); )*
        $( impl_floor_sum_via_try_from_inner!($ty_try); )*
    };
}

impl_floor_sum! {
    as:       [i8, i16, i32, i64, u8, u16, u32, u64],
    try_from: [isize, u128, usize],
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    use crate::{math::div_floor::DivFloorCompat, utils::test_utils::random::get_test_rng};

    /// O(n)の愚直実装
    fn naive_floor_sum(n: i128, m: i128, a: i128, b: i128) -> i128 {
        (0..n).map(|i| (a * i + b).div_floor_compat(m)).sum()
    }

    // ========== スモークテスト ==========

    #[test]
    fn test_smoke_all_integer_types() {
        macro_rules! test {
            ($ty:ty) => {
                assert_eq!(
                    <$ty>::floor_sum(4, 3, 2, 1),
                    4,
                    "{}: floor_sum(4, 3, 2, 1)",
                    stringify!($ty)
                );
                assert_eq!(
                    <$ty>::checked_floor_sum(4, 3, 2, 1),
                    Some(4),
                    "{}: checked_floor_sum(4, 3, 2, 1)",
                    stringify!($ty)
                );
                assert_eq!(
                    <$ty>::checked_floor_sum(4, 0, 2, 1),
                    None,
                    "{}: checked_floor_sum(4, 0, 2, 1)",
                    stringify!($ty)
                );
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

    // ========== 基本ケース(分岐通過) ==========

    #[test]
    fn test_floor_sum_basic() {
        // a >= m でa_quotが非ゼロになるケース
        assert_eq!(
            i64::floor_sum(5, 3, 7, 1) as i128,
            naive_floor_sum(5, 3, 7, 1),
            "a_quot != 0"
        );

        // b >= m でb_quotが非ゼロになるケース
        assert_eq!(
            i64::floor_sum(5, 3, 1, 7) as i128,
            naive_floor_sum(5, 3, 1, 7),
            "b_quot != 0"
        );

        // 内部ループが複数回反復するケース
        assert_eq!(
            i64::floor_sum(10, 7, 5, 3) as i128,
            naive_floor_sum(10, 7, 5, 3),
            "multiple loop iterations"
        );
    }

    // ========== 符号付き入力ケース ==========

    #[test]
    fn test_floor_sum_signed_inputs() {
        let cases: &[(i128, i128, i128, i128)] = &[
            (5, 3, -2, 1),
            (5, 3, 2, -1),
            (5, 3, -2, -1),
            (7, 4, -3, 5),
            (10, 7, -5, -3),
        ];
        for &(n, m, a, b) in cases {
            assert_eq!(
                i64::floor_sum(n as i64, m as i64, a as i64, b as i64) as i128,
                naive_floor_sum(n, m, a, b),
                "signed: floor_sum({n}, {m}, {a}, {b})"
            );
        }
    }

    // ========== エッジケース ==========

    #[test]
    fn test_floor_sum_edge_cases() {
        // n=0: 空の和
        assert_eq!(i64::floor_sum(0, 1030, 1030, 1030), 0, "n=0");

        // n=1: 単一項 | floor(7/3) = 2
        assert_eq!(i64::floor_sum(1, 3, 1030, 7), 2, "n=1");

        // a=0: 全項同一 | 5 * floor(7/3) = 10
        assert_eq!(i64::floor_sum(5, 3, 0, 7), 10, "a=0");

        // b=0
        assert_eq!(
            i64::floor_sum(5, 3, 2, 0) as i128,
            naive_floor_sum(5, 3, 2, 0),
            "b=0"
        );

        // m=1: closed-form | a*n*(n-1)/2 + b*n = 3*5*4/2 + 2*5 = 40
        assert_eq!(i64::floor_sum(5, 1, 3, 2), 40, "m=1");
    }

    // ========== 負のm ==========

    #[test]
    fn test_floor_sum_negative_m() {
        let cases: &[(i128, i128, i128, i128)] = &[
            (5, -3, 2, 1),
            (3, -7, 4, -2),
            (10, -5, 3, 7),
            (7, -4, -3, 5),
            (8, -6, -2, -3),
            (1, -1, 0, 0),
            (5, -1, 3, 2),
        ];
        for &(n, m, a, b) in cases {
            let expected = naive_floor_sum(n, m, a, b);
            assert_eq!(
                i128::floor_sum(n, m, a, b),
                expected,
                "negative m: floor_sum({n}, {m}, {a}, {b})"
            );
        }
    }

    // ========== 負のm + b == i64::MIN ==========

    #[test]
    fn test_floor_sum_negative_m_b_i64_min() {
        let b = i64::MIN as i128;

        // b = i64::MIN, m = -2, a = 0, n = 1
        let expected = naive_floor_sum(1, -2, 0, b);
        assert_eq!(
            i128::checked_floor_sum(1, -2, 0, b),
            Some(expected),
            "n=1, m=-2, a=0, b=i64::MIN"
        );

        // b = i64::MIN, m = -3, a = 1, n = 2
        let expected = naive_floor_sum(2, -3, 1, b);
        assert_eq!(
            i128::checked_floor_sum(2, -3, 1, b),
            Some(expected),
            "n=2, m=-3, a=1, b=i64::MIN"
        );
    }

    #[test]
    fn test_floor_sum_negative_m_i64_min_checked() {
        let n = 2i128;
        let m = i64::MIN as i128;
        let a = i64::MIN as i128;
        let b = i64::MIN as i128;
        let expected = naive_floor_sum(n, m, a, b);

        assert_eq!(
            i128::checked_floor_sum(n, m, a, b),
            Some(expected),
            "negative m extreme: checked_floor_sum({n}, {m}, {a}, {b})"
        );
    }

    // ========== None返却(入力不正) ==========

    #[test]
    fn test_checked_floor_sum_returns_none_invalid_input() {
        // m == 0
        assert_eq!(
            i64::checked_floor_sum(5, 0, 2, 1),
            None,
            "m == 0 should return None"
        );

        // n < 0
        assert_eq!(
            i64::checked_floor_sum(-1, 1, 0, 0),
            None,
            "n < 0 should return None"
        );
    }

    // ========== None返却(入力制約外) ==========

    #[test]
    fn test_checked_floor_sum_returns_none_out_of_i64_range() {
        let over = (i64::MAX as i128) + 1;
        let under = (i64::MIN as i128) - 1;

        assert_eq!(
            i128::checked_floor_sum(over, 1, 0, 0),
            None,
            "n > i64::MAX should return None"
        );
        assert_eq!(
            i128::checked_floor_sum(1, over, 0, 0),
            None,
            "m > i64::MAX should return None"
        );
        assert_eq!(
            i128::checked_floor_sum(1, 1, over, 0),
            None,
            "a > i64::MAX should return None"
        );
        assert_eq!(
            i128::checked_floor_sum(1, 1, 0, over),
            None,
            "b > i64::MAX should return None"
        );
        assert_eq!(
            i128::checked_floor_sum(under, 1, 0, 0),
            None,
            "n < i64::MIN should return None"
        );
        assert_eq!(
            i128::checked_floor_sum(1, under, 0, 0),
            None,
            "m < i64::MIN should return None"
        );
        assert_eq!(
            i128::checked_floor_sum(1, 1, under, 0),
            None,
            "a < i64::MIN should return None"
        );
        assert_eq!(
            i128::checked_floor_sum(1, 1, 0, under),
            None,
            "b < i64::MIN should return None"
        );
    }

    // ========== None返却(内部オーバーフロー) ==========

    #[test]
    fn test_checked_floor_sum_returns_none_overflow() {
        // 型オーバーフロー: 結果がi8の範囲を超える
        assert_eq!(
            i8::checked_floor_sum(100, 1, 100, 0),
            None,
            "i8 overflow should return None"
        );

        // i128オーバーフロー: 非常に大きい結果
        assert_eq!(
            i128::checked_floor_sum(i64::MAX as i128, 1, i64::MAX as i128, 0),
            None,
            "i128 overflow should return None"
        );
    }

    // ========== None返却(制約外入力: 型変換失敗を含む) ==========

    #[test]
    fn test_checked_floor_sum_returns_none_out_of_i64_range_in_wide_types() {
        // u128: 入力がi64に収まらない(入力制約違反)
        assert_eq!(
            u128::checked_floor_sum((i64::MAX as u128) + 1, 1, 0, 0),
            None,
            "u128 n exceeds i64::MAX should return None"
        );
        assert_eq!(
            u128::checked_floor_sum(1, (i64::MAX as u128) + 1, 0, 0),
            None,
            "u128 m exceeds i64::MAX should return None"
        );
        assert_eq!(
            u128::checked_floor_sum(1, 1, (i64::MAX as u128) + 1, 0),
            None,
            "u128 a exceeds i64::MAX should return None"
        );
        assert_eq!(
            u128::checked_floor_sum(1, 1, 0, (i64::MAX as u128) + 1),
            None,
            "u128 b exceeds i64::MAX should return None"
        );

        // isize: 結果が型範囲外(try_from結果変換失敗)
        assert_eq!(
            isize::checked_floor_sum(isize::MAX, 1, 1, 0),
            None,
            "isize result overflow should return None"
        );

        // usize: 入力がi64に収まらない(入力制約違反)
        assert_eq!(
            usize::checked_floor_sum(usize::MAX, 1, 1, 0),
            None,
            "usize input exceeds i64::MAX should return None"
        );
    }

    // ========== 全探索 (i8) ==========

    #[test]
    fn test_floor_sum_exhaustive_small_signed() {
        for n in 0..=7i128 {
            for m in -7..=7i128 {
                if m == 0 {
                    continue;
                }
                for a in -7..=7i128 {
                    for b in -7..=7i128 {
                        let expected = naive_floor_sum(n, m, a, b);
                        if let Ok(expected_i8) = i8::try_from(expected) {
                            assert_eq!(
                                i8::checked_floor_sum(n as i8, m as i8, a as i8, b as i8),
                                Some(expected_i8),
                                "i8 checked: ({n}, {m}, {a}, {b})"
                            );
                            assert_eq!(
                                i8::floor_sum(n as i8, m as i8, a as i8, b as i8),
                                expected_i8,
                                "i8 floor_sum: ({n}, {m}, {a}, {b})"
                            );
                        } else {
                            assert_eq!(
                                i8::checked_floor_sum(n as i8, m as i8, a as i8, b as i8),
                                None,
                                "i8 overflow: ({n}, {m}, {a}, {b}) = {expected}"
                            );
                        }
                    }
                }
            }
        }
    }

    // ========== 全探索 (u8) ==========

    #[test]
    fn test_floor_sum_exhaustive_small_unsigned() {
        for n in 0..=7i128 {
            for m in 1..=7i128 {
                for a in 0..=7i128 {
                    for b in 0..=7i128 {
                        let expected = naive_floor_sum(n, m, a, b);
                        if let Ok(expected_u8) = u8::try_from(expected) {
                            assert_eq!(
                                u8::checked_floor_sum(n as u8, m as u8, a as u8, b as u8),
                                Some(expected_u8),
                                "u8 checked: ({n}, {m}, {a}, {b})"
                            );
                            assert_eq!(
                                u8::floor_sum(n as u8, m as u8, a as u8, b as u8),
                                expected_u8,
                                "u8 floor_sum: ({n}, {m}, {a}, {b})"
                            );
                        } else {
                            assert_eq!(
                                u8::checked_floor_sum(n as u8, m as u8, a as u8, b as u8),
                                None,
                                "u8 overflow: ({n}, {m}, {a}, {b}) = {expected}"
                            );
                        }
                    }
                }
            }
        }
    }

    // ========== ランダムテスト (i64) ==========

    #[test]
    fn test_floor_sum_random_signed() {
        let mut rng = get_test_rng();
        for _ in 0..500 {
            let n = rng.random_range(0..=100i64);
            let m = loop {
                let v = rng.random_range(-1030..=1030i64);
                if v != 0 {
                    break v;
                }
            };
            let a = rng.random_range(-1030..=1030i64);
            let b = rng.random_range(-1030..=1030i64);
            let expected = naive_floor_sum(n as i128, m as i128, a as i128, b as i128);
            assert_eq!(
                i64::checked_floor_sum(n, m, a, b),
                Some(expected as i64),
                "random signed: ({n}, {m}, {a}, {b})"
            );
        }
    }

    // ========== ランダムテスト (u64) ==========

    #[test]
    fn test_floor_sum_random_unsigned() {
        let mut rng = get_test_rng();
        for _ in 0..500 {
            let n = rng.random_range(0..=100u64);
            let m = rng.random_range(1..=1030u64);
            let a = rng.random_range(0..=1030u64);
            let b = rng.random_range(0..=1030u64);
            let expected = naive_floor_sum(n as i128, m as i128, a as i128, b as i128);
            assert_eq!(
                u64::floor_sum(n, m, a, b) as i128,
                expected,
                "random unsigned: ({n}, {m}, {a}, {b})"
            );
        }
    }

    // ========== 代数的性質 ==========

    #[test]
    fn test_floor_sum_algebraic_m1_closed_form() {
        // エッジケースの m=1 テストと補完する形で，複数の(n, a, b)の組で検証
        let cases: &[(i128, i128, i128)] = &[
            (0, 5, 3),    // n=0
            (1, 5, 3),    // n=1
            (10, 0, 7),   // a=0
            (10, 3, 0),   // b=0
            (10, -3, 5),  // 負のa
            (10, 3, -5),  // 負のb
            (10, -3, -5), // 負のa,b
            (50, 7, 11),  // やや大きいn
        ];
        for &(n, a, b) in cases {
            assert_eq!(
                i128::floor_sum(n, 1, a, b),
                a * n * (n - 1) / 2 + b * n,
                "m=1 closed-form: n={n}, a={a}, b={b}"
            );
        }
    }

    // ========== ランダムテスト (代数的性質) ==========

    #[test]
    fn test_floor_sum_random_algebraic_properties() {
        let mut rng = get_test_rng();
        for _ in 0..300 {
            let n = rng.random_range(0..=50i128);
            let m = loop {
                let v = rng.random_range(-50..=50i128);
                if v != 0 {
                    break v;
                }
            };
            let a = rng.random_range(-50..=50i128);
            let b = rng.random_range(-50..=50i128);

            // b-shift: floor_sum(n, m, a, b+m) == floor_sum(n, m, a, b) + n
            assert_eq!(
                i128::floor_sum(n, m, a, b + m),
                i128::floor_sum(n, m, a, b) + n,
                "random b-shift: ({n}, {m}, {a}, {b})"
            );

            // a-shift: floor_sum(n, m, a+m, b) == floor_sum(n, m, a, b) + n*(n-1)/2
            assert_eq!(
                i128::floor_sum(n, m, a + m, b),
                i128::floor_sum(n, m, a, b) + n * (n - 1) / 2,
                "random a-shift: ({n}, {m}, {a}, {b})"
            );
        }
    }

    // ========== should_panic ==========

    #[test]
    #[should_panic]
    fn test_floor_sum_panic_m_zero() {
        i64::floor_sum(5, 0, 2, 1);
    }

    #[test]
    #[should_panic]
    fn test_floor_sum_panic_n_negative() {
        i64::floor_sum(-1, 1, 0, 0);
    }

    #[test]
    #[should_panic]
    fn test_floor_sum_panic_overflow_i8() {
        i8::floor_sum(100, 1, 100, 0);
    }

    #[test]
    #[should_panic]
    fn test_floor_sum_panic_overflow_i128() {
        i128::floor_sum(i64::MAX as i128, 1, i64::MAX as i128, 0);
    }

    #[test]
    #[should_panic]
    fn test_floor_sum_panic_overflow_with_m_i64_min() {
        i64::floor_sum(i64::MAX, i64::MIN, i64::MIN, i64::MIN);
    }

    #[test]
    #[should_panic]
    fn test_floor_sum_panic_out_of_i64_range_i128() {
        i128::floor_sum((i64::MAX as i128) + 1, 1, 0, 0);
    }

    #[test]
    #[should_panic]
    fn test_floor_sum_panic_u128_exceeds_i64_max() {
        u128::floor_sum((i64::MAX as u128) + 1, 1, 0, 0);
    }

    // ========== 代数的性質: 負のm変換 ==========

    #[test]
    fn test_floor_sum_random_negative_m_identity() {
        // S(n, -p, a, b) = -S(n, p, a, b-1) - n  (p > 0)
        // 負のm分岐の式変形ロジックを性質として検証する
        let mut rng = get_test_rng();
        for _ in 0..300 {
            let n = rng.random_range(0..=50i128);
            let p = rng.random_range(1..=50i128);
            let a = rng.random_range(-50..=50i128);
            let b = rng.random_range(-50..=50i128);

            let s_neg = i128::floor_sum(n, -p, a, b);
            let s_pos = i128::floor_sum(n, p, a, b - 1);

            assert_eq!(
                s_neg,
                -s_pos - n,
                "negative m identity: S({n}, -{p}, {a}, {b}) should equal -S({n}, {p}, {a}, {}) - {n}",
                b - 1
            );
        }
    }
}
