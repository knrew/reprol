//! GCD(最大公約数)演算
//!
//! GCD(最大公約数)を演算とする冪等かつ可換なモノイド．
//! 単位元は各型のゼロ値(`0`)．
//!
//! # Examples
//!
//! ```
//! use reprol::ops::monoid::Monoid;
//! use reprol::ops::op_gcd::OpGcd;
//!
//! let m = OpGcd::<u64>::default();
//! assert_eq!(m.op(&12, &8), 4);
//! assert_eq!(m.op(&m.id(), &42), 42);
//! ```
//!
//! # Notes
//!
//! - 本演算は非負整数専用である．
//! - `op` に負の値を渡すとpanicする．

use std::marker::PhantomData;

use crate::{
    math::gcd::Gcd,
    ops::monoid::{CommutativeMonoid, IdempotentMonoid, Monoid},
};

/// GCD(最大公約数)演算
///
/// 二項演算として `gcd` を，単位元としてゼロ値を持つ冪等かつ可換なモノイド．
/// 標準のプリミティブ整数型に対応する．
///
/// # Notes
///
/// 非負整数のみを要素として利用することを想定している．
///
/// # Panics
///
/// `op` に負の値を渡した場合にpanicする．
#[derive(Default, Clone, Copy)]
pub struct OpGcd<T>(PhantomData<T>);

impl<T> Monoid for OpGcd<T>
where
    T: Copy + Ord + Gcd + HasZeroValue,
{
    type Element = T;

    #[inline]
    fn op(&self, &lhs: &Self::Element, &rhs: &Self::Element) -> Self::Element {
        assert!(lhs >= T::ZERO && rhs >= T::ZERO);
        lhs.gcd(rhs)
    }

    #[inline]
    fn id(&self) -> Self::Element {
        T::ZERO
    }
}

impl<T> IdempotentMonoid for OpGcd<T> where T: Copy + Ord + Gcd + HasZeroValue {}

impl<T> CommutativeMonoid for OpGcd<T> where T: Copy + Ord + Gcd + HasZeroValue {}

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

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_idempotent_monoid<T: IdempotentMonoid>() {}
    fn assert_commutative_monoid<T: CommutativeMonoid>() {}

    #[test]
    fn test_op_basic() {
        let m = OpGcd::<u64>::default();
        assert_eq!(m.op(&12, &8), 4, "gcd(12, 8) = 4");
        assert_eq!(m.op(&1030, &15), 5, "gcd(1030, 15) = 5");
    }

    #[test]
    fn test_id_returns_zero() {
        let m = OpGcd::<u64>::default();
        assert_eq!(m.id(), 0);
    }

    #[test]
    fn test_op_identity() {
        let m = OpGcd::<u64>::default();
        assert_eq!(m.op(&m.id(), &1030), 1030, "gcd(id, x) = x");
        assert_eq!(m.op(&1030, &m.id()), 1030, "gcd(x, id) = x");
    }

    #[test]
    fn test_op_commutativity() {
        let m = OpGcd::<u64>::default();
        assert_eq!(m.op(&12, &1030), m.op(&1030, &12), "gcd(a, b) = gcd(b, a)");
    }

    #[test]
    fn test_op_associativity() {
        let m = OpGcd::<u64>::default();
        let (a, b, c) = (12, 1030, 18);
        assert_eq!(
            m.op(&m.op(&a, &b), &c),
            m.op(&a, &m.op(&b, &c)),
            "gcd(gcd(a, b), c) = gcd(a, gcd(b, c))"
        );
    }

    #[test]
    fn test_op_idempotency() {
        let m = OpGcd::<u64>::default();
        assert_eq!(m.op(&1030, &1030), 1030, "gcd(x, x) = x");
    }

    #[test]
    fn test_op_boundary() {
        let m = OpGcd::<u64>::default();
        assert_eq!(m.op(&0, &u64::MAX), u64::MAX, "gcd(0, MAX) = MAX");
        assert_eq!(m.op(&u64::MAX, &0), u64::MAX, "gcd(MAX, 0) = MAX");
        assert_eq!(m.op(&u64::MAX, &u64::MAX), u64::MAX, "gcd(MAX, MAX) = MAX");
    }

    #[test]
    #[should_panic]
    fn test_op_panics_on_negative_lhs() {
        let m = OpGcd::<i64>::default();
        m.op(&-1, &2);
    }

    #[test]
    #[should_panic]
    fn test_op_panics_on_negative_rhs() {
        let m = OpGcd::<i64>::default();
        m.op(&2, &-1);
    }

    #[test]
    fn test_op_smoke_all_types() {
        macro_rules! test {
            ($ty: ty) => {
                assert_idempotent_monoid::<OpGcd<$ty>>();
                assert_commutative_monoid::<OpGcd<$ty>>();
                let m = OpGcd::<$ty>::default();
                assert_eq!(m.op(&12, &8), 4, "op for {}", stringify!($ty));
                assert_eq!(m.id(), 0, "id for {}", stringify!($ty));
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
}
