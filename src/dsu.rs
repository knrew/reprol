use std::ops::{Add, Neg, Sub};

pub struct PotentializedDsu<T> {
    parents: Vec<usize>,
    sizes: Vec<usize>,
    potentials: Vec<T>,
    num_components: usize,
}

impl<T> PotentializedDsu<T>
where
    T: Copy + Clone + Eq + Add<Output = T> + Sub<Output = T> + Neg<Output = T>,
{
    pub fn new(n: usize, zero: T) -> Self {
        Self {
            parents: (0..n).collect(),
            sizes: vec![1; n],
            potentials: vec![zero; n],
            num_components: n,
        }
    }

    /// xのrootのindexを返す
    pub fn find(&mut self, v: usize) -> usize {
        debug_assert!(v < self.parents.len());

        if self.parents[v] == v {
            return v;
        }
        let root = self.find(self.parents[v]);

        let tmp = self.potentials[v] + self.potentials[self.parents[v]];
        self.potentials[v] = tmp;

        self.parents[v] = root;
        root
    }

    /// xが属するグループとyが属するグループを統合する
    /// すでに決まっているポテンシャルと矛盾があればfalseを返す
    pub fn merge(&mut self, u: usize, v: usize, w: T) -> bool {
        debug_assert!(u < self.parents.len());
        debug_assert!(v < self.parents.len());

        let mut w = w + self.potential(u) - self.potential(v);

        let mut u = self.find(u);
        let mut v = self.find(v);

        if u == v {
            return self.difference_potential(u, v) == w;
        }

        self.num_components -= 1;

        if self.sizes[u] < self.sizes[v] {
            (u, v) = (v, u);
            w = -w;
        }

        self.sizes[u] += self.sizes[v];
        self.parents[v] = u;
        self.potentials[v] = w;

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

    pub fn potential(&mut self, v: usize) -> T {
        debug_assert!(v < self.parents.len());
        let _ = self.find(v);
        self.potentials[v]
    }

    pub fn difference_potential(&mut self, u: usize, v: usize) -> T {
        debug_assert!(u < self.parents.len());
        debug_assert!(v < self.parents.len());
        debug_assert!(self.connected(u, v));
        self.potential(v) - self.potential(u)
    }

    /// 連結成分の個数
    pub fn num_components(&self) -> usize {
        self.num_components
    }
}

/// ポテンシャルなしDSU
pub struct Dsu(PotentializedDsu<i8>);

impl Dsu {
    pub fn new(n: usize) -> Self {
        Self(PotentializedDsu::new(n, 0))
    }

    pub fn find(&mut self, v: usize) -> usize {
        self.0.find(v)
    }

    pub fn merge(&mut self, u: usize, v: usize) {
        self.0.merge(u, v, 0);
    }

    pub fn connected(&mut self, u: usize, v: usize) -> bool {
        self.0.connected(u, v)
    }

    pub fn size(&mut self, v: usize) -> usize {
        self.0.size(v)
    }

    pub fn num_components(&self) -> usize {
        self.0.num_components()
    }
}

#[cfg(test)]
mod tests {
    use crate::dsu::Dsu;

    #[test]
    fn test_dsu() {
        let mut d = Dsu::new(4);
        d.merge(0, 1);
        assert!(d.connected(0, 1));
        d.merge(1, 2);
        assert!(d.connected(0, 2));
        assert_eq!(d.size(0), 3);
        assert!(!d.connected(0, 3));
    }
}

// pub struct Dsu {
//     parents: Vec<usize>,
//     sizes: Vec<usize>,
//     num_components: usize,
// }

// impl Dsu {
//     pub fn new(n: usize) -> Self {
//         Self {
//             parents: (0..n).collect(),
//             sizes: vec![1; n],
//             num_components: n,
//         }
//     }

//     /// xのrootのindexを返す
//     pub fn find(&mut self, x: usize) -> usize {
//         if self.parents[x] != x {
//             self.parents[x] = self.find(self.parents[x]);
//         }
//         self.parents[x]
//     }

//     /// xが属するグループとyが属するグループを統合する
//     pub fn merge(&mut self, x: usize, y: usize) {
//         let x = self.find(x);
//         let y = self.find(y);

//         if x == y {
//             return;
//         }

//         self.num_components -= 1;

//         if self.sizes[x] < self.sizes[y] {
//             self.sizes.swap(x, y);
//         }
//         self.sizes[x] += self.sizes[y];
//         self.parents[y] = x;
//     }

//     /// xとyが同じグループに属すか
//     pub fn connected(&mut self, x: usize, y: usize) -> bool {
//         self.find(x) == self.find(y)
//     }

//     /// xが属するグループの要素数
//     pub fn size(&mut self, x: usize) -> usize {
//         let y = self.find(x);
//         self.sizes[y]
//     }

//     /// 連結成分の個数
//     pub fn num_components(&self) -> usize {
//         self.num_components
//     }
// }
