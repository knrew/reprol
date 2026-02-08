use std::marker::PhantomData;

use crate::ops::{action::Action, monoid::Monoid};

/// LazySegmentTreeにおける区間代入作用
///
/// ## NOTE
/// - OpAddと組み合わせて使うことはできない
#[derive(Default, Clone)]
pub struct ActSet<T>(PhantomData<T>);

impl<T: Clone> Monoid for ActSet<T> {
    type Element = Option<T>;

    #[inline]
    fn id(&self) -> Self::Element {
        None
    }

    #[inline]
    fn op(&self, g: &Self::Element, f: &Self::Element) -> Self::Element {
        if g.is_none() { f } else { g }.clone()
    }
}

impl<O: Monoid> Action<O> for ActSet<O::Element>
where
    O::Element: Clone,
{
    #[inline]
    fn act(&self, f: &Self::Element, x: &<O as Monoid>::Element) -> <O as Monoid>::Element {
        if let Some(f) = f { f } else { x }.clone()
    }
}
