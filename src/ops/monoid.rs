//! モノイド(Monoid)
//!
//! 結合的な二項演算と単位元を持つ代数的構造を表すトレイト．
//!
//! # Examples
//!
//! ```
//! use reprol::ops::monoid::Monoid;
//!
//! struct OpAdd;
//!
//! impl Monoid for OpAdd {
//!     type Element = i64;
//!     fn op(&self, lhs: &i64, rhs: &i64) -> i64 { lhs + rhs }
//!     fn id(&self) -> i64 { 0 }
//! }
//!
//! let m = OpAdd;
//! assert_eq!(m.op(&3, &5), 8);
//! assert_eq!(m.op(&m.id(), &42), 42);
//! ```

/// モノイド(Monoid)
///
/// 単位元と結合則を満たす二項演算を持つ．
/// 単位元を `e` として，集合の任意の要素 `x` に対して `x * e = e * x = x` を満たす．
pub trait Monoid {
    type Element;

    /// 演算
    ///
    /// `lhs * rhs`
    fn op(&self, lhs: &Self::Element, rhs: &Self::Element) -> Self::Element;

    /// 単位元
    fn id(&self) -> Self::Element;
}

/// 冪等モノイド(Idempotent Monoid)
///
/// 集合の任意の要素 `x` に対して `x * x = x` を満たすモノイド．
pub trait IdempotentMonoid: Monoid {}

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

    struct OpMax;

    impl Monoid for OpMax {
        type Element = i64;
        fn op(&self, lhs: &i64, rhs: &i64) -> i64 {
            *lhs.max(rhs)
        }
        fn id(&self) -> i64 {
            i64::MIN
        }
    }

    impl IdempotentMonoid for OpMax {}

    #[test]
    fn test_op_identity() {
        let add = OpAdd;
        let max = OpMax;

        for x in [0, 1, -1, 42, i64::MAX, i64::MIN] {
            assert_eq!(add.op(&add.id(), &x), x, "OpAdd: id * {x} = {x}");
            assert_eq!(add.op(&x, &add.id()), x, "OpAdd: {x} * id = {x}");
            assert_eq!(max.op(&max.id(), &x), x, "OpMax: id * {x} = {x}");
            assert_eq!(max.op(&x, &max.id()), x, "OpMax: {x} * id = {x}");
        }
    }

    #[test]
    fn test_op_associativity() {
        let add = OpAdd;
        let max = OpMax;

        let cases = [(1, 2, 3), (-5, 0, 5), (0, 0, 0), (100, -50, 25)];
        for (a, b, c) in cases {
            let lhs = add.op(&add.op(&a, &b), &c);
            let rhs = add.op(&a, &add.op(&b, &c));
            assert_eq!(lhs, rhs, "OpAdd: ({a} * {b}) * {c} = {a} * ({b} * {c})");

            let lhs = max.op(&max.op(&a, &b), &c);
            let rhs = max.op(&a, &max.op(&b, &c));
            assert_eq!(lhs, rhs, "OpMax: ({a} * {b}) * {c} = {a} * ({b} * {c})");
        }
    }

    #[test]
    fn test_idempotent_idempotency() {
        let max = OpMax;

        for x in [0, 1, -1, 42, i64::MAX, i64::MIN] {
            assert_eq!(max.op(&x, &x), x, "OpMax: {x} * {x} = {x}");
        }
    }
}
