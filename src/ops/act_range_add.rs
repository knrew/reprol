//! 区間加算作用(区間和用)
//!
//! 各要素に定数を加算する作用を表す可換モノイド．
//! 作用の合成は加算(lhs + rhs)で，恒等作用はゼロ値(`0`)．
//!
//! [`OpRangeSum`](crate::ops::op_range_sum::OpRangeSum)と組み合わせて，
//! 区間加算・区間和クエリを処理するための作用として使用する．
//!
//! # Examples
//!
//! ```
//! use reprol::ops::monoid::Monoid;
//! use reprol::ops::action::Action;
//! use reprol::ops::act_range_add::ActRangeAdd;
//! use reprol::ops::op_range_sum::{OpRangeSum, OpRangeSumElement};
//!
//! let act = ActRangeAdd::<i64>::default();
//! // 作用の合成: 3を足してから5を足す = 8を足す
//! assert_eq!(act.op(&5, &3), 8);
//! // 恒等作用
//! assert_eq!(act.op(&act.id(), &3), 3);
//! // OpRangeSumへの作用: value' = value + f * len
//! let x = OpRangeSumElement::with_count(10, 3);
//! let result = Action::<OpRangeSum<i64>>::act(&act, &5, &x);
//! assert_eq!(result.value(), 25); // 10 + 5 * 3
//! assert_eq!(result.len(), 3);
//! ```
//!
//! # Notes
//!
//! - 整数型では wrapping 演算を用いるため，オーバーフロー時は折り返す．

use std::marker::PhantomData;

use crate::{
    math::modint::ModInt,
    ops::{
        action::Action,
        monoid::{CommutativeMonoid, Monoid},
        op_range_sum::OpRangeSumElement,
    },
};

/// 区間加算作用(区間和用)
///
/// [`OpRangeSum`](crate::ops::op_range_sum::OpRangeSum)に対する加算作用を表す可換モノイド．
/// 標準のプリミティブ整数型と[`ModInt`]に対応する．
///
/// # Notes
///
/// - 整数型では wrapping 演算を用いるため，オーバーフロー時は折り返す．
#[derive(Default, Clone)]
pub struct ActRangeAdd<T>(PhantomData<T>);

impl<T> Monoid for ActRangeAdd<T>
where
    T: Copy + HasZeroValue + HasAdd,
{
    type Element = T;

    #[inline]
    fn op(&self, &lhs: &Self::Element, &rhs: &Self::Element) -> Self::Element {
        lhs.add(rhs)
    }

    #[inline]
    fn id(&self) -> Self::Element {
        T::ZERO
    }
}

impl<T> CommutativeMonoid for ActRangeAdd<T> where T: Copy + HasZeroValue + HasAdd {}

impl<T, O> Action<O> for ActRangeAdd<T>
where
    O: Monoid<Element = OpRangeSumElement<T>>,
    T: Copy + HasZeroValue + HasAdd + HasMul,
{
    #[inline]
    fn act(&self, &f: &Self::Element, x: &O::Element) -> O::Element {
        OpRangeSumElement::with_count(x.value().add(f.mul(x.len())), x.len()) // value += f * len
    }
}

/// 型固有のゼロ値を提供するトレイト．
trait HasZeroValue {
    const ZERO: Self;
}

/// 型固有の加算を提供するトレイト．
trait HasAdd {
    fn add(self, rhs: Self) -> Self;
}

/// 型固有の乗算を提供するトレイト．
trait HasMul {
    fn mul(self, rhs: Self) -> Self;
}

macro_rules! impl_act_range_add_traits_inner {
    ($ty: ty) => {
        impl HasZeroValue for $ty {
            const ZERO: Self = 0;
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

macro_rules! impl_act_range_add_traits {
    ($($ty: ty),* $(,)?) => {
        $( impl_act_range_add_traits_inner!($ty); )*
    };
}

impl_act_range_add_traits! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

impl<const P: u64> HasZeroValue for ModInt<P> {
    const ZERO: Self = Self::new(0);
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
        monoid::{CommutativeMonoid, Monoid},
        op_range_sum::{OpRangeSum, OpRangeSumElement},
    };

    fn assert_commutative_monoid<T: CommutativeMonoid>() {}

    fn e<T>(value: T, len: T) -> OpRangeSumElement<T> {
        OpRangeSumElement::with_count(value, len)
    }

    // ========== Monoid 軸 ==========

    #[test]
    fn test_id_returns_zero() {
        let m = ActRangeAdd::<i64>::default();
        assert_eq!(m.id(), 0);
    }

    #[test]
    fn test_op_basic() {
        let m = ActRangeAdd::<i64>::default();
        assert_eq!(m.op(&5, &3), 8);
        assert_eq!(m.op(&-10, &7), -3);
        assert_eq!(m.op(&1030, &0), 1030);
    }

    #[test]
    fn test_op_identity() {
        let m = ActRangeAdd::<i64>::default();
        let id = m.id();
        for f in [0, 1, -1, 1030, i64::MAX, i64::MIN] {
            assert_eq!(m.op(&id, &f), f, "op(id, {f}) = {f}");
            assert_eq!(m.op(&f, &id), f, "op({f}, id) = {f}");
        }
    }

    #[test]
    fn test_op_associativity() {
        let m = ActRangeAdd::<i64>::default();
        let cases = [(3, 1030, 7), (-5, 0, 5), (100, -50, 25)];
        for (h, g, f) in cases {
            let lhs = m.op(&m.op(&h, &g), &f);
            let rhs = m.op(&h, &m.op(&g, &f));
            assert_eq!(lhs, rhs, "op(op({h}, {g}), {f}) = op({h}, op({g}, {f}))");
        }
    }

    #[test]
    fn test_op_commutativity() {
        let m = ActRangeAdd::<i64>::default();
        let cases = [(3, 1030), (-5, 7), (0, 0), (i64::MAX, 1)];
        for (a, b) in cases {
            assert_eq!(m.op(&a, &b), m.op(&b, &a), "op({a}, {b}) = op({b}, {a})");
        }
    }

    #[test]
    fn test_op_boundary() {
        let m = ActRangeAdd::<u64>::default();
        assert_eq!(m.op(&u64::MAX, &1), 0, "MAX + 1 wraps");
        let m = ActRangeAdd::<i64>::default();
        assert_eq!(m.op(&i64::MAX, &1), i64::MIN, "MAX + 1 wraps to MIN");
        assert_eq!(m.op(&i64::MIN, &-1), i64::MAX, "MIN - 1 wraps to MAX");
    }

    // ========== Action 軸 (OpRangeSum) ==========

    #[test]
    fn test_act_identity() {
        let act = ActRangeAdd::<i64>::default();
        let id = act.id();
        let cases = [e(10, 3), e(0, 0), e(-5, 1), e(1030, 7)];
        for x in &cases {
            assert_eq!(
                Action::<OpRangeSum<i64>>::act(&act, &id, x),
                x.clone(),
                "act(0, {x:?}) = {x:?}"
            );
        }
    }

    #[test]
    fn test_act_basic() {
        let act = ActRangeAdd::<i64>::default();
        // act(5, e(10, 3)) = e(10 + 5*3, 3) = e(25, 3)
        assert_eq!(
            Action::<OpRangeSum<i64>>::act(&act, &5, &e(10, 3)),
            e(25, 3)
        );
        // act(-2, e(6, 4)) = e(6 + (-2)*4, 4) = e(-2, 4)
        assert_eq!(
            Action::<OpRangeSum<i64>>::act(&act, &-2, &e(6, 4)),
            e(-2, 4)
        );
    }

    #[test]
    fn test_act_len_zero_value_unchanged() {
        let act = ActRangeAdd::<i64>::default();
        for f in [0, 1, -1, 1030, i64::MAX] {
            let x = e(42, 0);
            assert_eq!(
                Action::<OpRangeSum<i64>>::act(&act, &f, &x),
                e(42, 0),
                "act({f}, e(42, 0)) = e(42, 0)"
            );
        }
    }

    #[test]
    fn test_act_preserves_len() {
        let act = ActRangeAdd::<i64>::default();
        for len in [0, 1, 3, 7, 1030] {
            let result = Action::<OpRangeSum<i64>>::act(&act, &5, &e(10, len));
            assert_eq!(result.len(), len, "act preserves len={len}");
        }
    }

    #[test]
    fn test_act_composition() {
        let act = ActRangeAdd::<i64>::default();
        let cases = [
            (5, 3, e(10, 3)),
            (0, 0, e(0, 0)),
            (-1, 1, e(1030, 7)),
            (100, -50, e(0, 1)),
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
                "act(op({g}, {f}), {x:?}) = act({g}, act({f}, {x:?}))"
            );
        }
    }

    #[test]
    fn test_act_boundary() {
        let act = ActRangeAdd::<u64>::default();
        // 大きな f*len でのwrapping
        assert_eq!(
            Action::<OpRangeSum<u64>>::act(&act, &u64::MAX, &e(1, 2)),
            e(1u64.wrapping_add(u64::MAX.wrapping_mul(2)), 2),
            "large f*len wraps"
        );
    }

    // ========== 全型スモークテスト ==========

    #[test]
    fn test_op_smoke_all_integer_types() {
        macro_rules! test {
            ($ty: ty) => {
                assert_commutative_monoid::<ActRangeAdd<$ty>>();
                let m = ActRangeAdd::<$ty>::default();
                assert_eq!(m.op(&3, &5), 8 as $ty, "op for {}", stringify!($ty));
                assert_eq!(m.id(), 0 as $ty, "id for {}", stringify!($ty));
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
        let m = ActRangeAdd::<M>::default();
        assert_commutative_monoid::<ActRangeAdd<M>>();

        assert_eq!(m.op(&M::new(3), &M::new(5)), M::new(8));
        assert_eq!(m.id(), M::new(0));

        let f = M::new(1030);
        assert_eq!(m.op(&m.id(), &f), f, "op(id, f) = f");
        assert_eq!(m.op(&f, &m.id()), f, "op(f, id) = f");

        let (a, b, c) = (M::new(1030), M::new(998_244_000), M::new(7));
        assert_eq!(
            m.op(&m.op(&a, &b), &c),
            m.op(&a, &m.op(&b, &c)),
            "associativity"
        );
        assert_eq!(m.op(&a, &b), m.op(&b, &a), "commutativity");
    }

    #[test]
    fn test_act_modint() {
        type M = ModInt<998_244_353>;
        let act = ActRangeAdd::<M>::default();

        // act(5, e(10, 3)) = e(10 + 5*3, 3) = e(25, 3)
        assert_eq!(
            Action::<OpRangeSum<M>>::act(&act, &M::new(5), &e(M::new(10), M::new(3))),
            e(M::new(25), M::new(3))
        );

        // act(id, x) = x
        let x = e(M::new(1030), M::new(7));
        assert_eq!(
            Action::<OpRangeSum<M>>::act(&act, &act.id(), &x),
            x,
            "act(id, x) = x"
        );

        // 合成則
        let (g, f) = (M::new(5), M::new(3));
        let composed = act.op(&g, &f);
        let lhs = Action::<OpRangeSum<M>>::act(&act, &composed, &x);
        let rhs =
            Action::<OpRangeSum<M>>::act(&act, &g, &Action::<OpRangeSum<M>>::act(&act, &f, &x));
        assert_eq!(lhs, rhs, "composition over ModInt");
    }
}
