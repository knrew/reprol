use std::mem::swap;

use crate::ops::monoid::Monoid;

pub struct MonoidDsu<O: Monoid> {
    parents: Vec<usize>,
    sizes: Vec<usize>,
    states: Vec<O::Element>,
    count_components: usize,
    monoid: O,
}

impl<O: Monoid> MonoidDsu<O> {
    pub fn new(initial_states: Vec<O::Element>, monoid: O) -> Self {
        let n = initial_states.len();
        Self {
            parents: (0..n).collect(),
            sizes: vec![1; n],
            states: initial_states,
            count_components: n,
            monoid,
        }
    }

    /// xのrootのindexを返す
    pub fn find(&mut self, v: usize) -> usize {
        debug_assert!(v < self.parents.len());

        if self.parents[v] == v {
            return v;
        }
        let root = self.find(self.parents[v]);

        self.parents[v] = root;
        root
    }

    /// xが属するグループとyが属するグループを統合する
    pub fn merge(&mut self, u: usize, v: usize) {
        let mut u = self.find(u);
        let mut v = self.find(v);

        if u == v {
            return;
        }

        self.count_components -= 1;

        if self.sizes[u] < self.sizes[v] {
            swap(&mut u, &mut v);
        }

        self.parents[v] = u;
        self.sizes[u] += self.sizes[v];
        self.states[u] = self.monoid.op(&self.states[u], &self.states[v]);
    }

    /// xとyが同じグループに属すか
    pub fn connected(&mut self, u: usize, v: usize) -> bool {
        self.find(u) == self.find(v)
    }

    /// xが属するグループの要素数
    pub fn size(&mut self, v: usize) -> usize {
        let v = self.find(v);
        self.sizes[v]
    }

    pub fn state(&mut self, v: usize) -> &O::Element {
        let v = self.find(v);
        &self.states[v]
    }

    /// 連結成分の個数
    pub fn count_components(&self) -> usize {
        self.count_components
    }
}
