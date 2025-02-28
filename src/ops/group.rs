use crate::ops::monoid::Monoid;

/// 群
pub trait Group: Monoid {
    /// 逆元$x^{-1}$を返す
    /// 単位元を$e$として，任意の元$x$に対して，元$x^{-1}$が存在して，
    /// $x \cdot x^{-1} = x^{-1} \cdot x = e$を満たす
    fn inv(&self, x: &<Self as Monoid>::Value) -> Self::Value;
}
