/// 木の重心を求める
pub mod tree_centroid {
    pub fn find_tree_centroid(tree: &[Vec<usize>]) -> usize {
        let mut sizes = vec![0; tree.len()];
        dfs(tree, &mut sizes, 0, tree.len());
        solve(tree, &sizes, 0, tree.len()).unwrap()
    }

    fn dfs(g: &[Vec<usize>], sizes: &mut [usize], v: usize, pv: usize) -> usize {
        sizes[v] = 1;

        for &nv in &g[v] {
            if nv == pv {
                continue;
            }
            sizes[v] += dfs(g, sizes, nv, v);
        }

        sizes[v]
    }

    fn solve(g: &[Vec<usize>], sizes: &[usize], v: usize, pv: usize) -> Option<usize> {
        let n = g.len();

        let mut is_centroid = true;

        for &nv in &g[v] {
            if nv == pv {
                continue;
            }

            if let Some(res) = solve(g, sizes, nv, v) {
                return Some(res);
            }

            if sizes[nv] > n / 2 {
                is_centroid = false;
            }
        }

        if (n - sizes[v]) > n / 2 {
            is_centroid = false;
        }

        if is_centroid {
            Some(v)
        } else {
            None
        }
    }
}
