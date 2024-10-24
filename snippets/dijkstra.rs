use std::{cmp::Reverse, collections::BinaryHeap};

pub fn dijkstra(g: &[Vec<(usize, u64)>], start: usize) -> Vec<u64> {
    let n = g.len();

    let mut distances = vec![u64::MAX; n];
    let mut heap = BinaryHeap::new();

    distances[start] = 0;
    heap.push(Reverse((0, start)));

    while let Some(Reverse((distance, v))) = heap.pop() {
        if distances[v] < distance {
            continue;
        }

        for &(nv, delta_distance) in &g[v] {
            let new_distance = distance.saturating_add(delta_distance);

            if distances[nv] <= new_distance {
                continue;
            }
            distances[nv] = new_distance;
            heap.push(Reverse((new_distance, nv)));
        }
    }

    distances
}
