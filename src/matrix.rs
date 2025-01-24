pub trait Matrix {
    type Output;

    /// NxM行列を時計回りに90度回転させる
    /// 処理後の行列はMxN行列になる
    fn rotate_clockwise(&self) -> Self::Output;

    /// NxM行列を反時計回りに90度回転させる
    /// 処理後の行列はMxN行列になる
    fn rotate_anticlockwise(&self) -> Self::Output;

    /// 転置行列を計算する
    fn transpose(&self) -> Self::Output;
}

impl<T> Matrix for Vec<Vec<T>>
where
    T: Clone,
{
    type Output = Vec<Vec<T>>;
    fn rotate_clockwise(&self) -> Self::Output {
        rotate_clockwise(self)
    }
    fn rotate_anticlockwise(&self) -> Self::Output {
        rotate_anticlockwise(self)
    }
    fn transpose(&self) -> Self::Output {
        transpose(self)
    }
}

/// NxM行列を時計回りに90度回転させる
/// 処理後の行列はMxN行列になる
fn rotate_clockwise<T>(m: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Clone,
{
    let h = m[0].len();
    let w = m.len();

    let mut res = vec![vec![m[0][0].clone(); w]; h];

    for i in 0..w {
        for j in 0..h {
            res[j][w - 1 - i] = m[i][j].clone();
        }
    }

    res
}

/// NxM行列を反時計回りに90度回転させる
/// 処理後の行列はMxN行列になる
fn rotate_anticlockwise<T>(m: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Clone,
{
    let h = m[0].len();
    let w = m.len();

    let mut res = vec![vec![m[0][0].clone(); w]; h];

    for i in 0..w {
        for j in 0..h {
            res[h - 1 - j][i] = m[i][j].clone();
        }
    }

    res
}

/// 転置行列を計算する
fn transpose<T>(m: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Clone,
{
    if m.is_empty() {
        return vec![];
    }

    let nrows = m.len();
    let ncols = m[0].len();

    let mut res = vec![vec![m[0][0].clone(); nrows]; ncols];

    for i in 0..nrows {
        for j in 0..ncols {
            res[j][i] = m[i][j].clone();
        }
    }

    res
}

// TODO: 正方行列以外のテストを書く
#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn test_rotate_clockwise() {
        let m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let rotated = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
        assert_eq!(m.rotate_clockwise(), rotated);
    }

    #[test]
    fn test_rotate_anticlockwise() {
        let m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let rotated = vec![vec![3, 6, 9], vec![2, 5, 8], vec![1, 4, 7]];
        assert_eq!(m.rotate_anticlockwise(), rotated);
    }

    #[test]
    fn test_transpose() {
        let m = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let transposed = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        assert_eq!(m.transpose(), transposed);

        let m = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        let transposed = vec![vec![1, 3, 5], vec![2, 4, 6]];
        assert_eq!(m.transpose(), transposed);

        let m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let transposed = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
        assert_eq!(m.transpose(), transposed);

        let m = vec![vec![1, 2, 3]];
        let transposed = vec![vec![1], vec![2], vec![3]];
        assert_eq!(m.transpose(), transposed);

        let m = vec![vec![1], vec![2], vec![3]];
        let transposed = vec![vec![1, 2, 3]];
        assert_eq!(m.transpose(), transposed);

        let m = vec![vec![6]];
        assert_eq!(m.transpose(), m);

        let m: Vec<Vec<i32>> = vec![];
        assert_eq!(m.transpose(), m);
    }
}
