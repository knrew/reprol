//! ランレングス圧縮(run-length encoding)
//!
//! 配列をランレングス圧縮する．
//! 連続する同じ要素をまとめて，その値とその個数のペアで表現する．
//! たとえば，文字列`"aaabbbbcc"`に対してランレングス圧縮を行うと，`[('a', 3), ('b', 4), ('c', 2)]`となる．
//!
//! # 使用例
//! ```
//! use reprol::rle::Rle;
//!
//! let s = b"aaabbbbcc";
//! assert!(s.rle().eq([(&b'a', 3), (&b'b', 4), (&b'c', 2)]));
//!
//! let v = vec![1, 1, 2, 2, 2, 3, 3];
//! assert!(v.rle().eq([(&1, 2), (&2, 3), (&3, 2)]));
//! ```

pub struct RleIter<'a, T> {
    slice: &'a [T],
    l: usize,
    r: usize,
}

impl<'a, T: PartialEq> RleIter<'a, T> {
    fn new(s: &'a [T]) -> Self {
        Self {
            slice: s,
            l: 0,
            r: s.len(),
        }
    }
}

impl<'a, T: PartialEq> Iterator for RleIter<'a, T> {
    type Item = (&'a T, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.l >= self.r {
            return None;
        }

        let start = self.l;
        self.l += 1;
        while self.l < self.r && self.slice[self.l] == self.slice[start] {
            self.l += 1;
        }

        Some((&self.slice[start], self.l - start))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.r - self.l))
    }
}

impl<'a, T: PartialEq> DoubleEndedIterator for RleIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.l >= self.r {
            return None;
        }

        let mut j = self.r - 1;
        let value = &self.slice[j];

        while j > self.l && self.slice[j - 1] == *value {
            j -= 1;
        }

        let count = self.r - j;
        self.r = j;

        Some((value, count))
    }
}

pub trait Rle {
    type Item;
    type Iter<'a>: Iterator<Item = (&'a Self::Item, usize)>
    where
        Self: 'a;

    fn rle(&self) -> Self::Iter<'_>;
}

impl<T: PartialEq> Rle for [T] {
    type Item = T;
    type Iter<'a> = RleIter<'a, T> where T:'a;

    fn rle(&self) -> Self::Iter<'_> {
        RleIter::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rle() {
        let s = "aaabbbcccddeee".chars().collect::<Vec<char>>();
        let expected = vec![(&'a', 3), (&'b', 3), (&'c', 3), (&'d', 2), (&'e', 3)];
        assert!(s.rle().eq(expected));

        let s = "abcde".chars().collect::<Vec<char>>();
        let expected = vec![(&'a', 1), (&'b', 1), (&'c', 1), (&'d', 1), (&'e', 1)];
        assert!(s.rle().eq(expected));

        assert!(Vec::<char>::new().rle().eq(vec![]));
    }
}
