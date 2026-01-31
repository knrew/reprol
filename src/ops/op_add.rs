use std::marker::PhantomData;

use crate::{
    math::modint::ModInt,
    ops::{group::Group, monoid::Monoid},
};

#[derive(Default, Clone)]
pub struct OpAdd<T> {
    phantom: PhantomData<T>,
}

impl<T: Copy + OpAddUtils> Monoid for OpAdd<T> {
    type Value = T;

    #[inline]
    fn identity(&self) -> Self::Value {
        T::ZERO
    }

    #[inline]
    fn op(&self, &lhs: &Self::Value, &rhs: &Self::Value) -> Self::Value {
        lhs.add_(rhs)
    }
}

impl<T> Group for OpAdd<T>
where
    T: Copy + OpAddUtils,
{
    #[inline]
    fn inv(&self, &x: &Self::Value) -> Self::Value {
        x.neg_()
    }
}

trait OpAddUtils {
    const ZERO: Self;
    fn add_(self, rhs: Self) -> Self;
    fn neg_(self) -> Self;
}

macro_rules! impl_opaddutils_signed {
    ($ty: ty) => {
        impl OpAddUtils for $ty {
            const ZERO: Self = 0;

            #[inline(always)]
            fn add_(self, rhs: Self) -> Self {
                self + rhs
            }

            #[inline(always)]
            fn neg_(self) -> Self {
                -self
            }
        }
    };
}

macro_rules! impl_opaddutils_unsigned {
    ($ty: ty) => {
        impl OpAddUtils for $ty {
            const ZERO: Self = 0;

            #[inline(always)]
            fn add_(self, rhs: Self) -> Self {
                self.wrapping_add(rhs)
            }

            #[inline(always)]
            fn neg_(self) -> Self {
                self.wrapping_neg()
            }
        }
    };
}

macro_rules! impl_opaddutils_for {
    (unsigned: [$($u:ty),* $(,)?], signed: [$($i:ty),* $(,)?]$(,)?) => {
        $( impl_opaddutils_unsigned!($u); )*
        $( impl_opaddutils_signed!($i); )*
    };
}

impl_opaddutils_for! {
    unsigned: [u8, u16, u32, u64, u128, usize],
    signed:   [i8, i16, i32, i64, i128, isize],
}

impl<const P: u64> OpAddUtils for ModInt<P> {
    const ZERO: Self = ModInt::new(0);

    #[inline(always)]
    fn add_(self, rhs: Self) -> Self {
        self + rhs
    }

    #[inline(always)]
    fn neg_(self) -> Self {
        -self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::{group::Group, monoid::Monoid};

    #[test]
    fn test_opadd() {
        let op = OpAdd::<i64>::default();
        assert_eq!(op.op(&74, &33), 107);
        assert_eq!(op.op(&18, &65), 83);
        assert_eq!(op.op(&22, &-4), 18);
        assert_eq!(op.op(&22, &-33), -11);
        assert_eq!(op.op(&-7, &10), 3);
        assert_eq!(op.op(&-8, &12), 4);
        assert_eq!(op.op(&-8, &-55), -63);
        assert_eq!(op.op(&op.identity(), &5), 5);
        assert_eq!(op.op(&3332, &op.identity()), 3332);
        assert_eq!(op.inv(&111), -111);
        assert_eq!(op.op(&81, &op.inv(&6)), 75);
        assert_eq!(op.op(&51, &op.inv(&33)), 18);
        assert_eq!(op.op(&op.inv(&87), &70), -17);
        assert_eq!(op.op(&op.inv(&49), &0), -49);

        let op = OpAdd::<u64>::default();
        assert_eq!(op.op(&74, &33), 107);
        assert_eq!(op.op(&18, &65), 83);
        assert_eq!(op.op(&66, &17), 83);
        assert_eq!(op.op(&24, &3), 27);
        assert_eq!(op.op(&88, &87), 175);
        assert_eq!(op.op(&op.identity(), &5), 5);
        assert_eq!(op.op(&op.identity(), &3332), 3332);
    }
}
