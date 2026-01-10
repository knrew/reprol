//! Disjoint Set Union(Union-Find)
//!
//! # 使用例
//! ```
//! use reprol::ds::dsu::Dsu;
//!
//! let mut dsu = Dsu::new(5);
//!
//! dsu.merge(0, 1);
//! dsu.merge(3, 4);
//! assert!(dsu.connected(0, 1));
//! assert!(!dsu.connected(0, 3));
//! assert_eq!(dsu.size(0), 2);
//! assert_eq!(dsu.count_components(), 3);
//! ```
//!
//! # 問題例
//! - [Disjoint Set: Union Find Tree](https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/1/DSL_1_A)

use std::mem::swap;

pub struct Dsu {
    parents: Vec<usize>,
    sizes: Vec<usize>,
    count_components: usize,
}

impl Dsu {
    /// 要素数`n`で初期化する．
    pub fn new(n: usize) -> Self {
        assert!(n > 0);
        Self {
            parents: (0..n).collect(),
            sizes: vec![1; n],
            count_components: n,
        }
    }

    /// 要素`v`が属する集合の代表元を返す．
    pub fn find(&mut self, v: usize) -> usize {
        if self.parents[v] == v {
            return v;
        }
        let root = self.find(self.parents[v]);
        self.parents[v] = root;
        root
    }

    /// 要素`u`と`v`が属する集合を統合する．
    pub fn merge(&mut self, u: usize, v: usize) {
        let mut u = self.find(u);
        let mut v = self.find(v);

        if u == v {
            return;
        }

        if self.sizes[u] < self.sizes[v] {
            swap(&mut u, &mut v);
        }

        self.sizes[u] += self.sizes[v];
        self.parents[v] = u;
        self.count_components -= 1;
    }

    /// 要素`u`と`v`が同じ集合に属するかを判定する．
    pub fn connected(&mut self, u: usize, v: usize) -> bool {
        self.find(u) == self.find(v)
    }

    /// 要素`v`が属する集合の要素数を返す．
    pub fn size(&mut self, v: usize) -> usize {
        let v = self.find(v);
        self.sizes[v]
    }

    /// すべての連結成分を列挙する．
    pub fn components(&mut self) -> impl Iterator<Item = Vec<usize>> {
        let n = self.parents.len();
        let mut components = vec![vec![]; n];
        for v in 0..n {
            components[self.find(v)].push(v);
        }
        components.retain(|c| !c.is_empty());
        components.into_iter()
    }

    /// 連結成分の個数を返す．
    pub fn count_components(&self) -> usize {
        self.count_components
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    use crate::utils::test_utils::initialize_rng;

    #[test]
    fn test() {
        let mut dsu = Dsu::new(6);

        dsu.merge(0, 1);
        dsu.merge(2, 3);
        assert!(dsu.connected(0, 1));
        assert!(dsu.connected(2, 3));
        assert!(!dsu.connected(0, 2));
        assert_eq!(dsu.count_components(), 4);

        dsu.merge(1, 2);
        assert!(dsu.connected(0, 3));
        assert_eq!(dsu.size(1), 4);
        assert_eq!(dsu.count_components(), 3);

        dsu.merge(4, 5);
        assert!(dsu.connected(4, 5));
        assert_eq!(dsu.count_components(), 2);

        let mut dsu = Dsu::new(3);
        dsu.merge(0, 1);
        dsu.merge(1, 0);
        assert!(dsu.connected(0, 1));
        assert_eq!(dsu.count_components(), 2);
    }

    #[test]
    fn test_components() {
        let mut dsu = Dsu::new(6);
        dsu.merge(0, 1);
        dsu.merge(2, 3);
        dsu.merge(4, 5);

        let mut components = dsu.components().collect::<Vec<_>>();
        components.iter_mut().for_each(|v| v.sort_unstable());
        components.sort_unstable();

        assert_eq!(components, vec![vec![0, 1], vec![2, 3], vec![4, 5]]);
    }

    #[test]
    fn test_merge_and_connected_random() {
        let mut rng = initialize_rng();

        const T: usize = 10;
        const N: usize = 100;
        const Q: usize = 10000;

        for _ in 0..T {
            let mut dsu = Dsu::new(N);

            // 愚直実装
            // naive[v]: vが属する集合の代表元
            let mut naive_parents = (0..N).collect::<Vec<_>>();

            for _ in 0..Q {
                let u = rng.random_range(0..N);
                let v = rng.random_range(0..N);

                if rng.random_ratio(1, 2) {
                    // merge

                    dsu.merge(u, v);

                    // 愚直実装の更新
                    {
                        let u = naive_parents[u];
                        let v = naive_parents[v];
                        for p in &mut naive_parents {
                            if *p == u {
                                *p = v;
                            }
                        }
                    }
                } else {
                    // connected
                    assert_eq!(dsu.connected(u, v), naive_parents[u] == naive_parents[v]);
                }
            }
        }
    }
}
