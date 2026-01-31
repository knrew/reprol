//! モノイド(Monoid)

/// モノイド(Monoid)
///
/// 単位元と結合則を満たす演算を持つ．
/// 単位元を`e`として，集合の任意の要素`x`に対して，`x * e = e * x =x`を満たす．
pub trait Monoid {
    type Value;

    /// 演算
    /// `lhs * rhs`
    fn op(&self, lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;

    /// 演算に関する単位元を返す．
    fn identity(&self) -> Self::Value;
}

/// 冪等モノイド(Idenmpotent Monoid)
/// 集合の任意の要素`x`に対して，`x * x = x`を満たす．
pub trait IdempotentMonoid: Monoid {}
