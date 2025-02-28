use std::mem::swap;

use crate::ops::group::Group;

pub struct PotentializedDsu<O: Group> {
    parents: Vec<usize>,
    sizes: Vec<usize>,
    potentials: Vec<O::Value>,
    num_components: usize,
    op: O,
}

impl<O: Group> PotentializedDsu<O> {
    pub fn new(n: usize) -> Self
    where
        O: Default,
    {
        Self::with_op(n, O::default())
    }

    pub fn with_op(n: usize, op: O) -> Self {
        Self {
            parents: (0..n).collect(),
            sizes: vec![1; n],
            potentials: (0..n).map(|_| op.identity()).collect(),
            num_components: n,
            op,
        }
    }

    /// xのrootのindexを返す
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

    /// xが属するグループとyが属するグループを統合する
    /// potential[u]+w=potential[v]となるように頂点にポテンシャルを置く
    /// 既存のポテンシャルと矛盾があれば，もとのポテンシャルを維持して返り値としてfalseを返す
    pub fn merge(&mut self, u: usize, v: usize, d: O::Value) -> bool
    where
        O::Value: Clone + PartialEq,
    {
        let mut w = {
            let pu = self.potential(u);
            let pv = self.potential(v);
            self.op.op(&self.op.op(&d, &pu), &self.op.inv(&pv))
        };

        let mut u = self.find(u);
        let mut v = self.find(v);

        if u == v {
            return self.difference_potential(u, v) == w;
        }

        self.num_components -= 1;

        if self.sizes[u] < self.sizes[v] {
            swap(&mut u, &mut v);
            w = self.op.inv(&w);
        }

        self.sizes[u] += self.sizes[v];
        self.parents[v] = u;
        self.potentials[v] = w;

        true
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

    /// vに置かれたポテンシャル
    pub fn potential(&mut self, v: usize) -> O::Value
    where
        O::Value: Clone,
    {
        let _ = self.find(v);
        self.potentials[v].clone()
    }

    /// uとvのポテンシャルの差
    /// potential[v] - potential[u]
    pub fn difference_potential(&mut self, u: usize, v: usize) -> O::Value
    where
        O::Value: Clone,
    {
        assert!(self.connected(u, v));
        let pv = self.potential(v);
        let pu = self.potential(u);
        self.op.op(&pv, &self.op.inv(&pu))
    }

    /// 連結成分の個数
    pub fn num_components(&self) -> usize {
        self.num_components
    }
}

#[cfg(test)]
mod tests {
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

    use crate::ops::op_add::OpAdd;

    use super::PotentializedDsu;

    /// クエリを順に実行する
    /// 各実行結果を返す
    fn run_queries(n: usize, queries: &[Query]) -> Vec<i64> {
        let mut res = vec![];

        let mut dsu = PotentializedDsu::<OpAdd<i64>>::new(n);

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
