use crate::monoid::Monoid;

/// 群
pub trait Group: Monoid {
    /// 逆元
    /// 元xに対して，op(x, y)=op(y, x)=eを満たすyを返す
    fn inv(&self, x: &<Self as Monoid>::Value) -> Self::Value;
}
