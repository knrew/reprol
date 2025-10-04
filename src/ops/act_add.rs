use std::marker::PhantomData;

use crate::{
    math::modint::ModInt,
    ops::{action::Action, monoid::Monoid},
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
    T: Copy + ZeroAdd,
{
    type Value = T;

    #[inline]
    fn identity(&self) -> Self::Value {
        T::zero()
    }

    #[inline]
    fn op(&self, &lhs: &Self::Value, &rhs: &Self::Value) -> Self::Value {
        lhs.add_(rhs)
    }
}

impl<O> Action<O> for ActAdd<O::Value>
where
    O: Monoid,
    O::Value: Copy + ZeroAdd,
{
    #[inline]
    fn act(&self, &f: &Self::Value, &x: &<O as Monoid>::Value) -> <O as Monoid>::Value {
        x.add_(f)
    }
}

trait ZeroAdd {
    fn zero() -> Self;

    fn add_(self, rhs: Self) -> Self;
}

macro_rules! impl_zeroadd_signed {
    ($ty: ty) => {
        impl ZeroAdd for $ty {
            #[inline(always)]
            fn zero() -> Self {
                0
            }

            #[inline(always)]
            fn add_(self, rhs: Self) -> Self {
                self + rhs
            }
        }
    };
}

macro_rules! impl_zeroadd_unsigned {
    ($ty: ty) => {
        impl ZeroAdd for $ty {
            #[inline(always)]
            fn zero() -> Self {
                0
            }

            #[inline(always)]
            fn add_(self, rhs: Self) -> Self {
                self.wrapping_add(rhs)
            }
        }
    };
}

macro_rules! impl_zeroadd_for {
    (unsigned: [$($u:ty),* $(,)?], signed: [$($s:ty),* $(,)?]$(,)?) => {
        $( impl_zeroadd_unsigned!($u); )*
        $( impl_zeroadd_signed!($s); )*
    };
}

impl_zeroadd_for! {
    unsigned: [u8, u16, u32, u64, u128, usize],
    signed:   [i8, i16, i32, i64, i128, isize],
}

impl<const P: u64> ZeroAdd for ModInt<P> {
    #[inline(always)]
    fn zero() -> Self {
        0.into()
    }

    #[inline(always)]
    fn add_(self, rhs: Self) -> Self {
        self + rhs
    }
}
