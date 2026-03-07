//! 加算作用
//!
//! 各要素に定数を加算する作用を表す可換モノイド．
//! 作用の合成は加算(lhs + rhs)で，恒等作用はゼロ値(`0`)．
//!
//! [`OpMax`](crate::ops::op_max::OpMax)や[`OpMin`](crate::ops::op_min::OpMin)など，
//! 各要素に独立に加算できるモノイドと組み合わせて使用する．
//!
//! # Examples
//!
//! ```
//! use reprol::ops::monoid::Monoid;
//! use reprol::ops::action::Action;
//! use reprol::ops::act_add::ActAdd;
//! use reprol::ops::op_max::OpMax;
//!
//! let act = ActAdd::<i64>::default();
//! // 作用の合成: 3を足してから5を足す = 8を足す
//! assert_eq!(act.op(&5, &3), 8);
//! // 恒等作用
//! assert_eq!(act.op(&act.id(), &3), 3);
//! // OpMaxへの作用: 値に加算
//! let result = Action::<OpMax<i64>>::act(&act, &10, &3);
//! assert_eq!(result, 13);
//! ```
//!
//! # Notes
//!
//! - [`OpAdd`](crate::ops::op_add::OpAdd)や[`OpRangeSum`](crate::ops::op_range_sum::OpRangeSum)，[`OpGcd`](crate::ops::op_gcd::OpGcd)には使用できない．
//!   区間和と組み合わせる場合は[`ActRangeAdd`](crate::ops::act_range_add::ActRangeAdd)を使用する．
//! - 整数型では wrapping 演算を用いるため，オーバーフロー時は折り返す．

use std::marker::PhantomData;

use crate::{
    math::modint::ModInt,
    ops::{
        action::Action,
        monoid::{CommutativeMonoid, Monoid},
    },
};

/// 加算作用
///
/// 各要素に定数を加算する作用を表す可換モノイド．
/// 標準のプリミティブ整数型と[`ModInt`]に対応する．
///
/// # Notes
///
/// - [`OpMax`](crate::ops::op_max::OpMax)や[`OpMin`](crate::ops::op_min::OpMin)と組み合わせて使用する．
/// - [`OpAdd`](crate::ops::op_add::OpAdd)や[`OpRangeSum`](crate::ops::op_range_sum::OpRangeSum)，[`OpGcd`](crate::ops::op_gcd::OpGcd)には使用できない．
#[derive(Default, Clone, Copy)]
pub struct ActAdd<T>(PhantomData<T>);

impl<T> Monoid for ActAdd<T>
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

impl<T> CommutativeMonoid for ActAdd<T> where T: Copy + HasZeroValue + HasAdd {}

impl<O> Action<O> for ActAdd<O::Element>
where
    O: Monoid,
    O::Element: Copy + HasZeroValue + HasAdd,
{
    #[inline]
    fn act(&self, &f: &Self::Element, &x: &<O as Monoid>::Element) -> <O as Monoid>::Element {
        x.add(f)
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

macro_rules! impl_act_add_traits_inner {
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
    };
}

macro_rules! impl_act_add_traits {
    ($($ty: ty),* $(,)?) => {
        $( impl_act_add_traits_inner!($ty); )*
    };
}

impl_act_add_traits! {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::{
        action::Action,
        monoid::{CommutativeMonoid, Monoid},
        op_max::OpMax,
    };

    fn assert_commutative_monoid<T: CommutativeMonoid>() {}

    // ========== Monoid 軸 ==========

    #[test]
    fn test_id_returns_zero() {
        let m = ActAdd::<i64>::default();
        assert_eq!(m.id(), 0);
    }

    #[test]
    fn test_op_basic() {
        let m = ActAdd::<u64>::default();
        assert_eq!(m.op(&74, &33), 107);
        assert_eq!(m.op(&0, &1030), 1030);
        let m = ActAdd::<i64>::default();
        assert_eq!(m.op(&-3, &7), 4);
        assert_eq!(m.op(&-10, &-20), -30);
    }

    #[test]
    fn test_op_identity() {
        let m = ActAdd::<i64>::default();
        let id = m.id();
        for f in [0, 1, -1, 1030, i64::MAX, i64::MIN] {
            assert_eq!(m.op(&id, &f), f, "op(id, {f}) = {f}");
            assert_eq!(m.op(&f, &id), f, "op({f}, id) = {f}");
        }
    }

    #[test]
    fn test_op_associativity() {
        let m = ActAdd::<i64>::default();
        let cases = [(3, 1030, 7), (-5, 0, 5), (100, -50, 25), (0, 0, 0)];
        for (h, g, f) in cases {
            let lhs = m.op(&m.op(&h, &g), &f);
            let rhs = m.op(&h, &m.op(&g, &f));
            assert_eq!(lhs, rhs, "op(op({h}, {g}), {f}) = op({h}, op({g}, {f}))");
        }
    }

    #[test]
    fn test_op_commutativity() {
        let m = ActAdd::<i64>::default();
        let cases = [(3, 1030), (-5, 7), (0, 0), (i64::MAX, 1)];
        for (a, b) in cases {
            assert_eq!(m.op(&a, &b), m.op(&b, &a), "op({a}, {b}) = op({b}, {a})");
        }
    }

    #[test]
    fn test_op_boundary() {
        let m = ActAdd::<u64>::default();
        assert_eq!(m.op(&u64::MAX, &1), 0, "MAX + 1 wraps to 0");
        assert_eq!(
            m.op(&u64::MAX, &u64::MAX),
            u64::MAX.wrapping_add(u64::MAX),
            "MAX + MAX wraps"
        );
        let m = ActAdd::<i64>::default();
        assert_eq!(m.op(&i64::MAX, &1), i64::MIN, "MAX + 1 wraps to MIN");
        assert_eq!(m.op(&i64::MIN, &-1), i64::MAX, "MIN + (-1) wraps to MAX");
    }

    // ========== Action 軸 (OpMax) ==========

    #[test]
    fn test_act_identity_op_max() {
        let act = ActAdd::<i64>::default();
        let id = act.id();
        for x in [0, 1, -1, 1030, i64::MAX, i64::MIN] {
            assert_eq!(
                Action::<OpMax<i64>>::act(&act, &id, &x),
                x,
                "act(id, {x}) = {x}"
            );
        }
    }

    #[test]
    fn test_act_basic_op_max() {
        let act = ActAdd::<i64>::default();
        assert_eq!(Action::<OpMax<i64>>::act(&act, &10, &3), 13);
        assert_eq!(Action::<OpMax<i64>>::act(&act, &-5, &10), 5);
    }

    #[test]
    fn test_act_composition_op_max() {
        let act = ActAdd::<i64>::default();
        let cases = [(5, 3, 10), (0, 0, 0), (-1, 1, 1030), (100, -50, 0)];
        for (g, f, x) in cases {
            let composed = act.op(&g, &f);
            let lhs = Action::<OpMax<i64>>::act(&act, &composed, &x);
            let rhs = Action::<OpMax<i64>>::act(&act, &g, &Action::<OpMax<i64>>::act(&act, &f, &x));
            assert_eq!(lhs, rhs, "act(op({g}, {f}), {x}) = act({g}, act({f}, {x}))");
        }
    }

    // ========== 全型スモークテスト ==========

    #[test]
    fn test_op_smoke_all_integer_types() {
        macro_rules! test {
            ($ty: ty) => {
                assert_commutative_monoid::<ActAdd<$ty>>();
                let m = ActAdd::<$ty>::default();
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
        let m = ActAdd::<M>::default();
        assert_commutative_monoid::<ActAdd<M>>();

        // 基本演算
        assert_eq!(m.op(&M::new(3), &M::new(5)), M::new(8));
        assert_eq!(m.id(), M::new(0));

        // 恒等元則
        let f = M::new(1030);
        assert_eq!(m.op(&m.id(), &f), f, "op(id, f) = f");
        assert_eq!(m.op(&f, &m.id()), f, "op(f, id) = f");

        // 結合律
        let (a, b, c) = (M::new(1030), M::new(998_244_000), M::new(7));
        assert_eq!(
            m.op(&m.op(&a, &b), &c),
            m.op(&a, &m.op(&b, &c)),
            "associativity"
        );

        // 可換律
        assert_eq!(m.op(&a, &b), m.op(&b, &a), "commutativity");

        // mod 折り返し
        assert_eq!(
            m.op(&M::new(998_244_350), &M::new(10)),
            M::new(7),
            "998244350 + 10 = 7 (mod P)"
        );
    }
}
