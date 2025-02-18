fn bellman_ford(g: &[Vec<(usize, i64)>], start: usize) -> (Vec<Option<i64>>, Vec<bool>) {
    let n = g.len();

    let mut costs = vec![None; n];
    costs[start] = Some(0);

    for _ in 0..n {
        for v in 0..n {
            for &(nv, dcost) in &g[v] {
                let new_cost = match costs[v] {
                    Some(cost_v) => cost_v + dcost,
                    None => continue,
                };
                match costs[nv] {
                    Some(cost_nv) if cost_nv > new_cost => {}
                    _ => {
                        continue;
                    }
                }
                costs[nv] = Some(new_cost);
            }
        }
    }

    let mut is_negative = vec![false; n];

    for _ in 0..n {
        for v in 0..n {
            for &(nv, dcost) in &g[v] {
                let new_cost = match costs[v] {
                    Some(cost_v) => cost_v + dcost,
                    None => continue,
                };
                match costs[nv] {
                    Some(cost_nv) if cost_nv <= new_cost => {}
                    _ => {
                        costs[nv] = Some(new_cost);
                        is_negative[nv] = true;
                    }
                }
                if is_negative[v] {
                    is_negative[nv] = true;
                }
            }
        }
    }

    (costs, is_negative)
}
