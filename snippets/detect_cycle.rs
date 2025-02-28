/// グラフに閉路があればそのうちの1つを返す
fn detect_cycle(g: &[Vec<usize>], is_edge_minimal: bool) -> Option<Vec<usize>> {
    fn dfs(
        g: &[Vec<usize>],
        is_edge_minimal: bool,
        seen: &mut [bool],
        finished: &mut [bool],
        path: &mut Vec<usize>,
        v: usize,
        pv: usize,
    ) -> Option<usize> {
        seen[v] = true;
        path.push(v);

        for &nv in &g[v] {
            if is_edge_minimal && nv == pv {
                continue;
            }

            if finished[nv] {
                continue;
            }

            if seen[nv] && !finished[nv] {
                return Some(nv);
            } else {
                if let Some(u) = dfs(g, is_edge_minimal, seen, finished, path, nv, v) {
                    return Some(u);
                }
            }
        }

        finished[v] = true;
        path.pop();

        None
    }

    let n = g.len();
    let mut seen = vec![false; n];
    let mut finished = vec![false; n];

    for v in 0..n {
        if seen[v] {
            continue;
        }

        let mut path = vec![];
        if let Some(v) = dfs(
            &g,
            is_edge_minimal,
            &mut seen,
            &mut finished,
            &mut path,
            v,
            1usize.wrapping_neg(),
        ) {
            let mut res = vec![];
            while let Some(u) = path.pop() {
                res.push(u);
                if u == v {
                    break;
                }
            }
            res.reverse();
            return Some(res);
        }
    }

    None
}
