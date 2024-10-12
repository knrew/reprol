use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Dijkstra {
    /// グラフの頂点の個数
    /// 頂点番号は0, 1, ..., len-1
    len: usize,

    /// 始点頂点
    start: usize,

    /// costs[v]: 頂点startからvへの最短経路のコスト
    /// startから到達不可能である場合はNone
    costs: Vec<Option<u64>>,

    /// previous_vertices[v]: 最短経路においてvの直前に訪問する頂点
    previous_vertices: Vec<Option<usize>>,
}

impl Dijkstra {
    /// 隣接リストから最短経路を計算する
    pub fn from_adjacencies(g: &[Vec<(usize, u64)>], start: usize) -> Self {
        Dijkstra::from_mapping(g.len(), start, |&i| g[i].iter().copied())
    }

    /// 頂点を引数にとり隣接頂点リストのイテレータを返す関数neighborsを用いて最短経路を計算する
    pub fn from_mapping<E>(len: usize, start: usize, mut neighbors: impl FnMut(&usize) -> E) -> Self
    where
        E: Iterator<Item = (usize, u64)>,
    {
        debug_assert!(start < len);

        let mut costs = vec![None; len];
        let mut previous_vertices = vec![None; len];

        let mut heap = BinaryHeap::new();

        costs[start] = Some(0u64);
        heap.push(Reverse((0, start)));

        while let Some(Reverse((cost, v))) = heap.pop() {
            if let Some(min_distance) = costs[v] {
                if cost > min_distance {
                    continue;
                }
            }

            for (nv, diff_cost) in neighbors(&v) {
                let new_distance = cost.saturating_add(diff_cost);

                if let Some(min_distance) = costs[nv] {
                    if min_distance <= new_distance {
                        continue;
                    }
                }

                costs[nv] = Some(new_distance);
                previous_vertices[nv] = Some(v);
                heap.push(Reverse((new_distance, nv)));
            }
        }

        Self {
            len,
            start,
            costs,
            previous_vertices,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn costs(&self) -> &[Option<u64>] {
        &self.costs
    }

    pub fn cost(&self, v: usize) -> Option<u64> {
        self.costs[v]
    }

    /// 頂点endへの最短経路を構築する
    pub fn construct_path(&self, end: usize) -> Option<Vec<usize>> {
        if self.costs[end].is_none() {
            return None;
        }

        let mut res = vec![end];

        while let Some(p) = self.previous_vertices[*res.last().unwrap()] {
            res.push(p);
        }

        debug_assert!(res.last().unwrap() == &self.start);

        res.reverse();

        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::dijkstra::Dijkstra;

    #[test]
    fn test_dijkstra() {
        let g = vec![
            vec![(1, 2)],
            vec![(2, 3), (4, 9)],
            vec![(4, 4)],
            vec![(0, 1)],
            vec![],
        ];
        let dijkstra = Dijkstra::from_adjacencies(&g, 0);
        let costs = dijkstra.costs().to_vec();
        assert_eq!(costs, vec![Some(0), Some(2), Some(5), None, Some(9)]);

        let g = vec![vec![(1, 2)], vec![(2, 3)], vec![], vec![(4, 1)], vec![]];
        let dijkstra = Dijkstra::from_adjacencies(&g, 0);
        let costs = dijkstra.costs().to_vec();
        assert_eq!(costs, vec![Some(0), Some(2), Some(5), None, None]);

        let g = vec![vec![(1, 2)], vec![(2, 3)], vec![], vec![(4, 1)], vec![]];
        let dijkstra = Dijkstra::from_adjacencies(&g, 0);
        let costs = dijkstra.costs().to_vec();
        assert_eq!(costs, vec![Some(0), Some(2), Some(5), None, None]);

        let g = vec![
            vec![(1, 10), (2, 1)],
            vec![(3, 2)],
            vec![(1, 1), (3, 4)],
            vec![],
        ];
        let dijkstra = Dijkstra::from_adjacencies(&g, 0);
        let costs = dijkstra.costs().to_vec();
        assert_eq!(costs, vec![Some(0), Some(2), Some(1), Some(4)]);

        let g = vec![vec![(1, 1)], vec![(2, 1)], vec![(0, 1)]];
        let dijkstra = Dijkstra::from_adjacencies(&g, 0);
        let costs = dijkstra.costs().to_vec();
        assert_eq!(costs, vec![Some(0), Some(1), Some(2)]);

        let g = vec![
            vec![(1, 2), (2, 5)],
            vec![(2, 2), (3, 1)],
            vec![(3, 3), (4, 1)],
            vec![(4, 2)],
            vec![],
        ];
        let dijkstra = Dijkstra::from_adjacencies(&g, 0);
        let costs = dijkstra.costs().to_vec();
        assert_eq!(costs, vec![Some(0), Some(2), Some(4), Some(3), Some(5)]);

        let g = vec![
            vec![(1, 2)],
            vec![(2, 3), (4, 9)],
            vec![(4, 4)],
            vec![(0, 1)],
            vec![],
        ];
        let dijkstra = Dijkstra::from_adjacencies(&g, 1);
        let costs = dijkstra.costs().to_vec();
        assert_eq!(costs, vec![None, Some(0), Some(3), None, Some(7)]);
        let dijkstra = Dijkstra::from_adjacencies(&g, 2);
        let costs = dijkstra.costs().to_vec();
        assert_eq!(costs, vec![None, None, Some(0), None, Some(4)]);
        let dijkstra = Dijkstra::from_adjacencies(&g, 3);
        let costs = dijkstra.costs().to_vec();
        assert_eq!(costs, vec![Some(1), Some(3), Some(6), Some(0), Some(10)]);
        let dijkstra = Dijkstra::from_adjacencies(&g, 4);
        let costs = dijkstra.costs().to_vec();
        assert_eq!(costs, vec![None, None, None, None, Some(0)]);
    }
}
