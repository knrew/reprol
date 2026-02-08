use std::marker::PhantomData;

use crate::{
    math::modint::ModInt,
    ops::{action::Action, monoid::Monoid, op_add_with_len::OpAddWithLenElement},
};

/// LazySegmentTreeにおける区間加算作用
///
/// 区間加算 + 区間和取得を行うための作用
#[derive(Default, Clone)]
pub struct ActAddWithLen<T>(PhantomData<T>);

impl<T> Monoid for ActAddWithLen<T>
where
    T: Copy + ActAddWithLenUtils,
{
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

impl<T, O> Action<O> for ActAddWithLen<T>
where
    O: Monoid<Element = OpAddWithLenElement<T>>,
    T: Copy + ActAddWithLenUtils,
{
    #[inline]
    fn act(&self, &f: &Self::Element, x: &O::Element) -> O::Element {
        OpAddWithLenElement {
            value: x.value.add_(f.mul_(x.len)), // value += f * len
            len: x.len,
        }
    }
}

trait ActAddWithLenUtils {
    const ZERO: Self;
    fn add_(self, rhs: Self) -> Self;
    fn mul_(self, rhs: Self) -> Self;
}

macro_rules! impl_actaddwithlenutils_signed {
    ($ty: ty) => {
        impl ActAddWithLenUtils for $ty {
            const ZERO: Self = 0;

            #[inline(always)]
            fn add_(self, rhs: Self) -> Self {
                self + rhs
            }

            #[inline(always)]
            fn mul_(self, rhs: Self) -> Self {
                self * rhs
            }
        }
    };
}

macro_rules! impl_actaddwithlenutils_unsigned {
    ($ty: ty) => {
        impl ActAddWithLenUtils for $ty {
            const ZERO: Self = 0;

            #[inline(always)]
            fn add_(self, rhs: Self) -> Self {
                self.wrapping_add(rhs)
            }

            #[inline(always)]
            fn mul_(self, rhs: Self) -> Self {
                self.wrapping_mul(rhs)
            }
        }
    };
}

macro_rules! impl_actaddwithlenutils_for {
    (unsigned: [$($u:ty),* $(,)?], signed: [$($s:ty),* $(,)?]$(,)?) => {
        $( impl_actaddwithlenutils_unsigned!($u); )*
        $( impl_actaddwithlenutils_signed!($s); )*
    };
}

impl_actaddwithlenutils_for! {
    unsigned: [u8, u16, u32, u64, u128, usize],
    signed:   [i8, i16, i32, i64, i128, isize],
}

impl<const P: u64> ActAddWithLenUtils for ModInt<P> {
    const ZERO: Self = ModInt::new(0);

    #[inline(always)]
    fn add_(self, rhs: Self) -> Self {
        self + rhs
    }

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
    fn test_actrangeadd_monoid() {
        let act = ActAddWithLen::<i64>::default();

        // 単位元
        assert_eq!(act.id(), 0);

        // 作用の合成（可換）
        assert_eq!(act.op(&5, &3), 8);
        assert_eq!(act.op(&3, &5), 8);

        // 単位元との演算
        assert_eq!(act.op(&0, &5), 5);
        assert_eq!(act.op(&5, &0), 5);
    }

    #[test]
    fn test_actrangeadd_action() {
        let act = ActAddWithLen::<i64>::default();

        // 作用の適用
        let node = OpAddWithLenElement { value: 10, len: 3 };
        let _op = OpDummy::<i64>::default();

        // Actionトレイトを通じて呼び出し
        let result = Action::<OpDummy<i64>>::act(&act, &5, &node);

        // sum' = 10 + 5 * 3 = 25
        assert_eq!(result.value, 25);
        assert_eq!(result.len, 3);

        // 単位元での作用は何もしない
        let result_id = Action::<OpDummy<i64>>::act(&act, &0, &node);
        assert_eq!(result_id.value, 10);
        assert_eq!(result_id.len, 3);
    }

    #[test]
    fn test_actrangeadd_unsigned() {
        let act = ActAddWithLen::<u64>::default();

        let node = OpAddWithLenElement { value: 10, len: 3 };
        let _op = OpDummy::<u64>::default();

        let result = Action::<OpDummy<u64>>::act(&act, &5, &node);

        assert_eq!(result.value, 25);
        assert_eq!(result.len, 3);
    }

    // テスト用のダミーOp
    #[derive(Default)]
    struct OpDummy<T>(std::marker::PhantomData<T>);

    impl<T: Copy + ActAddWithLenUtils> Monoid for OpDummy<T> {
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
