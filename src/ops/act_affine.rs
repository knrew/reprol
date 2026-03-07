//! アフィン変換作用
//!
//! アフィン変換 f(x) = a*x + b を作用とするモノイド．
//! 作用の合成は関数合成 g∘f(x) = (g.a*f.a)*x + (g.a*f.b + g.b) で，
//! 恒等作用は `{ a: 1, b: 0 }`(恒等写像)．
//!
//! [`OpRangeSum`](crate::ops::op_range_sum::OpRangeSum)と組み合わせて，
//! アフィン変換・区間和クエリを処理するための作用として使用する．
//!
//! # Examples
//!
//! ```
//! use reprol::ops::monoid::Monoid;
//! use reprol::ops::action::Action;
//! use reprol::ops::act_affine::{ActAffine, ActAffineElement};
//! use reprol::ops::op_range_sum::{OpRangeSum, OpRangeSumElement};
//!
//! let act = ActAffine::<i64>::default();
//! let f = ActAffineElement { a: 2, b: 3 }; // f(x) = 2x + 3
//! let g = ActAffineElement { a: 5, b: 7 }; // g(x) = 5x + 7
//! // 作用の合成: g∘f(x) = 5(2x + 3) + 7 = 10x + 22
//! let gf = act.op(&g, &f);
//! assert_eq!(gf.a, 10);
//! assert_eq!(gf.b, 22);
//! // 恒等作用
//! assert_eq!(act.op(&act.id(), &f), f);
//! // OpRangeSumへの作用: value' = a * value + b * len
//! let x = OpRangeSumElement::with_count(10, 3);
//! let result = Action::<OpRangeSum<i64>>::act(&act, &f, &x);
//! assert_eq!(result.value(), 29); // 2 * 10 + 3 * 3
//! assert_eq!(result.len(), 3);
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

/// アフィン変換パラメータ
///
/// f(x) = a*x + b を表す．
/// [`ActAffine`] の要素型として使用される．
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ActAffineElement<T> {
    /// 一次係数
    pub a: T,
    /// 定数項
    pub b: T,
}

/// アフィン変換作用
///
/// [`OpRangeSum`](crate::ops::op_range_sum::OpRangeSum)に対するアフィン変換作用を表すモノイド．
/// 標準のプリミティブ整数型と[`ModInt`]に対応する．
///
/// # Notes
///
/// - 整数型では wrapping 演算を用いるため，オーバーフロー時は折り返す．
#[derive(Default, Clone)]
pub struct ActAffine<T>(PhantomData<T>);

impl<T> Monoid for ActAffine<T>
where
    T: Copy + HasZeroValue + HasOneValue + HasAdd + HasMul,
{
    type Element = ActAffineElement<T>;

    #[inline]
    fn op(&self, g: &Self::Element, f: &Self::Element) -> Self::Element {
        // アフィン変換の合成:
        // g∘f(x) = g(f(x)) = g.a*(f.a*x + f.b) + g.b = (g.a*f.a)*x + (g.a*f.b + g.b)
        ActAffineElement {
            a: g.a.mul(f.a),
            b: g.a.mul(f.b).add(g.b),
        }
    }

    #[inline]
    fn id(&self) -> Self::Element {
        ActAffineElement {
            a: T::ONE,
            b: T::ZERO,
        }
    }
}

impl<T, O> Action<O> for ActAffine<T>
where
    O: Monoid<Element = OpRangeSumElement<T>>,
    T: Copy + HasZeroValue + HasOneValue + HasAdd + HasMul,
{
    #[inline]
    fn act(&self, f: &Self::Element, x: &O::Element) -> O::Element {
        // 区間和に対するアフィン変換: sum' = a*sum + b*len
        OpRangeSumElement::with_count(f.a.mul(x.value()).add(f.b.mul(x.len())), x.len())
    }
}

/// 型固有のゼロ値を提供するトレイト．
trait HasZeroValue {
    const ZERO: Self;
}

/// 型固有の1値を提供するトレイト．
trait HasOneValue {
    const ONE: Self;
}

/// 型固有の加算を提供するトレイト．
trait HasAdd {
    fn add(self, rhs: Self) -> Self;
}

/// 型固有の乗算を提供するトレイト．
trait HasMul {
    fn mul(self, rhs: Self) -> Self;
}

macro_rules! impl_act_affine_traits_inner {
    ($ty: ty) => {
        impl HasZeroValue for $ty {
            const ZERO: Self = 0;
        }

        impl HasOneValue for $ty {
            const ONE: Self = 1;
        }

        impl HasAdd for $ty {
            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                self.wrapping_add(rhs)
            }
        }

        impl HasMul for $ty {
            #[inline(always)]
            fn mul(self, rhs: Self) -> Self {
                self.wrapping_mul(rhs)
            }
        }
    };
}

macro_rules! impl_act_affine_traits {
    ($($ty: ty),* $(,)?) => {
        $( impl_act_affine_traits_inner!($ty); )*
    };
}

impl_act_affine_traits! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

impl<const P: u64> HasZeroValue for ModInt<P> {
    const ZERO: Self = Self::new(0);
}

impl<const P: u64> HasOneValue for ModInt<P> {
    const ONE: Self = Self::new(1);
}

impl<const P: u64> HasAdd for ModInt<P> {
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
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

    fn af(a: i64, b: i64) -> ActAffineElement<i64> {
        ActAffineElement { a, b }
    }

    // ========== Monoid 軸 ==========

    #[test]
    fn test_id_returns_identity_map() {
        let m = ActAffine::<i64>::default();
        let id = m.id();
        assert_eq!(id.a, 1, "id.a = 1");
        assert_eq!(id.b, 0, "id.b = 0");
    }

    #[test]
    fn test_op_basic() {
        let m = ActAffine::<i64>::default();
        // f(x) = 2x + 3, g(x) = 5x + 7
        // g∘f(x) = 5(2x+3) + 7 = 10x + 22
        let f = af(2, 3);
        let g = af(5, 7);
        let gf = m.op(&g, &f);
        assert_eq!(gf.a, 10, "g∘f: a = 10");
        assert_eq!(gf.b, 22, "g∘f: b = 22");
    }

    #[test]
    fn test_op_identity() {
        let m = ActAffine::<i64>::default();
        let id = m.id();
        let cases = [af(2, 3), af(0, 0), af(1, 0), af(0, 5), af(-3, 7)];
        for f in &cases {
            assert_eq!(m.op(&id, f), *f, "op(id, {f:?}) = {f:?}");
            assert_eq!(m.op(f, &id), *f, "op({f:?}, id) = {f:?}");
        }
    }

    #[test]
    fn test_op_associativity() {
        let m = ActAffine::<i64>::default();
        let cases = [
            (af(2, 3), af(5, 7), af(11, 13)),
            (af(0, 1), af(1, 0), af(-1, 1030)),
            (af(3, 0), af(0, 4), af(2, 2)),
        ];
        for (h, g, f) in &cases {
            let lhs = m.op(&m.op(h, g), f);
            let rhs = m.op(h, &m.op(g, f));
            assert_eq!(
                lhs, rhs,
                "op(op({h:?}, {g:?}), {f:?}) = op({h:?}, op({g:?}, {f:?}))"
            );
        }
    }

    #[test]
    fn test_op_not_commutative() {
        let m = ActAffine::<i64>::default();
        // f(x)=2x+1, g(x)=3x+4
        // g∘f = 6x+7, f∘g = 6x+9
        let f = af(2, 1);
        let g = af(3, 4);
        assert_ne!(m.op(&g, &f), m.op(&f, &g), "g∘f != f∘g");
    }

    #[test]
    fn test_op_boundary() {
        let m = ActAffine::<u64>::default();
        let f = ActAffineElement {
            a: u64::MAX,
            b: 1u64,
        };
        let g = ActAffineElement { a: 2u64, b: 0u64 };
        let gf = m.op(&g, &f);
        assert_eq!(gf.a, 2u64.wrapping_mul(u64::MAX), "wrapping a");
        assert_eq!(gf.b, 2u64.wrapping_mul(1), "wrapping b");
    }

    // ========== Action 軸 (OpRangeSum) ==========

    #[test]
    fn test_act_identity() {
        let act = ActAffine::<i64>::default();
        let id = act.id();
        let cases = [e(10, 3), e(0, 0), e(-5, 1), e(1030, 7)];
        for x in &cases {
            assert_eq!(
                Action::<OpRangeSum<i64>>::act(&act, &id, x),
                x.clone(),
                "act(id, {x:?}) = {x:?}"
            );
        }
    }

    #[test]
    fn test_act_basic() {
        let act = ActAffine::<i64>::default();
        // act({2, 3}, e(10, 3)) = e(2*10 + 3*3, 3) = e(29, 3)
        assert_eq!(
            Action::<OpRangeSum<i64>>::act(&act, &af(2, 3), &e(10, 3)),
            e(29, 3)
        );
    }

    #[test]
    fn test_act_len_zero_ignores_b() {
        let act = ActAffine::<i64>::default();
        // act({a, b}, e(v, 0)) = e(a*v, 0) — b*len項が消える
        let cases = [(2, 999, 5), (0, 100, 7), (3, 0, 10), (-1, 42, 1030)];
        for (a, b, v) in cases {
            assert_eq!(
                Action::<OpRangeSum<i64>>::act(&act, &af(a, b), &e(v, 0)),
                e(a * v, 0),
                "act(({a}, {b}), e({v}, 0)) = e({}, 0)",
                a * v
            );
        }
    }

    #[test]
    fn test_act_preserves_len() {
        let act = ActAffine::<i64>::default();
        for len in [0, 1, 3, 7, 1030] {
            let result = Action::<OpRangeSum<i64>>::act(&act, &af(2, 3), &e(10, len));
            assert_eq!(result.len(), len, "act preserves len={len}");
        }
    }

    #[test]
    fn test_act_composition() {
        let act = ActAffine::<i64>::default();
        let cases = [
            (af(2, 3), af(5, 7), e(10, 3)),
            (af(0, 1), af(1, 0), e(1030, 1)),
            (af(3, 0), af(0, 4), e(-5, 7)),
            (af(1, 1), af(1, 1), e(0, 0)),
        ];
        for (g, f, x) in &cases {
            let composed = act.op(g, f);
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

    #[test]
    fn test_act_boundary() {
        let act = ActAffine::<u64>::default();
        let f = ActAffineElement {
            a: u64::MAX,
            b: u64::MAX,
        };
        let x = e(2u64, 3u64);
        let expected = e(
            u64::MAX
                .wrapping_mul(2)
                .wrapping_add(u64::MAX.wrapping_mul(3)),
            3,
        );
        assert_eq!(
            Action::<OpRangeSum<u64>>::act(&act, &f, &x),
            expected,
            "act({{MAX, MAX}}, e(2, 3)) wraps correctly"
        );
    }

    // ========== 全型スモークテスト ==========

    #[test]
    fn test_op_smoke_all_integer_types() {
        macro_rules! test {
            ($ty: ty) => {
                assert_monoid::<ActAffine<$ty>>();
                let m = ActAffine::<$ty>::default();
                let id = m.id();
                assert_eq!(id.a, 1 as $ty, "id.a for {}", stringify!($ty));
                assert_eq!(id.b, 0 as $ty, "id.b for {}", stringify!($ty));
                // f(x) = 2x+3, g(x) = 5x+7 → g∘f = 10x+22
                let f = ActAffineElement {
                    a: 2 as $ty,
                    b: 3 as $ty,
                };
                let g = ActAffineElement {
                    a: 5 as $ty,
                    b: 7 as $ty,
                };
                let gf = m.op(&g, &f);
                assert_eq!(gf.a, 10 as $ty, "op.a for {}", stringify!($ty));
                assert_eq!(gf.b, 22 as $ty, "op.b for {}", stringify!($ty));
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
    fn test_op_modint() {
        type M = ModInt<998_244_353>;
        let m = ActAffine::<M>::default();
        assert_monoid::<ActAffine<M>>();

        let id = m.id();
        assert_eq!(id.a, M::new(1));
        assert_eq!(id.b, M::new(0));

        // f(x) = 2x+3, g(x) = 5x+7 → g∘f = 10x+22
        let f = ActAffineElement {
            a: M::new(2),
            b: M::new(3),
        };
        let g = ActAffineElement {
            a: M::new(5),
            b: M::new(7),
        };
        let gf = m.op(&g, &f);
        assert_eq!(gf.a, M::new(10));
        assert_eq!(gf.b, M::new(22));

        // 恒等元則
        assert_eq!(m.op(&id, &f), f, "op(id, f) = f");
        assert_eq!(m.op(&f, &id), f, "op(f, id) = f");

        // 結合律
        let h = ActAffineElement {
            a: M::new(11),
            b: M::new(13),
        };
        assert_eq!(
            m.op(&m.op(&h, &g), &f),
            m.op(&h, &m.op(&g, &f)),
            "associativity"
        );
    }

    #[test]
    fn test_act_modint() {
        type M = ModInt<998_244_353>;
        let act = ActAffine::<M>::default();

        // act({2, 3}, e(10, 3)) = e(2*10 + 3*3, 3) = e(29, 3)
        let f = ActAffineElement {
            a: M::new(2),
            b: M::new(3),
        };
        assert_eq!(
            Action::<OpRangeSum<M>>::act(&act, &f, &e(M::new(10), M::new(3))),
            e(M::new(29), M::new(3))
        );

        // act(id, x) = x
        let x = e(M::new(1030), M::new(7));
        assert_eq!(
            Action::<OpRangeSum<M>>::act(&act, &act.id(), &x),
            x,
            "act(id, x) = x"
        );

        // 合成則
        let g = ActAffineElement {
            a: M::new(5),
            b: M::new(7),
        };
        let composed = act.op(&g, &f);
        let lhs = Action::<OpRangeSum<M>>::act(&act, &composed, &x);
        let rhs =
            Action::<OpRangeSum<M>>::act(&act, &g, &Action::<OpRangeSum<M>>::act(&act, &f, &x));
        assert_eq!(lhs, rhs, "composition over ModInt");
    }
}
