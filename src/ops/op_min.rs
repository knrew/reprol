use std::marker::PhantomData;

use crate::monoid::Monoid;

pub struct OpMin<T> {
    _phantom: PhantomData<T>,
}

impl<T> Default for OpMin<T> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
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
            fn max() -> Self {
                $ty::MAX
            }
        }
    )*};
}

impl_integer! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }
