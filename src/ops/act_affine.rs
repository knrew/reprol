use std::marker::PhantomData;

use crate::{
    math::modint::ModInt,
    ops::{action::Action, monoid::Monoid, op_add_with_len::OpAddWithLenElement},
};

/// アフィン変換パラメータ: f(x) = a*x + b
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ActAffineElement<T> {
    pub a: T,
    pub b: T,
}

/// LazySegmentTreeにおけるアフィン変換作用
///
/// アフィン変換 + 区間和取得を行うためのAction
#[derive(Default, Clone)]
pub struct ActAffine<T>(PhantomData<T>);

impl<T: Copy + ActAffineUtils> Monoid for ActAffine<T> {
    type Element = ActAffineElement<T>;

    #[inline]
    fn id(&self) -> Self::Element {
        ActAffineElement {
            a: T::ONE,
            b: T::ZERO,
        }
    }

    #[inline]
    fn op(&self, g: &Self::Element, f: &Self::Element) -> Self::Element {
        // アフィン変換の合成:
        // g∘f(x) = g(f(x)) = g.a*(f.a*x + f.b) + g.b = (g.a*f.a)*x + (g.a*f.b + g.b)
        ActAffineElement {
            a: g.a.mul_(f.a),
            b: g.a.mul_(f.b).add_(g.b),
        }
    }
}

impl<T, O> Action<O> for ActAffine<T>
where
    O: Monoid<Element = OpAddWithLenElement<T>>,
    T: Copy + ActAffineUtils,
{
    #[inline]
    fn act(&self, f: &Self::Element, x: &O::Element) -> O::Element {
        // 区間和に対するアフィン変換: sum' = a*sum + b*len
        OpAddWithLenElement {
            value: f.a.mul_(x.value).add_(f.b.mul_(x.len)),
            len: x.len,
        }
    }
}

trait ActAffineUtils {
    const ZERO: Self;
    const ONE: Self;
    fn add_(self, rhs: Self) -> Self;
    fn mul_(self, rhs: Self) -> Self;
}

macro_rules! impl_actaffineutils_signed {
    ($ty: ty) => {
        impl ActAffineUtils for $ty {
            const ZERO: Self = 0;
            const ONE: Self = 1;

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

macro_rules! impl_actaffineutils_unsigned {
    ($ty: ty) => {
        impl ActAffineUtils for $ty {
            const ZERO: Self = 0;
            const ONE: Self = 1;

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

macro_rules! impl_actaffineutils_for {
    (unsigned: [$($u:ty),* $(,)?], signed: [$($s:ty),* $(,)?]$(,)?) => {
        $( impl_actaffineutils_unsigned!($u); )*
        $( impl_actaffineutils_signed!($s); )*
    };
}

impl_actaffineutils_for! {
    unsigned: [u8, u16, u32, u64, u128, usize],
    signed:   [i8, i16, i32, i64, i128, isize],
}

impl<const P: u64> ActAffineUtils for ModInt<P> {
    const ZERO: Self = ModInt::new(0);
    const ONE: Self = ModInt::new(1);

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
    fn test_actaffine_monoid() {
        let act = ActAffine::<i64>::default();

        // 単位元: f(x) = 1*x + 0 = x
        let id = act.id();
        assert_eq!(id.a, 1);
        assert_eq!(id.b, 0);

        // 作用の合成
        // f(x) = 2x + 3, g(x) = 3x + 1
        // g∘f(x) = 3(2x + 3) + 1 = 6x + 10
        let f = ActAffineElement { a: 2, b: 3 };
        let g = ActAffineElement { a: 3, b: 1 };
        let composed = act.op(&g, &f);
        assert_eq!(composed.a, 6);
        assert_eq!(composed.b, 10);

        // 単位元との演算
        assert_eq!(act.op(&id, &f), f);
        assert_eq!(act.op(&f, &id), f);
    }

    #[test]
    fn test_actaffine_action() {
        let act = ActAffine::<i64>::default();

        // 作用の適用
        let node = OpAddWithLenElement { value: 10, len: 3 };
        let affine = ActAffineElement { a: 2, b: 3 };
        let _op = OpDummy::<i64>::default();

        let result = Action::<OpDummy<i64>>::act(&act, &affine, &node);

        // sum' = 2*10 + 3*3 = 20 + 9 = 29
        assert_eq!(result.value, 29);
        assert_eq!(result.len, 3);

        // 単位元での作用は何もしない
        let result_id = Action::<OpDummy<i64>>::act(&act, &act.id(), &node);
        assert_eq!(result_id.value, 10);
        assert_eq!(result_id.len, 3);
    }

    #[test]
    fn test_actaffine_unsigned() {
        let act = ActAffine::<u64>::default();

        let node = OpAddWithLenElement { value: 10, len: 3 };
        let affine = ActAffineElement { a: 2, b: 3 };
        let _op = OpDummy::<u64>::default();

        let result = Action::<OpDummy<u64>>::act(&act, &affine, &node);

        assert_eq!(result.value, 29);
        assert_eq!(result.len, 3);
    }

    #[test]
    fn test_actaffine_composition_complex() {
        let act = ActAffine::<i64>::default();

        // 複雑な合成のテスト
        // f(x) = 2x + 3
        // g(x) = 5x + 7
        // h(x) = 3x + 1
        // h∘g∘f(x) = ?
        let f = ActAffineElement { a: 2, b: 3 };
        let g = ActAffineElement { a: 5, b: 7 };
        let h = ActAffineElement { a: 3, b: 1 };

        let gf = act.op(&g, &f);
        // g∘f(x) = 5(2x + 3) + 7 = 10x + 22
        assert_eq!(gf.a, 10);
        assert_eq!(gf.b, 22);

        let hgf = act.op(&h, &gf);
        // h∘g∘f(x) = 3(10x + 22) + 1 = 30x + 67
        assert_eq!(hgf.a, 30);
        assert_eq!(hgf.b, 67);
    }

    // テスト用のダミーOp
    #[derive(Default)]
    struct OpDummy<T>(std::marker::PhantomData<T>);

    impl<T: Copy + ActAffineUtils> Monoid for OpDummy<T> {
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
