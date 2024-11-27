use std::mem::swap;

use crate::monoid::Monoid;

pub struct MonoidDsu<M>
where
    M: Monoid,
{
    parents: Vec<usize>,
    sizes: Vec<usize>,
    states: Vec<M::Value>,
    num_components: usize,
    monoid: M,
}

impl<M> MonoidDsu<M>
where
    M: Monoid,
    M::Value: Clone,
{
    pub fn new(n: usize, initial_states: &[M::Value], monoid: M) -> Self {
        Self {
            parents: (0..n).collect(),
            sizes: vec![1; n],
            states: initial_states.to_vec(),
            num_components: n,
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
    pub fn merge(&mut self, u: usize, v: usize) -> bool {
        debug_assert!(u < self.parents.len());
        debug_assert!(v < self.parents.len());

        let mut u = self.find(u);
        let mut v = self.find(v);

        if u == v {
            return true;
        }

        self.num_components -= 1;

        if self.sizes[u] < self.sizes[v] {
            swap(&mut u, &mut v);
        }

        self.parents[v] = u;
        self.sizes[u] += self.sizes[v];
        self.states[u] = self.monoid.op(&self.states[u], &self.states[v]);

        true
    }

    /// xとyが同じグループに属すか
    pub fn connected(&mut self, u: usize, v: usize) -> bool {
        debug_assert!(u < self.parents.len());
        debug_assert!(v < self.parents.len());
        self.find(u) == self.find(v)
    }

    /// xが属するグループの要素数
    pub fn size(&mut self, v: usize) -> usize {
        debug_assert!(v < self.parents.len());
        let v = self.find(v);
        self.sizes[v]
    }

    pub fn state(&mut self, v: usize) -> &M::Value {
        debug_assert!(v < self.parents.len());
        let v = self.find(v);
        &self.states[v]
    }

    /// 連結成分の個数
    pub fn num_components(&self) -> usize {
        self.num_components
    }
}
