//! 2次元グリッド(配列)に対する操作群
//!
//! - [`rotate_clockwise`] : グリッドを時計回り90度回転．
//! - [`rotate_anticlockwise`] : グリッドを反時計回り90度回転．
//! - [`transpose`] : グリッドを転置する．

/// グリッドを時計回り(右回り)に90度回転させる．
pub fn rotate_clockwise<T>(m: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Clone,
{
    if m.is_empty() || m[0].is_empty() {
        return vec![];
    }

    debug_assert!(m.iter().all(|mi| mi.len() == m[0].len()));

    let h = m[0].len();
    let w = m.len();

    let mut res = Vec::with_capacity(h);
    for j in 0..h {
        let mut row = Vec::with_capacity(w);
        for i in (0..w).rev() {
            row.push(m[i][j].clone());
        }
        res.push(row);
    }

    res
}

/// グリッドを反時計回りに90度回転させる．
pub fn rotate_anticlockwise<T>(m: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Clone,
{
    if m.is_empty() || m[0].is_empty() {
        return vec![];
    }

    debug_assert!(m.iter().all(|mi| mi.len() == m[0].len()));

    let h = m[0].len();
    let w = m.len();

    let mut res = Vec::with_capacity(h);
    for j in (0..h).rev() {
        let mut row = Vec::with_capacity(w);
        for i in 0..w {
            row.push(m[i][j].clone());
        }
        res.push(row);
    }

    res
}

/// グリッドを転置させる．
pub fn transpose<T>(m: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Clone,
{
    if m.is_empty() || m[0].is_empty() {
        return vec![];
    }

    debug_assert!(m.iter().all(|mi| mi.len() == m[0].len()));

    let h = m[0].len();
    let w = m.len();

    let mut res = Vec::with_capacity(h);
    for j in 0..h {
        let mut col = Vec::with_capacity(w);
        for i in 0..w {
            col.push(m[i][j].clone());
        }
        res.push(col);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_clockwise() {
        {
            let m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
            let expected = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
            assert_eq!(rotate_clockwise(&m), expected);
        }

        {
            let m = vec![vec![1, 2, 3], vec![4, 5, 6]];
            let expected = vec![vec![4, 1], vec![5, 2], vec![6, 3]];
            assert_eq!(rotate_clockwise(&m), expected);
        }

        {
            let m: Vec<Vec<i32>> = vec![];
            let expected: Vec<Vec<i32>> = vec![];
            assert_eq!(rotate_clockwise(&m), expected);
        }

        assert_eq!(rotate_clockwise(&vec![vec![30]]), vec![vec![30]]);
    }

    #[test]
    fn test_rotate_anticlockwise() {
        {
            let m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
            let rotated = vec![vec![3, 6, 9], vec![2, 5, 8], vec![1, 4, 7]];
            assert_eq!(rotate_anticlockwise(&m), rotated);
        }

        {
            let m = vec![vec![1, 2, 3], vec![4, 5, 6]];
            let expected = vec![vec![3, 6], vec![2, 5], vec![1, 4]];
            assert_eq!(rotate_anticlockwise(&m), expected);
        }

        {
            let m: Vec<Vec<i32>> = vec![];
            let expected: Vec<Vec<i32>> = vec![];
            assert_eq!(rotate_anticlockwise(&m), expected);
        }

        assert_eq!(rotate_anticlockwise(&vec![vec![30]]), vec![vec![30]]);
    }

    #[test]
    fn test_transpose() {
        {
            let m = vec![vec![1, 2, 3], vec![4, 5, 6]];
            let expected = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
            assert_eq!(transpose(&m), expected);
        }

        {
            let m = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
            let expected = vec![vec![1, 3, 5], vec![2, 4, 6]];
            assert_eq!(transpose(&m), expected);
        }

        {
            let m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
            let expected = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
            assert_eq!(transpose(&m), expected);
        }

        {
            let m = vec![vec![1, 2, 3]];
            let expected = vec![vec![1], vec![2], vec![3]];
            assert_eq!(transpose(&m), expected);
        }

        {
            let m = vec![vec![1], vec![2], vec![3]];
            let expected = vec![vec![1, 2, 3]];
            assert_eq!(transpose(&m), expected);
        }

        {
            let m: Vec<Vec<i32>> = vec![];
            let expected: Vec<Vec<i32>> = vec![];
            assert_eq!(transpose(&m), expected);
        }

        assert_eq!(transpose(&vec![vec![30]]), vec![vec![30]]);
    }
}
