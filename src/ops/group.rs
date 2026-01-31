//! 群(Group)

use crate::ops::monoid::Monoid;

/// 群(Group)
///
/// 逆元を持つモノイド．
/// 単位元を`e`として，集合の任意の要素`x`に対して，逆元`y`が存在して，
/// `x * y =y * x　= e`を満たす．
pub trait Group: Monoid {
    /// `x`の逆元を返す．
    fn inv(&self, x: &<Self as Monoid>::Element) -> Self::Element;
}
