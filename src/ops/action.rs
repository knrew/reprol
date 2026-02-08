use crate::ops::monoid::Monoid;

/// モノイドに対する作用
/// 作用としてのモノイドの演算`op(g, f)`では合成作用g \circ f$を計算する
/// ($h(x)=f(g(x))$となる$h$を求める)
pub trait Action<Operand>: Monoid
where
    Operand: Monoid,
{
    // f: M -> M
    // xにfを作用させる(f(x)をする)
    fn act(&self, f: &Self::Element, x: &Operand::Element) -> Operand::Element;
}
