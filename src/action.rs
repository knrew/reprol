use crate::monoid::Monoid;

/// モノイドに対する作用
pub trait Action<Operand>: Monoid
where
    Operand: Monoid,
{
    // f: M -> M
    // xにfを作用させる(f(x)をする)
    fn act(&self, f: &<Self as Monoid>::Value, x: &Operand::Value) -> Operand::Value;
}
