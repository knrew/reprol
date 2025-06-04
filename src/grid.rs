//! 2次元グリッド(配列)に対する操作群
//!
//! - [`Grid::rotate_clockwise`] : グリッドを時計回り90度回転．
//! - [`Grid::rotate_anticlockwise`] : グリッドを反時計回り90度回転．
//! - [`Grid::transpose`] : グリッドを転置する．

use std::mem::take;

pub trait Grid {
    fn rotate_clockwise(&mut self);
    fn rotate_anticlockwise(&mut self);
    fn transpose(&mut self);
}

impl<T> Grid for Vec<Vec<T>>
where
    T: Clone,
{
    fn rotate_clockwise(&mut self) {
        if self.is_empty() || self[0].is_empty() {
            return;
        }

        debug_assert!(self.iter().all(|r| r.len() == self[0].len()));

        let h = self[0].len();
        let w = self.len();

        let orig = take(self);

        *self = Vec::with_capacity(h);
        for j in 0..h {
            let mut row = Vec::with_capacity(w);
            for i in (0..w).rev() {
                row.push(orig[i][j].clone());
            }
            self.push(row);
        }
    }

    fn rotate_anticlockwise(&mut self) {
        if self.is_empty() || self.is_empty() {
            return;
        }

        debug_assert!(self.iter().all(|mi| mi.len() == self[0].len()));

        let h = self[0].len();
        let w = self.len();

        let orig = take(self);

        *self = Vec::with_capacity(h);
        for j in (0..h).rev() {
            let mut row = Vec::with_capacity(w);
            for i in 0..w {
                row.push(orig[i][j].clone());
            }
            self.push(row);
        }
    }

    fn transpose(&mut self) {
        if self.is_empty() || self[0].is_empty() {
            return;
        }

        debug_assert!(self.iter().all(|r| r.len() == self[0].len()));

        let h = self[0].len();
        let w = self.len();

        let orig = take(self);

        *self = Vec::with_capacity(h);
        for j in 0..h {
            let mut col = Vec::with_capacity(w);
            for i in 0..w {
                col.push(orig[i][j].clone());
            }
            self.push(col);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_clockwise() {
        {
            let mut m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
            let expected = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
            m.rotate_clockwise();
            assert_eq!(m, expected);
        }

        {
            let mut m = vec![vec![1, 2, 3], vec![4, 5, 6]];
            let expected = vec![vec![4, 1], vec![5, 2], vec![6, 3]];
            m.rotate_clockwise();
            assert_eq!(m, expected);
        }

        {
            let mut m: Vec<Vec<i32>> = vec![];
            let expected: Vec<Vec<i32>> = vec![];
            m.rotate_clockwise();
            assert_eq!(m, expected);
        }

        {
            let mut m = vec![vec![30]];
            let expected = vec![vec![30]];
            m.rotate_clockwise();
            assert_eq!(m, expected);
        }
    }

    #[test]
    fn test_rotate_anticlockwise() {
        {
            let mut m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
            let expected = vec![vec![3, 6, 9], vec![2, 5, 8], vec![1, 4, 7]];
            m.rotate_anticlockwise();
            assert_eq!(m, expected);
        }

        {
            let mut m = vec![vec![1, 2, 3], vec![4, 5, 6]];
            let expected = vec![vec![3, 6], vec![2, 5], vec![1, 4]];
            m.rotate_anticlockwise();
            assert_eq!(m, expected);
        }

        {
            let mut m: Vec<Vec<i32>> = vec![];
            let expected: Vec<Vec<i32>> = vec![];
            m.rotate_anticlockwise();
            assert_eq!(m, expected);
        }

        {
            let mut m = vec![vec![30]];
            let expected = vec![vec![30]];
            m.rotate_anticlockwise();
            assert_eq!(m, expected);
        }
    }

    #[test]
    fn test_transpose() {
        {
            let mut m = vec![vec![1, 2, 3], vec![4, 5, 6]];
            let expected = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
            m.transpose();
            assert_eq!(m, expected);
        }

        {
            let mut m = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
            let expected = vec![vec![1, 3, 5], vec![2, 4, 6]];
            m.transpose();
            assert_eq!(m, expected);
        }

        {
            let mut m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
            let expected = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
            m.transpose();
            assert_eq!(m, expected);
        }

        {
            let mut m = vec![vec![1, 2, 3]];
            let expected = vec![vec![1], vec![2], vec![3]];
            m.transpose();
            assert_eq!(m, expected);
        }

        {
            let mut m = vec![vec![1], vec![2], vec![3]];
            let expected = vec![vec![1, 2, 3]];
            m.transpose();
            assert_eq!(m, expected);
        }

        {
            let mut m: Vec<Vec<i32>> = vec![];
            let expected: Vec<Vec<i32>> = vec![];
            m.transpose();
            assert_eq!(m, expected);
        }

        {
            let mut m = vec![vec![30]];
            let expected = vec![vec![30]];
            m.transpose();
            assert_eq!(m, expected);
        }
    }
}
