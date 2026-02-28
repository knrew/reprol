//! モノイド作用(Action)
//!
//! モノイドに対する作用を表すトレイト．
//! 作用の合成もモノイドをなす．
//!
//! # Examples
//!
//! ```
//! use reprol::ops::monoid::Monoid;
//! use reprol::ops::action::Action;
//!
//! struct OpAdd;
//! impl Monoid for OpAdd {
//!     type Element = i64;
//!     fn op(&self, lhs: &i64, rhs: &i64) -> i64 { lhs + rhs }
//!     fn id(&self) -> i64 { 0 }
//! }
//!
//! struct ActAdd;
//! impl Monoid for ActAdd {
//!     type Element = i64;
//!     fn op(&self, lhs: &i64, rhs: &i64) -> i64 { lhs + rhs }
//!     fn id(&self) -> i64 { 0 }
//! }
//!
//! impl Action<OpAdd> for ActAdd {
//!     fn act(&self, f: &i64, x: &i64) -> i64 { x + f }
//! }
//!
//! let act = ActAdd;
//! assert_eq!(act.act(&3, &10), 13);
//! assert_eq!(act.act(&act.id(), &10), 10);
//! ```

use crate::ops::monoid::Monoid;

/// モノイド作用(Action)
///
/// `Operand` のモノイドに対して作用する．
/// 作用の合成はモノイドをなし，`op` で合成，`id` で恒等作用を表す．
///
/// 以下の条件を満たす:
/// - 恒等作用: `act(id(), x) = x`
/// - 合成との整合: `act(op(g, f), x) = act(g, act(f, x))`
///
/// # Notes
///
/// `op(g, f)` は `f` を先に適用し `g` を後に適用する合成を表す．
pub trait Action<Operand>: Monoid
where
    Operand: Monoid,
{
    /// 作用素 `f` を `x` に適用する．
    fn act(&self, f: &Self::Element, x: &Operand::Element) -> Operand::Element;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct OpAdd;

    impl Monoid for OpAdd {
        type Element = i64;
        fn op(&self, lhs: &i64, rhs: &i64) -> i64 {
            lhs + rhs
        }
        fn id(&self) -> i64 {
            0
        }
    }

    struct ActAdd;

    impl Monoid for ActAdd {
        type Element = i64;
        fn op(&self, lhs: &i64, rhs: &i64) -> i64 {
            lhs + rhs
        }
        fn id(&self) -> i64 {
            0
        }
    }

    impl Action<OpAdd> for ActAdd {
        fn act(&self, f: &i64, x: &i64) -> i64 {
            x + f
        }
    }

    #[test]
    fn test_act_identity() {
        let act = ActAdd;

        for x in [0, 1, -1, 42, i64::MAX, i64::MIN] {
            assert_eq!(act.act(&act.id(), &x), x, "act(id, {x}) = {x}");
        }
    }

    #[test]
    fn test_act_composition() {
        let act = ActAdd;

        let cases = [(3, 5, 10), (0, 0, 0), (-1, 1, 42), (100, -50, 0)];
        for (g, f, x) in cases {
            let composed = act.op(&g, &f);
            let lhs = act.act(&composed, &x);
            let rhs = act.act(&g, &act.act(&f, &x));
            assert_eq!(lhs, rhs, "act(op({g}, {f}), {x}) = act({g}, act({f}, {x}))");
        }
    }
}
