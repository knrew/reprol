use std::{
    mem::swap,
    ops::{Add, Neg, Sub},
};

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
    /// potential[u]+w=potential[v]となるように頂点にポテンシャルを置く
    /// 既存のポテンシャルと矛盾があれば，もとのポテンシャルを維持して返り値としてfalseを返す
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
            swap(&mut u, &mut v);
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

    /// vに置かれたポテンシャル
    pub fn potential(&mut self, v: usize) -> T {
        debug_assert!(v < self.parents.len());
        let _ = self.find(v);
        self.potentials[v]
    }

    /// uとvのポテンシャルの差
    /// potential[v] - potential[u]
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
mod tests_dsu {
    use super::Dsu;

    // (ポテンシャルなし)DSUに対するクエリ
    enum Query {
        /// Merge(u, v): uが属する集合とvが属する集合を結合する
        Merge(usize, usize),

        /// Same(u, v): uとvが同じ集合に属するかを判定する
        Connected(usize, usize),

        /// Size(v): vが属する集合の要素数
        Size(usize),

        /// 連結成分の個数をカウントする
        CountComponents,
    }
    use Query::{Connected, CountComponents, Merge, Size};

    /// クエリを順に実行する
    /// 各実行結果を返す
    fn run_queries(n: usize, queries: &[Query]) -> Vec<usize> {
        let mut res = vec![];

        let mut dsu = Dsu::new(n);

        for query in queries {
            match query {
                &Merge(u, v) => {
                    dsu.merge(u, v);
                }
                &Connected(u, v) => {
                    res.push(if dsu.connected(u, v) { 1 } else { 0 });
                }
                &Size(v) => {
                    res.push(dsu.size(v));
                }
                &CountComponents => {
                    res.push(dsu.num_components());
                }
            }
        }

        res
    }

    #[test]
    fn test_dsu() {
        {
            // https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_A
            let n = 5;
            let queries = vec![
                Merge(1, 4),
                Merge(2, 3),
                Connected(1, 2),
                Connected(3, 4),
                Connected(1, 4),
                Connected(3, 2),
                Merge(1, 3),
                Connected(2, 4),
                Connected(3, 0),
                Merge(0, 4),
                Connected(0, 2),
                Connected(3, 0),
            ];
            let expected = vec![0, 0, 1, 1, 1, 0, 1, 1];
            let result = run_queries(n, &queries);
            assert_eq!(expected, result);
        }

        {
            let n = 5;
            let queries = vec![
                Merge(1, 0),
                Connected(2, 0),
                Merge(0, 2),
                Connected(4, 0),
                Connected(3, 4),
                Merge(1, 2),
                Connected(2, 0),
                Connected(4, 1),
                Connected(1, 4),
                Connected(3, 3),
                Merge(2, 1),
                Connected(3, 2),
                Merge(4, 2),
                Connected(4, 1),
            ];
            let expected = vec![0, 0, 0, 1, 0, 0, 1, 0, 1];
            let result = run_queries(n, &queries);
            assert_eq!(expected, result);
        }

        {
            let n = 10;
            let queries = vec![
                Connected(5, 4),
                Connected(4, 8),
                Merge(8, 3),
                Connected(4, 7),
                Merge(8, 4),
                CountComponents,
                Merge(5, 8),
                Merge(7, 7),
                Connected(8, 6),
                CountComponents,
                Merge(3, 5),
                Connected(6, 0),
                CountComponents,
                Size(4),
                Size(1),
                Size(2),
                Connected(8, 7),
                Connected(0, 2),
                Merge(5, 6),
                Merge(6, 3),
                CountComponents,
            ];
            let expected = vec![0, 0, 0, 8, 0, 7, 0, 7, 4, 1, 1, 0, 0, 6];
            let result = run_queries(n, &queries);
            assert_eq!(expected, result);
        }

        {
            let n = 12;
            let queries = vec![
                Connected(0, 9),
                Merge(0, 11),
                Merge(8, 2),
                Merge(10, 0),
                Connected(0, 2),
                CountComponents,
                Merge(0, 11),
                Merge(4, 10),
                CountComponents,
                Merge(0, 2),
                Connected(4, 3),
                Merge(4, 1),
                Merge(11, 2),
                Connected(11, 4),
                Merge(6, 2),
                Connected(8, 3),
                CountComponents,
                Merge(8, 5),
                Size(6),
                Merge(3, 7),
                Size(10),
                CountComponents,
            ];
            let expected = vec![0, 0, 9, 8, 0, 1, 0, 5, 9, 9, 3];
            let result = run_queries(n, &queries);
            assert_eq!(expected, result);
        }

        {
            let n = 15;
            let queries = vec![
                Merge(10, 14),
                Connected(5, 8),
                Merge(5, 14),
                Size(3),
                Merge(12, 6),
                Merge(7, 4),
                Merge(12, 6),
                Merge(7, 13),
                Connected(10, 13),
                Connected(5, 10),
                Connected(0, 1),
                Connected(6, 12),
                Connected(9, 4),
                Merge(2, 7),
                Merge(1, 12),
                Merge(7, 14),
                Size(5),
                Size(12),
                Merge(13, 5),
                Connected(6, 7),
                Connected(14, 14),
                CountComponents,
            ];
            let expected = vec![0, 1, 0, 1, 0, 1, 0, 7, 3, 0, 1, 7];
            let result = run_queries(n, &queries);
            assert_eq!(expected, result);
        }
    }
}

#[cfg(test)]
mod tests_potentialized_dsu {
    #[derive(Debug)]
    enum Query {
        /// Merge(u, v, w): uが属する集合とvが属する集合を結合し，
        /// u+w=vとなるように頂点にポテンシャルを置く
        /// すでに設定されているポテンシャルと矛盾があれば，ポテンシャルは変更せず，
        /// クエリの出力として-1を返す
        /// 矛盾がなければ0を返す
        Merge(usize, usize, i64),

        /// DifferencePotential(u, v): uとvが同じ集合に属するならその差(v-u)を出力する．
        /// そうでなければ-1を出力する
        DifferencePotential(usize, usize),

        /// Size(v): vが属する集合の要素数
        Size(usize),

        /// 連結成分の個数をカウントする
        CountComponents,
    }
    use Query::{CountComponents, DifferencePotential, Merge, Size};

    use super::PotentializedDsu;

    /// クエリを順に実行する
    /// 各実行結果を返す
    fn run_queries(n: usize, queries: &[Query]) -> Vec<i64> {
        let mut res = vec![];

        let mut dsu = PotentializedDsu::new(n, 0i64);

        for query in queries {
            match query {
                &Merge(u, v, w) => {
                    res.push(if dsu.merge(u, v, w) { 0 } else { -1 });
                }
                &DifferencePotential(u, v) => {
                    if dsu.connected(u, v) {
                        res.push(dsu.difference_potential(u, v));
                    } else {
                        res.push(-1)
                    }
                }
                &Size(v) => {
                    res.push(dsu.size(v) as i64);
                }
                &CountComponents => {
                    res.push(dsu.num_components() as i64);
                }
            }
        }

        res
    }

    #[test]
    fn test_potentialzized_dsu() {
        {
            let n = 5;
            let queries = vec![
                Merge(2, 2, 4),
                Merge(2, 2, 0),
                DifferencePotential(4, 4),
                CountComponents,
                Merge(0, 2, 3),
                Merge(1, 2, -5),
                Merge(0, 1, 48),
                DifferencePotential(4, 0),
                CountComponents,
                Size(4),
                Size(2),
                DifferencePotential(1, 2),
                DifferencePotential(0, 1),
                Size(4),
                Merge(4, 0, -30),
                Merge(4, 3, -68),
                DifferencePotential(2, 4),
                DifferencePotential(4, 2),
                DifferencePotential(3, 1),
                DifferencePotential(2, 2),
                CountComponents,
            ];
            let expected = vec![
                -1, 0, 0, 5, 0, 0, -1, -1, 3, 1, 3, -5, 8, 1, 0, 0, 27, -27, 46, 0, 1,
            ];
            let result = run_queries(n, &queries);
            assert_eq!(expected, result);
        }
    }
}
