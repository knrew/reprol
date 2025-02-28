use std::ops::Add;

/// ワーシャルフロイド法によって頂点間の最小コストを計算する
pub struct WarshallFloyd<T> {
    /// 頂点の個数
    n: usize,

    /// cost[u][v]: u->vの最小コスト
    costs: Vec<Vec<Option<T>>>,

    // previous: Vec<Vec<Option<usize>>>,
    /// 負の閉路が存在するか
    has_negative_cycle: bool,
}

impl<T> WarshallFloyd<T>
where
    T: Clone + PartialOrd + Add<Output = T>,
{
    /// 隣接リスト表現のグラフから最短経路を計算する
    pub fn new(g: &[Vec<(usize, T)>], zero: T) -> Self {
        let n = g.len();
        let mut costs = vec![vec![None; n]; n];
        // let mut previous = vec![vec![None; n]; n];

        for i in 0..n {
            costs[i][i] = Some(zero.clone());
        }

        for i in 0..n {
            for (j, c) in &g[i] {
                match &costs[i][*j] {
                    Some(x) if x <= c => {}
                    _ => {
                        costs[i][*j] = Some(c.clone());
                        // previous[u][*v] = Some(u);
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
                                    // previous[i][j] = previous[k][j];
                                }
                            }
                        }
                    }
                }
            }
        }

        let has_negative_cycle = (0..n).any(|v| costs[v][v].as_ref().unwrap() < &zero);

        Self {
            n,
            costs,
            // previous,
            has_negative_cycle,
        }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    /// u->vの最小コスト
    pub fn cost(&self, u: usize, v: usize) -> Option<&T> {
        self.costs[u][v].as_ref()
    }

    /// u->vの最短経路においてvの1個前の頂点
    // pub fn previous(&self, u: usize, v: usize) -> Option<usize> {
    //     self.previous[u][v]
    // }

    /// 頂点uからvへ到達可能ならばその最短経路を構築する
    /// uとvを含む
    // pub fn path(&self, u: usize, v: usize) -> Option<Vec<usize>> {
    //     if self.previous[u][v].is_none() {
    //         return None;
    //     }

    //     let mut res = vec![v];

    //     while let Some(x) = self.previous[u][*res.last().unwrap()] {
    //         res.push(x);
    //     }

    //     res.reverse();
    //     Some(res)
    // }

    /// 負の閉路が存在するか
    pub fn has_negative_cycle(&self) -> bool {
        self.has_negative_cycle
    }
}

#[cfg(test)]
mod tests {
    use super::WarshallFloyd;

    #[test]
    fn test_warshall_floyd() {
        // case 1: 連結
        {
            let n = 3;
            // (u, v, c): u->vのコストがc
            let edges: Vec<(usize, usize, i32)> = vec![(0, 1, 1), (1, 0, 2), (1, 2, 3), (2, 0, 4)];
            let g = {
                let mut g = vec![vec![]; n];
                for &(u, v, c) in &edges {
                    g[u].push((v, c));
                }
                g
            };
            let wf = WarshallFloyd::new(&g, 0);
            assert!(!wf.has_negative_cycle());
            assert_eq!(wf.cost(0, 0), Some(&0));
            assert_eq!(wf.cost(0, 1), Some(&1));
            assert_eq!(wf.cost(0, 2), Some(&4));
            assert_eq!(wf.cost(1, 0), Some(&2));
            assert_eq!(wf.cost(1, 1), Some(&0));
            assert_eq!(wf.cost(1, 2), Some(&3));
            assert_eq!(wf.cost(2, 0), Some(&4));
            assert_eq!(wf.cost(2, 1), Some(&5));
            assert_eq!(wf.cost(2, 2), Some(&0));
        }

        // case 2
        {
            let n = 5;
            let edges: Vec<(usize, usize, i64)> =
                vec![(0, 1, 4), (1, 2, 5), (0, 2, -10), (3, 4, 8), (4, 3, 12)];
            let g = {
                let mut g = vec![vec![]; n];
                for &(u, v, c) in &edges {
                    g[u].push((v, c));
                }
                g
            };
            let wf = WarshallFloyd::new(&g, 0);
            assert!(!wf.has_negative_cycle());
            assert_eq!(wf.cost(0, 0), Some(&0));
            assert_eq!(wf.cost(0, 1), Some(&4));
            assert_eq!(wf.cost(0, 2), Some(&-10));
            assert_eq!(wf.cost(0, 3), None);
            assert_eq!(wf.cost(0, 4), None);
            assert_eq!(wf.cost(1, 0), None);
            assert_eq!(wf.cost(1, 1), Some(&0));
            assert_eq!(wf.cost(1, 2), Some(&5));
            assert_eq!(wf.cost(3, 2), None);
            assert_eq!(wf.cost(3, 4), Some(&8));
            assert_eq!(wf.cost(4, 3), Some(&12));
        }

        // case 3: 負の閉路あり
        {
            let n = 4;
            let edges: Vec<(usize, usize, i64)> = vec![
                (0, 1, 1),
                (0, 2, 5),
                (1, 2, 2),
                (1, 3, 4),
                (2, 3, 1),
                (3, 2, -7),
            ];
            let g = {
                let mut g = vec![vec![]; n];
                for &(u, v, c) in &edges {
                    g[u].push((v, c));
                }
                g
            };
            let wf = WarshallFloyd::new(&g, 0);
            assert!(wf.has_negative_cycle());
        }
    }
}
