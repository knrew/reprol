use std::marker::PhantomData;

use crate::ops::monoid::{IdempotentMonoid, Monoid};

#[derive(Default, Clone)]
pub struct OpMax<T> {
    phantom: PhantomData<T>,
}

impl<T> Monoid for OpMax<T>
where
    T: Copy + PartialOrd + OpMaxUtils,
{
    type Element = T;

    #[inline]
    fn id(&self) -> Self::Element {
        T::MIN
    }

    #[inline]
    fn op(&self, &x: &Self::Element, &y: &Self::Element) -> Self::Element {
        if x > y { x } else { y }
    }
}

impl<T: Copy + PartialOrd + OpMaxUtils> IdempotentMonoid for OpMax<T> {}

trait OpMaxUtils {
    const MIN: Self;
}

macro_rules! impl_opmaxutils {
    ($ty: ty) => {
        impl OpMaxUtils for $ty {
            const MIN: Self = <$ty>::MIN;
        }
    };
}

macro_rules! impl_opmaxutils_for {
    ($($ty: ty),* $(,)?) => {
        $( impl_opmaxutils!($ty); )*
    };
}

impl_opmaxutils_for! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::monoid::Monoid;

    #[test]
    fn test_opmax() {
        let op = OpMax::<i64>::default();
        assert_eq!(op.op(&73, &11), 73);
        assert_eq!(op.op(&46, &79), 79);
        assert_eq!(op.op(&59, &65), 65);
        assert_eq!(op.op(&68, &26), 68);
        assert_eq!(op.op(&18, &48), 48);
        assert_eq!(op.op(&op.id(), &5), 5);
        assert_eq!(op.op(&op.id(), &3332), 3332);
    }
}
