use std::{collections::VecDeque, ops::Add};

/// BFSで最小コストを計算する
/// V: 頂点の型
/// C: コストの型
/// I: 頂点からインデックスへの写像
pub struct Bfs<V, C, I> {
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

impl<V, C, I> Bfs<V, C, I>
where
    V: Clone,
    C: Clone + Add<Output = C>,
    I: Fn(&V) -> usize,
{
    /// to_index: 状態からインデックスへの写像
    /// neighbors: 頂点から1ステップで到達できる頂点のイテレータ
    pub fn new<E>(
        n: usize,
        start: &V,
        zero: &C,
        one: &C,
        to_index: I,
        mut neighbors: impl FnMut(&V) -> E,
    ) -> Self
    where
        E: Iterator<Item = V>,
    {
        let mut costs = vec![None; n];
        let mut previous_vertices = vec![None; n];

        let mut queue = VecDeque::new();

        costs[to_index(&start)] = Some(zero.clone());
        queue.push_back(start.clone());

        while let Some(v) = queue.pop_front() {
            let new_cost = (&costs[to_index(&v)]).clone().unwrap() + one.clone();
            for nv in neighbors(&v) {
                if costs[to_index(&nv)].is_some() {
                    continue;
                }

                costs[to_index(&nv)] = Some(new_cost.clone());
                previous_vertices[to_index(&nv)] = Some(v.clone());
                queue.push_back(nv);
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

    /// 頂点endへ到達可能ならばendまでの最短経路を構築する
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
pub fn bfs_adjacencies(
    g: &[Vec<usize>],
    start: usize,
) -> Bfs<usize, u64, impl Fn(&usize) -> usize> {
    Bfs::new(
        g.len(),
        &start,
        &0,
        &1,
        |&v: &usize| v,
        |&v| g[v].iter().cloned(),
    )
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
            assert_eq!(*bfs.cost(&v), expected[v]);
        }
        assert_eq!(bfs.costruct_path(&3), Some(vec![0, 1, 2, 3]));

        let start = 0;
        let graph = vec![vec![1], vec![2], vec![], vec![4], vec![]];
        let expected = vec![Some(0), Some(1), Some(2), None, None];
        let bfs = bfs_adjacencies(&graph, start);
        for v in 0..graph.len() {
            assert_eq!(*bfs.cost(&v), expected[v]);
        }

        let start = 0;
        let graph = vec![vec![1], vec![2], vec![0]];
        let expected = vec![Some(0), Some(1), Some(2)];
        let bfs = bfs_adjacencies(&graph, start);
        for v in 0..graph.len() {
            assert_eq!(*bfs.cost(&v), expected[v]);
        }

        let start = 2;
        let graph = vec![vec![1], vec![2], vec![3], vec![]];
        let expected = vec![None, None, Some(0), Some(1)];
        let bfs = bfs_adjacencies(&graph, start);
        for v in 0..graph.len() {
            assert_eq!(*bfs.cost(&v), expected[v]);
        }

        let start = 0;
        let graph = vec![vec![1, 2], vec![3], vec![3, 4], vec![5], vec![5], vec![]];
        let expected = vec![Some(0), Some(1), Some(1), Some(2), Some(2), Some(3)];
        let bfs = bfs_adjacencies(&graph, start);
        for v in 0..graph.len() {
            assert_eq!(*bfs.cost(&v), expected[v]);
        }
        assert_eq!(bfs.costruct_path(&4), Some(vec![0, 2, 4]));
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
            &0,
            &1,
            |[i, j]| i * w + j,
            |&[i, j]| {
                DIJ.iter()
                    .map(move |&[di, dj]| [i.wrapping_add(di), j.wrapping_add(dj)])
                    .filter(|&[ni, nj]| ni < h && nj < w && grid[ni][nj] == b'.')
            },
        );

        assert_eq!(bfs.cost(&[1, 1]), &Some(0));
        assert_eq!(bfs.cost(&[1, 2]), &Some(1));
        assert_eq!(bfs.cost(&[1, 4]), &Some(3));
        assert_eq!(bfs.cost(&[4, 5]), &Some(9));
        assert_eq!(bfs.cost(&[3, 4]), &Some(11));
        assert_eq!(bfs.cost(&[1, 6]), &None);
        assert_eq!(bfs.cost(&[0, 0]), &None);
        assert_eq!(bfs.cost(&[6, 7]), &None);
    }
}
