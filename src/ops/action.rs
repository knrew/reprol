use crate::ops::monoid::Monoid;

/// モノイドに対する作用
/// 作用としてのモノイドの演算(op)では合成作用$g \circ f$を計算する
/// ($h(x)=f(g(x))$となる$h$を求める)
/// ```rust
/// // NOTE: gが先に作用する
/// fn op(&self, g: &Self::Value, f: &Self::Value) -> Self::Value {
///     todo!()
/// }
/// ```
pub trait MonoidAction<Operand>: Monoid
where
    Operand: Monoid,
{
    // f: M -> M
    // xにfを作用させる(f(x)をする)
    fn act(&self, f: &Self::Value, x: &Operand::Value) -> Operand::Value;
}
