use std::ops::Add;

/// ワーシャルフロイド法によって頂点間の最小コストを計算する
pub struct WarshallFloyd<T> {
    /// 頂点の個数
    n: usize,

    /// cost[u][v]: u->vの最小コスト
    costs: Vec<Vec<Option<T>>>,

    /// 0
    zero: T,
}

impl<T> WarshallFloyd<T>
where
    T: Clone + PartialOrd + Add<Output = T>,
{
    /// 隣接リスト表現のグラフから最短経路を計算する
    pub fn new(g: &[Vec<(usize, T)>], zero: T) -> Self {
        let n = g.len();
        let mut costs = vec![vec![None; n]; n];

        for i in 0..n {
            costs[i][i] = Some(zero.clone());
        }

        for u in 0..n {
            for (v, c) in &g[u] {
                match &costs[u][*v] {
                    Some(x) if x <= c => {}
                    _ => {
                        costs[u][*v] = Some(c.clone());
                    }
                }
            }
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    // i->k, k->jがともに存在すれば，i->jが存在する
                    if let Some(cost_ik) = &costs[i][k] {
                        if let Some(cost_kj) = &costs[k][j] {
                            let new_cost = cost_ik.clone() + cost_kj.clone();
                            match &costs[i][j] {
                                Some(cost_ij) if cost_ij <= &new_cost => {}
                                _ => {
                                    costs[i][j] = Some(new_cost);
                                }
                            }
                        }
                    }
                }
            }
        }

        Self { n, costs, zero }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    /// u->vの最小コスト
    pub fn cost(&self, u: usize, v: usize) -> Option<&T> {
        self.costs[u][v].as_ref()
    }

    /// 負の閉路が存在するか
    pub fn has_negative_cycle(&self) -> bool {
        (0..self.n).any(|v| self.costs[v][v].as_ref().unwrap() < &self.zero)
    }
}

// TODO:テストを書く
