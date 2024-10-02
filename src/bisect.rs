use crate::integer::Integer;

/// x \in [l, r)の範囲を探索
/// !f(x)となる最小のxを返す(f(x-1)==true and f(x)==false)
pub fn bisect<T: Integer>(l: T, r: T, mut f: impl FnMut(&T) -> bool) -> T {
    if !f(&l) {
        return l;
    }
    let (mut ok, mut ng) = (l, r);
    while ng > ok + T::ONE {
        // TODO: implement checked_mid
        let mid = ok + (ng - ok) / T::TWO;
        *if f(&mid) { &mut ok } else { &mut ng } = mid;
    }
    ng
}

pub trait LowerBound {
    type Item: Ord;
    fn lower_bound(&self, x: &Self::Item) -> usize;
}

impl<T: Ord> LowerBound for [T] {
    type Item = T;
    fn lower_bound(&self, x: &Self::Item) -> usize {
        bisect(0, self.len(), |&i| unsafe { self.get_unchecked(i) } < x)
    }
}

#[cfg(test)]
mod tests {
    use crate::bisect::LowerBound;

    #[test]
    fn test_lower_bound() {
        let v = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let test_cases = vec![(5, 2), (6, 3), (1, 0), (19, 9), (0, 0), (20, 10)];
        for (input, expected) in test_cases {
            assert_eq!(v.lower_bound(&input), expected);
        }

        let v = vec![
            2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40,
        ];
        let test_cases = vec![
            (10, 4),
            (25, 12),
            (0, 0),
            (40, 19),
            (41, 20),
            (15, 7),
            (5, 2),
        ];
        for (input, expected) in test_cases {
            assert_eq!(v.lower_bound(&input), expected);
        }
    }
}
