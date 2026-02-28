//! 1次元いもす法(Imos)
//!
//! 1次元配列に対する範囲加算と最終値取得を O(1) で行うデータ構造．
//! `add`で半開区間を加算し，最後に`build`を呼び出すことで各要素の値が得られる．
//!
//! # 使用例
//! ```
//! use reprol::ds::imos::Imos;
//!
//! let mut imos = Imos::new(5);
//! imos.add(1..4, 3);
//! imos.add(0..=2, -1);
//! imos.build();
//! assert_eq!(imos.get(0), -1);
//! assert_eq!(imos.get(3), 3);
//! ```

use std::ops::{Index, Range, RangeBounds};

use crate::utils::normalize_range::normalize_index;

/// 1次元配列上の区間加算を管理するいもす法
pub struct Imos {
    n: usize,
    imos: Vec<i64>,
    has_built: bool,
}

impl Imos {
    /// 長さ`n`のゼロ初期化されたテーブルを構築する．
    pub fn new(n: usize) -> Self {
        Self {
            n,
            imos: vec![0; n + 1],
            has_built: false,
        }
    }

    /// 区間`range`に`value`を加算する．`range`は閉区間・半開区間の混用を許す．
    ///
    /// `build`前にのみ呼び出せる．
    pub fn add(&mut self, range: impl RangeBounds<usize>, value: i64) {
        assert!(!self.has_built);

        let Range { start: l, end: r } = normalize_index(range, self.n + 1);

        assert!(l <= r && r <= self.n);

        self.imos[l] += value;
        self.imos[r] -= value;
    }

    /// いもす法の前計算を行い，各位置の値を確定させる．
    pub fn build(&mut self) {
        assert!(!self.has_built);

        for i in 0..self.n {
            self.imos[i + 1] += self.imos[i];
        }

        self.has_built = true;
    }

    /// `build`後に`i`番目の値を返す．
    pub fn get(&self, i: usize) -> i64 {
        assert!(self.has_built);
        self.imos[i]
    }
}

impl Index<usize> for Imos {
    type Output = i64;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(self.has_built);
        &self.imos[index]
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    use crate::utils::test_utils::random::get_test_rng;

    #[test]
    fn test_basic() {
        let mut imos = Imos::new(5);
        imos.add(0..3, 2);
        imos.add(2..=4, -1);
        imos.add(1..5, 1);
        imos.build();
        assert_eq!(imos.get(0), 2);
        assert_eq!(imos.get(1), 3);
        assert_eq!(imos.get(2), 2);
        assert_eq!(imos.get(3), 0);
        assert_eq!(imos.get(4), 0);
        for i in 0..imos.n {
            assert_eq!(imos[i], imos.get(i));
        }
    }

    #[test]
    fn test_random() {
        let mut rng = get_test_rng();

        const T: usize = 100;
        const N_MAX: usize = 100;
        const Q_MAX: usize = 100;

        for _ in 0..T {
            let n = rng.random_range(1..=N_MAX);
            let mut imos = Imos::new(n);
            let mut arr = vec![0i64; n];
            let q = rng.random_range(1..=Q_MAX);

            for _ in 0..q {
                let l = rng.random_range(0..n);
                let r = rng.random_range(l + 1..=n);
                let value = rng.random_range(-100000..=100000);
                imos.add(l..r, value);
                for i in l..r {
                    arr[i] += value;
                }
            }

            imos.build();

            for i in 0..n {
                assert_eq!(imos.get(i), arr[i]);
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_add_after_build() {
        let mut imos = Imos::new(3);
        imos.add(0..2, 1);
        imos.build();
        imos.add(1..3, 2);
    }

    #[test]
    #[should_panic]
    fn test_get_before_build() {
        let mut imos = Imos::new(3);
        imos.add(0..2, 1);
        let _ = imos.get(0);
    }
}
