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

    fn op(&self, &x: &Self::Value, &y: &Self::Value) -> Self::Value {
        x + y
    }
}

impl<T> Group for OpAdd<T>
where
    T: Copy + Add<Output = T> + Neg<Output = T> + Zero,
{
    fn inv(&self, &x: &<Self as Monoid>::Value) -> Self::Value {
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
