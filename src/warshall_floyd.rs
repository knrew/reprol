use std::ops::Add;

/// ワーシャルフロイド法によって頂点間の最小コストを計算する
pub struct WarshallFloyd<C> {
    costs: Vec<Vec<Option<C>>>,
    has_negative_cycle: bool,
}

impl<C> WarshallFloyd<C>
where
    C: Clone + PartialOrd + Add<Output = C>,
{
    /// 隣接リスト表現のグラフから最短経路を計算する
    pub fn new(graph: &[Vec<(usize, C)>], zero: &C) -> Self {
        let n = graph.len();
        let mut costs = vec![vec![None; n]; n];

        for i in 0..n {
            costs[i][i] = Some(zero.clone());
        }

        for i in 0..n {
            for &(j, ref c) in &graph[i] {
                match &costs[i][j] {
                    Some(x) if x <= c => {}
                    _ => {
                        costs[i][j] = Some(c.clone());
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

        let has_negative_cycle = (0..n).any(|v| costs[v][v].as_ref().is_some_and(|c| c < &zero));

        Self {
            costs,
            has_negative_cycle,
        }
    }

    /// u->vの最小コスト
    pub fn cost(&self, u: usize, v: usize) -> Option<&C> {
        self.costs[u][v].as_ref()
    }

    /// 負の閉路が存在するか
    pub fn has_negative_cycle(&self) -> bool {
        self.has_negative_cycle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            let wf = WarshallFloyd::new(&g, &0);
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
            let wf = WarshallFloyd::new(&g, &0);
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
            let wf = WarshallFloyd::new(&g, &0);
            assert!(wf.has_negative_cycle());
        }
    }
}
