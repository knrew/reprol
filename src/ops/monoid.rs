//! モノイド(Monoid)

/// モノイド(Monoid)
///
/// 単位元と結合則を満たす演算を持つ．
/// 単位元を`e`として，集合の任意の要素`x`に対して，`x * e = e * x =x`を満たす．
pub trait Monoid {
    type Element;

    /// 演算
    ///
    /// `lhs * rhs`
    fn op(&self, lhs: &Self::Element, rhs: &Self::Element) -> Self::Element;

    /// 単位元
    fn id(&self) -> Self::Element;
}

/// 冪等モノイド(Idenmpotent Monoid)
/// 集合の任意の要素`x`に対して，`x * x = x`を満たす．
pub trait IdempotentMonoid: Monoid {}
