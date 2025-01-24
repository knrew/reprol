use std::marker::PhantomData;

use crate::monoid::Monoid;

pub struct OpMax<T> {
    _phantom: PhantomData<T>,
}

impl<T> Default for OpMax<T> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> Monoid for OpMax<T>
where
    T: Copy + PartialOrd + Min,
{
    type Value = T;

    fn identity(&self) -> Self::Value {
        T::min()
    }

    fn op(&self, &x: &Self::Value, &y: &Self::Value) -> Self::Value {
        if x > y {
            x
        } else {
            y
        }
    }
}

pub trait Min {
    fn min() -> Self;
}

macro_rules! impl_integer {
    ($($ty:ident),*) => {$(
        impl Min for $ty {
            fn min() -> Self {
                $ty::MIN
            }
        }
    )*};
}

impl_integer! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }
