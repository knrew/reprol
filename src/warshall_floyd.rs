mod warshall_floyd {
    pub fn warshall_floyd(
        n: usize,
        edges: &[(usize, usize, i64)],
        is_directed: bool,
    ) -> Vec<Vec<i64>> {
        let mut distances = vec![vec![i64::MAX; n]; n];

        for i in 0..n {
            distances[i][i] = 0;
        }

        for &(u, v, c) in edges {
            distances[u][v] = distances[u][v].min(c);
            if !is_directed {
                distances[v][u] = distances[v][u].min(c);
            }
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    distances[i][j] =
                        distances[i][j].min(distances[i][k].saturating_add(distances[k][j]));
                }
            }
        }

        distances
    }
}
