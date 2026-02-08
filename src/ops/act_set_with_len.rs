use std::marker::PhantomData;

use crate::{
    math::modint::ModInt,
    ops::{action::Action, monoid::Monoid, op_add_with_len::OpAddWithLenElement},
};

/// LazySegmentTreeにおける区間代入作用
///
/// 区間代入 + 区間和取得を行うためのAction
#[derive(Default, Clone)]
pub struct ActSetWithLen<T>(PhantomData<T>);

impl<T: Copy + Clone> Monoid for ActSetWithLen<T> {
    type Element = Option<T>;

    #[inline]
    fn id(&self) -> Self::Element {
        None
    }

    #[inline]
    fn op(&self, g: &Self::Element, f: &Self::Element) -> Self::Element {
        *if g.is_none() { f } else { g }
    }
}

impl<T, O> Action<O> for ActSetWithLen<T>
where
    O: Monoid<Element = OpAddWithLenElement<T>>,
    T: Copy + ActSetWithLenUtils,
{
    #[inline]
    fn act(&self, f: &Self::Element, x: &O::Element) -> O::Element {
        if let Some(f) = f {
            OpAddWithLenElement {
                value: f.mul_(x.len), // value = f * len
                len: x.len,
            }
        } else {
            x.clone()
        }
    }
}

trait ActSetWithLenUtils {
    fn mul_(self, rhs: Self) -> Self;
}

macro_rules! impl_actsetwithlenutils_signed {
    ($ty: ty) => {
        impl ActSetWithLenUtils for $ty {
            #[inline(always)]
            fn mul_(self, rhs: Self) -> Self {
                self * rhs
            }
        }
    };
}

macro_rules! impl_actsetwithlenutils_unsigned {
    ($ty: ty) => {
        impl ActSetWithLenUtils for $ty {
            #[inline(always)]
            fn mul_(self, rhs: Self) -> Self {
                self.wrapping_mul(rhs)
            }
        }
    };
}

macro_rules! impl_actsetwithlenutils_for {
    (unsigned: [$($u:ty),* $(,)?], signed: [$($s:ty),* $(,)?]$(,)?) => {
        $( impl_actsetwithlenutils_unsigned!($u); )*
        $( impl_actsetwithlenutils_signed!($s); )*
    };
}

impl_actsetwithlenutils_for! {
    unsigned: [u8, u16, u32, u64, u128, usize],
    signed:   [i8, i16, i32, i64, i128, isize],
}

impl<const P: u64> ActSetWithLenUtils for ModInt<P> {
    #[inline(always)]
    fn mul_(self, rhs: Self) -> Self {
        self * rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::action::Action;

    #[test]
    fn test_actrangeset_monoid() {
        let act = ActSetWithLen::<i64>::default();

        // 単位元
        assert_eq!(act.id(), None);

        // 作用の合成（後勝ち）
        // Some(5) ⋅ None = Some(5)
        assert_eq!(act.op(&None, &Some(5)), Some(5));
        // Some(3) ⋅ Some(5) = Some(3)（後勝ち）
        assert_eq!(act.op(&Some(3), &Some(5)), Some(3));

        // 単位元との演算
        assert_eq!(act.op(&None, &Some(5)), Some(5));
        assert_eq!(act.op(&Some(5), &None), Some(5));
    }

    #[test]
    fn test_actrangeset_action() {
        let act = ActSetWithLen::<i64>::default();

        // Someでの作用の適用
        let node = OpAddWithLenElement { value: 10, len: 3 };
        let _op = OpDummy::<i64>::default();

        let result = Action::<OpDummy<i64>>::act(&act, &Some(5), &node);

        // value' = 5 * 3 = 15
        assert_eq!(result.value, 15);
        assert_eq!(result.len, 3);

        // Noneでの作用は何もしない
        let result_none = Action::<OpDummy<i64>>::act(&act, &None, &node);
        assert_eq!(result_none.value, 10);
        assert_eq!(result_none.len, 3);
    }

    #[test]
    fn test_actrangeset_unsigned() {
        let act = ActSetWithLen::<u64>::default();

        let node = OpAddWithLenElement { value: 10, len: 3 };
        let _op = OpDummy::<u64>::default();

        let result = Action::<OpDummy<u64>>::act(&act, &Some(5), &node);

        assert_eq!(result.value, 15);
        assert_eq!(result.len, 3);
    }

    // テスト用のダミーOp
    #[derive(Default)]
    struct OpDummy<T>(std::marker::PhantomData<T>);

    trait ActSetWithLenUtilsExt {
        const ZERO: Self;
        fn add_(self, rhs: Self) -> Self;
    }

    impl ActSetWithLenUtilsExt for i64 {
        const ZERO: Self = 0;
        fn add_(self, rhs: Self) -> Self {
            self + rhs
        }
    }

    impl ActSetWithLenUtilsExt for u64 {
        const ZERO: Self = 0;
        fn add_(self, rhs: Self) -> Self {
            self.wrapping_add(rhs)
        }
    }

    impl<T: Copy + ActSetWithLenUtilsExt> Monoid for OpDummy<T> {
        type Element = OpAddWithLenElement<T>;

        fn id(&self) -> Self::Element {
            OpAddWithLenElement {
                value: T::ZERO,
                len: T::ZERO,
            }
        }

        fn op(&self, lhs: &Self::Element, rhs: &Self::Element) -> Self::Element {
            OpAddWithLenElement {
                value: lhs.value.add_(rhs.value),
                len: lhs.len.add_(rhs.len),
            }
        }
    }
}
