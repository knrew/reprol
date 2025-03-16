//! AvlTreeによるordered setの実装
//! k番目に小さい(大きい)要素の取得のためと実装をサボるため
//! 多少効率は悪いがAvlTreeVecのwrapperとして実装している
//! 基本的にmulti setとして機能する
//! 重複を許さない場合は`insert_unique`で挿入する

use std::{
    cmp::Ordering,
    fmt::Debug,
    hash::Hash,
    mem::{swap, take},
    ops::Index,
};

use crate::ds::avl_tree_vec::{AvlTreeVec, IntoIter, Iter};

pub struct AvlTreeSet<T> {
    vec: AvlTreeVec<T>,
}

impl<T> AvlTreeSet<T> {
    pub fn new() -> Self {
        Self {
            vec: AvlTreeVec::new(),
        }
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn insert(&mut self, value: T) -> bool
    where
        T: Ord,
    {
        let i = self.vec.lower_bound(&value);
        self.vec.insert(i, value);
        true
    }

    /// valueが存在しない場合のみinsertする
    pub fn insert_unique(&mut self, value: T) -> bool
    where
        T: Ord,
    {
        let i = self.vec.lower_bound(&value);
        if !self.vec.get(i).is_some_and(|e| e == &value) {
            self.vec.insert(i, value);
            true
        } else {
            false
        }
    }

    pub fn contains(&self, value: &T) -> bool
    where
        T: Ord,
    {
        let i = self.vec.lower_bound(value);
        self.vec.get(i).is_some_and(|e| e == value)
    }

    /// valueの個数
    pub fn count(&self, value: &T) -> usize
    where
        T: Ord,
    {
        self.vec.upper_bound(value) - self.vec.lower_bound(value)
    }

    pub fn remove(&mut self, value: &T) -> bool
    where
        T: Ord,
    {
        let i = self.vec.lower_bound(value);
        if self.vec.get(i).is_some_and(|e| e == value) {
            self.vec.remove(i);
            true
        } else {
            false
        }
    }

    pub fn remove_by_index(&mut self, index: usize) -> Option<T> {
        self.vec.remove(index)
    }

    pub fn append(&mut self, other: &mut Self)
    where
        T: Ord,
    {
        if self.len() < other.len() {
            swap(self, other);
        }
        take(&mut other.vec).into_iter().for_each(|item| {
            self.insert(item);
        });
    }

    pub fn split_off(&mut self, value: &T) -> Self
    where
        T: Ord,
    {
        let i = self.vec.lower_bound(value);
        let sub = self.vec.split_off(i);
        Self { vec: sub }
    }

    pub fn nth(&self, index: usize) -> Option<&T> {
        self.vec.get(index)
    }

    pub fn nth_back(&self, index: usize) -> Option<&T> {
        self.vec.get(self.vec.len().checked_sub(index + 1)?)
    }

    pub fn bisect(&self, f: impl FnMut(&T) -> bool) -> usize {
        self.vec.bisect(f)
    }

    pub fn lower_bound(&self, value: &T) -> usize
    where
        T: Ord,
    {
        self.vec.lower_bound_by(|e| e.cmp(value))
    }

    pub fn lower_bound_by(&self, mut f: impl FnMut(&T) -> Ordering) -> usize {
        self.vec.bisect(|e| f(e) == Ordering::Less)
    }

    pub fn lower_bound_by_key<K: Ord>(&self, k: &K, mut f: impl FnMut(&T) -> K) -> usize {
        self.vec.lower_bound_by(|e| f(e).cmp(k))
    }

    pub fn upper_bound(&self, value: &T) -> usize
    where
        T: Ord,
    {
        self.vec.upper_bound_by(|e| e.cmp(value))
    }

    pub fn upper_bound_by(&self, mut f: impl FnMut(&T) -> Ordering) -> usize {
        self.vec.bisect(|e| f(e) != Ordering::Greater)
    }

    pub fn upper_bound_by_key<K: Ord>(&self, k: &K, mut f: impl FnMut(&T) -> K) -> usize {
        self.vec.upper_bound_by(|x| f(x).cmp(k))
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.vec.iter()
    }
}

impl<T> Default for AvlTreeSet<T> {
    fn default() -> Self {
        Self {
            vec: AvlTreeVec::default(),
        }
    }
}

impl<T> Index<usize> for AvlTreeSet<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.nth(index).unwrap()
    }
}

impl<'a, T> IntoIterator for &'a AvlTreeSet<T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> IntoIterator for AvlTreeSet<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

impl<T: PartialEq> PartialEq for AvlTreeSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<T: Eq> Eq for AvlTreeSet<T> {}

impl<T: PartialOrd> PartialOrd for AvlTreeSet<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other.iter())
    }
}

impl<T: Ord> Ord for AvlTreeSet<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other.iter())
    }
}

impl<T: Hash> Hash for AvlTreeSet<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.iter().for_each(|item| item.hash(state));
    }
}

impl<T: Ord> Extend<T> for AvlTreeSet<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        iter.into_iter().for_each(|x| {
            self.insert(x);
        });
    }
}

impl<'a, T: 'a + Ord + Copy> Extend<&'a T> for AvlTreeSet<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().cloned());
    }
}

impl<T: Ord> FromIterator<T> for AvlTreeSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut res = Self::new();
        res.extend(iter);
        res
    }
}

impl<T: Ord> From<Vec<T>> for AvlTreeSet<T> {
    fn from(v: Vec<T>) -> Self {
        Self::from_iter(v)
    }
}

impl<T: Ord, const N: usize> From<[T; N]> for AvlTreeSet<T> {
    fn from(v: [T; N]) -> Self {
        Self::from_iter(v)
    }
}

impl<T: Clone> Clone for AvlTreeSet<T> {
    fn clone(&self) -> Self {
        Self {
            vec: self.vec.clone(),
        }
    }
}

impl<T: Debug> Debug for AvlTreeSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    mod multi {
        use super::super::AvlTreeSet;

        #[test]
        fn test_insert_and_contains() {
            let mut ms = AvlTreeSet::new();
            assert!(!ms.contains(&3));
            assert!(!ms.contains(&1));
            assert!(!ms.contains(&4));
            assert!(!ms.contains(&5));
            assert!(!ms.contains(&100));
            assert!(ms.insert(3));
            assert!(ms.insert(1));
            assert!(ms.insert(4));
            assert!(ms.insert(1));
            assert!(ms.insert(5));
            assert!(ms.contains(&3));
            assert!(ms.contains(&1));
            assert!(ms.contains(&4));
            assert!(ms.contains(&5));
            assert!(!ms.contains(&100));
            assert_eq!(ms.len(), 5);
            assert_eq!(ms.count(&3), 1);
            assert_eq!(ms.count(&1), 2);
            assert_eq!(ms.count(&4), 1);
            assert_eq!(ms.count(&5), 1);
            assert_eq!(ms.count(&100), 0);
        }

        #[test]
        fn test_count() {
            let ms = AvlTreeSet::from([1, 2, 2, 3, 7, 7, 7, 9, 100, 100, 201, 201, 201]);
            assert_eq!(ms.count(&0), 0);
            assert_eq!(ms.count(&1), 1);
            assert_eq!(ms.count(&2), 2);
            assert_eq!(ms.count(&3), 1);
            assert_eq!(ms.count(&7), 3);
            assert_eq!(ms.count(&8), 0);
            assert_eq!(ms.count(&9), 1);
            assert_eq!(ms.count(&100), 2);
            assert_eq!(ms.count(&101), 0);
            assert_eq!(ms.count(&201), 3);
        }

        #[test]
        fn test_avl_tree_set_remove() {
            let mut ms = AvlTreeSet::from([52, 73, 63, 27, 44, 94, 31, 82, 70, 37, 82, 37]);
            assert!(ms
                .iter()
                .copied()
                .eq([27, 31, 37, 37, 44, 52, 63, 70, 73, 82, 82, 94]));
            assert!(ms.remove(&44));
            assert!(ms.remove(&52));
            assert!(ms.remove(&63));
            assert!(!ms.remove(&100));
            assert!(ms.remove(&82));
            assert!(!ms.remove(&44));
            assert!(ms.iter().copied().eq([27, 31, 37, 37, 70, 73, 82, 94]));
            assert!(ms.remove(&82));
            assert!(!ms.remove(&82));
            assert!(ms.iter().copied().eq([27, 31, 37, 37, 70, 73, 94]));
        }

        #[test]
        fn test_nth() {
            let ms = AvlTreeSet::from([1, 3, 5, 7, 7, 9]);
            assert_eq!(ms.nth(0), Some(&1));
            assert_eq!(ms.nth(1), Some(&3));
            assert_eq!(ms.nth(2), Some(&5));
            assert_eq!(ms.nth(3), Some(&7));
            assert_eq!(ms.nth(4), Some(&7));
            assert_eq!(ms.nth(5), Some(&9));
            assert_eq!(ms.nth(6), None);
        }

        #[test]
        fn test_nth_back() {
            let ms = AvlTreeSet::from([2, 4, 6, 8, 10]);
            assert_eq!(ms.nth_back(0), Some(&10));
            assert_eq!(ms.nth_back(1), Some(&8));
            assert_eq!(ms.nth_back(2), Some(&6));
            assert_eq!(ms.nth_back(3), Some(&4));
            assert_eq!(ms.nth_back(4), Some(&2));
            assert_eq!(ms.nth_back(5), None);
        }

        #[test]
        fn test_append() {
            let mut ms1 = AvlTreeSet::from([1, 3, 5]);
            let mut ms2 = AvlTreeSet::from([2, 4, 6]);
            ms1.append(&mut ms2);
            assert!(ms1.iter().copied().eq([1, 2, 3, 4, 5, 6]));
            assert!(ms2.is_empty());

            let mut ms1 = AvlTreeSet::new();
            let mut ms2 = AvlTreeSet::new();
            ms1.append(&mut ms2);
            assert!(ms1.is_empty());
            assert!(ms2.is_empty());
            ms1.insert(10);
            ms1.append(&mut AvlTreeSet::new());
            assert!(ms1.iter().copied().eq([10]));
            assert!(ms2.is_empty());

            let mut ms1 = AvlTreeSet::new();
            let mut ms2 = AvlTreeSet::from([7, 8]);
            ms1.append(&mut ms2);
            assert!(ms1.iter().copied().eq([7, 8]));
            assert!(ms2.is_empty());

            let mut ms1 = AvlTreeSet::from([2, 4, 6]);
            let mut ms2 = AvlTreeSet::from([3, 4, 5]);
            ms1.append(&mut ms2);
            assert!(ms1.iter().copied().eq([2, 3, 4, 4, 5, 6]));
            assert!(ms2.is_empty());
        }

        #[test]
        fn test_avl_tree_set_split_off() {
            let mut ms1 = AvlTreeSet::from([1, 2, 3, 4, 5, 6]);
            let ms2 = ms1.split_off(&4);
            assert!(ms1.iter().copied().eq([1, 2, 3]));
            assert!(ms2.iter().copied().eq([4, 5, 6]));

            let mut ms1 = AvlTreeSet::from([2, 4, 6, 8, 10]);
            let ms2 = ms1.split_off(&5);
            assert!(ms1.iter().copied().eq([2, 4]));
            assert!(ms2.iter().copied().eq([6, 8, 10]));

            let mut ms1 = AvlTreeSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
            let ms2 = ms1.split_off(&5);
            assert!(ms1.iter().copied().eq([1, 2, 3, 4]));
            assert!(ms2.iter().copied().eq([5, 6, 7, 8, 9]));

            let mut ms1 = AvlTreeSet::from([10, 20, 30, 40, 50]);
            let ms2 = ms1.split_off(&25);
            assert!(ms1.iter().copied().eq([10, 20]));
            assert!(ms2.iter().copied().eq([30, 40, 50]));

            let mut ms1 = AvlTreeSet::new();
            let ms2 = ms1.split_off(&10);
            assert!(ms1.is_empty());
            assert!(ms2.is_empty());

            let mut ms1 = AvlTreeSet::from([1, 2, 3, 3, 4, 4, 4, 5, 5, 6]);
            let ms2 = ms1.split_off(&4);
            assert!(ms1.iter().copied().eq([1, 2, 3, 3]));
            assert!(ms2.iter().copied().eq([4, 4, 4, 5, 5, 6]));
        }
    }
}
