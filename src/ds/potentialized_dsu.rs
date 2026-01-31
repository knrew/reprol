//! ポテンシャルつきDSU
//!
//! # 使用例
//! ```
//! use reprol::ds::potentialized_dsu::PotentializedDsu;
//! use reprol::ops::op_add::OpAdd;
//!
//! let mut dsu = PotentializedDsu::<OpAdd<i64>>::new(5);
//! assert!(dsu.merge(0, 1, 4));
//! assert!(dsu.connected(0, 1));
//! assert_eq!(dsu.diff_potential(0, 1), 4);
//! assert_eq!(dsu.diff_potential(1, 0), -4);
//!
//! assert!(!dsu.merge(0, 1, 6));
//! assert_eq!(dsu.diff_potential(0, 1), 4);
//!
//! assert!(dsu.merge(0, 3, 5));
//! assert_eq!(dsu.diff_potential(0, 3), 5);
//! assert_eq!(dsu.diff_potential(1, 3), 1);
//! ```
//!
//! # 問題例
//! - [Weighted Union Find Trees](https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/1/DSL_1_B)
//! - [ABC373 D](https://atcoder.jp/contests/abc373/tasks/abc373_d)

use std::mem::swap;

use crate::ops::group::Group;

pub struct PotentializedDsu<O: Group> {
    parents: Vec<usize>,
    sizes: Vec<usize>,
    potentials: Vec<O::Element>,
    count_components: usize,
    op: O,
}

impl<O: Group> PotentializedDsu<O> {
    /// 要素数`n`で初期化する．
    pub fn new(n: usize) -> Self
    where
        O: Default,
    {
        Self::with_op(n, O::default())
    }

    /// 演算(群)`op`を明示的に渡して要素数`n`で初期化する．
    pub fn with_op(n: usize, op: O) -> Self {
        Self {
            parents: (0..n).collect(),
            sizes: vec![1; n],
            potentials: (0..n).map(|_| op.id()).collect(),
            count_components: n,
            op,
        }
    }

    /// 要素`v`が属する集合の代表元を返す．
    pub fn find(&mut self, v: usize) -> usize {
        if self.parents[v] == v {
            return v;
        }
        let root = self.find(self.parents[v]);

        self.potentials[v] = self
            .op
            .op(&self.potentials[v], &self.potentials[self.parents[v]]);

        self.parents[v] = root;
        root
    }

    /// 要素`u`と`v`が属する集合を統合する．
    /// `potential[u]+d=potential[v]`となるようにポテンシャルを更新する．
    /// すでに`u`と`v`が同じ集合に属しており，既存の差と矛盾があれば更新は行われずfalseを返す．
    /// そうでない場合にはtrueを返す．
    pub fn merge(&mut self, u: usize, v: usize, d: O::Element) -> bool
    where
        O::Element: PartialEq,
    {
        let mut w = {
            let _ = self.find(u);
            let _ = self.find(v);
            let pu = &self.potentials[u];
            let pv = &self.potentials[v];
            self.op.op(&self.op.op(&d, pu), &self.op.inv(pv))
        };

        let mut u = self.find(u);
        let mut v = self.find(v);

        if u == v {
            return self.diff_potential(u, v) == w;
        }

        if self.sizes[u] < self.sizes[v] {
            swap(&mut u, &mut v);
            w = self.op.inv(&w);
        }

        self.sizes[u] += self.sizes[v];
        self.parents[v] = u;
        self.potentials[v] = w;
        self.count_components -= 1;

        true
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

    /// 要素`v`に置かれたポテンシャルを返す．
    pub fn potential(&mut self, v: usize) -> &O::Element {
        let _ = self.find(v);
        &self.potentials[v]
    }

    /// 要素`u`と`v`が同じ集合に属している場合に，
    /// それらのポテンシャルの差(`potential[v] - potential[u]`)を返す．
    /// `u`と`v`が同じ集合に属していない場合はpanicする．
    pub fn diff_potential(&mut self, u: usize, v: usize) -> O::Element {
        assert!(self.connected(u, v));
        let _ = self.find(u);
        let _ = self.find(v);
        let pu = &self.potentials[u];
        let pv = &self.potentials[v];
        self.op.op(pv, &self.op.inv(pu))
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
    use crate::ops::{op_add::OpAdd, op_xor::OpXor};

    use super::*;

    #[test]
    fn test() {
        let mut dsu = PotentializedDsu::<OpAdd<i64>>::new(5);

        assert!(dsu.merge(0, 1, 4));
        assert!(dsu.connected(0, 1));
        assert_eq!(dsu.diff_potential(0, 1), 4);
        assert_eq!(dsu.diff_potential(1, 0), -4);

        assert!(!dsu.merge(0, 1, 6));
        assert_eq!(dsu.diff_potential(0, 1), 4);

        assert!(dsu.merge(0, 3, 5));
        assert_eq!(dsu.diff_potential(0, 3), 5);
        assert_eq!(dsu.diff_potential(1, 3), 1);
        assert_eq!(dsu.diff_potential(3, 1), -1);

        let mut dsu = PotentializedDsu::<OpAdd<i64>>::new(4);

        assert!(dsu.merge(0, 1, 2));
        assert!(dsu.merge(1, 2, 3));
        assert!(dsu.merge(2, 3, 4));

        assert_eq!(dsu.diff_potential(0, 3), 9);
        assert_eq!(dsu.diff_potential(3, 0), -9);
        assert_eq!(dsu.diff_potential(1, 3), 7);
        assert_eq!(dsu.diff_potential(0, 2), 5);

        let mut dsu = PotentializedDsu::<OpAdd<i64>>::new(4);

        assert!(dsu.merge(0, 1, 3));
        assert!(dsu.merge(2, 3, 7));
        assert!(dsu.merge(1, 3, 0));
        assert_eq!(dsu.diff_potential(0, 2), -4);
        assert_eq!(dsu.diff_potential(2, 0), 4);
        assert_eq!(dsu.diff_potential(1, 3), 0);
    }

    #[test]
    fn test_xor() {
        let mut dsu = PotentializedDsu::<OpXor<u32>>::new(4);

        assert!(dsu.merge(0, 1, 1));
        assert_eq!(dsu.diff_potential(0, 1), 1);
        assert_eq!(dsu.diff_potential(1, 0), 1);

        assert!(dsu.merge(1, 2, 2));
        assert_eq!(dsu.diff_potential(0, 2), 3);
        assert_eq!(dsu.diff_potential(2, 0), 3);

        assert!(dsu.merge(2, 3, 9));
        assert_eq!(dsu.diff_potential(0, 3), 10);
        assert_eq!(dsu.diff_potential(3, 0), 10);

        assert!(dsu.merge(3, 0, 10));
        assert_eq!(dsu.diff_potential(3, 0), 10);

        assert!(!dsu.merge(0, 3, 12));
    }

    #[test]
    #[should_panic]
    fn test_diff_potential_with_unconnected_nodes() {
        // `u`と`v`が同じ集合に属していない場合に`diff_potential(u, v)`を呼ぶとpanicする．
        let mut dsu = PotentializedDsu::<OpAdd<_>>::new(4);
        dsu.merge(1, 2, 3);
        dsu.diff_potential(0, 3);
    }
}
