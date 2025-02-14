use std::cmp::Ordering;

pub fn next_permutation<T: Ord>(v: &mut [T]) -> bool {
    next_permutation_by(v, |x, y| x.cmp(y))
}

pub fn next_permutation_by<T, F>(v: &mut [T], mut f: F) -> bool
where
    F: FnMut(&T, &T) -> Ordering,
{
    if v.len() < 2 {
        return false;
    }

    if let Some(i) = v
        .windows(2)
        .rposition(|w| f(&w[0], &w[1]) == Ordering::Less)
    {
        if let Some(j) = v.iter().rposition(|x| f(&x, &v[i]) == Ordering::Greater) {
            v.swap(i, j);
            v[i + 1..].reverse();
            return true;
        }
    }

    false
}

pub fn next_permutation_by_key<T, F, K>(v: &mut [T], mut f: F) -> bool
where
    F: FnMut(&T) -> K,
    K: Ord,
{
    next_permutation_by(v, |x, y| f(x).cmp(&f(y)))
}

#[cfg(test)]
mod tests {
    use super::next_permutation;

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
            if !next_permutation(&mut v) {
                break;
            }
        }
        assert_eq!(count, expected.len());
    }
}
