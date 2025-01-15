use std::{collections::VecDeque, ops::Add};

/// 01BFSで最小コストを計算する
/// V: 頂点の型
/// C: コストの型
/// I: 頂点からインデックスへの写像
pub struct Bfs01<V, C, I> {
    /// 頂点数(状態数)
    n: usize,

    /// 始点
    start: V,

    /// 状態からインデックスへの写像
    /// 例: H*Wの2次元グリッドのマス(i, j) -> i*w+j
    to_index: I,

    /// costs[v]: startからvへの最小コスト
    /// 到達不可能である場合はNone
    costs: Vec<Option<C>>,

    /// previous_vertices[v]: 最短経路においてvの直前に訪問する頂点
    previous_vertices: Vec<Option<V>>,
}

impl<V, C, I> Bfs01<V, C, I>
where
    V: Clone,
    C: Copy + PartialOrd + Add<Output = C>,
    I: Fn(&V) -> usize,
{
    /// to_index: 状態からインデックスへの写像
    /// neighbors: 頂点から1ステップで到達できる頂点のイテレータ
    pub fn new<E>(
        n: usize,
        start: &V,
        zero: C,
        to_index: I,
        mut neighbors: impl FnMut(&V) -> E,
    ) -> Self
    where
        E: Iterator<Item = (V, C)>,
    {
        let mut costs = vec![None; n];
        let mut previous_vertices = vec![None; n];

        let mut queue = VecDeque::new();

        costs[to_index(&start)] = Some(zero);
        queue.push_back(start.clone());

        while let Some(v) = queue.pop_front() {
            let index_v = to_index(&v);

            for (nv, dcost) in neighbors(&v) {
                let index_nv = to_index(&nv);

                match costs[index_nv] {
                    Some(cost_nv) if cost_nv <= costs[index_v].unwrap() + dcost => {}
                    _ => {
                        costs[index_nv] = Some(costs[index_v].unwrap() + dcost);
                        previous_vertices[index_nv] = Some(v.clone());
                        if dcost == zero {
                            queue.push_front(nv);
                        } else {
                            queue.push_back(nv);
                        }
                    }
                }
            }
        }

        Self {
            n,
            start: start.clone(),
            to_index,
            costs,
            previous_vertices,
        }
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn start(&self) -> &V {
        &self.start
    }

    /// start->vの最小コスト
    /// startからvへ到達不可能であればNone
    pub fn cost(&self, v: &V) -> &Option<C> {
        &self.costs[(self.to_index)(&v)]
    }

    /// 頂点endへの最短経路を構築する
    /// startとendを含む
    pub fn costruct_path(&self, end: &V) -> Option<Vec<V>> {
        if self.costs[(self.to_index)(end)].is_none() {
            return None;
        }

        let mut res = vec![end.clone()];

        while let Some(p) = {
            let id = (self.to_index)(res.last().unwrap());
            &self.previous_vertices[id]
        } {
            res.push(p.clone());
        }

        res.reverse();

        Some(res)
    }
}

/// 隣接リスト表現のグラフをBFSして最短経路を計算する
pub fn bfs01_adjacencies(
    g: &[Vec<usize>],
    start: usize,
) -> Bfs01<usize, u64, impl Fn(&usize) -> usize> {
    Bfs01::new(
        g.len(),
        &start,
        0,
        |&v: &usize| v,
        |&v| g[v].iter().map(|&v| (v, 1)),
    )
}

// TODO: パス復元のテストを書く
#[cfg(test)]
mod tests {
    use super::bfs01_adjacencies;

    #[test]
    fn test_bfs() {
        let start = 0;
        let graph = vec![vec![1], vec![2], vec![3], vec![]];
        let expected = vec![Some(0), Some(1), Some(2), Some(3)];
        let bfs = bfs01_adjacencies(&graph, start);
        for v in 0..graph.len() {
            assert_eq!(*bfs.cost(&v), expected[v]);
        }

        let start = 0;
        let graph = vec![vec![1], vec![2], vec![], vec![4], vec![]];
        let expected = vec![Some(0), Some(1), Some(2), None, None];
        let bfs = bfs01_adjacencies(&graph, start);
        for v in 0..graph.len() {
            assert_eq!(*bfs.cost(&v), expected[v]);
        }

        let start = 0;
        let graph = vec![vec![1], vec![2], vec![0]];
        let expected = vec![Some(0), Some(1), Some(2)];
        let bfs = bfs01_adjacencies(&graph, start);
        for v in 0..graph.len() {
            assert_eq!(*bfs.cost(&v), expected[v]);
        }

        let start = 2;
        let graph = vec![vec![1], vec![2], vec![3], vec![]];
        let expected = vec![None, None, Some(0), Some(1)];
        let bfs = bfs01_adjacencies(&graph, start);
        for v in 0..graph.len() {
            assert_eq!(*bfs.cost(&v), expected[v]);
        }

        let start = 0;
        let graph = vec![vec![1, 2], vec![3], vec![3, 4], vec![5], vec![5], vec![]];
        let expected = vec![Some(0), Some(1), Some(1), Some(2), Some(2), Some(3)];
        let bfs = bfs01_adjacencies(&graph, start);
        for v in 0..graph.len() {
            assert_eq!(*bfs.cost(&v), expected[v]);
        }
    }
}
