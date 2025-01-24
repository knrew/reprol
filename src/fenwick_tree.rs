use std::ops::{Add, Range, RangeBounds, Sub};

use crate::utilities::to_open_range;

pub struct FenwickTree<T> {
    n: usize,
    nodes: Vec<T>,
    zero: T,
}

impl<T> FenwickTree<T>
where
    T: Clone + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(n: usize, zero: T) -> Self {
        Self {
            n,
            nodes: vec![zero.clone(); n],
            zero,
        }
    }

    pub fn add(&mut self, mut index: usize, value: T) {
        assert!(index < self.n);
        index += 1;
        while index <= self.n {
            self.nodes[index - 1] = self.nodes[index - 1].clone() + value.clone();
            index += index & index.wrapping_neg();
        }
    }

    pub fn sum(&self, range: impl RangeBounds<usize>) -> T {
        let Range { start: l, end: r } = to_open_range(range, self.n);
        assert!(l <= r);
        self.cum(r) - self.cum(l)
    }

    fn cum(&self, mut r: usize) -> T {
        let mut res = self.zero.clone();
        while r > 0 {
            res = res + self.nodes[r - 1].clone();
            r -= r & r.wrapping_neg();
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::FenwickTree;

    #[test]
    fn test_fenwick_tree() {
        let mut ft = FenwickTree::new(10, 0);
        ft.add(0, 5);
        ft.add(2, 10);
        ft.add(6, 20);
        assert_eq!(ft.sum(..1), 5);
        assert_eq!(ft.sum(..3), 15);
        assert_eq!(ft.sum(..7), 35);
        assert_eq!(ft.sum(..), 35);
        assert_eq!(ft.sum(0..3), 15);
        assert_eq!(ft.sum(3..=6), 20);
        ft.add(9, 10);
        assert_eq!(ft.sum(0..10), 45);
    }
}
