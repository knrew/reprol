use std::ops::{Add, Neg, Sub};

pub struct PotentializedDsu<T> {
    parents: Vec<usize>,
    sizes: Vec<usize>,
    weights: Vec<T>,
}

impl<T> PotentializedDsu<T>
where
    T: Copy + Clone + Eq + Add<Output = T> + Sub<Output = T> + Neg<Output = T>,
{
    pub fn new(n: usize, zero: T) -> Self {
        Self {
            parents: (0..n).collect(),
            sizes: vec![1; n],
            weights: vec![zero; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            return x;
        }
        let root = self.find(self.parents[x]);

        let tmp = self.weights[x] + self.weights[self.parents[x]];
        self.weights[x] = tmp;

        self.parents[x] = root;
        root
    }

    pub fn merge(&mut self, x: usize, y: usize, w: T) -> bool {
        let mut w = w + self.weight(x) - self.weight(y);

        let mut x = self.find(x);
        let mut y = self.find(y);

        if x == y {
            return self.diff(x, y) == w;
        }

        if self.sizes[x] < self.sizes[y] {
            (x, y) = (y, x);
            w = -w;
        }

        self.sizes[x] += self.sizes[x];
        self.parents[y] = x;
        self.weights[y] = w;

        true
    }

    pub fn diff(&mut self, x: usize, y: usize) -> T {
        self.weight(y) - self.weight(x)
    }

    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn size(&mut self, x: usize) -> usize {
        let x = self.find(x);
        self.parents[x]
    }

    pub fn weight(&mut self, x: usize) -> T {
        let _ = self.find(x);
        self.weights[x]
    }
}
