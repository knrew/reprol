//! 最小値演算
//!
//! 最小値を演算とする冪等かつ可換なモノイド．
//! 単位元は各型の最大値(`T::MAX`)．
//!
//! # Examples
//!
//! ```
//! use reprol::ops::monoid::Monoid;
//! use reprol::ops::op_min::OpMin;
//!
//! let m = OpMin::<i64>::default();
//! assert_eq!(m.op(&3, &5), 3);
//! assert_eq!(m.op(&m.id(), &42), 42);
//! ```

use std::marker::PhantomData;

use crate::ops::monoid::{CommutativeMonoid, IdempotentMonoid, Monoid};

/// 最小値演算
///
/// 二項演算として `min` を，単位元として型の最大値を持つ冪等かつ可換なモノイド．
/// 標準のプリミティブ整数型に対応する．
#[derive(Default, Clone, Copy)]
pub struct OpMin<T>(PhantomData<T>);

impl<T> Monoid for OpMin<T>
where
    T: Copy + Ord + HasMaxValue,
{
    type Element = T;

    #[inline]
    fn op(&self, &lhs: &Self::Element, &rhs: &Self::Element) -> Self::Element {
        lhs.min(rhs)
    }

    #[inline]
    fn id(&self) -> Self::Element {
        T::MAX
    }
}

impl<T> IdempotentMonoid for OpMin<T> where T: Copy + Ord + HasMaxValue {}

impl<T> CommutativeMonoid for OpMin<T> where T: Copy + Ord + HasMaxValue {}

/// 型固有の最大値を提供するトレイト．
trait HasMaxValue {
    const MAX: Self;
}

macro_rules! impl_has_max_value_inner {
    ($ty: ty) => {
        impl HasMaxValue for $ty {
            const MAX: Self = Self::MAX;
        }
    };
}

macro_rules! impl_has_max_value {
    ($($ty: ty),* $(,)?) => {
        $( impl_has_max_value_inner!($ty); )*
    };
}

impl_has_max_value! {
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
        let m = OpMin::<i64>::default();
        assert_eq!(m.op(&3, &7), 3);
        assert_eq!(m.op(&1030, &5), 5);
    }

    #[test]
    fn test_id_returns_max() {
        let m = OpMin::<i64>::default();
        assert_eq!(m.id(), i64::MAX);
    }

    #[test]
    fn test_op_identity() {
        let m = OpMin::<i64>::default();
        assert_eq!(m.op(&m.id(), &1030), 1030);
        assert_eq!(m.op(&1030, &m.id()), 1030);
    }

    #[test]
    fn test_op_commutativity() {
        let m = OpMin::<i64>::default();
        assert_eq!(m.op(&3, &1030), m.op(&1030, &3));
    }

    #[test]
    fn test_op_associativity() {
        let m = OpMin::<i64>::default();
        let (a, b, c) = (3, 1030, 7);
        assert_eq!(m.op(&m.op(&a, &b), &c), m.op(&a, &m.op(&b, &c)));
    }

    #[test]
    fn test_op_idempotency() {
        let m = OpMin::<i64>::default();
        assert_eq!(m.op(&1030, &1030), 1030);
    }

    #[test]
    fn test_op_boundary() {
        let m = OpMin::<i64>::default();
        assert_eq!(m.op(&i64::MIN, &i64::MAX), i64::MIN);
        assert_eq!(m.op(&i64::MAX, &i64::MIN), i64::MIN);
        assert_eq!(m.op(&i64::MIN, &i64::MIN), i64::MIN);
    }

    #[test]
    fn test_op_smoke_all_types() {
        macro_rules! test {
            ($ty: ty) => {
                assert_commutative_monoid::<OpMin<$ty>>();
                assert_idempotent_monoid::<OpMin<$ty>>();
                let m = OpMin::<$ty>::default();
                assert_eq!(m.op(&3, &7), 3);
                assert_eq!(m.id(), <$ty>::MAX);
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
