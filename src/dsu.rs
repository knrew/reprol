pub struct Dsu {
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            sizes: vec![1; n],
        }
    }

    /// xのrootのindexを返す
    pub fn find(&mut self, x: usize) -> usize {
        if self.parents[x] != x {
            self.parents[x] = self.find(self.parents[x]);
        }
        self.parents[x]
    }

    /// xが属するグループとyが属するグループを統合する
    pub fn merge(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);

        if x != y {
            if self.sizes[x] < self.sizes[y] {
                self.sizes.swap(x, y);
            }
            self.sizes[x] += self.sizes[y];
            self.parents[y] = x;
        }
    }

    /// xとyが同じグループに属すか
    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// xが属するグループの要素数
    pub fn size(&mut self, x: usize) -> usize {
        let y = self.find(x);
        self.sizes[y]
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
        // assert_eq!(d.groups(), vec![vec![0, 1, 2], vec![3]]);
    }
}
