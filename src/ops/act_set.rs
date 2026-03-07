//! 代入作用
//!
//! 各要素を定数で上書きする作用を表すモノイド．
//! 作用の合成では，後の作用が優先される(`op(g, f)` で f を先に適用し g が上書き)．
//! 恒等作用は`None`．
//!
//! [`OpMax`](crate::ops::op_max::OpMax)や[`OpMin`](crate::ops::op_min::OpMin)など，
//! 各要素を独立に置換できるモノイドと組み合わせて使用する．
//!
//! # Examples
//!
//! ```
//! use reprol::ops::monoid::Monoid;
//! use reprol::ops::action::Action;
//! use reprol::ops::act_set::ActSet;
//! use reprol::ops::op_max::OpMax;
//!
//! let act = ActSet::<i64>::default();
//! // 作用の合成: 後が優先
//! assert_eq!(act.op(&Some(7), &Some(3)), Some(7));
//! assert_eq!(act.op(&None, &Some(3)), Some(3));
//! // 恒等作用
//! assert_eq!(act.op(&act.id(), &Some(5)), Some(5));
//! // OpMaxへの作用: 値を上書き
//! let result = Action::<OpMax<i64>>::act(&act, &Some(10), &3);
//! assert_eq!(result, 10);
//! // None(恒等作用)では元の値を保持
//! let result = Action::<OpMax<i64>>::act(&act, &None, &3);
//! assert_eq!(result, 3);
//! ```
//!
//! # Notes
//!
//! - [`OpAdd`](crate::ops::op_add::OpAdd)や[`OpRangeSum`](crate::ops::op_range_sum::OpRangeSum)には使用できない．
//!   区間和と組み合わせる場合は[`ActRangeSet`](crate::ops::act_range_set::ActRangeSet)を使用する．

use std::marker::PhantomData;

use crate::ops::{action::Action, monoid::Monoid};

/// 代入作用
///
/// 各要素を定数で上書きする作用を表すモノイド．
/// 作用素の型は`Option<T>`で，`None`が恒等作用，`Some(v)`が値`v`への上書きを表す．
///
/// # Notes
///
/// - [`OpMax`](crate::ops::op_max::OpMax)や[`OpMin`](crate::ops::op_min::OpMin)と組み合わせて使用する．
/// - [`OpAdd`](crate::ops::op_add::OpAdd)や[`OpRangeSum`](crate::ops::op_range_sum::OpRangeSum)には使用できない．
#[derive(Default, Clone, Copy)]
pub struct ActSet<T>(PhantomData<T>);

impl<T: Clone> Monoid for ActSet<T> {
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

impl<O: Monoid> Action<O> for ActSet<O::Element>
where
    O::Element: Clone,
{
    #[inline]
    fn act(&self, f: &Self::Element, x: &<O as Monoid>::Element) -> <O as Monoid>::Element {
        if let Some(f) = f { f } else { x }.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::{action::Action, monoid::Monoid, op_max::OpMax};

    fn assert_monoid<T: Monoid>() {}

    // ========== Monoid 軸 ==========

    #[test]
    fn test_id_returns_none() {
        let m = ActSet::<i64>::default();
        assert_eq!(m.id(), None);
    }

    #[test]
    fn test_op_both_some() {
        let m = ActSet::<i64>::default();
        assert_eq!(m.op(&Some(7), &Some(3)), Some(7));
    }

    #[test]
    fn test_op_identity() {
        let m = ActSet::<i64>::default();
        let id = m.id();
        let cases: &[Option<i64>] = &[None, Some(0), Some(1030), Some(-1), Some(i64::MAX)];
        for f in cases {
            assert_eq!(m.op(&id, f), *f, "op(id, {f:?}) = {f:?}");
            assert_eq!(m.op(f, &id), *f, "op({f:?}, id) = {f:?}");
        }
    }

    #[test]
    fn test_op_associativity() {
        let m = ActSet::<i64>::default();
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

    // ========== Action 軸 (OpMax) ==========

    #[test]
    fn test_act_identity_op_max() {
        let act = ActSet::<i64>::default();
        let id = act.id(); // None
        for x in [0, 1, -1, 1030, i64::MAX, i64::MIN] {
            assert_eq!(
                Action::<OpMax<i64>>::act(&act, &id, &x),
                x,
                "act(None, {x}) = {x}"
            );
        }
    }

    #[test]
    fn test_act_some_overwrites_op_max() {
        let act = ActSet::<i64>::default();
        assert_eq!(
            Action::<OpMax<i64>>::act(&act, &Some(10), &3),
            10,
            "act(Some(10), 3) = 10"
        );
        assert_eq!(
            Action::<OpMax<i64>>::act(&act, &Some(-5), &100),
            -5,
            "act(Some(-5), 100) = -5"
        );
    }

    #[test]
    fn test_act_composition_op_max() {
        let act = ActSet::<i64>::default();
        let vals: &[Option<i64>] = &[None, Some(3), Some(1030), Some(-7)];
        for g in vals {
            for f in vals {
                let composed = act.op(g, f);
                for &x in &[0i64, 1030, -1, i64::MAX] {
                    let lhs = Action::<OpMax<i64>>::act(&act, &composed, &x);
                    let rhs =
                        Action::<OpMax<i64>>::act(&act, g, &Action::<OpMax<i64>>::act(&act, f, &x));
                    assert_eq!(
                        lhs, rhs,
                        "act(op({g:?}, {f:?}), {x}) = act({g:?}, act({f:?}, {x}))"
                    );
                }
            }
        }
    }

    // ========== 全型スモークテスト ==========

    #[test]
    fn test_op_smoke_all_integer_types() {
        macro_rules! test {
            ($ty: ty) => {
                assert_monoid::<ActSet<$ty>>();
                let m = ActSet::<$ty>::default();
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

    #[test]
    fn test_op_clone_only_type() {
        let m = ActSet::<String>::default();

        // 恒等元
        assert_eq!(m.id(), None, "id() == None");

        // op(Some, Some): g優先
        assert_eq!(
            m.op(&Some("b".to_string()), &Some("a".to_string())),
            Some("b".to_string()),
            "op(Some(b), Some(a)) = Some(b)"
        );

        // op(None, Some): fを返す
        assert_eq!(
            m.op(&None, &Some("a".to_string())),
            Some("a".to_string()),
            "op(None, Some(a)) = Some(a)"
        );

        // 結合則(3要素)
        let h = Some("h".to_string());
        let g = Some("g".to_string());
        let f = Some("f".to_string());
        let lhs = m.op(&m.op(&h, &g), &f);
        let rhs = m.op(&h, &m.op(&g, &f));
        assert_eq!(lhs, rhs, "associativity with String");
    }
}
