/// モノイド
/// 単位元と結合則を満たす演算を持つ
pub trait Monoid {
    type Value;

    /// 演算
    /// $x \cdot y$
    fn op(&self, lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;

    /// 単位元$e$を返す
    /// 任意の$x$に対して，$x \cdot e = e \cdot x = x$
    fn identity(&self) -> Self::Value;
}
