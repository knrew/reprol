use std::ops::Add;

pub struct BellmanFord<T> {
    n: usize,

    start: usize,

    costs: Vec<Option<T>>,

    has_negative_cycle: bool,
}

impl<T> BellmanFord<T>
where
    T: Clone + PartialOrd + Add<Output = T>,
{
    pub fn new(g: &[Vec<(usize, T)>], start: usize, zero: T) -> Self {
        let n = g.len();

        let mut costs = vec![None; n];
        costs[start] = Some(zero.clone());

        for _ in 0..n {
            for v in 0..n {
                for (nv, dcost) in &g[v] {
                    let cost_v = if let Some(cost_v) = &costs[v] {
                        cost_v
                    } else {
                        continue;
                    };
                    let new_cost = cost_v.clone() + dcost.clone();
                    match &costs[*nv] {
                        Some(cost_nv) if cost_nv <= &new_cost => {}
                        _ => {
                            costs[*nv] = Some(new_cost);
                        }
                    }
                }
            }
        }

        let mut is_negative = vec![false; n];

        for _ in 0..n {
            for v in 0..n {
                for (nv, dcost) in &g[v] {
                    let cost_v = if let Some(cost_v) = &costs[v] {
                        cost_v
                    } else {
                        continue;
                    };
                    let new_cost = cost_v.clone() + dcost.clone();
                    match &costs[*nv] {
                        Some(cost_nv) if cost_nv <= &new_cost => {}
                        _ => {
                            costs[*nv] = Some(new_cost);
                            is_negative[*nv] = true;
                        }
                    }
                    if is_negative[v] {
                        is_negative[*nv] = true;
                    }
                }
            }
        }

        let has_negative_cycle = is_negative.iter().any(|&f| f);

        Self {
            n,
            start,
            costs,
            has_negative_cycle,
        }
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn cost(&self, v: usize) -> Option<&T> {
        self.costs[v].as_ref()
    }

    pub fn has_negative_cycle(&self) -> bool {
        self.has_negative_cycle
    }
}

// TODO:テストを書く
