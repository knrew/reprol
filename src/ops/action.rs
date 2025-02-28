use crate::ops::monoid::Monoid;

/// モノイドに対する作用
pub trait MonoidAction<Operand>: Monoid
where
    Operand: Monoid,
{
    // f: M -> M
    // xにfを作用させる(f(x)をする)
    fn act(&self, f: &Self::Value, x: &Operand::Value) -> Operand::Value;
}
