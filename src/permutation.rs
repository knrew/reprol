pub fn next_permutation<T: PartialOrd>(v: &mut [T]) -> bool {
    if v.len() < 2 {
        return false;
    }

    if let Some(i) = v.windows(2).rposition(|w| w[0] < w[1]) {
        if let Some(j) = v.iter().rposition(|x| x > &v[i]) {
            v.swap(i, j);
            v[i + 1..].reverse();
            return true;
        }
    }

    false
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
