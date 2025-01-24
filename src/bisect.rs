use std::ops::Range;

/// x \in [l, r)の範囲を探索
/// !f(x)となる最小のxを返す(f(x-1)==true,  f(x)==false)
pub trait Bisect {
    type Item;
    fn bisect(&self, f: impl FnMut(&Self::Item) -> bool) -> Self::Item;
}

macro_rules! impl_integer {
    ($($ty:ident),*) => {$(
        impl Bisect for Range<$ty> {
            type Item = $ty;
            fn bisect(&self, mut f: impl FnMut(&Self::Item) -> bool) -> Self::Item {
                assert!(self.start < self.end);

                let Range {
                    start: mut ok,
                    end: mut ng,
                } = *self;

                if !f(&ok) {
                    return ok;
                }

                while ng > ok + 1 {
                    // TODO: implement checked_mid
                    let mid = ok + (ng - ok) / 2;
                    *if f(&mid) { &mut ok } else { &mut ng } = mid;
                }

                ng
            }
        }
    )*};
}

impl_integer! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

pub trait LowerBound {
    type Item: Ord;
    fn lower_bound(&self, x: &Self::Item) -> usize;
}

impl<T: Ord> LowerBound for [T] {
    type Item = T;
    fn lower_bound(&self, x: &Self::Item) -> usize {
        if self.is_empty() {
            return 0;
        }
        (0..self.len()).bisect(|&i| &self[i] < x)
    }
}

pub trait UpperBound {
    type Item: Ord;
    fn upper_bound(&self, x: &Self::Item) -> usize;
}

impl<T: Ord> UpperBound for [T] {
    type Item = T;
    fn upper_bound(&self, x: &Self::Item) -> usize {
        if self.is_empty() {
            return 0;
        }
        (0..self.len()).bisect(|&i| &self[i] <= x)
    }
}

#[cfg(test)]
mod tests {
    use super::{LowerBound, UpperBound};

    #[test]
    fn test_lower_bound_basic() {
        let v = vec![1, 3, 3, 5, 7, 9, 9, 9, 11, 13];
        assert_eq!(v.lower_bound(&0), 0);
        assert_eq!(v.lower_bound(&3), 1);
        assert_eq!(v.lower_bound(&9), 5);
        assert_eq!(v.lower_bound(&10), 8);
        assert_eq!(v.lower_bound(&13), 9);
        assert_eq!(v.lower_bound(&14), 10);

        let v: Vec<i32> = vec![];
        assert_eq!(v.lower_bound(&5), 0);

        let v = vec![10];
        assert_eq!(v.lower_bound(&5), 0);
        assert_eq!(v.lower_bound(&10), 0);
        assert_eq!(v.lower_bound(&15), 1);

        let v = vec![4, 4, 4, 4, 4];
        assert_eq!(v.lower_bound(&4), 0);
        assert_eq!(v.lower_bound(&3), 0);
        assert_eq!(v.lower_bound(&5), 5);

        let v = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        assert_eq!(v.lower_bound(&5), 2);
        assert_eq!(v.lower_bound(&6), 3);
        assert_eq!(v.lower_bound(&1), 0);
        assert_eq!(v.lower_bound(&19), 9);
        assert_eq!(v.lower_bound(&0), 0);
        assert_eq!(v.lower_bound(&20), 10);

        let v = vec![
            2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40,
        ];
        assert_eq!(v.lower_bound(&10), 4);
        assert_eq!(v.lower_bound(&25), 12);
        assert_eq!(v.lower_bound(&0), 0);
        assert_eq!(v.lower_bound(&40), 19);
        assert_eq!(v.lower_bound(&41), 20);
        assert_eq!(v.lower_bound(&15), 7);
        assert_eq!(v.lower_bound(&5), 2);
    }

    #[test]
    fn test_upper_bound() {
        let v = vec![1, 3, 3, 5, 7, 9, 9, 9, 11, 13];
        assert_eq!(v.upper_bound(&0), 0);
        assert_eq!(v.upper_bound(&3), 3);
        assert_eq!(v.upper_bound(&9), 8);
        assert_eq!(v.upper_bound(&10), 8);
        assert_eq!(v.upper_bound(&13), 10);
        assert_eq!(v.upper_bound(&14), 10);

        let v: Vec<i32> = vec![];
        assert_eq!(v.upper_bound(&5), 0);

        let v = vec![10];
        assert_eq!(v.upper_bound(&5), 0);
        assert_eq!(v.upper_bound(&10), 1);
        assert_eq!(v.upper_bound(&15), 1);

        let v = vec![4, 4, 4, 4, 4];
        assert_eq!(v.upper_bound(&4), 5);
        assert_eq!(v.upper_bound(&3), 0);
        assert_eq!(v.upper_bound(&5), 5);
    }
}
