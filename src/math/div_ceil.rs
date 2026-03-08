//! 切り上げ除算(div_ceil)
//!
//! 整数の切り上げ除算を計算するトレイト．
//!
//! # Examples
//!
//! ```
//! use reprol::math::div_ceil::DivCeilCompat;
//!
//! assert_eq!(7u64.div_ceil_compat(3), 3);
//! assert_eq!(6u64.div_ceil_compat(3), 2);
//! assert_eq!((-7i64).div_ceil_compat(2), -3);
//! assert_eq!(7i64.checked_div_ceil_compat(0), None);
//! ```
//!
//! # Notes
//!
//! 標準ライブラリの提供状況に依存せず，全プリミティブ整数型で統一的に使える除算インターフェースを提供する．
//! `div_ceil_compat`は前提条件違反でpanicし，`checked_div_ceil_compat`はゼロ除算・オーバーフロー時に`None`を返す．

/// 切り上げ除算を行うトレイト．
pub trait DivCeilCompat: Sized {
    /// `self` を `rhs` で割った商を正の無限大方向に丸めて返す．
    ///
    /// # Panics
    ///
    /// - `rhs == 0` の場合
    /// - 符号付き整数型で`self`が最小値かつ`rhs == -1`の場合
    fn div_ceil_compat(self, rhs: Self) -> Self;

    /// `self` を `rhs` で割った商を正の無限大方向に丸めて返す．
    ///
    /// オーバーフローまたはゼロ除算の場合は`None`を返す．
    fn checked_div_ceil_compat(self, rhs: Self) -> Option<Self>;
}

macro_rules! impl_div_ceil_compat_inner {
    ($ty: ty) => {
        impl DivCeilCompat for $ty {
            fn div_ceil_compat(self, rhs: Self) -> Self {
                assert!(rhs != 0);
                let q = self / rhs;
                let r = self % rhs;
                #[allow(unused_comparisons)]
                if r != 0 && (self >= 0) == (rhs >= 0) {
                    q + 1
                } else {
                    q
                }
            }

            fn checked_div_ceil_compat(self, rhs: Self) -> Option<Self> {
                let q = self.checked_div(rhs)?;
                let r = self.checked_rem(rhs)?;
                #[allow(unused_comparisons)]
                if r != 0 && (self >= 0) == (rhs >= 0) {
                    q.checked_add(1)
                } else {
                    Some(q)
                }
            }
        }
    };
}

macro_rules! impl_div_ceil_compat {
    ($($ty: ty),* $(,)?) => {
        $( impl_div_ceil_compat_inner!($ty); )*
    };
}

impl_div_ceil_compat! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// i128不等式による数学的検証
    ///
    /// - `b > 0`: `(q-1)*b < a <= q*b`
    /// - `b < 0`: `(q-1)*b > a >= q*b`
    fn verify_div_ceil_i128(a: i128, b: i128, q: i128) {
        assert_ne!(b, 0);
        if b > 0 {
            assert!(
                (q - 1) * b < a && a <= q * b,
                "ceil({a}/{b})={q} violates (q-1)*b < a <= q*b: lhs={}, rhs={}",
                (q - 1) * b,
                q * b,
            );
        } else {
            assert!(
                (q - 1) * b > a && a >= q * b,
                "ceil({a}/{b})={q} violates (q-1)*b > a >= q*b: lhs={}, rhs={}",
                (q - 1) * b,
                q * b,
            );
        }
    }

    #[test]
    fn test_smoke_all_integer_types() {
        macro_rules! test {
            ($ty: ty) => {
                let lhs: $ty = 7;
                let rhs: $ty = 3;
                let expected: $ty = 3;
                assert_eq!(lhs.div_ceil_compat(rhs), expected, stringify!($ty));
                assert_eq!(
                    lhs.checked_div_ceil_compat(rhs),
                    Some(expected),
                    stringify!($ty)
                );
                assert_eq!(lhs.checked_div_ceil_compat(0), None, stringify!($ty));
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
    fn test_div_ceil_compat_signed_branch_matrix() {
        // r!=0かつ同符号: q+1
        assert_eq!(7i64.div_ceil_compat(3), 3, "7/3");
        // r!=0かつ異符号: q
        assert_eq!(7i64.div_ceil_compat(-3), -2, "7/(-3)");
        // r!=0かつ異符号: q
        assert_eq!((-7i64).div_ceil_compat(3), -2, "(-7)/3");
        // r!=0かつ同符号: q+1
        assert_eq!((-7i64).div_ceil_compat(-3), 3, "(-7)/(-3)");
        // r==0: q
        assert_eq!(6i64.div_ceil_compat(3), 2, "6/3");
        // self==0: q=0
        assert_eq!(0i64.div_ceil_compat(3), 0, "0/3");
    }

    #[test]
    fn test_div_ceil_compat_unsigned_branch_matrix() {
        // r==0
        assert_eq!(6u64.div_ceil_compat(3), 2, "6/3");
        // r!=0
        assert_eq!(7u64.div_ceil_compat(3), 3, "7/3");
        // self==0
        assert_eq!(0u64.div_ceil_compat(3), 0, "0/3");
        // 境界値
        assert_eq!(u64::MAX.div_ceil_compat(1), u64::MAX, "MAX/1");
        assert_eq!(u64::MAX.div_ceil_compat(u64::MAX), 1, "MAX/MAX");
    }

    #[test]
    fn test_checked_div_ceil_compat_overflow_min_div_minus_one_large_signed() {
        assert_eq!(
            i64::MIN.checked_div_ceil_compat(-1),
            None,
            "{}/(-1)",
            i64::MIN
        );
        assert_eq!(
            i128::MIN.checked_div_ceil_compat(-1),
            None,
            "{}/(-1)",
            i128::MIN
        );
    }

    #[test]
    fn test_div_ceil_compat_exhaustive_i8() {
        for a in i8::MIN..=i8::MAX {
            for b in i8::MIN..=i8::MAX {
                let checked = a.checked_div_ceil_compat(b);
                if b == 0 || (a == i8::MIN && b == -1) {
                    assert_eq!(checked, None, "case: a={a}, b={b} should be None");
                    continue;
                }
                let unchecked = a.div_ceil_compat(b);
                assert_eq!(checked, Some(unchecked), "case: a={a}, b={b}");
                verify_div_ceil_i128(a as i128, b as i128, unchecked as i128);
            }
        }
    }

    #[test]
    fn test_div_ceil_compat_exhaustive_u8() {
        for a in 0..=u8::MAX {
            for b in 0..=u8::MAX {
                let checked = a.checked_div_ceil_compat(b);
                if b == 0 {
                    assert_eq!(checked, None, "case: a={a}, b={b} should be None");
                    continue;
                }
                let unchecked = a.div_ceil_compat(b);
                assert_eq!(checked, Some(unchecked), "case: a={a}, b={b}");
                verify_div_ceil_i128(a as i128, b as i128, unchecked as i128);
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_div_ceil_compat_panic_divide_by_zero() {
        let _ = 1i64.div_ceil_compat(0);
    }

    #[test]
    #[should_panic]
    fn test_div_ceil_compat_panic_overflow_min_div_minus_one() {
        let _ = i64::MIN.div_ceil_compat(-1);
    }
}
