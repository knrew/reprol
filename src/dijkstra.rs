use std::{cmp::Reverse, collections::BinaryHeap};

pub fn dijkstra(g: &[Vec<(usize, u64)>], start: usize) -> Vec<Option<u64>> {
    let n = g.len();

    let mut distances = vec![None; n];
    let mut heap = BinaryHeap::new();

    distances[start] = Some(0u64);
    heap.push(Reverse((0, start)));

    while let Some(Reverse((distance, v))) = heap.pop() {
        if let Some(min_distance) = distances[v] {
            if distance > min_distance {
                continue;
            }
        }

        for &(nv, delta_distance) in &g[v] {
            let new_distance = distance.saturating_add(delta_distance);

            if let Some(min_distance) = distances[nv] {
                if min_distance <= new_distance {
                    continue;
                }
            }

            distances[nv] = Some(new_distance);
            heap.push(Reverse((new_distance, nv)));
        }
    }

    distances
}

#[cfg(test)]
mod tests {
    use super::dijkstra;

    #[test]
    fn test_dijkstra() {
        let g = vec![
            vec![(1, 2)],
            vec![(2, 3), (4, 9)],
            vec![(4, 4)],
            vec![(0, 1)],
            vec![],
        ];
        assert_eq!(
            dijkstra(&g, 0),
            vec![Some(0), Some(2), Some(5), None, Some(9)]
        );

        let g = vec![vec![(1, 2)], vec![(2, 3)], vec![], vec![(4, 1)], vec![]];
        assert_eq!(dijkstra(&g, 0), vec![Some(0), Some(2), Some(5), None, None]);

        let g = vec![vec![(1, 2)], vec![(2, 3)], vec![], vec![(4, 1)], vec![]];
        assert_eq!(dijkstra(&g, 0), vec![Some(0), Some(2), Some(5), None, None]);

        let g = vec![
            vec![(1, 10), (2, 1)],
            vec![(3, 2)],
            vec![(1, 1), (3, 4)],
            vec![],
        ];
        assert_eq!(dijkstra(&g, 0), vec![Some(0), Some(2), Some(1), Some(4)]);

        let g = vec![vec![(1, 1)], vec![(2, 1)], vec![(0, 1)]];
        assert_eq!(dijkstra(&g, 0), vec![Some(0), Some(1), Some(2)]);

        let g = vec![
            vec![(1, 2), (2, 5)],
            vec![(2, 2), (3, 1)],
            vec![(3, 3), (4, 1)],
            vec![(4, 2)],
            vec![],
        ];
        assert_eq!(
            dijkstra(&g, 0),
            vec![Some(0), Some(2), Some(4), Some(3), Some(5)]
        );

        let g = vec![
            vec![(1, 2)],
            vec![(2, 3), (4, 9)],
            vec![(4, 4)],
            vec![(0, 1)],
            vec![],
        ];
        assert_eq!(dijkstra(&g, 1), vec![None, Some(0), Some(3), None, Some(7)]);
        assert_eq!(dijkstra(&g, 2), vec![None, None, Some(0), None, Some(4)]);
        assert_eq!(
            dijkstra(&g, 3),
            vec![Some(1), Some(3), Some(6), Some(0), Some(10)]
        );
        assert_eq!(dijkstra(&g, 4), vec![None, None, None, None, Some(0)]);
    }
}
