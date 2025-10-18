use std::marker::PhantomData;

use crate::ops::monoid::Monoid;

#[derive(Default, Clone)]
pub struct OpMax<T> {
    _phantom: PhantomData<T>,
}

impl<T> Monoid for OpMax<T>
where
    T: Copy + PartialOrd + Min,
{
    type Value = T;

    #[inline]
    fn identity(&self) -> Self::Value {
        T::min()
    }

    #[inline]
    fn op(&self, &x: &Self::Value, &y: &Self::Value) -> Self::Value {
        if x > y { x } else { y }
    }
}

pub trait Min {
    fn min() -> Self;
}

macro_rules! impl_min {
    ($ty: ty) => {
        impl Min for $ty {
            #[inline(always)]
            fn min() -> Self {
                <$ty>::MIN
            }
        }
    };
}

macro_rules! impl_min_for {
    ($($ty: ty),* $(,)?) => {
        $( impl_min!($ty); )*
    };
}

impl_min_for! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

#[cfg(test)]
mod tests {
    use crate::ops::monoid::Monoid;

    use super::*;

    #[test]
    fn test_opmax() {
        let op = OpMax::<i64>::default();
        assert_eq!(op.op(&73, &11), 73);
        assert_eq!(op.op(&46, &79), 79);
        assert_eq!(op.op(&59, &65), 65);
        assert_eq!(op.op(&68, &26), 68);
        assert_eq!(op.op(&18, &48), 48);
        assert_eq!(op.op(&op.identity(), &5), 5);
        assert_eq!(op.op(&op.identity(), &3332), 3332);
    }
}
