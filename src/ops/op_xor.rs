//! XOR演算
//!
//! XOR(排他的論理和)を演算とする群．
//! 単位元は各型のゼロ値(整数型では`0`，`bool`では`false`)．
//!
//! # Examples
//!
//! ```
//! use reprol::ops::monoid::Monoid;
//! use reprol::ops::group::Group;
//! use reprol::ops::op_xor::OpXor;
//!
//! let m = OpXor::<u64>::default();
//! assert_eq!(m.op(&3, &5), 6);
//! assert_eq!(m.op(&m.id(), &1030), 1030);
//! // XORの逆元は自身
//! assert_eq!(m.inv(&42), 42);
//! assert_eq!(m.op(&42, &m.inv(&42)), m.id());
//! ```

use std::{marker::PhantomData, ops::BitXor};

use crate::ops::{
    group::{AbelianGroup, Group},
    monoid::{CommutativeMonoid, Monoid},
};

/// XOR演算
///
/// 二項演算として `^`(XOR) を，単位元としてゼロ値を持つ群．
/// 標準のプリミティブ整数型と`bool`に対応する．
#[derive(Default, Clone, Copy)]
pub struct OpXor<T>(PhantomData<T>);

impl<T> Monoid for OpXor<T>
where
    T: Copy + BitXor<Output = T> + HasZeroValue,
{
    type Element = T;

    #[inline]
    fn op(&self, &lhs: &Self::Element, &rhs: &Self::Element) -> Self::Element {
        lhs ^ rhs
    }

    #[inline]
    fn id(&self) -> Self::Element {
        T::ZERO
    }
}

impl<T> CommutativeMonoid for OpXor<T> where T: Copy + BitXor<Output = T> + HasZeroValue {}

impl<T> Group for OpXor<T>
where
    T: Copy + BitXor<Output = T> + HasZeroValue,
{
    #[inline]
    fn inv(&self, &x: &Self::Element) -> Self::Element {
        x
    }
}

impl<T> AbelianGroup for OpXor<T> where T: Copy + BitXor<Output = T> + HasZeroValue {}

/// 型固有のゼロ値を提供するトレイト．
trait HasZeroValue {
    const ZERO: Self;
}

macro_rules! impl_has_zero_value_inner {
    ($ty: ty) => {
        impl HasZeroValue for $ty {
            const ZERO: $ty = 0;
        }
    };
}

macro_rules! impl_has_zero_value {
    ($($ty: ty),* $(,)?) => {
        $( impl_has_zero_value_inner!($ty); )*
    };
}

impl_has_zero_value! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

impl HasZeroValue for bool {
    const ZERO: bool = false;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::{
        group::{AbelianGroup, Group},
        monoid::Monoid,
    };

    fn assert_abelian_group<T: AbelianGroup>() {}

    #[test]
    fn test_op_basic_unsigned() {
        let m = OpXor::<u64>::default();
        assert_eq!(m.op(&3, &5), 6, "3 ^ 5 = 6");
        assert_eq!(m.op(&0xFF, &0x0F), 0xF0, "0xFF ^ 0x0F = 0xF0");
    }

    #[test]
    fn test_id_returns_zero() {
        let m = OpXor::<u64>::default();
        assert_eq!(m.id(), 0);
    }

    #[test]
    fn test_op_identity_unsigned() {
        let m = OpXor::<u64>::default();
        assert_eq!(m.op(&m.id(), &1030), 1030, "id ^ x = x");
        assert_eq!(m.op(&1030, &m.id()), 1030, "x ^ id = x");
    }

    #[test]
    fn test_op_commutativity_unsigned() {
        let m = OpXor::<u64>::default();
        assert_eq!(m.op(&3, &1030), m.op(&1030, &3), "a ^ b = b ^ a");
    }

    #[test]
    fn test_op_associativity_unsigned() {
        let m = OpXor::<u64>::default();
        let (a, b, c) = (3, 1030, 7);
        assert_eq!(
            m.op(&m.op(&a, &b), &c),
            m.op(&a, &m.op(&b, &c)),
            "(a ^ b) ^ c = a ^ (b ^ c)"
        );
    }

    #[test]
    fn test_inv_self_inverse_unsigned() {
        let m = OpXor::<u64>::default();
        for x in [0, 1, 42, 1030, u64::MAX] {
            assert_eq!(m.inv(&x), x, "inv(x) = x for x={x}");
            assert_eq!(m.op(&x, &m.inv(&x)), m.id(), "x ^ inv(x) = id for x={x}");
            assert_eq!(m.op(&m.inv(&x), &x), m.id(), "inv(x) ^ x = id for x={x}");
        }
    }

    #[test]
    fn test_op_boundary_unsigned() {
        let m = OpXor::<u64>::default();
        assert_eq!(m.op(&0, &u64::MAX), u64::MAX, "0 ^ MAX = MAX");
        assert_eq!(m.op(&u64::MAX, &0), u64::MAX, "MAX ^ 0 = MAX");
        assert_eq!(m.op(&u64::MAX, &u64::MAX), 0, "MAX ^ MAX = 0");
    }

    #[test]
    fn test_op_basic_signed() {
        let m = OpXor::<i64>::default();
        assert_eq!(m.op(&-1, &1), -2, "-1 ^ 1 = -2");
        assert_eq!(m.op(&-1, &0), -1, "-1 ^ 0 = -1");
    }

    #[test]
    fn test_op_identity_signed() {
        let m = OpXor::<i64>::default();
        let id = m.id();
        assert_eq!(m.op(&id, &-1030), -1030, "id ^ (-1030) = -1030");
        assert_eq!(m.op(&-1030, &id), -1030, "(-1030) ^ id = -1030");
    }

    #[test]
    fn test_op_commutativity_signed() {
        let m = OpXor::<i64>::default();
        assert_eq!(
            m.op(&-3, &1030),
            m.op(&1030, &-3),
            "(-3) ^ 1030 = 1030 ^ (-3)"
        );
    }

    #[test]
    fn test_op_associativity_signed() {
        let m = OpXor::<i64>::default();
        let (a, b, c) = (-3i64, 1030, 7);
        assert_eq!(
            m.op(&m.op(&a, &b), &c),
            m.op(&a, &m.op(&b, &c)),
            "((-3) ^ 1030) ^ 7 = (-3) ^ (1030 ^ 7)"
        );
    }

    #[test]
    fn test_inv_self_inverse_signed() {
        let m = OpXor::<i64>::default();
        for x in [0, -1, -1030, i64::MIN, i64::MAX] {
            assert_eq!(m.inv(&x), x, "inv(x) = x for x={x}");
            assert_eq!(m.op(&x, &m.inv(&x)), m.id(), "x ^ inv(x) = id for x={x}");
            assert_eq!(m.op(&m.inv(&x), &x), m.id(), "inv(x) ^ x = id for x={x}");
        }
    }

    #[test]
    fn test_op_boundary_signed() {
        let m = OpXor::<i64>::default();
        assert_eq!(m.op(&i64::MIN, &i64::MAX), -1, "MIN ^ MAX = -1");
        assert_eq!(m.op(&-1, &-1), 0, "(-1) ^ (-1) = 0");
        assert_eq!(m.op(&i64::MIN, &i64::MIN), 0, "MIN ^ MIN = 0");
        assert_eq!(m.op(&i64::MIN, &0), i64::MIN, "MIN ^ 0 = MIN");
    }

    #[test]
    fn test_op_exhaustive_bool() {
        assert_abelian_group::<OpXor<bool>>();
        let m = OpXor::<bool>::default();
        assert_eq!(m.op(&false, &false), false, "op(false, false) for bool");
        assert_eq!(m.op(&false, &true), true, "op(false, true) for bool");
        assert_eq!(m.op(&true, &false), true, "op(true, false) for bool");
        assert_eq!(m.op(&true, &true), false, "op(true, true) for bool");
        assert_eq!(m.id(), false, "id for bool");
        assert_eq!(m.inv(&true), true, "inv(true) for bool");
        assert_eq!(m.inv(&false), false, "inv(false) for bool");
    }

    #[test]
    fn test_op_exhaustive_u8() {
        assert_abelian_group::<OpXor<u8>>();
        let m = OpXor::<u8>::default();
        for a in 0..=u8::MAX {
            assert_eq!(m.op(&m.id(), &a), a, "id ^ {a} = {a}");
            assert_eq!(m.op(&a, &m.inv(&a)), m.id(), "op(a, inv(a)) = id for a={a}");
            for b in 0..=u8::MAX {
                assert_eq!(m.op(&a, &b), a ^ b, "{a} ^ {b}");
            }
        }
    }

    #[test]
    fn test_op_smoke_all_integer_types() {
        macro_rules! test {
            ($ty: ty) => {
                assert_abelian_group::<OpXor<$ty>>();
                let m = OpXor::<$ty>::default();
                assert_eq!(m.op(&3, &5), 6, "op for {}", stringify!($ty));
                assert_eq!(m.id(), 0, "id for {}", stringify!($ty));
                assert_eq!(m.inv(&7), 7, "inv for {}", stringify!($ty));
            };
        }

        macro_rules! test_signed {
            ($ty: ty) => {
                test!($ty);
                let m = OpXor::<$ty>::default();
                assert_eq!(
                    m.op(&-3, &5),
                    (-3 as $ty) ^ 5,
                    "op(-3, 5) for {}",
                    stringify!($ty)
                );
                assert_eq!(m.inv(&-1), -1, "inv(-1) for {}", stringify!($ty));
                let x: $ty = -3;
                assert_eq!(
                    m.op(&x, &m.inv(&x)),
                    m.id(),
                    "x ^ inv(x) = id for {}",
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
