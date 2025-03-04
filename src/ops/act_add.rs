use std::{marker::PhantomData, ops::Add};

use crate::{
    math::modint::ModInt,
    ops::{action::MonoidAction, monoid::Monoid},
};

/// LazySegmentTree用
/// 値の区間加算を行う作用
/// `seg.act(l..r, &x)`のように書くと[l, r)の区間にそれぞれxを加算する
#[derive(Default, Clone)]
pub struct ActAdd<T> {
    _phantom: PhantomData<T>,
}

impl<T> Monoid for ActAdd<T>
where
    T: Copy + Add<Output = T> + Integer,
{
    type Value = T;

    fn identity(&self) -> Self::Value {
        T::zero()
    }

    fn op(&self, &lhs: &Self::Value, &rhs: &Self::Value) -> Self::Value {
        lhs.add_(rhs)
    }
}

impl<O> MonoidAction<O> for ActAdd<O::Value>
where
    O: Monoid,
    O::Value: Copy + Add<Output = O::Value> + Integer,
{
    fn act(&self, &f: &Self::Value, &x: &<O as Monoid>::Value) -> <O as Monoid>::Value {
        x + f
    }
}

trait Integer {
    fn zero() -> Self;
    fn add_(self, rhs: Self) -> Self;
    fn neg_(self) -> Self;
}

macro_rules! impl_unsigned {
    ($($ty:ident),*) => {$(
        impl Integer for $ty {
            #[inline]
            fn zero() -> Self {
                0
            }
            #[inline]
            fn add_(self, rhs: Self) -> Self {
                self + rhs
            }
            #[inline]
            fn neg_(self) -> Self{
                -self
            }
        }
    )*};
}

impl_unsigned! { i8, i16, i32, i64, i128, isize }

macro_rules! impl_signed {
    ($($ty:ident),*) => {$(
        impl Integer for $ty {
            #[inline]
            fn zero() -> Self {
                0
            }
            #[inline]
            fn add_(self, rhs: Self) -> Self {
                self.wrapping_add(rhs)
            }
            #[inline]
            fn neg_(self) -> Self{
                self.wrapping_neg()
            }
        }
    )*};
}

impl_signed! { u8, u16, u32, u64, u128, usize }

impl<const P: u64> Integer for ModInt<P> {
    #[inline]
    fn zero() -> Self {
        0.into()
    }
    #[inline]
    fn add_(self, rhs: Self) -> Self {
        self + rhs
    }
    #[inline]
    fn neg_(self) -> Self {
        -self
    }
}
