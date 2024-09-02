mod dijkstra {
    use std::{cmp::Reverse, collections::BinaryHeap};

    pub fn dijkstra(g: &[Vec<(usize, u64)>], start: usize) -> Vec<u64> {
        let mut distances = vec![u64::MAX; g.len()];
        let mut heap = BinaryHeap::new();

        distances[start] = 0;
        heap.push(Reverse((0, start)));

        while let Some(Reverse((distance, v))) = heap.pop() {
            if distance > distances[v] {
                continue;
            }

            for &(nv, d) in &g[v] {
                let new_distance = distance.saturating_add(d);
                if new_distance < distances[nv] {
                    distances[nv] = new_distance;
                    heap.push(Reverse((distances[nv], nv)));
                }
            }
        }

        distances
    }
}
