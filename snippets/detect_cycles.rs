// グラフの閉路をすべて検出する
// 連結成分ごとに閉路の個数がたかだか1個のグラフにのみ適用可能
// is_edge_minimal: 辺素な閉路のみを検出するかどうか
fn detect_cycles(g: &[Vec<usize>], is_edge_minimal: bool) -> Vec<Vec<usize>> {
    fn dfs(
        g: &[Vec<usize>],
        is_edge_minimal: bool,
        visited: &mut [bool],
        finished: &mut [bool],
        path: &mut Vec<usize>,
        v: usize,
        pv: usize,
    ) -> Option<usize> {
        visited[v] = true;
        path.push(v);

        for &nv in &g[v] {
            if is_edge_minimal && nv == pv {
                continue;
            }

            if finished[nv] {
                continue;
            }

            if visited[nv] && !finished[nv] {
                return Some(nv);
            } else if let Some(u) = dfs(g, is_edge_minimal, visited, finished, path, nv, v) {
                return Some(u);
            }
        }

        finished[v] = true;
        path.pop();

        None
    }

    let n = g.len();
    let mut visited = vec![false; n];
    let mut finished = vec![false; n];
    let mut res = vec![];

    for v in 0..n {
        if visited[v] {
            continue;
        }

        let mut path = vec![];
        if let Some(v) = dfs(
            &g,
            is_edge_minimal,
            &mut visited,
            &mut finished,
            &mut path,
            v,
            1usize.wrapping_neg(),
        ) {
            let mut cycle = vec![];
            while let Some(u) = path.pop() {
                cycle.push(u);
                if u == v {
                    break;
                }
            }

            cycle.reverse();
            res.push(cycle);
        }
    }

    res
}
