use std::marker::PhantomData;

use crate::{math::modint::ModInt, ops::monoid::Monoid};

/// 区間和を保持するノード
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpAddWithLenElement<T> {
    /// 区間和
    pub value: T,

    /// 区間の長さ
    pub len: T,
}

#[allow(private_bounds)]
impl<T: OpAddWithLenUtils> OpAddWithLenElement<T> {
    pub fn leaf(value: T) -> Self {
        Self { value, len: T::ONE }
    }
}

/// 区間和モノイド
///
/// LazySegmentTreeで区間加算/代入/アフィン変換 + 区間和取得を行うためのOp
#[derive(Default, Clone)]
pub struct OpAddWithLen<T>(PhantomData<T>);

impl<T: Copy + OpAddWithLenUtils> Monoid for OpAddWithLen<T> {
    type Element = OpAddWithLenElement<T>;

    #[inline]
    fn id(&self) -> Self::Element {
        OpAddWithLenElement {
            value: T::ZERO,
            len: T::ZERO,
        }
    }

    #[inline]
    fn op(&self, lhs: &Self::Element, rhs: &Self::Element) -> Self::Element {
        OpAddWithLenElement {
            value: lhs.value.add_(rhs.value),
            len: lhs.len.add_(rhs.len),
        }
    }
}

trait OpAddWithLenUtils {
    const ZERO: Self;
    const ONE: Self;
    fn add_(self, rhs: Self) -> Self;
}

macro_rules! impl_opaddwithlenutils_signed {
    ($ty: ty) => {
        impl OpAddWithLenUtils for $ty {
            const ZERO: Self = 0;
            const ONE: Self = 1;

            #[inline(always)]
            fn add_(self, rhs: Self) -> Self {
                self + rhs
            }
        }
    };
}

macro_rules! impl_opaddwithlenutils_unsigned {
    ($ty: ty) => {
        impl OpAddWithLenUtils for $ty {
            const ZERO: Self = 0;
            const ONE: Self = 1;

            #[inline(always)]
            fn add_(self, rhs: Self) -> Self {
                self.wrapping_add(rhs)
            }
        }
    };
}

macro_rules! impl_opaddwithlenutils_for {
    (unsigned: [$($u:ty),* $(,)?], signed: [$($s:ty),* $(,)?]$(,)?) => {
        $( impl_opaddwithlenutils_unsigned!($u); )*
        $( impl_opaddwithlenutils_signed!($s); )*
    };
}

impl_opaddwithlenutils_for! {
    unsigned: [u8, u16, u32, u64, u128, usize],
    signed:   [i8, i16, i32, i64, i128, isize],
}

impl<const P: u64> OpAddWithLenUtils for ModInt<P> {
    const ZERO: Self = ModInt::new(0);
    const ONE: Self = ModInt::new(1);

    #[inline(always)]
    fn add_(self, rhs: Self) -> Self {
        self + rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_add_with_len_signed() {
        let op = OpAddWithLen::<i64>::default();

        // 単位元
        let id = op.id();
        assert_eq!(id.value, 0);
        assert_eq!(id.len, 0);

        // 基本的な演算
        let lhs = OpAddWithLenElement { value: 5, len: 2 };
        let rhs = OpAddWithLenElement { value: 3, len: 1 };
        let result = op.op(&lhs, &rhs);
        assert_eq!(result.value, 8);
        assert_eq!(result.len, 3);

        // 単位元との演算
        assert_eq!(op.op(&id, &lhs), lhs);
        assert_eq!(op.op(&lhs, &id), lhs);

        // 結合律
        let a = OpAddWithLenElement { value: 1, len: 1 };
        let b = OpAddWithLenElement { value: 2, len: 1 };
        let c = OpAddWithLenElement { value: 3, len: 1 };
        assert_eq!(op.op(&op.op(&a, &b), &c), op.op(&a, &op.op(&b, &c)));
    }

    #[test]
    fn test_op_add_with_len_unsigned() {
        let op = OpAddWithLen::<u64>::default();

        let lhs = OpAddWithLenElement { value: 5, len: 2 };
        let rhs = OpAddWithLenElement { value: 3, len: 1 };
        let result = op.op(&lhs, &rhs);
        assert_eq!(result.value, 8);
        assert_eq!(result.len, 3);
    }
}
