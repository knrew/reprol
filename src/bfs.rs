use std::collections::VecDeque;

/// BFSで最短経路を計算する
pub struct Bfs {
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

impl Bfs {
    /// 隣接リストから最短経路を計算する
    pub fn from_adjacencies(g: &[Vec<usize>], start: usize) -> Self {
        Bfs::from_mapping(g.len(), start, |&i| g[i].iter().copied())
    }

    /// 頂点を引数にとり隣接頂点リストのイテレータを返す関数neighborsを用いて最短経路を計算する
    pub fn from_mapping<E>(len: usize, start: usize, mut neighbors: impl FnMut(&usize) -> E) -> Self
    where
        E: Iterator<Item = usize>,
    {
        debug_assert!(start < len);

        let mut costs = vec![None; len];
        let mut previous_vertices = vec![None; len];

        let mut queue = VecDeque::new();

        costs[start] = Some(0u64);
        queue.push_back(start);

        while let Some(v) = queue.pop_front() {
            for nv in neighbors(&v) {
                if costs[nv].is_some() {
                    continue;
                }
                costs[nv] = Some(costs[v].unwrap().saturating_add(1));
                previous_vertices[nv] = Some(v);
                queue.push_back(nv);
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
    use super::Bfs;

    #[test]
    fn test_bfs() {
        let graph = vec![vec![1], vec![2], vec![3], vec![]];
        let costs = Bfs::from_adjacencies(&graph, 0).costs().to_vec();
        assert_eq!(costs, vec![Some(0), Some(1), Some(2), Some(3)]);

        let graph = vec![vec![1], vec![2], vec![], vec![4], vec![]];
        let costs = Bfs::from_adjacencies(&graph, 0).costs().to_vec();
        assert_eq!(costs, vec![Some(0), Some(1), Some(2), None, None]);

        let graph = vec![vec![1], vec![2], vec![0]];
        let costs = Bfs::from_adjacencies(&graph, 0).costs().to_vec();
        assert_eq!(costs, vec![Some(0), Some(1), Some(2)]);

        let graph = vec![vec![1], vec![2], vec![3], vec![]];
        let costs = Bfs::from_adjacencies(&graph, 2).costs().to_vec();
        assert_eq!(costs, vec![None, None, Some(0), Some(1)]);

        let graph = vec![vec![1, 2], vec![3], vec![3, 4], vec![5], vec![5], vec![]];
        let costs = Bfs::from_adjacencies(&graph, 0).costs().to_vec();
        assert_eq!(
            costs,
            vec![Some(0), Some(1), Some(1), Some(2), Some(2), Some(3)]
        );
    }
}
