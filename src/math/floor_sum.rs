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
//! `floor_sum`はオーバーフローや不正な引数でpanicし，`checked_floor_sum`は`None`を返す．
//! `u128`/`usize`/`isize`では，`i128`に変換できない入力(例: `u128`で`i128::MAX`超)は`None`となる．

use std::mem::swap;

fn checked_floor_sum_i128(mut n: i128, mut m: i128, mut a: i128, mut b: i128) -> Option<i128> {
    // `n * (n - 1) / 2`
    fn checked_n_choose_2(n: i128) -> Option<i128> {
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

    // `0 <= a, b < m`, `0 <= n`, `0 < m` のとき，`a*n + b` を `m` で割った商と余りを返す．
    //
    // `a*n + b` は`i128`でオーバーフローし得るため，繰り返し二乗法で分解計算する．
    fn checked_mul_add_div_rem_nonneg_i128(
        a: i128,
        n: i128,
        b: i128,
        m: i128,
    ) -> Option<(i128, i128)> {
        debug_assert!(0 <= a && a < m);
        debug_assert!(0 <= b && b < m);
        debug_assert!(0 <= n);
        debug_assert!(0 < m);

        let a = u128::try_from(a).ok()?;
        let mut n = u128::try_from(n).ok()?;
        let b = u128::try_from(b).ok()?;
        let m = u128::try_from(m).ok()?;

        // 累積値: q*m + r = a*n + b
        let mut q = 0u128;
        let mut r = b;

        // 現在項: term_q*m + term_r = a*2^k
        let mut term_q = 0;
        let mut term_r = a;

        while n > 0 {
            if n & 1 == 1 {
                q = q.checked_add(term_q)?;
                let sum_r = r.checked_add(term_r)?;
                if sum_r >= m {
                    r = sum_r.checked_sub(m)?;
                    q = q.checked_add(1)?;
                } else {
                    r = sum_r;
                }
            }

            n >>= 1;

            if n == 0 {
                break;
            }

            term_q = term_q.checked_mul(2)?;
            let doubled_r = term_r.checked_mul(2)?;
            if doubled_r >= m {
                term_r = doubled_r.checked_sub(m)?;
                term_q = term_q.checked_add(1)?;
            } else {
                term_r = doubled_r;
            }
        }

        let q = i128::try_from(q).ok()?;
        let r = i128::try_from(r).ok()?;

        Some((q, r))
    }

    if n < 0 || m == 0 {
        return None;
    }

    if m < 0 {
        let p = m.checked_neg()?;

        // floor(x / -p) = -floor((x - 1) / p) - 1
        if let Some(b_minus_one) = b.checked_sub(1) {
            let s = checked_floor_sum_i128(n, p, a, b_minus_one)?;
            return s.checked_neg()?.checked_sub(n);
        }

        // b == i128::MIN のとき:
        // floor(x / -p) = -floor((x + p - 1) / p)
        let b_plus_p_minus_one = b.checked_add(p.checked_sub(1)?)?;
        let s = checked_floor_sum_i128(n, p, a, b_plus_p_minus_one)?;
        return s.checked_neg();
    }

    let mut res = 0i128;

    loop {
        let a_quot = a.div_euclid(m);
        if a_quot != 0 {
            res = res.checked_add(a_quot.checked_mul(checked_n_choose_2(n)?)?)?;
        }
        a = a.rem_euclid(m);

        let b_quot = b.div_euclid(m);
        if b_quot != 0 {
            res = res.checked_add(b_quot.checked_mul(n)?)?;
        }
        b = b.rem_euclid(m);

        let (next_n, next_b) = checked_mul_add_div_rem_nonneg_i128(a, n, b, m)?;
        if next_n == 0 {
            return Some(res);
        }

        n = next_n;
        b = next_b;
        swap(&mut m, &mut a);
    }
}

/// floor sumを計算するトレイト．
pub trait FloorSum: Sized {
    /// `sum_{i=0}^{n-1} floor((a*i + b) / m)` を返す．
    ///
    /// # Panics
    ///
    /// - `m == 0` の場合
    /// - 符号付き整数型で `n < 0` の場合
    /// - 結果がオーバーフローする場合
    /// - `u128`/`usize`/`isize`で入力が`i128`に変換できない場合
    fn floor_sum(n: Self, m: Self, a: Self, b: Self) -> Self {
        FloorSum::checked_floor_sum(n, m, a, b).expect(
            "floor_sum failed: invalid input (m == 0 or n < 0) or overflow/conversion error",
        )
    }

    /// `sum_{i=0}^{n-1} floor((a*i + b) / m)` を返す．
    ///
    /// オーバーフローまたは不正な引数の場合は`None`を返す．
    /// `u128`/`usize`/`isize`では，`i128`に変換できない入力も`None`となる．
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

    #[test]
    fn test_checked_floor_sum_i128_prevent_false_overflow_in_y_max() {
        assert_eq!(
            i128::floor_sum(2, i128::MAX, -1, 0),
            -1,
            "i128 boundary: floor_sum(2, i128::MAX, -1, 0) should be -1"
        );
        assert_eq!(
            i128::checked_floor_sum(2, i128::MAX, -1, 0),
            Some(-1),
            "i128 boundary: checked_floor_sum(2, i128::MAX, -1, 0) should be Some(-1)"
        );
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

    // ========== 負のm + b == i128::MIN ==========

    #[test]
    fn test_floor_sum_negative_m_b_i128_min() {
        // b = i128::MIN, m = -2, a = 0, n = 1
        let expected = naive_floor_sum(1, -2, 0, i128::MIN);
        assert_eq!(
            i128::checked_floor_sum(1, -2, 0, i128::MIN),
            Some(expected),
            "n=1, m=-2, a=0, b=i128::MIN"
        );

        // b = i128::MIN, m = -3, a = 1, n = 2
        let expected = naive_floor_sum(2, -3, 1, i128::MIN);
        assert_eq!(
            i128::checked_floor_sum(2, -3, 1, i128::MIN),
            Some(expected),
            "n=2, m=-3, a=1, b=i128::MIN"
        );

        // b = i128::MIN, m = -1
        assert_eq!(
            i128::checked_floor_sum(1, -1, 0, i128::MIN),
            None,
            "neg overflow: n=1, m=-1, a=0, b=i128::MIN"
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

        // m == i128::MIN
        assert_eq!(
            i128::checked_floor_sum(1, i128::MIN, 0, 0),
            None,
            "m == i128::MIN should return None"
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
            i128::checked_floor_sum(i128::MAX, 1, i128::MAX, 0),
            None,
            "i128 overflow should return None"
        );
    }

    // ========== None返却(型変換失敗) ==========

    #[test]
    fn test_checked_floor_sum_returns_none_type_conversion() {
        // u128: 入力がi128に収まらない(try_from変換失敗)
        assert_eq!(
            u128::checked_floor_sum((i128::MAX as u128) + 1, 1, 0, 0),
            None,
            "u128 n exceeds i128::MAX should return None"
        );
        assert_eq!(
            u128::checked_floor_sum(1, (i128::MAX as u128) + 1, 0, 0),
            None,
            "u128 m exceeds i128::MAX should return None"
        );
        assert_eq!(
            u128::checked_floor_sum(1, 1, (i128::MAX as u128) + 1, 0),
            None,
            "u128 a exceeds i128::MAX should return None"
        );
        assert_eq!(
            u128::checked_floor_sum(1, 1, 0, (i128::MAX as u128) + 1),
            None,
            "u128 b exceeds i128::MAX should return None"
        );

        // isize/usize: 結果が型範囲外(try_from結果変換失敗)
        assert_eq!(
            isize::checked_floor_sum(isize::MAX, 1, 1, 0),
            None,
            "isize result overflow should return None"
        );
        assert_eq!(
            usize::checked_floor_sum(usize::MAX, 1, 1, 0),
            None,
            "usize result overflow should return None"
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
        i128::floor_sum(i128::MAX, 1, i128::MAX, 0);
    }

    #[test]
    #[should_panic]
    fn test_floor_sum_panic_u128_exceeds_i128_max() {
        u128::floor_sum((i128::MAX as u128) + 1, 1, 0, 0);
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
