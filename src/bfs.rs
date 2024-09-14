use std::collections::VecDeque;

pub fn bfs(g: &[Vec<usize>], start: usize) -> Vec<Option<u64>> {
    let mut distances = vec![None; g.len()];
    let mut queue = VecDeque::new();

    queue.push_back(start);
    distances[start] = Some(0u64);

    while let Some(v) = queue.pop_front() {
        for &nv in &g[v] {
            if distances[nv].is_none() {
                distances[nv] = Some(distances[v].unwrap() + 1);
                queue.push_back(nv);
            }
        }
    }

    distances
}

pub fn bfs2<V, E>(
    n: usize,
    start: V,
    mut hash: impl FnMut(&V) -> usize,
    mut neighbors: impl FnMut(&V) -> E,
) -> Vec<Option<u64>>
where
    V: Ord,
    E: Iterator<Item = V>,
{
    let mut distances = vec![None; n];
    let mut queue = VecDeque::new();

    distances[hash(&start)] = Some(0u64);
    queue.push_back(start);

    while let Some(v) = queue.pop_front() {
        let d = distances[hash(&v)].unwrap();
        for nv in neighbors(&v) {
            let nvi = hash(&nv);
            if distances[nvi].is_none() {
                distances[nvi] = Some(d + 1);
                queue.push_back(nv);
            }
        }
    }

    distances
}
