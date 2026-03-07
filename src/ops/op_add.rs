//! 加算演算
//!
//! 加算を演算とするアーベル群．
//! 単位元はゼロ値(整数型では`0`，`ModInt`では`ModInt::new(0)`)．
//!
//! # Examples
//!
//! ```
//! use reprol::ops::monoid::Monoid;
//! use reprol::ops::group::Group;
//! use reprol::ops::op_add::OpAdd;
//!
//! let m = OpAdd::<i64>::default();
//! assert_eq!(m.op(&3, &5), 8);
//! assert_eq!(m.op(&m.id(), &42), 42);
//! assert_eq!(m.inv(&3), -3);
//! assert_eq!(m.op(&10, &m.inv(&3)), 7);
//! ```
//!
//! # Notes
//!
//! 整数型では wrapping 演算を用いるため，オーバーフロー時は折り返す．

use std::marker::PhantomData;

use crate::{
    math::modint::ModInt,
    ops::{
        group::{AbelianGroup, Group},
        monoid::{CommutativeMonoid, Monoid},
    },
};

/// 加算演算
///
/// 二項演算として加算を，単位元としてゼロ値を持つアーベル群．
/// 標準のプリミティブ整数型と[`ModInt`]に対応する．
///
/// # Notes
///
/// 整数型では wrapping 演算を用いるため，オーバーフロー時は折り返す．
#[derive(Default, Clone, Copy)]
pub struct OpAdd<T>(PhantomData<T>);

impl<T> Monoid for OpAdd<T>
where
    T: Copy + HasZeroValue + HasAdd,
{
    type Element = T;

    #[inline]
    fn id(&self) -> Self::Element {
        T::ZERO
    }

    #[inline]
    fn op(&self, &lhs: &Self::Element, &rhs: &Self::Element) -> Self::Element {
        lhs.add(rhs)
    }
}

impl<T> CommutativeMonoid for OpAdd<T> where T: Copy + HasZeroValue + HasAdd {}

impl<T> Group for OpAdd<T>
where
    T: Copy + HasZeroValue + HasAdd + HasNeg,
{
    #[inline]
    fn inv(&self, &x: &Self::Element) -> Self::Element {
        x.neg()
    }
}

impl<T> AbelianGroup for OpAdd<T> where T: Copy + HasZeroValue + HasAdd + HasNeg {}

/// 型固有のゼロ値を提供するトレイト．
trait HasZeroValue {
    const ZERO: Self;
}

/// 型固有の加算を提供するトレイト．
trait HasAdd {
    fn add(self, rhs: Self) -> Self;
}

/// 型固有の符号反転を提供するトレイト．
trait HasNeg {
    fn neg(self) -> Self;
}

macro_rules! impl_op_add_traits_inner {
    ($ty: ty) => {
        impl HasZeroValue for $ty {
            const ZERO: Self = 0;
        }

        impl HasAdd for $ty {
            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                self.wrapping_add(rhs)
            }
        }

        impl HasNeg for $ty {
            #[inline(always)]
            fn neg(self) -> Self {
                self.wrapping_neg()
            }
        }
    };
}

macro_rules! impl_op_add_traits {
    ($($ty: ty),* $(,)?) => {
        $( impl_op_add_traits_inner!($ty); )*
    };
}

impl_op_add_traits! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

impl<const P: u64> HasZeroValue for ModInt<P> {
    const ZERO: Self = Self::new(0);
}

impl<const P: u64> HasAdd for ModInt<P> {
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
}

impl<const P: u64> HasNeg for ModInt<P> {
    #[inline(always)]
    fn neg(self) -> Self {
        -self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::{
        group::{AbelianGroup, Group},
        monoid::Monoid,
    };

    fn assert_abelian_group<T: AbelianGroup>() {}

    // ========== 符号なし整数 (u64) ==========

    #[test]
    fn test_op_basic_unsigned() {
        let m = OpAdd::<u64>::default();
        assert_eq!(m.op(&74, &33), 107, "74 + 33 = 107");
        assert_eq!(m.op(&18, &65), 83, "18 + 65 = 83");
        assert_eq!(m.op(&1030, &7), 1037, "1030 + 7 = 1037");
    }

    #[test]
    fn test_id_returns_zero() {
        let m = OpAdd::<u64>::default();
        assert_eq!(m.id(), 0);
    }

    #[test]
    fn test_op_identity_unsigned() {
        let m = OpAdd::<u64>::default();
        assert_eq!(m.op(&m.id(), &1030), 1030, "id + x = x");
        assert_eq!(m.op(&1030, &m.id()), 1030, "x + id = x");
    }

    #[test]
    fn test_op_commutativity_unsigned() {
        let m = OpAdd::<u64>::default();
        assert_eq!(m.op(&3, &1030), m.op(&1030, &3), "a + b = b + a");
    }

    #[test]
    fn test_op_associativity_unsigned() {
        let m = OpAdd::<u64>::default();
        let (a, b, c) = (3, 1030, 7);
        assert_eq!(
            m.op(&m.op(&a, &b), &c),
            m.op(&a, &m.op(&b, &c)),
            "(a + b) + c = a + (b + c)"
        );
    }

    #[test]
    fn test_inv_unsigned() {
        let m = OpAdd::<u64>::default();
        for x in [0, 1, 1030, u64::MAX] {
            let inv_x = m.inv(&x);
            assert_eq!(inv_x, x.wrapping_neg(), "inv({x}) = wrapping_neg({x})");
            assert_eq!(m.op(&x, &inv_x), m.id(), "x + inv(x) = id for x={x}");
            assert_eq!(m.op(&inv_x, &x), m.id(), "inv(x) + x = id for x={x}");
        }
    }

    #[test]
    fn test_op_boundary_unsigned() {
        let m = OpAdd::<u64>::default();
        assert_eq!(m.op(&0, &0), 0, "0 + 0 = 0");
        assert_eq!(m.op(&u64::MAX, &0), u64::MAX, "MAX + 0 = MAX");
        assert_eq!(m.op(&0, &u64::MAX), u64::MAX, "0 + MAX = MAX");
        assert_eq!(m.op(&u64::MAX, &1), 0, "MAX + 1 wraps to 0");
        assert_eq!(
            m.op(&u64::MAX, &u64::MAX),
            u64::MAX.wrapping_add(u64::MAX),
            "MAX + MAX wraps"
        );
    }

    // ========== 符号付き整数 (i64) ==========

    #[test]
    fn test_op_basic_signed() {
        let m = OpAdd::<i64>::default();
        assert_eq!(m.op(&22, &-4), 18, "22 + (-4) = 18");
        assert_eq!(m.op(&-7, &10), 3, "(-7) + 10 = 3");
        assert_eq!(m.op(&-8, &-55), -63, "(-8) + (-55) = -63");
    }

    #[test]
    fn test_op_identity_signed() {
        let m = OpAdd::<i64>::default();
        let id = m.id();
        assert_eq!(m.op(&id, &-1030), -1030, "id + (-1030) = -1030");
        assert_eq!(m.op(&-1030, &id), -1030, "(-1030) + id = -1030");
    }

    #[test]
    fn test_op_commutativity_signed() {
        let m = OpAdd::<i64>::default();
        assert_eq!(
            m.op(&-3, &1030),
            m.op(&1030, &-3),
            "(-3) + 1030 = 1030 + (-3)"
        );
    }

    #[test]
    fn test_op_associativity_signed() {
        let m = OpAdd::<i64>::default();
        let (a, b, c) = (-3i64, 1030, 7);
        assert_eq!(
            m.op(&m.op(&a, &b), &c),
            m.op(&a, &m.op(&b, &c)),
            "((-3) + 1030) + 7 = (-3) + (1030 + 7)"
        );
    }

    #[test]
    fn test_inv_signed() {
        let m = OpAdd::<i64>::default();
        // 正・負・0
        assert_eq!(m.inv(&0), 0, "inv(0) = 0");
        assert_eq!(m.inv(&1030), -1030, "inv(1030) = -1030");
        assert_eq!(m.inv(&-1030), 1030, "inv(-1030) = 1030");
        // 境界値
        assert_eq!(m.inv(&i64::MAX), -i64::MAX, "inv(MAX) = -MAX");
        assert_eq!(
            m.inv(&i64::MIN),
            i64::MIN.wrapping_neg(),
            "inv(MIN) = wrapping_neg(MIN)"
        );
        // inv の逆元性
        for x in [0, 1030, -1030, i64::MIN, i64::MAX] {
            let inv_x = m.inv(&x);
            assert_eq!(m.op(&x, &inv_x), m.id(), "x + inv(x) = id for x={x}");
            assert_eq!(m.op(&inv_x, &x), m.id(), "inv(x) + x = id for x={x}");
        }
    }

    #[test]
    fn test_op_boundary_signed() {
        let m = OpAdd::<i64>::default();
        assert_eq!(m.op(&i64::MIN, &i64::MAX), -1, "MIN + MAX = -1");
        assert_eq!(m.op(&i64::MIN, &-1), i64::MAX, "MIN + (-1) wraps to MAX");
        assert_eq!(m.op(&i64::MAX, &1), i64::MIN, "MAX + 1 wraps to MIN");
        assert_eq!(
            m.op(&i64::MIN, &i64::MIN),
            i64::MIN.wrapping_add(i64::MIN),
            "MIN + MIN wraps"
        );
    }

    // ========== 全探索 (u8) ==========

    #[test]
    fn test_op_exhaustive_u8() {
        assert_abelian_group::<OpAdd<u8>>();
        let m = OpAdd::<u8>::default();
        for a in 0..=u8::MAX {
            assert_eq!(m.op(&m.id(), &a), a, "id + {a} = {a}");
            assert_eq!(m.op(&a, &m.inv(&a)), m.id(), "a + inv(a) = id for a={a}");
            assert_eq!(m.op(&m.inv(&a), &a), m.id(), "inv(a) + a = id for a={a}");
            for b in 0..=u8::MAX {
                assert_eq!(m.op(&a, &b), a.wrapping_add(b), "{a} + {b}");
            }
        }
    }

    // ========== ModInt ==========

    #[test]
    fn test_op_abelian_group_modint() {
        assert_abelian_group::<OpAdd<ModInt<998_244_353>>>();
    }

    #[test]
    fn test_op_basic_modint() {
        let m = OpAdd::<ModInt<998_244_353>>::default();
        let a = ModInt::new(3);
        let b = ModInt::new(5);
        assert_eq!(m.op(&a, &b), ModInt::new(8), "3 + 5 = 8 (mod P)");
        // mod演算の確認
        let x = ModInt::new(998_244_350);
        let y = ModInt::new(10);
        assert_eq!(
            m.op(&x, &y),
            ModInt::new(7),
            "998244350 + 10 = 7 (mod 998244353)"
        );
    }

    #[test]
    fn test_op_identity_modint() {
        let m = OpAdd::<ModInt<998_244_353>>::default();
        let id = m.id();
        assert_eq!(id, ModInt::new(0), "id = ModInt::new(0)");
        let x = ModInt::new(1030);
        assert_eq!(m.op(&id, &x), x, "id + x = x");
        assert_eq!(m.op(&x, &id), x, "x + id = x");
    }

    #[test]
    fn test_inv_modint() {
        let m = OpAdd::<ModInt<998_244_353>>::default();
        let x = ModInt::new(1030);
        let inv_x = m.inv(&x);
        assert_eq!(inv_x, -x, "inv(x) = -x");
        assert_eq!(m.op(&x, &inv_x), m.id(), "x + inv(x) = id");
        assert_eq!(m.op(&inv_x, &x), m.id(), "inv(x) + x = id");
        // inv(0) = 0
        let zero = ModInt::new(0);
        assert_eq!(m.inv(&zero), zero, "inv(0) = 0");
    }

    #[test]
    fn test_op_commutativity_modint() {
        let m = OpAdd::<ModInt<998_244_353>>::default();
        let a = ModInt::new(1030);
        let b = ModInt::new(998_244_000);
        assert_eq!(m.op(&a, &b), m.op(&b, &a), "a + b = b + a (mod P)");
    }

    #[test]
    fn test_op_associativity_modint() {
        let m = OpAdd::<ModInt<998_244_353>>::default();
        let a = ModInt::new(1030);
        let b = ModInt::new(998_244_000);
        let c = ModInt::new(7);
        assert_eq!(
            m.op(&m.op(&a, &b), &c),
            m.op(&a, &m.op(&b, &c)),
            "(a + b) + c = a + (b + c) (mod P)"
        );
    }

    // ========== 全型スモークテスト ==========

    #[test]
    fn test_op_smoke_all_integer_types() {
        macro_rules! test {
            ($ty: ty) => {
                assert_abelian_group::<OpAdd<$ty>>();
                let m = OpAdd::<$ty>::default();
                assert_eq!(m.op(&3, &5), 8, "op for {}", stringify!($ty));
                assert_eq!(m.id(), 0, "id for {}", stringify!($ty));
                let inv_3 = m.inv(&3);
                assert_eq!(
                    m.op(&3, &inv_3),
                    m.id(),
                    "3 + inv(3) = id for {}",
                    stringify!($ty)
                );
            };
        }

        macro_rules! test_signed {
            ($ty: ty) => {
                test!($ty);
                let m = OpAdd::<$ty>::default();
                assert_eq!(m.op(&-3, &5), 2, "op(-3, 5) for {}", stringify!($ty));
                assert_eq!(m.inv(&-1), 1, "inv(-1) for {}", stringify!($ty));
                let x: $ty = -3;
                assert_eq!(
                    m.op(&x, &m.inv(&x)),
                    m.id(),
                    "x + inv(x) = id for {}",
                    stringify!($ty)
                );
            };
        }

        test_signed!(i8);
        test_signed!(i16);
        test_signed!(i32);
        test_signed!(i64);
        test_signed!(i128);
        test_signed!(isize);
        test!(u8);
        test!(u16);
        test!(u32);
        test!(u64);
        test!(u128);
        test!(usize);
    }
}
