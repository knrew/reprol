use std::marker::PhantomData;

use crate::ops::monoid::{IdempotentMonoid, Monoid};

#[derive(Default, Clone)]
pub struct OpMin<T> {
    phantom: PhantomData<T>,
}

impl<T: Copy + PartialOrd + OpMinUtils> Monoid for OpMin<T> {
    type Value = T;

    #[inline]
    fn identity(&self) -> Self::Value {
        T::MAX
    }

    #[inline]
    fn op(&self, &x: &Self::Value, &y: &Self::Value) -> Self::Value {
        if x < y { x } else { y }
    }
}

impl<T: Copy + PartialOrd + OpMinUtils> IdempotentMonoid for OpMin<T> {}

pub trait OpMinUtils {
    const MAX: Self;
}

macro_rules! impl_opminutils {
    ($ty: ty) => {
        impl OpMinUtils for $ty {
            const MAX: Self = <$ty>::MAX;
        }
    };
}

macro_rules! impl_opminutils_for {
    ($($ty: ty),* $(,)?) => {
        $( impl_opminutils!($ty); )*
    };
}

impl_opminutils_for! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::monoid::Monoid;

    #[test]
    fn test_opmin() {
        let op = OpMin::<i64>::default();
        assert_eq!(op.op(&90, &67), 67);
        assert_eq!(op.op(&1, &61), 1);
        assert_eq!(op.op(&2, &28), 2);
        assert_eq!(op.op(&38, &69), 38);
        assert_eq!(op.op(&13, &48), 13);
        assert_eq!(op.op(&op.identity(), &5), 5);
        assert_eq!(op.op(&op.identity(), &3332), 3332);
    }
}
