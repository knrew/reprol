use std::ops::Add;

pub struct BellmanFord<C> {
    start: usize,
    costs: Vec<Option<C>>,
    has_negative_cycle: bool,
}

impl<C> BellmanFord<C>
where
    C: Clone + PartialOrd + Add<Output = C>,
{
    pub fn new(g: &[Vec<(usize, C)>], start: usize, zero: &C) -> Self {
        let n = g.len();

        let mut costs = vec![None; n];
        costs[start] = Some(zero.clone());

        for _ in 0..n {
            for v in 0..n {
                for &(nv, ref dcost) in &g[v] {
                    if let Some(cost_v) = &costs[v] {
                        let new_cost = cost_v.clone() + dcost.clone();
                        if !costs[nv]
                            .as_ref()
                            .is_some_and(|cost_nv| cost_nv <= &new_cost)
                        {
                            costs[nv] = Some(new_cost);
                        }
                    }
                }
            }
        }

        let mut is_negative = vec![false; n];

        for _ in 0..n {
            for v in 0..n {
                for &(nv, ref dcost) in &g[v] {
                    if let Some(cost_v) = &costs[v] {
                        let new_cost = cost_v.clone() + dcost.clone();
                        if !costs[nv]
                            .as_ref()
                            .is_some_and(|cost_nv| cost_nv <= &new_cost)
                        {
                            costs[nv] = Some(new_cost);
                            is_negative[nv] = true;
                        }

                        if is_negative[v] {
                            is_negative[nv] = true;
                        }
                    }
                }
            }
        }

        let has_negative_cycle = is_negative.into_iter().any(|f| f);

        Self {
            start,
            costs,
            has_negative_cycle,
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn cost(&self, v: usize) -> Option<&C> {
        self.costs[v].as_ref()
    }

    pub fn has_negative_cycle(&self) -> bool {
        self.has_negative_cycle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bellman_ford() {
        {
            let g = vec![
                vec![(1, 2)],
                vec![(2, 3), (4, 9)],
                vec![(4, 4)],
                vec![(0, 1)],
                vec![],
            ];
            let start = 0;
            let expected = vec![Some(&0), Some(&2), Some(&5), None, Some(&9)];
            let bf = BellmanFord::new(&g, start, &0);
            let result = (0..g.len()).map(|v| bf.cost(v)).collect::<Vec<_>>();
            assert!(!bf.has_negative_cycle());
            assert_eq!(result, expected);
        }

        {
            let g = vec![vec![(1, 1)], vec![(2, 2)], vec![]];
            let start = 0;
            let expected = vec![Some(&0), Some(&1), Some(&3)];
            let bf = BellmanFord::new(&g, start, &0);
            let result = (0..g.len()).map(|v| bf.cost(v)).collect::<Vec<_>>();
            assert!(!bf.has_negative_cycle());
            assert_eq!(result, expected);
        }

        {
            let g = vec![vec![(1, 4), (2, 5)], vec![(2, -2)], vec![]];
            let start = 0;
            let expected = vec![Some(&0), Some(&4), Some(&2)];
            let bf = BellmanFord::new(&g, start, &0);
            let result = (0..g.len()).map(|v| bf.cost(v)).collect::<Vec<_>>();
            assert!(!bf.has_negative_cycle());
            assert_eq!(result, expected);
        }

        {
            let g = vec![vec![(1, 1)], vec![(2, -1)], vec![(0, -1)]];
            let start = 0;
            let bf = BellmanFord::new(&g, start, &0);
            assert!(bf.has_negative_cycle());
        }

        {
            let g = vec![vec![(1, 2)], vec![(2, 3)], vec![], vec![]];
            let start = 0;
            let expected = vec![Some(&0), Some(&2), Some(&5), None];
            let bf = BellmanFord::new(&g, start, &0);
            let result = (0..g.len()).map(|v| bf.cost(v)).collect::<Vec<_>>();
            assert!(!bf.has_negative_cycle());
            assert_eq!(result, expected);
        }

        {
            let g = vec![vec![]];
            let start = 0;
            let expected = vec![Some(&0)];
            let bf = BellmanFord::new(&g, start, &0);
            let result = (0..g.len()).map(|v| bf.cost(v)).collect::<Vec<_>>();
            assert!(!bf.has_negative_cycle());
            assert_eq!(result, expected);
        }
    }
}
