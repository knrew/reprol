use std::ops::Add;

pub struct WarshallFloyd<T> {
    /// 頂点の個数
    len: usize,

    /// cost[u][v]: u->vの最小コスト
    costs: Vec<Vec<Option<T>>>,
}

impl<T> WarshallFloyd<T>
where
    T: Copy + PartialOrd + Add<Output = T>,
{
    fn new(len: usize, zero: T) -> Self {
        let mut costs = vec![vec![None; len]; len];

        for i in 0..len {
            costs[i][i] = Some(zero);
        }

        Self { len, costs }
    }

    fn add_edge(&mut self, u: usize, v: usize, c: T) {
        debug_assert!(u < self.len);
        debug_assert!(v < self.len);
        self.costs[u][v] = Some(if let Some(x) = self.costs[u][v] {
            if x < c {
                x
            } else {
                c
            }
        } else {
            c
        });
    }

    /// 隣接リスト表現のグラフから最短経路を計算する
    pub fn from_adjacencies(g: &[Vec<(usize, T)>], zero: T) -> Self {
        let n = g.len();
        let mut wf = Self::new(n, zero);

        for u in 0..n {
            for &(v, c) in &g[u] {
                wf.add_edge(u, v, c);
            }
        }

        wf.calculate();

        wf
    }

    /// 辺集合から最短経路を計算する
    pub fn from_edges(n: usize, edges: &[(usize, usize, T)], zero: T, is_directed: bool) -> Self {
        let mut wf = Self::new(n, zero);

        for &(u, v, c) in edges {
            wf.add_edge(u, v, c);
            if !is_directed {
                wf.add_edge(v, u, c);
            }
        }

        wf.calculate();

        wf
    }

    fn calculate(&mut self) {
        for k in 0..self.len {
            for i in 0..self.len {
                for j in 0..self.len {
                    // x: i->kの最小コスト
                    // y: k->jの最小コスト
                    // i->k, k->jがともに存在すれば，i->jが存在する
                    if let Some(x) = self.costs[i][k] {
                        if let Some(y) = self.costs[k][j] {
                            self.costs[i][j] = Some(if let Some(z) = self.costs[i][j] {
                                if z < x + y {
                                    z
                                } else {
                                    x + y
                                }
                            } else {
                                x + y
                            });
                        }
                    }
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn costs(&self) -> &[Vec<Option<T>>] {
        &self.costs
    }

    // u->vの最小コスト
    pub fn cost(&self, u: usize, v: usize) -> Option<T> {
        debug_assert!(u < self.len);
        debug_assert!(v < self.len);
        self.costs[u][v]
    }
}

// pub fn warshall_floyd(n: usize, edges: &[(usize, usize, i64)], is_directed: bool) -> Vec<Vec<i64>> {
//     let mut distances = vec![vec![i64::MAX; n]; n];

//     for i in 0..n {
//         distances[i][i] = 0;
//     }

//     for &(u, v, c) in edges {
//         distances[u][v] = distances[u][v].min(c);
//         if !is_directed {
//             distances[v][u] = distances[v][u].min(c);
//         }
//     }

//     for k in 0..n {
//         for i in 0..n {
//             for j in 0..n {
//                 distances[i][j] =
//                     distances[i][j].min(distances[i][k].saturating_add(distances[k][j]));
//             }
//         }
//     }

//     distances
// }
