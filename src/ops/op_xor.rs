use std::{marker::PhantomData, ops::BitXor};

use crate::{ops::group::Group, ops::monoid::Monoid};

#[derive(Default, Clone)]
pub struct OpXor<T> {
    _phantom: PhantomData<T>,
}

impl<T> Monoid for OpXor<T>
where
    T: Copy + BitXor<Output = T> + Zero,
{
    type Value = T;

    #[inline]
    fn identity(&self) -> Self::Value {
        T::zero()
    }

    #[inline]
    fn op(&self, &lhs: &Self::Value, &rhs: &Self::Value) -> Self::Value {
        lhs ^ rhs
    }
}

impl<T> Group for OpXor<T>
where
    T: Copy + BitXor<Output = T> + Zero,
{
    #[inline]
    fn inv(&self, &x: &Self::Value) -> Self::Value {
        x
    }
}

trait Zero {
    fn zero() -> Self;
}

macro_rules! impl_integer {
    ($($ty:ident),*) => {$(
        impl Zero for $ty {
            #[inline(always)]
            fn zero() -> Self {
                0
            }
        }
    )*};
}

impl_integer! { i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize  }
