use std::marker::PhantomData;

use crate::{
    math::modint::ModInt,
    ops::{action::Action, monoid::Monoid},
};

/// LazySegmentTreeにおける区間加算作用
///
/// ## NOTE
/// - OpAdd/OpGcdと組み合わせて使うことはできない
#[derive(Default, Clone)]
pub struct ActAdd<T>(PhantomData<T>);

impl<T: Copy + ActAddUtils> Monoid for ActAdd<T> {
    type Element = T;

    #[inline]
    fn id(&self) -> Self::Element {
        T::ZERO
    }

    #[inline]
    fn op(&self, &lhs: &Self::Element, &rhs: &Self::Element) -> Self::Element {
        lhs.add_(rhs)
    }
}

impl<O> Action<O> for ActAdd<O::Element>
where
    O: Monoid,
    O::Element: Copy + ActAddUtils,
{
    #[inline]
    fn act(&self, &f: &Self::Element, &x: &<O as Monoid>::Element) -> <O as Monoid>::Element {
        x.add_(f)
    }
}

trait ActAddUtils {
    const ZERO: Self;

    fn add_(self, rhs: Self) -> Self;
}

macro_rules! impl_actaddutils_signed {
    ($ty: ty) => {
        impl ActAddUtils for $ty {
            const ZERO: Self = 0;

            #[inline(always)]
            fn add_(self, rhs: Self) -> Self {
                self + rhs
            }
        }
    };
}

macro_rules! impl_actaddutils_unsigned {
    ($ty: ty) => {
        impl ActAddUtils for $ty {
            const ZERO: Self = 0;

            #[inline(always)]
            fn add_(self, rhs: Self) -> Self {
                self.wrapping_add(rhs)
            }
        }
    };
}

macro_rules! impl_actaddutils_for {
    (unsigned: [$($u:ty),* $(,)?], signed: [$($s:ty),* $(,)?]$(,)?) => {
        $( impl_actaddutils_unsigned!($u); )*
        $( impl_actaddutils_signed!($s); )*
    };
}

impl_actaddutils_for! {
    unsigned: [u8, u16, u32, u64, u128, usize],
    signed:   [i8, i16, i32, i64, i128, isize],
}

impl<const P: u64> ActAddUtils for ModInt<P> {
    const ZERO: Self = ModInt::new(0);

    #[inline(always)]
    fn add_(self, rhs: Self) -> Self {
        self + rhs
    }
}
