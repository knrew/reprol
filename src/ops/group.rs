//! 群(Group)
//!
//! 逆元を持つモノイドを表すトレイト．
//!
//! # Examples
//!
//! ```
//! use reprol::ops::monoid::Monoid;
//! use reprol::ops::group::Group;
//!
//! struct OpAdd;
//!
//! impl Monoid for OpAdd {
//!     type Element = i64;
//!     fn op(&self, lhs: &i64, rhs: &i64) -> i64 { lhs + rhs }
//!     fn id(&self) -> i64 { 0 }
//! }
//!
//! impl Group for OpAdd {
//!     fn inv(&self, x: &i64) -> i64 { -x }
//! }
//!
//! let m = OpAdd;
//! let x = 42;
//! assert_eq!(m.op(&x, &m.inv(&x)), m.id());
//! assert_eq!(m.op(&m.inv(&x), &x), m.id());
//! ```

use crate::ops::monoid::Monoid;

/// 群(Group)
///
/// 逆元を持つモノイド．
/// 単位元を `e` として，集合の任意の要素 `x` に対して逆元 `y` が存在し，
/// `x * y = y * x = e` を満たす．
pub trait Group: Monoid {
    /// `x` の逆元を返す．
    fn inv(&self, x: &<Self as Monoid>::Element) -> Self::Element;
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

    impl Group for OpAdd {
        fn inv(&self, x: &i64) -> i64 {
            -x
        }
    }

    #[test]
    fn test_inv_inverse() {
        let add = OpAdd;

        for x in [0, 1, -1, 42, i64::MAX] {
            let inv_x = add.inv(&x);
            assert_eq!(add.op(&x, &inv_x), add.id(), "x * inv(x) = id for x={x}");
            assert_eq!(add.op(&inv_x, &x), add.id(), "inv(x) * x = id for x={x}");
        }
    }
}
