//! 区間代入作用(区間和用)
//!
//! 各要素を定数で上書きする作用を表すモノイド．
//! 作用の合成は後が優先される(`op(g, f)` で f を先に適用し g が上書き)．
//! 恒等作用は`None`．
//!
//! [`OpRangeSum`](crate::ops::op_range_sum::OpRangeSum)と組み合わせて，
//! 区間代入・区間和クエリを処理するための作用として使用する．
//!
//! # Examples
//!
//! ```
//! use reprol::ops::monoid::Monoid;
//! use reprol::ops::action::Action;
//! use reprol::ops::act_range_set::ActRangeSet;
//! use reprol::ops::op_range_sum::{OpRangeSum, OpRangeSumElement};
//!
//! let act = ActRangeSet::<i64>::default();
//! // 作用の合成: 後が優先
//! assert_eq!(act.op(&Some(7), &Some(3)), Some(7));
//! assert_eq!(act.op(&None, &Some(3)), Some(3));
//! // 恒等作用
//! assert_eq!(act.op(&act.id(), &Some(5)), Some(5));
//! // OpRangeSumへの作用: value' = f * len
//! let x = OpRangeSumElement::with_count(10, 3);
//! let result = Action::<OpRangeSum<i64>>::act(&act, &Some(5), &x);
//! assert_eq!(result.value(), 15); // 5 * 3
//! assert_eq!(result.len(), 3);
//! // None(恒等作用)では元の値を保持
//! let result = Action::<OpRangeSum<i64>>::act(&act, &None, &x);
//! assert_eq!(result.value(), 10);
//! ```
//!
//! # Notes
//!
//! - 整数型では wrapping 演算を用いるため，オーバーフロー時は折り返す．

use std::marker::PhantomData;

use crate::{
    math::modint::ModInt,
    ops::{action::Action, monoid::Monoid, op_range_sum::OpRangeSumElement},
};

/// 区間代入作用(区間和用)
///
/// [`OpRangeSum`](crate::ops::op_range_sum::OpRangeSum)に対する代入作用を表すモノイド．
/// 作用素の型は`Option<T>`で，`None`が恒等作用，`Some(v)`が値`v`への上書きを表す．
/// 標準のプリミティブ整数型と[`ModInt`]に対応する．
///
/// # Notes
///
/// - 整数型では wrapping 演算を用いるため，オーバーフロー時は折り返す．
#[derive(Default, Clone)]
pub struct ActRangeSet<T>(PhantomData<T>);

impl<T: Clone> Monoid for ActRangeSet<T> {
    type Element = Option<T>;

    #[inline]
    fn op(&self, g: &Self::Element, f: &Self::Element) -> Self::Element {
        if g.is_none() { f } else { g }.clone()
    }

    #[inline]
    fn id(&self) -> Self::Element {
        None
    }
}

impl<T, O> Action<O> for ActRangeSet<T>
where
    O: Monoid<Element = OpRangeSumElement<T>>,
    T: Copy + HasMul,
{
    #[inline]
    fn act(&self, f: &Self::Element, x: &O::Element) -> O::Element {
        if let Some(f) = f {
            OpRangeSumElement::with_count(f.mul(x.len()), x.len()) // value = f * len
        } else {
            x.clone()
        }
    }
}

/// 型固有の乗算を提供するトレイト．
trait HasMul {
    fn mul(self, rhs: Self) -> Self;
}

macro_rules! impl_act_range_set_traits_inner {
    ($ty: ty) => {
        impl HasMul for $ty {
            #[inline(always)]
            fn mul(self, rhs: Self) -> Self {
                self.wrapping_mul(rhs)
            }
        }
    };
}

macro_rules! impl_act_range_set_traits {
    ($($ty: ty),* $(,)?) => {
        $( impl_act_range_set_traits_inner!($ty); )*
    };
}

impl_act_range_set_traits! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

impl<const P: u64> HasMul for ModInt<P> {
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        self * rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::{
        action::Action,
        monoid::Monoid,
        op_range_sum::{OpRangeSum, OpRangeSumElement},
    };

    fn assert_monoid<T: Monoid>() {}

    fn e<T>(value: T, len: T) -> OpRangeSumElement<T> {
        OpRangeSumElement::with_count(value, len)
    }

    // ========== Monoid 軸 ==========

    #[test]
    fn test_id_returns_none() {
        let m = ActRangeSet::<i64>::default();
        assert_eq!(m.id(), None);
    }

    #[test]
    fn test_op_both_some() {
        let m = ActRangeSet::<i64>::default();
        assert_eq!(m.op(&Some(7), &Some(3)), Some(7));
    }

    #[test]
    fn test_op_identity() {
        let m = ActRangeSet::<i64>::default();
        let id = m.id();
        let cases: &[Option<i64>] = &[None, Some(0), Some(1030), Some(-1), Some(i64::MAX)];
        for f in cases {
            assert_eq!(m.op(&id, f), *f, "op(id, {f:?}) = {f:?}");
            assert_eq!(m.op(f, &id), *f, "op({f:?}, id) = {f:?}");
        }
    }

    #[test]
    fn test_op_associativity() {
        let m = ActRangeSet::<i64>::default();
        let vals: &[Option<i64>] = &[None, Some(3), Some(1030), Some(-7)];
        for h in vals {
            for g in vals {
                for f in vals {
                    let lhs = m.op(&m.op(h, g), f);
                    let rhs = m.op(h, &m.op(g, f));
                    assert_eq!(
                        lhs, rhs,
                        "op(op({h:?}, {g:?}), {f:?}) = op({h:?}, op({g:?}, {f:?}))"
                    );
                }
            }
        }
    }

    // ========== Action 軸 (OpRangeSum) ==========

    #[test]
    fn test_act_identity() {
        let act = ActRangeSet::<i64>::default();
        let id = act.id(); // None
        let cases = [e(10, 3), e(0, 0), e(-5, 1), e(1030, 7)];
        for x in &cases {
            assert_eq!(
                Action::<OpRangeSum<i64>>::act(&act, &id, x),
                x.clone(),
                "act(None, {x:?}) = {x:?}"
            );
        }
    }

    #[test]
    fn test_act_some_basic() {
        let act = ActRangeSet::<i64>::default();
        // act(Some(5), e(10, 3)) = e(5*3, 3) = e(15, 3)
        assert_eq!(
            Action::<OpRangeSum<i64>>::act(&act, &Some(5), &e(10, 3)),
            e(15, 3)
        );
        // act(Some(-2), e(6, 4)) = e(-2*4, 4) = e(-8, 4)
        assert_eq!(
            Action::<OpRangeSum<i64>>::act(&act, &Some(-2), &e(6, 4)),
            e(-8, 4)
        );
    }

    #[test]
    fn test_act_some_len_zero_value_zero() {
        let act = ActRangeSet::<i64>::default();
        for f in [0, 1, -1, 1030, i64::MAX] {
            let x = e(42, 0);
            assert_eq!(
                Action::<OpRangeSum<i64>>::act(&act, &Some(f), &x),
                e(0, 0),
                "act(Some({f}), e(42, 0)) = e(0, 0)"
            );
        }
    }

    #[test]
    fn test_act_preserves_len() {
        let act = ActRangeSet::<i64>::default();
        for len in [0, 1, 3, 7, 1030] {
            let result = Action::<OpRangeSum<i64>>::act(&act, &Some(5), &e(10, len));
            assert_eq!(result.len(), len, "act preserves len={len}");
        }
        for len in [0, 1, 3, 7, 1030] {
            let result = Action::<OpRangeSum<i64>>::act(&act, &None, &e(10, len));
            assert_eq!(result.len(), len, "act(None) preserves len={len}");
        }
    }

    #[test]
    fn test_act_composition() {
        let act = ActRangeSet::<i64>::default();
        let fs: &[Option<i64>] = &[None, Some(3), Some(1030), Some(-7)];
        for g in fs {
            for f in fs {
                let composed = act.op(g, f);
                for x in &[e(10, 3), e(0, 1), e(-5, 7)] {
                    let lhs = Action::<OpRangeSum<i64>>::act(&act, &composed, x);
                    let rhs = Action::<OpRangeSum<i64>>::act(
                        &act,
                        g,
                        &Action::<OpRangeSum<i64>>::act(&act, f, x),
                    );
                    assert_eq!(
                        lhs, rhs,
                        "act(op({g:?}, {f:?}), {x:?}) = act({g:?}, act({f:?}, {x:?}))"
                    );
                }
            }
        }
    }

    #[test]
    fn test_act_boundary() {
        let act = ActRangeSet::<u64>::default();
        // wrapping_mul の折り返し: u64::MAX * 2 は wrapping で MAX - 1
        assert_eq!(
            Action::<OpRangeSum<u64>>::act(&act, &Some(u64::MAX), &e(0, 2)),
            e(u64::MAX.wrapping_mul(2), 2),
            "act(Some(u64::MAX), e(_, 2)) uses wrapping_mul"
        );
    }

    // ========== 全型スモークテスト ==========

    #[test]
    fn test_op_smoke_all_integer_types() {
        macro_rules! test {
            ($ty: ty) => {
                assert_monoid::<ActRangeSet<$ty>>();
                let m = ActRangeSet::<$ty>::default();
                assert_eq!(m.id(), None, "id for {}", stringify!($ty));
                assert_eq!(
                    m.op(&Some(7 as $ty), &Some(3 as $ty)),
                    Some(7 as $ty),
                    "op(Some(7), Some(3)) for {}",
                    stringify!($ty)
                );
                assert_eq!(
                    m.op(&None, &Some(3 as $ty)),
                    Some(3 as $ty),
                    "op(None, Some(3)) for {}",
                    stringify!($ty)
                );
            };
        }

        test!(i8);
        test!(i16);
        test!(i32);
        test!(i64);
        test!(i128);
        test!(isize);
        test!(u8);
        test!(u16);
        test!(u32);
        test!(u64);
        test!(u128);
        test!(usize);
    }

    // ========== ModInt ==========

    #[test]
    fn test_act_modint() {
        type M = ModInt<998_244_353>;
        let act = ActRangeSet::<M>::default();

        // act(Some(5), e(10, 3)) = e(5*3, 3) = e(15, 3)
        assert_eq!(
            Action::<OpRangeSum<M>>::act(&act, &Some(M::new(5)), &e(M::new(10), M::new(3))),
            e(M::new(15), M::new(3))
        );

        // act(None, x) = x
        let x = e(M::new(1030), M::new(7));
        assert_eq!(
            Action::<OpRangeSum<M>>::act(&act, &None, &x),
            x,
            "act(None, x) = x"
        );

        // 合成則
        let g = Some(M::new(5));
        let f = Some(M::new(3));
        let composed = act.op(&g, &f);
        let lhs = Action::<OpRangeSum<M>>::act(&act, &composed, &x);
        let rhs =
            Action::<OpRangeSum<M>>::act(&act, &g, &Action::<OpRangeSum<M>>::act(&act, &f, &x));
        assert_eq!(lhs, rhs, "composition over ModInt");
    }
}
