use std::marker::PhantomData;

use crate::ops::{action::Action, monoid::Monoid};

/// LazySegmentTree用
/// 値をセット(上書き)する作用
/// `seg.act(l..r, &Some(x))`のように書くと[l, r)の区間の値をxにする
#[derive(Default, Clone)]
pub struct ActSet<T> {
    _phantom: PhantomData<T>,
}

impl<T> Monoid for ActSet<T>
where
    T: Clone,
{
    type Value = Option<T>;

    #[inline]
    fn identity(&self) -> Self::Value {
        None
    }

    #[inline]
    fn op(&self, g: &Self::Value, f: &Self::Value) -> Self::Value {
        if g.is_none() { f } else { g }.clone()
    }
}

impl<O> Action<O> for ActSet<O::Value>
where
    O: Monoid,
    O::Value: Clone,
{
    #[inline]
    fn act(&self, f: &Self::Value, x: &<O as Monoid>::Value) -> <O as Monoid>::Value {
        if let Some(f) = f { f } else { x }.clone()
    }
}
