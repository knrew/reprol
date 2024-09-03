use std::{
    fmt::Debug,
    ops::{Add, BitAnd, BitOr, Div, Mul, Neg, Rem, Shl, Shr, Sub},
};

pub trait Integer:
    Sized
    + Copy
    + PartialOrd
    + Debug
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Shr<usize, Output = Self>
    + Shl<usize, Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + Ord
{
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const MIN: Self;
    const MAX: Self;
}

macro_rules! impl_integer {
        ($($ty:ident),*) => {$(
            impl Integer for $ty {
                const ZERO: Self = 0;
                const ONE: Self = 1;
                const TWO: Self = 2;
                const MIN: Self = std::$ty::MIN;
                const MAX: Self = std::$ty::MAX;
            }
        )*};
    }

impl_integer! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize  }

pub trait Signed:
    Sized
    + Copy
    + PartialOrd
    + Debug
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Neg<Output = Self>
    + Shr<usize, Output = Self>
    + Shl<usize, Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + Ord
{
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const MIN: Self;
    const MAX: Self;
}

macro_rules! impl_signed {
        ($($ty:ident),*) => {$(
            impl Signed for $ty {
                const ZERO: Self = 0;
                const ONE: Self = 1;
                const TWO: Self = 2;
                const MIN: Self = std::$ty::MIN;
                const MAX: Self = std::$ty::MAX;
            }
        )*};
    }

impl_signed! { i8, i16, i32, i64, i128, isize }

pub trait Unsigned:
    Sized
    + Copy
    + PartialOrd
    + Debug
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Shr<usize, Output = Self>
    + Shl<usize, Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + Ord
{
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const MIN: Self;
    const MAX: Self;
}

macro_rules! impl_unsigned {
        ($($ty:ident),*) => {$(
            impl Unsigned for $ty {
                const ZERO: Self = 0;
                const ONE: Self = 1;
                const TWO: Self = 2;
                const MIN: Self = std::$ty::MIN;
                const MAX: Self = std::$ty::MAX;
            }
        )*};
    }

impl_unsigned! { u8, u16, u32, u64, u128, usize }
