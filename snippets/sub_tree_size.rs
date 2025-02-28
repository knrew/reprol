//! 頂点0を根とした木の各部分木のサイズを求める

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
    }

    let g = {
        let mut g = vec![vec![]; n];
        for &(u, v) in &edges {
            g[u].push(v);
            g[v].push(u);
        }
        g
    };

    let mut sub = vec![1; n];
    dfs(&g, &mut sub, 0, n);
}

fn dfs(g: &[Vec<usize>], sub: &mut [usize], v: usize, pv: usize) {
    for &nv in &g[v] {
        if nv == pv {
            continue;
        }
        dfs(g, sub, nv, v);
        sub[v] += sub[nv];
    }
}
