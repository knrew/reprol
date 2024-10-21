/// モノイド
/// 単位元と結合則を満たす演算を持つ
pub trait Monoid {
    type Value;

    /// 単位元eを返す
    /// 任意のxに対して，op(x, e)=op(e, x)=x
    fn identity(&self) -> Self::Value;

    /// 演算
    fn op(&self, x: &Self::Value, y: &Self::Value) -> Self::Value;
}
