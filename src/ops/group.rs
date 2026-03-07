//! 群(Group)
//!
//! 逆元を持つモノイドを表すトレイト．
//!
//! - [`Group`]: 群
//! - [`AbelianGroup`]: 可換性を持つ群
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

use crate::ops::monoid::{CommutativeMonoid, Monoid};

/// 群(Group)
///
/// 逆元を持つモノイド．
/// 単位元を `e` として，集合の任意の要素 `x` に対して逆元 `y` が存在し，
/// `x * y = y * x = e` を満たす．
pub trait Group: Monoid {
    /// `x` の逆元を返す．
    fn inv(&self, x: &<Self as Monoid>::Element) -> Self::Element;
}

/// アーベル群(Abelian Group)
///
/// 集合の任意の要素 `x`, `y` に対して `x * y = y * x` を満たす群．
pub trait AbelianGroup: Group + CommutativeMonoid {}

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

    impl CommutativeMonoid for OpAdd {}

    impl AbelianGroup for OpAdd {}

    fn assert_abelian_group<T: AbelianGroup>() {}

    fn assert_group<T: Group>() {}

    fn assert_commutative_monoid<T: CommutativeMonoid>() {}

    #[test]
    fn test_abelian_group_supertraits() {
        assert_abelian_group::<OpAdd>();
        assert_group::<OpAdd>();
        assert_commutative_monoid::<OpAdd>();
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
