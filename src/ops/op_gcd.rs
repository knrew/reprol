use std::marker::PhantomData;

use crate::{
    math::gcd::Gcd,
    ops::monoid::{IdempotentMonoid, Monoid},
};

#[derive(Default, Clone)]
pub struct OpGcd<T> {
    phantom: PhantomData<T>,
}

impl<T: Copy + Gcd + OpGcdUtils> Monoid for OpGcd<T> {
    type Element = T;

    #[inline]
    fn id(&self) -> Self::Element {
        T::ZERO
    }

    #[inline]
    fn op(&self, lhs: &Self::Element, rhs: &Self::Element) -> Self::Element {
        lhs.gcd(*rhs)
    }
}

impl<T: Copy + Gcd + OpGcdUtils> IdempotentMonoid for OpGcd<T> {}

trait OpGcdUtils {
    const ZERO: Self;
}

macro_rules! impl_opgcdutils {
    ($ty: ty) => {
        impl OpGcdUtils for $ty {
            const ZERO: $ty = 0;
        }
    };
}

macro_rules! impl_opgcdutils_for {
    ($($ty: ty),* $(,)?) => {
        $( impl_opgcdutils!($ty); )*
    };
}

impl_opgcdutils_for! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}
