use std::collections::VecDeque;

/// BFSで最小コストを計算する
/// V: 頂点の型
/// I: 頂点からインデックスへの写像
pub struct Bfs<V, I> {
    /// 頂点数(状態数)
    n: usize,

    /// 始点
    start: V,

    /// 状態からインデックスへの写像
    /// 例: H*Wの2次元グリッドのマス(i, j) -> i*w+j
    to_index: I,

    /// costs[v]: startからvへの最小コスト
    /// 到達不可能である場合はNone
    costs: Vec<Option<usize>>,

    /// previous_vertices[v]: 最短経路においてvの直前に訪問する頂点
    previous_vertices: Vec<Option<V>>,
}

impl<V, I> Bfs<V, I>
where
    I: Fn(&V) -> usize,
{
    /// BFS
    /// to_index: 状態からインデックスへの写像
    /// neighbors: 頂点から1ステップで到達できる頂点のイテレータ
    pub fn new<E>(n: usize, start: &V, to_index: I, mut neighbors: impl FnMut(&V) -> E) -> Self
    where
        V: Clone,
        E: Iterator<Item = V>,
    {
        let mut costs = vec![None; n];
        let mut previous_vertices = vec![None; n];

        let mut queue = VecDeque::new();

        costs[to_index(&start)] = Some(0);
        queue.push_back(start.clone());

        while let Some(v) = queue.pop_front() {
            let index_v = to_index(&v);
            let cost_v = costs[index_v].unwrap();

            for nv in neighbors(&v) {
                let index_nv = to_index(&nv);
                let new_cost_nv = cost_v + 1;

                match costs[index_nv] {
                    Some(_) => {}
                    _ => {
                        costs[index_nv] = Some(new_cost_nv);
                        previous_vertices[index_nv] = Some(v.clone());
                        queue.push_back(nv);
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

    /// 01-bfs
    /// to_index: 状態からインデックスへの写像
    /// neighbors: 頂点から1ステップで到達できる頂点とコストの組のイテレータ(コストは0または1)
    pub fn new_01<E>(n: usize, start: &V, to_index: I, mut neighbors: impl FnMut(&V) -> E) -> Self
    where
        V: Clone,
        E: Iterator<Item = (V, usize)>,
    {
        let mut costs = vec![None; n];
        let mut previous_vertices = vec![None; n];

        let mut queue = VecDeque::new();

        costs[to_index(&start)] = Some(0);
        queue.push_back(start.clone());

        while let Some(v) = queue.pop_front() {
            let index_v = to_index(&v);
            let cost_v = costs[index_v].unwrap();

            for (nv, dcost) in neighbors(&v) {
                let index_nv = to_index(&nv);
                let new_cost_nv = cost_v + dcost;

                match costs[index_nv] {
                    Some(cost_nv) if cost_nv <= new_cost_nv => {}
                    _ => {
                        costs[index_nv] = Some(new_cost_nv);
                        previous_vertices[index_nv] = Some(v.clone());
                        if dcost == 0 {
                            queue.push_front(nv);
                        } else if dcost == 1 {
                            queue.push_back(nv);
                        } else {
                            assert!(false);
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
    pub fn cost(&self, v: &V) -> Option<usize> {
        self.costs[(self.to_index)(v)]
    }

    /// startからの最短経路においてvの直前に訪れる頂点
    /// そのような頂点が存在しなければNone
    pub fn previous(&self, v: &V) -> Option<&V> {
        self.previous_vertices[(self.to_index)(v)].as_ref()
    }

    /// 頂点endへ到達可能ならばendまでの最短経路を構築する
    /// startとendを含む
    pub fn path(&self, end: &V) -> Option<Vec<V>>
    where
        V: Clone,
    {
        if self.costs[(self.to_index)(end)].is_none() {
            return None;
        }

        let mut res = vec![end];

        while let Some(pv) = self.previous(res.last().unwrap()) {
            res.push(pv);
        }

        Some(res.into_iter().rev().cloned().collect())
    }
}

/// 隣接リスト表現のグラフをBFSして最短経路を計算する
pub fn bfs_adjacencies(g: &[Vec<usize>], start: usize) -> Bfs<usize, impl Fn(&usize) -> usize> {
    Bfs::new(g.len(), &start, |&v: &usize| v, |&v| g[v].iter().cloned())
}

#[cfg(test)]
mod tests {
    use super::{bfs_adjacencies, Bfs};

    #[test]
    fn test_bfs() {
        let start = 0;
        let graph = vec![vec![1], vec![2], vec![3], vec![]];
        let expected = vec![Some(0), Some(1), Some(2), Some(3)];
        let bfs = bfs_adjacencies(&graph, start);
        for v in 0..graph.len() {
            assert_eq!(bfs.cost(&v), expected[v]);
        }
        assert_eq!(bfs.path(&3), Some(vec![0, 1, 2, 3]));

        let start = 0;
        let graph = vec![vec![1], vec![2], vec![], vec![4], vec![]];
        let expected = vec![Some(0), Some(1), Some(2), None, None];
        let bfs = bfs_adjacencies(&graph, start);
        for v in 0..graph.len() {
            assert_eq!(bfs.cost(&v), expected[v]);
        }

        let start = 0;
        let graph = vec![vec![1], vec![2], vec![0]];
        let expected = vec![Some(0), Some(1), Some(2)];
        let bfs = bfs_adjacencies(&graph, start);
        for v in 0..graph.len() {
            assert_eq!(bfs.cost(&v), expected[v]);
        }

        let start = 2;
        let graph = vec![vec![1], vec![2], vec![3], vec![]];
        let expected = vec![None, None, Some(0), Some(1)];
        let bfs = bfs_adjacencies(&graph, start);
        for v in 0..graph.len() {
            assert_eq!(bfs.cost(&v), expected[v]);
        }

        let start = 0;
        let graph = vec![vec![1, 2], vec![3], vec![3, 4], vec![5], vec![5], vec![]];
        let expected = vec![Some(0), Some(1), Some(1), Some(2), Some(2), Some(3)];
        let bfs = bfs_adjacencies(&graph, start);
        for v in 0..graph.len() {
            assert_eq!(bfs.cost(&v), expected[v]);
        }
        assert_eq!(bfs.path(&4), Some(vec![0, 2, 4]));
    }

    #[test]
    fn test_bfs_grid() {
        const DIJ: [[usize; 2]; 4] = [
            [1, 0],
            [0, 1],
            [1usize.wrapping_neg(), 0],
            [0, 1usize.wrapping_neg()],
        ];

        let h = 7;
        let w = 8;
        let grid = vec![
            b"########",
            b"#....#.#",
            b"#.######",
            b"#..#...#",
            b"#..##..#",
            b"##.....#",
            b"########",
        ];

        let bfs = Bfs::new(
            h * w,
            &[1, 1],
            |[i, j]| i * w + j,
            |&[i, j]| {
                DIJ.iter()
                    .map(move |&[di, dj]| [i.wrapping_add(di), j.wrapping_add(dj)])
                    .filter(|&[ni, nj]| ni < h && nj < w && grid[ni][nj] == b'.')
            },
        );

        assert_eq!(bfs.cost(&[1, 1]), Some(0));
        assert_eq!(bfs.cost(&[1, 2]), Some(1));
        assert_eq!(bfs.cost(&[1, 4]), Some(3));
        assert_eq!(bfs.cost(&[4, 5]), Some(9));
        assert_eq!(bfs.cost(&[3, 4]), Some(11));
        assert_eq!(bfs.cost(&[1, 6]), None);
        assert_eq!(bfs.cost(&[0, 0]), None);
        assert_eq!(bfs.cost(&[6, 7]), None);
    }

    #[test]
    fn test_01bfs() {
        // TODO:01-BFS特有のテストを追加する
        // 以下はBFSのテストの使い回し

        let start = 0;
        let graph = vec![vec![1], vec![2], vec![3], vec![]];
        let expected = vec![Some(0), Some(1), Some(2), Some(3)];
        let bfs = Bfs::new_01(
            graph.len(),
            &start,
            |&v| v,
            |&v| graph[v].iter().map(|&nv| (nv, 1)),
        );
        for v in 0..graph.len() {
            assert_eq!(bfs.cost(&v), expected[v]);
        }

        let start = 0;
        let graph = vec![vec![1], vec![2], vec![], vec![4], vec![]];
        let expected = vec![Some(0), Some(1), Some(2), None, None];
        let bfs = Bfs::new_01(
            graph.len(),
            &start,
            |&v| v,
            |&v| graph[v].iter().map(|&nv| (nv, 1)),
        );
        for v in 0..graph.len() {
            assert_eq!(bfs.cost(&v), expected[v]);
        }

        let start = 0;
        let graph = vec![vec![1], vec![2], vec![0]];
        let expected = vec![Some(0), Some(1), Some(2)];
        let bfs = Bfs::new_01(
            graph.len(),
            &start,
            |&v| v,
            |&v| graph[v].iter().map(|&nv| (nv, 1)),
        );
        for v in 0..graph.len() {
            assert_eq!(bfs.cost(&v), expected[v]);
        }

        let start = 2;
        let graph = vec![vec![1], vec![2], vec![3], vec![]];
        let expected = vec![None, None, Some(0), Some(1)];
        let bfs = Bfs::new_01(
            graph.len(),
            &start,
            |&v| v,
            |&v| graph[v].iter().map(|&nv| (nv, 1)),
        );
        for v in 0..graph.len() {
            assert_eq!(bfs.cost(&v), expected[v]);
        }

        let start = 0;
        let graph = vec![vec![1, 2], vec![3], vec![3, 4], vec![5], vec![5], vec![]];
        let expected = vec![Some(0), Some(1), Some(1), Some(2), Some(2), Some(3)];
        let bfs = Bfs::new_01(
            graph.len(),
            &start,
            |&v| v,
            |&v| graph[v].iter().map(|&nv| (nv, 1)),
        );
        for v in 0..graph.len() {
            assert_eq!(bfs.cost(&v), expected[v]);
        }
    }
}
