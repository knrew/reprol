use std::ops::{
    Add, AddAssign, BitAnd, BitOr, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Shl, Shr, Sub,
    SubAssign,
};

pub trait Integer:
    Sized
    + Copy
    + Ord
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Rem<Output = Self>
    + RemAssign
    + Shr<usize, Output = Self>
    + Shl<usize, Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
{
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const MIN: Self;
    const MAX: Self;
    fn as_usize(&self) -> usize;
    fn from_usize(x: usize) -> Self;
}

macro_rules! impl_integer {
    ($($ty:ident),*) => {$(
        impl Integer for $ty {
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const TWO: Self = 2;
            const MIN: Self = std::$ty::MIN;
            const MAX: Self = std::$ty::MAX;
            fn as_usize(&self) -> usize {
                *self as usize
            }
            fn from_usize(x: usize) -> Self {
                x as $ty
            }
        }
    )*};
}

impl_integer! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }
