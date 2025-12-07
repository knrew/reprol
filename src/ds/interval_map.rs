//! 区間を値とセットで保持する`IntervalMap`とその補助構造体群．
//!
//! 半開区間`[l, r)`を互いに素になるよう管理し，同じ値で連続する部分は自動的に併合される．
//! 区間の一部を削除すると自動で分割され，常に重ならない列として保持される．
//!
//! # 使用例
//! ```
//! use reprol::ds::interval_map::{Interval, IntervalMap, IntervalSet};
//!
//! let mut map = IntervalMap::new();
//! map.insert(0..3, "A");
//! map.insert(3..6, "A");
//! map.insert(6..9, "B");
//! assert_eq!(map.superset_of(1..5).map(|(_, v)| *v), Some("A"));
//! assert!(map.superset_of(2..8).is_none());
//!
//! let mut set = IntervalSet::new();
//! set.insert(1..4);
//! set.insert(4..6);
//! assert_eq!(set.iter().map(|itv| (itv.start(), itv.end())).collect::<Vec<_>>(), vec![(1, 6)]);
//! assert_eq!(set.remove(2..3), vec![Interval::new(2..3)]);
//! ```

use std::{
    collections::BTreeMap,
    fmt::Debug,
    iter::FusedIterator,
    ops::{Bound, Range, RangeBounds, Sub},
};

/// `IntervalMap`内部で扱う半開区間を表す構造体．
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct Interval<T> {
    range: Range<T>,
}

#[allow(private_bounds)]
impl<T: IntervalMapElement> Interval<T> {
    /// 半開区間`[range.start, range.end)`をそのまま保持する`Interval`を生成する．
    pub fn new(range: Range<T>) -> Self {
        Self { range }
    }

    /// 任意の`RangeBounds`を半開区間に正規化して`Interval`を生成する．
    pub fn from_range_bounds(range_bounds: impl RangeBounds<T>) -> Self {
        Self {
            range: T::to_half_open_range(range_bounds),
        }
    }

    /// 区間が`x`を含むかどうかを返す．
    pub fn contains(&self, x: &T) -> bool {
        self.range.contains(x)
    }

    /// 区間の長さを返す．
    pub fn len(&self) -> T
    where
        T: Sub<Output = T>,
    {
        assert!(!self.is_empty());
        self.end() - self.start()
    }

    /// 長さが0の区間かどうかを返す．
    pub fn is_empty(&self) -> bool {
        self.range.is_empty()
    }

    /// 区間の左端を返す．
    pub fn start(&self) -> T {
        self.range.start
    }

    /// 区間の右端を返す．
    pub fn end(&self) -> T {
        self.range.end
    }

    /// 左端の可変参照を返す．
    pub fn start_mut(&mut self) -> &mut T {
        &mut self.range.start
    }

    /// 右端の可変参照を返す．
    pub fn end_mut(&mut self) -> &mut T {
        &mut self.range.end
    }

    /// 他の区間との共通部分を返す．
    pub fn intersect(&self, other: &Self) -> Self {
        let start = self.start().max(other.start());
        let end = self.end().min(other.end());
        Self { range: start..end }
    }

    /// 左端のみを指す長さ0の区間を返す．
    fn start_point_inteval(&self) -> Self {
        Self {
            range: self.start()..self.start(),
        }
    }

    /// 右端のみを指す長さ0の区間を返す．
    fn end_point_inteval(&self) -> Self {
        Self {
            range: self.end()..self.end(),
        }
    }
}

impl<T> RangeBounds<T> for Interval<T> {
    fn start_bound(&self) -> Bound<&T> {
        self.range.start_bound()
    }

    fn end_bound(&self) -> Bound<&T> {
        self.range.end_bound()
    }
}

impl<T: Ord> PartialOrd for Interval<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Ord for Interval<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.range
            .start
            .cmp(&other.range.start)
            .then_with(|| self.range.end.cmp(&other.range.end))
    }
}

impl<T: Debug> Debug for Interval<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.range.start.fmt(f)?;
        write!(f, "..")?;
        self.range.end.fmt(f)?;
        Ok(())
    }
}

/// 区間と値`V`を対応付けて保持するデータ構造．
#[repr(transparent)]
#[derive(Clone, Default)]
pub struct IntervalMap<K, V> {
    inner: BTreeMap<Interval<K>, V>,
}

#[allow(private_bounds)]
impl<K: IntervalMapElement, V: Clone + PartialEq> IntervalMap<K, V> {
    /// 空の`IntervalMap`を生成する．
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    /// 保持している互いに素な区間の個数を返す．
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// 構造が空かどうかを返す．
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// すべての要素を削除する．
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// 区間`range`に値`value`を割り当て，同じ値で隣接する区間を結合する．
    pub fn insert(&mut self, range: impl RangeBounds<K>, value: V) {
        let mut new_interval = Interval::from_range_bounds(range);

        if new_interval.is_empty() {
            return;
        }

        self.remove(new_interval.clone());

        if let Some((interval, v)) = self
            .inner
            .range(..new_interval.start_point_inteval())
            .next_back()
            && interval.end() == new_interval.start()
            && v == &value
        {
            let key = interval.clone();
            *new_interval.start_mut() = key.start();
            self.inner.remove(&key);
        }

        if let Some((interval, v)) = self.inner.range(new_interval.end_point_inteval()..).next()
            && interval.start() == new_interval.end()
            && v == &value
        {
            let key = interval.clone();
            *new_interval.end_mut() = key.end();
            self.inner.remove(&key);
        }

        self.inner.insert(new_interval, value);
    }

    /// 指定した区間と重なる部分を削除し，切り取られた区間と値を返す．
    pub fn remove(&mut self, range: impl RangeBounds<K>) -> Vec<(Interval<K>, V)> {
        let to_remove_interval = Interval::from_range_bounds(range);

        if to_remove_interval.is_empty() {
            return Vec::new();
        }

        let mut to_remove = Vec::new();
        let mut cut_subsets = Vec::new();
        let mut removed = Vec::new();

        if let Some((interval, v)) = self
            .inner
            .range(..to_remove_interval.start_point_inteval())
            .next_back()
            && interval.end() > to_remove_interval.start()
        {
            to_remove.push(interval.clone());

            if interval.start() < to_remove_interval.start() {
                cut_subsets.push((
                    Interval::new(interval.start()..to_remove_interval.start()),
                    v.clone(),
                ));
            }

            if interval.end() > to_remove_interval.end() {
                cut_subsets.push((
                    Interval::new(to_remove_interval.end()..interval.end()),
                    v.clone(),
                ));
            }

            let interval = interval.intersect(&to_remove_interval);
            if !interval.is_empty() {
                removed.push((interval, v.clone()));
            }
        }

        for (interval, v) in self
            .inner
            .range(to_remove_interval.start_point_inteval()..to_remove_interval.end_point_inteval())
        {
            to_remove.push(interval.clone());

            if interval.end() > to_remove_interval.end() {
                cut_subsets.push((
                    Interval::new(to_remove_interval.end()..interval.end()),
                    v.clone(),
                ));
            }

            let interval = interval.intersect(&to_remove_interval);
            if !interval.is_empty() {
                removed.push((interval, v.clone()));
            }
        }

        for key in &to_remove {
            self.inner.remove(key);
        }

        for (interval, v) in cut_subsets {
            self.inner.insert(interval, v);
        }

        removed
    }

    /// 区間全体を包含するエントリがあればその参照を返す．
    pub fn superset_of(&self, range: impl RangeBounds<K>) -> Option<(&Interval<K>, &V)> {
        let target = Interval::from_range_bounds(range);

        if target.is_empty() {
            return None;
        }

        if let Some((interval, v)) = self.inner.range(..target.start_point_inteval()).next_back()
            && interval.end() >= target.end()
        {
            return Some((interval, v));
        }

        if let Some((interval, v)) = self.inner.range(target.start_point_inteval()..).next()
            && interval.start() == target.start()
            && interval.end() >= target.end()
        {
            return Some((interval, v));
        }

        None
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (&Interval<K>, &V)> + '_ {
        self.inner.iter()
    }
}

impl<'a, K, V> IntoIterator for &'a IntervalMap<K, V> {
    type Item = (&'a Interval<K>, &'a V);
    type IntoIter = std::collections::btree_map::Iter<'a, Interval<K>, V>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<K, V> IntoIterator for IntervalMap<K, V> {
    type Item = (Interval<K>, V);
    type IntoIter = std::collections::btree_map::IntoIter<Interval<K>, V>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<K: Debug, V: Debug> Debug for IntervalMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

#[repr(transparent)]
#[derive(Clone, Default)]
pub struct IntervalSet<K>(IntervalMap<K, ()>);

#[allow(private_bounds)]
impl<K: IntervalMapElement> IntervalSet<K> {
    /// 空の集合を生成する．
    pub fn new() -> Self {
        Self(IntervalMap::new())
    }

    /// 区間数を返す．
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// 空かどうかを返す．
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// すべての区間を削除する．
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// 区間を挿入し，必要なら隣接区間を併合する．
    pub fn insert(&mut self, range: impl RangeBounds<K>) {
        self.0.insert(range, ());
    }

    /// 区間を削除し，削除した半開区間を列挙して返す．
    pub fn remove(&mut self, range: impl RangeBounds<K>) -> Vec<Interval<K>> {
        self.0
            .remove(range)
            .into_iter()
            .map(|(interval, _)| interval)
            .collect()
    }

    /// 指定区間を包含する区間への参照を返す．
    pub fn superset_of(&self, range: impl RangeBounds<K>) -> Option<&Interval<K>> {
        self.0.superset_of(range).map(|(interval, _)| interval)
    }
}

impl<K> IntervalSet<K> {
    pub fn iter(&self) -> IntervalSetIter<'_, K> {
        IntervalSetIter {
            iter: self.0.inner.iter(),
        }
    }
}

pub struct IntervalSetIter<'a, K> {
    iter: std::collections::btree_map::Iter<'a, Interval<K>, ()>,
}

impl<'a, K> Iterator for IntervalSetIter<'a, K> {
    type Item = &'a Interval<K>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, _)| k)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K> DoubleEndedIterator for IntervalSetIter<'a, K> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|(k, _)| k)
    }
}

impl<'a, K> ExactSizeIterator for IntervalSetIter<'a, K> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K> FusedIterator for IntervalSetIter<'a, K> {}

impl<'a, K> IntoIterator for &'a IntervalSet<K> {
    type Item = &'a Interval<K>;
    type IntoIter = IntervalSetIter<'a, K>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct IntervalSetIntoIter<K> {
    iter: std::collections::btree_map::IntoIter<Interval<K>, ()>,
}

impl<K> Iterator for IntervalSetIntoIter<K> {
    type Item = Interval<K>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, _)| k)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<K> DoubleEndedIterator for IntervalSetIntoIter<K> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|(k, _)| k)
    }
}

impl<K> ExactSizeIterator for IntervalSetIntoIter<K> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K> FusedIterator for IntervalSetIntoIter<K> {}

impl<K> IntoIterator for IntervalSet<K> {
    type Item = Interval<K>;
    type IntoIter = IntervalSetIntoIter<K>;

    fn into_iter(self) -> Self::IntoIter {
        IntervalSetIntoIter {
            iter: self.0.inner.into_iter(),
        }
    }
}

impl<K: Debug> Debug for IntervalSet<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

trait IntervalMapElement: Ord + Copy {
    /// 任意の`RangeBounds`を半開区間`[l, r)`に正規化する．
    fn to_half_open_range(range_bounds: impl RangeBounds<Self>) -> Range<Self>;
}

macro_rules! impl_interval_element {
    ($ty: ty) => {
        impl IntervalMapElement for $ty {
            fn to_half_open_range(range_bounds: impl RangeBounds<Self>) -> Range<Self> {
                const NEGATIVE_INFINITY: $ty = <$ty>::MIN;
                const POSITIVE_INFINITY: $ty = <$ty>::MAX;

                let l = match range_bounds.start_bound() {
                    Bound::Unbounded => NEGATIVE_INFINITY,
                    Bound::Included(&x) => x,
                    Bound::Excluded(&x) => x + 1,
                };

                let r = match range_bounds.end_bound() {
                    Bound::Excluded(&x) => x,
                    Bound::Included(&x) => x + 1,
                    Bound::Unbounded => POSITIVE_INFINITY,
                };

                l..r
            }
        }
    };
}

macro_rules! impl_interval_element_for {
    ($($ty: ty),* $(,)?) => {
        $( impl_interval_element!($ty); )*
    };
}

impl_interval_element_for! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

#[cfg(test)]
mod tests {
    use super::{Interval, IntervalMap, IntervalSet};

    mod map {
        use super::{Interval, IntervalMap};

        #[test]
        fn insert_merges_adjacent_with_same_value() {
            let mut map = IntervalMap::new();
            map.insert(0..3, 1);
            map.insert(3..5, 1);
            map.insert(5..7, 2);

            let entries: Vec<_> = map
                .inner
                .iter()
                .map(|(interval, value)| (interval.start(), interval.end(), *value))
                .collect();
            assert_eq!(entries, vec![(0, 5, 1), (5, 7, 2)]);
        }

        #[test]
        fn remove_splits_and_returns_removed_segments() {
            let mut map = IntervalMap::new();
            map.insert(0..10, 1);
            map.insert(10..15, 2);

            let removed = map.remove(3..12);
            assert_eq!(
                removed,
                vec![(Interval::new(3..10), 1), (Interval::new(10..12), 2)]
            );

            let entries: Vec<_> = map
                .inner
                .iter()
                .map(|(interval, value)| (interval.start(), interval.end(), *value))
                .collect();
            assert_eq!(entries, vec![(0, 3, 1), (12, 15, 2)]);
        }

        #[test]
        fn superset_of_finds_covering_interval() {
            let mut map = IntervalMap::new();
            map.insert(0..5, 10);
            map.insert(5..8, 20);

            let (interval, value) = map.superset_of(1..4).expect("found superset");
            assert_eq!(interval.start(), 0);
            assert_eq!(interval.end(), 5);
            assert_eq!(*value, 10);

            let (interval, value) = map.superset_of(5..8).expect("exact match");
            assert_eq!(interval.start(), 5);
            assert_eq!(interval.end(), 8);
            assert_eq!(*value, 20);

            assert!(map.superset_of(4..6).is_none());
        }
    }

    mod set {
        use super::{Interval, IntervalSet};

        #[test]
        fn insert_merges_adjacent_intervals() {
            let mut set = IntervalSet::new();
            set.insert(1..4);
            set.insert(4..6);
            set.insert(8..9);

            let intervals: Vec<_> = set
                .iter()
                .map(|interval| (interval.start(), interval.end()))
                .collect();
            assert_eq!(intervals, vec![(1, 6), (8, 9)]);
        }

        #[test]
        fn remove_returns_removed_segments() {
            let mut set = IntervalSet::new();
            set.insert(0..10);

            let removed = set.remove(2..5);
            assert_eq!(removed, vec![Interval::new(2..5)]);

            let intervals: Vec<_> = set.iter().collect();
            assert_eq!(intervals, vec![&Interval::new(0..2), &Interval::new(5..10)]);
        }

        #[test]
        fn superset_of_works_for_interval_set() {
            let mut set = IntervalSet::new();
            set.insert(0..3);
            set.insert(5..9);

            assert_eq!(set.superset_of(6..8), Some(&Interval::new(5..9)));
            assert_eq!(set.superset_of(3..5), None);
        }
    }
}
