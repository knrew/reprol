//! 最大値演算
//!
//! 最大値を演算とする冪等かつ可換なモノイド．
//! 単位元は各型の最小値(`T::MIN`)．
//!
//! # Examples
//!
//! ```
//! use reprol::ops::monoid::Monoid;
//! use reprol::ops::op_max::OpMax;
//!
//! let m = OpMax::<i64>::default();
//! assert_eq!(m.op(&3, &5), 5);
//! assert_eq!(m.op(&m.id(), &42), 42);
//! ```

use std::marker::PhantomData;

use crate::ops::monoid::{CommutativeMonoid, IdempotentMonoid, Monoid};

/// 最大値演算
///
/// 二項演算として `max` を，単位元として型の最小値を持つ冪等かつ可換なモノイド．
/// 標準のプリミティブ整数型に対応する．
#[derive(Default, Clone, Copy)]
pub struct OpMax<T>(PhantomData<T>);

impl<T> Monoid for OpMax<T>
where
    T: Copy + Ord + HasMinValue,
{
    type Element = T;

    #[inline]
    fn op(&self, &lhs: &Self::Element, &rhs: &Self::Element) -> Self::Element {
        lhs.max(rhs)
    }

    #[inline]
    fn id(&self) -> Self::Element {
        T::MIN
    }
}

impl<T> IdempotentMonoid for OpMax<T> where T: Copy + Ord + HasMinValue {}

impl<T> CommutativeMonoid for OpMax<T> where T: Copy + Ord + HasMinValue {}

/// 型固有の最小値を提供するトレイト．
trait HasMinValue {
    const MIN: Self;
}

macro_rules! impl_has_min_value_inner {
    ($ty: ty) => {
        impl HasMinValue for $ty {
            const MIN: Self = Self::MIN;
        }
    };
}

macro_rules! impl_has_min_value {
    ($($ty: ty),* $(,)?) => {
        $( impl_has_min_value_inner!($ty); )*
    };
}

impl_has_min_value! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::monoid::{CommutativeMonoid, IdempotentMonoid, Monoid};

    fn assert_commutative_monoid<T: CommutativeMonoid>() {}

    fn assert_idempotent_monoid<T: IdempotentMonoid>() {}

    #[test]
    fn test_op_basic() {
        let m = OpMax::<i64>::default();
        assert_eq!(m.op(&3, &7), 7);
        assert_eq!(m.op(&1030, &5), 1030);
    }

    #[test]
    fn test_id_returns_min() {
        let m = OpMax::<i64>::default();
        assert_eq!(m.id(), i64::MIN);
    }

    #[test]
    fn test_op_identity() {
        let m = OpMax::<i64>::default();
        assert_eq!(m.op(&m.id(), &1030), 1030);
        assert_eq!(m.op(&1030, &m.id()), 1030);
    }

    #[test]
    fn test_op_commutativity() {
        let m = OpMax::<i64>::default();
        assert_eq!(m.op(&3, &1030), m.op(&1030, &3));
    }

    #[test]
    fn test_op_associativity() {
        let m = OpMax::<i64>::default();
        let (a, b, c) = (3, 1030, 7);
        assert_eq!(m.op(&m.op(&a, &b), &c), m.op(&a, &m.op(&b, &c)));
    }

    #[test]
    fn test_op_idempotency() {
        let m = OpMax::<i64>::default();
        assert_eq!(m.op(&1030, &1030), 1030);
    }

    #[test]
    fn test_op_boundary() {
        let m = OpMax::<i64>::default();
        assert_eq!(m.op(&i64::MIN, &i64::MAX), i64::MAX);
        assert_eq!(m.op(&i64::MAX, &i64::MIN), i64::MAX);
        assert_eq!(m.op(&i64::MAX, &i64::MAX), i64::MAX);
    }

    #[test]
    fn test_op_smoke_all_types() {
        macro_rules! test {
            ($ty: ty) => {
                assert_commutative_monoid::<OpMax<$ty>>();
                assert_idempotent_monoid::<OpMax<$ty>>();
                let m = OpMax::<$ty>::default();
                assert_eq!(m.op(&3, &7), 7);
                assert_eq!(m.id(), <$ty>::MIN);
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
