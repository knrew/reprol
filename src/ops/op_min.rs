use std::marker::PhantomData;

use crate::ops::monoid::Monoid;

#[derive(Default, Clone)]
pub struct OpMin<T> {
    _phantom: PhantomData<T>,
}

impl<T> Monoid for OpMin<T>
where
    T: Copy + PartialOrd + Max,
{
    type Value = T;

    fn identity(&self) -> Self::Value {
        T::max()
    }

    fn op(&self, &x: &Self::Value, &y: &Self::Value) -> Self::Value {
        if x < y {
            x
        } else {
            y
        }
    }
}

pub trait Max {
    fn max() -> Self;
}

macro_rules! impl_integer {
    ($($ty:ident),*) => {$(
        impl Max for $ty {
            #[inline(always)]
            fn max() -> Self {
                $ty::MAX
            }
        }
    )*};
}

impl_integer! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

#[cfg(test)]
mod tests {
    use crate::ops::monoid::Monoid;

    use super::OpMin;

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
