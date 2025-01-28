use std::{
    marker::PhantomData,
    ops::{Add, Neg},
};

use crate::{group::Group, math::modint::ModInt, monoid::Monoid};

pub struct OpAdd<T> {
    _phantom: PhantomData<T>,
}

impl<T> Default for OpAdd<T> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> Monoid for OpAdd<T>
where
    T: Copy + Add<Output = T> + Zero,
{
    type Value = T;

    fn identity(&self) -> Self::Value {
        T::zero()
    }

    fn op(&self, &lhs: &Self::Value, &rhs: &Self::Value) -> Self::Value {
        lhs + rhs
    }
}

impl<T> Group for OpAdd<T>
where
    T: Copy + Add<Output = T> + Neg<Output = T> + Zero,
{
    fn inv(&self, &x: &Self::Value) -> Self::Value {
        -x
    }
}

pub trait Zero {
    fn zero() -> Self;
}

macro_rules! impl_integer {
    ($($ty:ident),*) => {$(
        impl Zero for $ty {
            fn zero() -> Self {
                0
            }
        }
    )*};
}

impl_integer! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

impl<const P: u64> Zero for ModInt<P> {
    fn zero() -> Self {
        0.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::{group::Group, monoid::Monoid};

    use super::OpAdd;

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
