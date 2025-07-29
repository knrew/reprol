//! # 問題例
//! [ABC314 F](https://atcoder.jp/contests/abc314/tasks/abc314_f)

use std::{
    mem::{swap, take},
    ops::{Add, Sub},
};

/// 各頂点に重みを持つDSU.
pub struct LazyDsu<T> {
    parents: Vec<usize>,
    sizes: Vec<usize>,
    components: Vec<Vec<usize>>,
    values: Vec<T>,
    lazies: Vec<T>,
}

impl<T> LazyDsu<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T>,
{
    /// 要素数`n`で初期化する．
    pub fn new(n: usize, zero: T) -> Self {
        assert!(n > 0);
        Self {
            parents: (0..n).collect(),
            sizes: vec![1; n],
            components: (0..n).map(|v| vec![v]).collect(),
            values: vec![zero; n],
            lazies: vec![zero; n],
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

        for x in take(&mut self.components[v]) {
            self.values[x] = self.values[x] + self.lazies[v] - self.lazies[u];
            self.components[u].push(x);
        }

        self.sizes[u] += self.sizes[v];
        self.parents[v] = u;
    }

    /// 要素`v`が属する集合それぞれに`x`を加算する．
    pub fn add(&mut self, v: usize, x: T) {
        let v = self.find(v);
        self.lazies[v] = self.lazies[v] + x;
    }

    /// 要素`v`の値を返す．
    pub fn get(&mut self, v: usize) -> T {
        let parent = self.find(v);
        self.values[v] + self.lazies[parent]
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
}
