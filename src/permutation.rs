use std::cmp::Ordering;

pub trait Permutation {
    type Item: Ord;
    fn next_permutation(&mut self) -> bool;
    fn next_permutation_by(&mut self, f: impl FnMut(&Self::Item, &Self::Item) -> Ordering) -> bool;
    fn next_permutation_by_key<K: Ord>(&mut self, f: impl FnMut(&Self::Item) -> K) -> bool;
    fn prev_permutation(&mut self) -> bool;
    fn prev_permutation_by(&mut self, f: impl FnMut(&Self::Item, &Self::Item) -> Ordering) -> bool;
    fn prev_permutation_by_key<K: Ord>(&mut self, f: impl FnMut(&Self::Item) -> K) -> bool;
}

impl<T: Ord> Permutation for [T] {
    type Item = T;

    fn next_permutation(&mut self) -> bool {
        self.next_permutation_by(|x, y| x.cmp(y))
    }

    fn next_permutation_by(
        &mut self,
        mut f: impl FnMut(&Self::Item, &Self::Item) -> Ordering,
    ) -> bool {
        if self.len() < 2 {
            return false;
        }

        if let Some(i) = self
            .windows(2)
            .rposition(|w| f(&w[0], &w[1]) == Ordering::Less)
        {
            if let Some(j) = self
                .iter()
                .rposition(|x| f(&x, &self[i]) == Ordering::Greater)
            {
                self.swap(i, j);
                self[i + 1..].reverse();
                return true;
            }
        }

        false
    }

    fn next_permutation_by_key<K: Ord>(&mut self, mut f: impl FnMut(&Self::Item) -> K) -> bool {
        self.next_permutation_by(|x, y| f(x).cmp(&f(y)))
    }

    fn prev_permutation(&mut self) -> bool {
        self.prev_permutation_by(|x, y| x.cmp(y))
    }

    fn prev_permutation_by(
        &mut self,
        mut f: impl FnMut(&Self::Item, &Self::Item) -> Ordering,
    ) -> bool {
        self.next_permutation_by(|x, y| f(y, x))
    }

    fn prev_permutation_by_key<K: Ord>(&mut self, mut f: impl FnMut(&Self::Item) -> K) -> bool {
        self.prev_permutation_by(|x, y| f(x).cmp(&f(y)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_permutation() {
        let mut v = vec![0, 1, 2, 3];
        let expected = vec![
            vec![0, 1, 2, 3],
            vec![0, 1, 3, 2],
            vec![0, 2, 1, 3],
            vec![0, 2, 3, 1],
            vec![0, 3, 1, 2],
            vec![0, 3, 2, 1],
            vec![1, 0, 2, 3],
            vec![1, 0, 3, 2],
            vec![1, 2, 0, 3],
            vec![1, 2, 3, 0],
            vec![1, 3, 0, 2],
            vec![1, 3, 2, 0],
            vec![2, 0, 1, 3],
            vec![2, 0, 3, 1],
            vec![2, 1, 0, 3],
            vec![2, 1, 3, 0],
            vec![2, 3, 0, 1],
            vec![2, 3, 1, 0],
            vec![3, 0, 1, 2],
            vec![3, 0, 2, 1],
            vec![3, 1, 0, 2],
            vec![3, 1, 2, 0],
            vec![3, 2, 0, 1],
            vec![3, 2, 1, 0],
        ];

        let mut count = 0;
        loop {
            assert_eq!(v, expected[count]);
            count += 1;
            if !v.next_permutation() {
                break;
            }
        }
        assert_eq!(count, expected.len());
    }

    #[test]
    fn test_prev_permutation() {
        let mut v = vec![3, 2, 1, 0];
        let expected = vec![
            vec![3, 2, 1, 0],
            vec![3, 2, 0, 1],
            vec![3, 1, 2, 0],
            vec![3, 1, 0, 2],
            vec![3, 0, 2, 1],
            vec![3, 0, 1, 2],
            vec![2, 3, 1, 0],
            vec![2, 3, 0, 1],
            vec![2, 1, 3, 0],
            vec![2, 1, 0, 3],
            vec![2, 0, 3, 1],
            vec![2, 0, 1, 3],
            vec![1, 3, 2, 0],
            vec![1, 3, 0, 2],
            vec![1, 2, 3, 0],
            vec![1, 2, 0, 3],
            vec![1, 0, 3, 2],
            vec![1, 0, 2, 3],
            vec![0, 3, 2, 1],
            vec![0, 3, 1, 2],
            vec![0, 2, 3, 1],
            vec![0, 2, 1, 3],
            vec![0, 1, 3, 2],
            vec![0, 1, 2, 3],
        ];
        let mut count = 0;
        loop {
            assert_eq!(v, expected[count]);
            count += 1;
            if !v.prev_permutation() {
                break;
            }
        }
        assert_eq!(count, expected.len());
    }
}
