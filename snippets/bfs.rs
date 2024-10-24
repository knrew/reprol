use std::collections::VecDeque;

pub fn bfs(g: &[Vec<usize>], start: usize) -> Vec<u64> {
    let mut distances = vec![u64::MAX; g.len()];
    let mut queue = VecDeque::new();

    queue.push_back(start);
    distances[start] = 0;

    while let Some(v) = queue.pop_front() {
        for &nv in &g[v] {
            if distances[nv] < distances[v] {
                distances[nv] = distances[v].saturating_add(1);
                queue.push_back(nv);
            }
        }
    }

    distances
}
