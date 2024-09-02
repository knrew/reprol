mod bfs {
    use std::collections::VecDeque;

    pub fn bfs(g: &[Vec<usize>], start: usize) -> Vec<i64> {
        let mut distances = vec![-1; g.len()];
        let mut queue = VecDeque::new();

        queue.push_back(start);
        distances[start] = 0;

        while let Some(v) = queue.pop_front() {
            for &nv in &g[v] {
                if distances[nv] != -1 {
                    continue;
                }
                distances[nv] = distances[v] + 1;
                queue.push_back(nv);
            }
        }

        distances
    }

    pub fn bfs_grid(
        g: &[Vec<char>],
        wall_character: char,
        d: &[(i64, i64)],
        start: (usize, usize),
    ) -> Vec<Vec<i64>> {
        let h = g.len();
        let w = g[0].len();

        let mut distances = vec![vec![-1; w]; h];
        let mut queue = VecDeque::new();

        queue.push_back(start);
        distances[start.0][start.1] = 0;

        while let Some((x, y)) = queue.pop_front() {
            let neighbors = d
                .iter()
                .map(|&(dx, dy)| (x as i64 + dx, y as i64 + dy))
                .filter(|&(nx, ny)| 0 <= nx && nx < h as i64 && 0 <= ny && ny < w as i64)
                .map(|(nx, ny)| (nx as usize, ny as usize));

            for (nx, ny) in neighbors {
                if g[nx][ny] == wall_character || distances[nx][ny] != -1 {
                    continue;
                }
                distances[nx][ny] = distances[x][y] + 1;
                queue.push_back((nx, ny));
            }
        }

        distances
    }
}
