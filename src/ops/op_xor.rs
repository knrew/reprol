use std::{marker::PhantomData, ops::BitXor};

use crate::{ops::group::Group, ops::monoid::Monoid};

#[derive(Default, Clone)]
pub struct OpXor<T> {
    phantom: PhantomData<T>,
}

impl<T> Monoid for OpXor<T>
where
    T: Copy + BitXor<Output = T> + OpXorUtils,
{
    type Element = T;

    #[inline]
    fn id(&self) -> Self::Element {
        T::ZERO
    }

    #[inline]
    fn op(&self, &lhs: &Self::Element, &rhs: &Self::Element) -> Self::Element {
        lhs ^ rhs
    }
}

impl<T> Group for OpXor<T>
where
    T: Copy + BitXor<Output = T> + OpXorUtils,
{
    #[inline]
    fn inv(&self, &x: &Self::Element) -> Self::Element {
        x
    }
}

trait OpXorUtils {
    const ZERO: Self;
}

macro_rules! impl_opxorutils {
    ($ty: ty) => {
        impl OpXorUtils for $ty {
            const ZERO: $ty = 0;
        }
    };
}

macro_rules! impl_opxorutils_for {
    ($($ty: ty),* $(,)?) => {
        $( impl_opxorutils!($ty); )*
    };
}

impl_opxorutils_for! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}
