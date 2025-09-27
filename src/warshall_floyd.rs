//! ワーシャル・フロイド(Warshall Floyd)
//!
//! ワーシャル・フロイド法によって全点間最短経路を計算する．
//!
//! # 使用例
//! ```
//! use reprol::warshall_floyd::WarshallFloyd;
//! let mut wf = WarshallFloyd::new(3, 0);
//! wf.add_edge(0, 1, 4);
//! wf.add_edge(1, 2, 2);
//! wf.build();
//! assert_eq!(wf.cost(0, 2), Some(&6));
//! wf.add_edge_incremental(0, 2, 1);
//! assert_eq!(wf.cost(0, 2), Some(&1));
//! ```

use std::ops::Add;

/// ワーシャル・フロイド本体．
pub struct WarshallFloyd<C> {
    costs: Vec<Vec<Option<C>>>,
    zero: C,
    has_build: bool,
}

impl<C> WarshallFloyd<C>
where
    C: Clone + PartialOrd + Add<Output = C>,
{
    pub fn new(n: usize, zero: C) -> Self {
        let mut costs = vec![vec![None; n]; n];

        // costs[i][i] = 0
        for (i, cost) in costs.iter_mut().enumerate() {
            cost[i] = Some(zero.clone());
        }

        Self {
            costs,
            zero,
            has_build: true,
        }
    }

    /// 全点間最短経路を計算する．
    ///
    /// - 計算量: O(N^3)
    pub fn build(&mut self) {
        self.has_build = true;
        let n = self.costs.len();
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if let (Some(cost_ik), Some(cost_kj)) = (&self.costs[i][k], &self.costs[k][j]) {
                        let new_cost = cost_ik.clone() + cost_kj.clone();
                        if !self.costs[i][j]
                            .as_ref()
                            .is_some_and(|cost_ij| cost_ij <= &new_cost)
                        {
                            self.costs[i][j] = Some(new_cost);
                        }
                    }
                }
            }
        }
    }

    /// 頂点`u`から`v`へコスト`c`の有向辺を追加する．
    /// コストの計算は行われない．
    pub fn add_edge(&mut self, u: usize, v: usize, c: C) {
        self.has_build = false;
        if !self.costs[u][v]
            .as_ref()
            .is_some_and(|cost_uv| cost_uv <= &c)
        {
            self.costs[u][v] = Some(c);
        }
    }

    /// 頂点`u`から`v`へコスト`c`の有向辺を追加し，コストの(差分)更新を行う．
    ///
    /// - 計算量: O(N^2)
    pub fn add_edge_incremental(&mut self, u: usize, v: usize, c: C) {
        assert!(self.has_build);

        let n = self.costs.len();

        if !self.costs[u][v]
            .as_ref()
            .is_some_and(|cost_uv| cost_uv <= &c)
        {
            self.costs[u][v] = Some(c.clone());

            for s in 0..n {
                for g in 0..n {
                    if let (Some(cost_su), Some(cost_vg)) = (&self.costs[s][u], &self.costs[v][g]) {
                        let new_cost = cost_su.clone() + c.clone() + cost_vg.clone();
                        if !self.costs[s][g]
                            .as_ref()
                            .is_some_and(|cost_sg| cost_sg <= &new_cost)
                        {
                            self.costs[s][g] = Some(new_cost);
                        }
                    }
                }
            }
        }
    }

    /// 頂点`u`から`v`への最小コストを返す．
    /// `build()`を呼んでコストを計算してから呼び出す．
    pub fn cost(&self, u: usize, v: usize) -> Option<&C> {
        assert!(self.has_build);
        self.costs[u][v].as_ref()
    }

    /// 負の閉路が存在するかを判定する．
    ///
    /// - 計算量：O(N)
    pub fn has_negative_cycle(&self) -> bool {
        assert!(self.has_build);
        let n = self.costs.len();
        (0..n).any(|v| self.costs[v][v].as_ref().is_some_and(|c| c < &self.zero))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_cost() {
        let n = 3;
        let edges: Vec<(usize, usize, i32)> = vec![(0, 1, 1), (1, 0, 2), (1, 2, 3), (2, 0, 4)];

        let mut wf = WarshallFloyd::new(n, 0);
        for &(u, v, c) in &edges {
            wf.add_edge(u, v, c);
        }
        wf.build();

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

    #[test]
    fn test_negative_cost() {
        let n = 5;
        let edges: Vec<(usize, usize, i64)> =
            vec![(0, 1, 4), (1, 2, 5), (0, 2, -10), (3, 4, 8), (4, 3, 12)];

        let mut wf = WarshallFloyd::new(n, 0);
        for &(u, v, c) in &edges {
            wf.add_edge(u, v, c);
        }
        wf.build();

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

    #[test]
    fn test_negative_cycle() {
        let n = 4;
        let edges: Vec<(usize, usize, i64)> = vec![
            (0, 1, 1),
            (0, 2, 5),
            (1, 2, 2),
            (1, 3, 4),
            (2, 3, 1),
            (3, 2, -7),
        ];

        let mut wf = WarshallFloyd::new(n, 0);
        for &(u, v, c) in &edges {
            wf.add_edge(u, v, c);
        }
        wf.build();

        assert!(wf.has_negative_cycle());
    }

    #[test]
    fn test_add_edge_incremental() {
        let mut wf = WarshallFloyd::new(3, 0);
        wf.add_edge(0, 1, 5);
        wf.add_edge(1, 2, 7);
        wf.build();

        wf.add_edge_incremental(0, 2, 20);
        assert_eq!(wf.cost(0, 2), Some(&12));

        wf.add_edge_incremental(0, 2, 8);
        assert_eq!(wf.cost(0, 2), Some(&8));
    }

    #[test]
    #[should_panic]
    fn test_cost_without_build() {
        let mut wf = WarshallFloyd::new(2, 0);
        wf.add_edge(0, 1, 1);
        let _ = wf.cost(0, 1);
    }
}
