use std::{collections::BTreeSet, ops::RangeInclusive};

/// 区間をsetで管理するやつ
pub struct IntervalSet<T>(BTreeSet<(T, T)>);

impl<T> IntervalSet<T>
where
    T: Integer,
{
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn insert(&mut self, range: RangeInclusive<T>) {
        let mut l = *range.start();
        let mut r = *range.end();

        assert!(l <= r);

        if let Some(&(ll, rr)) = self.0.range(..=(l, T::sup())).max() {
            if l <= rr {
                l = ll;
            }
        }

        if let Some(&(_, rr)) = self.0.range(..=(r, T::sup())).max() {
            if r <= rr {
                r = rr;
            }
        }

        let mut sub = vec![];

        for &(ll, rr) in self.0.range((l, T::inf())..=(r, T::sup())) {
            sub.push((ll, rr));
        }

        for x in sub {
            self.0.remove(&x);
        }

        self.0.insert((l, r));
    }

    pub fn contains(&self, x: T) -> bool {
        if let Some(&(_, r)) = self.0.range((x, T::inf())..).min() {
            x <= r
        } else {
            false
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &(T, T)> {
        self.0.iter()
    }

    pub fn into_iter(self) -> impl IntoIterator<Item = (T, T)> {
        self.0.into_iter()
    }
}

pub trait Integer: Ord + Copy {
    fn inf() -> Self;
    fn sup() -> Self;
}

macro_rules! impl_integer {
    ($($ty:ident),*) => {$(
        impl Integer for $ty {
            fn inf() -> Self {
                $ty::MIN
            }
            fn sup() -> Self {
                $ty::MAX
            }
        }
    )*};
}

impl_integer! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }
