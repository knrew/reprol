/// NxM行列を時計回りに90度回転させる
/// 処理後の行列はMxN行列になる
pub fn rotate_clockwise<T>(m: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Clone,
{
    debug_assert!(!m.is_empty());
    let mut rotated = vec![vec![m[0][0].clone(); m.len()]; m[0].len()];
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            rotated[j][m.len() - 1 - i] = m[i][j].clone();
        }
    }
    rotated
}

/// NxM行列を反時計回りに90度回転させる
/// 処理後の行列はMxN行列になる
pub fn rotate_anticlockwise<T>(m: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Clone,
{
    debug_assert!(!m.is_empty());
    let mut rotated = vec![vec![m[0][0].clone(); m.len()]; m[0].len()];
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            rotated[m[i].len() - 1 - j][i] = m[i][j].clone();
        }
    }
    rotated
}

/// 転置行列を計算する
pub fn transpose<T>(m: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Clone,
{
    if m.is_empty() {
        return vec![];
    }
    debug_assert!(!m[0].is_empty());
    let nrows = m.len();
    let ncols = m[0].len();
    let mut transposed = vec![vec![m[0][0].clone(); nrows]; ncols];
    for i in 0..nrows {
        for j in 0..ncols {
            transposed[j][i] = m[i][j].clone();
        }
    }
    transposed
}

// TODO: 正方行列以外のテストを書く
#[cfg(test)]
mod tests {
    use super::{rotate_anticlockwise, rotate_clockwise, transpose};

    #[test]
    fn test_rotate() {
        let m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let clockwise_rotated = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
        assert_eq!(rotate_clockwise(&m), clockwise_rotated);

        let anticlockwise_rotated = vec![vec![3, 6, 9], vec![2, 5, 8], vec![1, 4, 7]];
        assert_eq!(rotate_anticlockwise(&m), anticlockwise_rotated);
    }

    #[test]
    fn test_transpose() {
        let m = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let transposed = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        assert_eq!(transpose(&m), transposed);

        let m = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        let transposed = vec![vec![1, 3, 5], vec![2, 4, 6]];
        assert_eq!(transpose(&m), transposed);

        let m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let transposed = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
        assert_eq!(transpose(&m), transposed);

        let m = vec![vec![1, 2, 3]];
        let transposed = vec![vec![1], vec![2], vec![3]];
        assert_eq!(transpose(&m), transposed);

        let m = vec![vec![1], vec![2], vec![3]];
        let transposed = vec![vec![1, 2, 3]];
        assert_eq!(transpose(&m), transposed);

        let m = vec![vec![6]];
        assert_eq!(transpose(&m), m);

        let m: Vec<Vec<i32>> = vec![];
        assert_eq!(transpose(&m), m);
    }
}
