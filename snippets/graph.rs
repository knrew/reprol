fn graph_unweighted(n: usize, edges: &[(usize, usize)]) {
    // 重みなしグラフ
    let g = {
        let mut g = vec![vec![]; n];
        for &(u, v) in edges {
            g[u].push(v);
            // g[v].push(u);
        }
        g
    };
}

fn graph_weighted(n: usize, edges: &[(usize, usize, u64)]) {
    // 重みつきグラフ
    let g = {
        let mut g = vec![vec![]; n];
        for &(u, v, c) in edges {
            g[u].push((v, c));
            // g[v].push((u, c));
        }
        g
    };
}
