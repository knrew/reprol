//! 2次元いもす法(Imos2d)
//!
//! 2次元の矩形グリッドに対する範囲加算と最終値の取得を O(1) で行うデータ構造．
//! `add`で矩形領域を加算し，最後に`build`を呼び出すことで各セルの値を取得できる．
//!
//! # 使用例
//! ```
//! use reprol::ds::imos2d::Imos2d;
//!
//! let mut imos = Imos2d::new(3, 4);
//! imos.add(0..2, 1..3, 2);
//! imos.add(1..=2, 2..4, -1);
//! imos.build();
//! assert_eq!(imos.get(0, 1), 2);
//! assert_eq!(imos.get(1, 2), 1);
//! assert_eq!(imos.get(2, 3), -1);
//! ```

use std::ops::{Index, Range, RangeBounds};

use crate::utils::range_utils::to_half_open_index_range;

/// 2次元グリッド上の矩形範囲加算を管理するいもす法
pub struct Imos2d {
    h: usize,
    w: usize,
    imos: Vec<Vec<i64>>,
    has_built: bool,
}

impl Imos2d {
    /// `h x w` のゼロ初期化されたいもす法テーブルを構築する．
    pub fn new(h: usize, w: usize) -> Self {
        Self {
            h,
            w,
            imos: vec![vec![0; w + 1]; h + 1],
            has_built: false,
        }
    }

    /// 行`row_range`，列`col_range`で指定される矩形領域に`value`を加算する．
    ///
    /// `build`前にのみ呼び出せる．
    pub fn add(
        &mut self,
        row_range: impl RangeBounds<usize>,
        col_range: impl RangeBounds<usize>,
        value: i64,
    ) {
        assert!(!self.has_built);

        let Range { start: il, end: ir } = to_half_open_index_range(row_range, self.h + 1);
        let Range { start: jl, end: jr } = to_half_open_index_range(col_range, self.w + 1);

        assert!(il <= ir && ir <= self.h);
        assert!(jl <= jr && jr <= self.w);

        self.imos[il][jl] += value;
        self.imos[il][jr] -= value;
        self.imos[ir][jl] -= value;
        self.imos[ir][jr] += value;
    }

    /// いもす法の前計算を行い，各セルの値を確定させる．
    pub fn build(&mut self) {
        assert!(!self.has_built);

        for i in 0..=self.h {
            for j in 0..self.w {
                self.imos[i][j + 1] += self.imos[i][j];
            }
        }

        for j in 0..=self.w {
            for i in 0..self.h {
                self.imos[i + 1][j] += self.imos[i][j];
            }
        }

        self.has_built = true;
    }

    /// `build`後に`(i, j)`の値を返す．
    pub fn get(&self, i: usize, j: usize) -> i64 {
        assert!(self.has_built);
        self.imos[i][j]
    }
}

impl Index<(usize, usize)> for Imos2d {
    type Output = i64;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(self.has_built);
        &self.imos[index.0][index.1]
    }
}

impl Index<[usize; 2]> for Imos2d {
    type Output = i64;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        assert!(self.has_built);
        &self.imos[index[0]][index[1]]
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    use crate::utils::test_utils::random::get_test_rng;

    #[test]
    fn test_basic() {
        let mut imos = Imos2d::new(3, 4);
        imos.add(0..2, 1..3, 2);
        imos.add(1..=2, 2..4, -1);
        imos.build();
        let expected = vec![vec![0, 2, 2, 0], vec![0, 2, 1, -1], vec![0, 0, -1, -1]];
        for i in 0..3 {
            for j in 0..4 {
                assert_eq!(imos.get(i, j), expected[i][j]);
                assert_eq!(imos[(i, j)], expected[i][j]);
                assert_eq!(imos[[i, j]], expected[i][j]);
            }
        }
    }

    #[test]
    fn test_random() {
        let mut rng = get_test_rng();

        const T: usize = 100;
        const H_MAX: usize = 100;
        const W_MAX: usize = 100;
        const Q_MAX: usize = 100;

        for _ in 0..T {
            let h = rng.random_range(1..=H_MAX);
            let w = rng.random_range(1..=W_MAX);
            let mut imos = Imos2d::new(h, w);
            let mut grid = vec![vec![0i64; w]; h];
            let q = rng.random_range(1..=Q_MAX);

            for _ in 0..q {
                let il = rng.random_range(0..h);
                let ir = rng.random_range(il + 1..=h);
                let jl = rng.random_range(0..w);
                let jr = rng.random_range(jl + 1..=w);
                let value = rng.random_range(-100000..=100000);

                imos.add(il..ir, jl..jr, value);
                for i in il..ir {
                    for j in jl..jr {
                        grid[i][j] += value;
                    }
                }
            }

            imos.build();

            for i in 0..h {
                for j in 0..w {
                    assert_eq!(imos.get(i, j), grid[i][j]);
                }
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_add_after_build() {
        let mut imos = Imos2d::new(3, 4);
        imos.add(0..2, 1..3, 2);
        imos.add(1..=2, 2..4, -1);
        imos.build();
        imos.add(1..2, 0..1, 1);
    }

    #[test]
    #[should_panic]
    fn test_get_before_build() {
        let mut imos = Imos2d::new(3, 4);
        imos.add(0..2, 1..3, 2);
        imos.add(1..=2, 2..4, -1);
        let _ = imos.get(0, 1);
    }
}
