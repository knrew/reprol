//! 幅優先探索(Breadth First Search, BFS)
//!
//! BFSで単一始点最短経路を計算する．
//! 01-BFSも実装．
//!
//! # 使用例
//! ## グラフ上のBFS
//! 以下の無向グラフの頂点0を始点とした最短距離を計算する．
//! ```txt
//!  [0] ─ [1]
//!   |     |
//!  [2] - [3] - [4]
//! ```
//!
//! ```
//! use reprol::bfs::Bfs;
//! let graph = vec![
//!     vec![1, 2],
//!     vec![0, 3],
//!     vec![0, 3],
//!     vec![1, 2, 4],
//!     vec![3],
//! ];
//! let bfs = Bfs::new(
//!     graph.len(),
//!     &0,
//!     |&v| v,
//!     |&v| graph[v].iter().cloned(),
//! );
//! assert_eq!(bfs.cost(&0), Some(0));
//! assert_eq!(bfs.cost(&1), Some(1));
//! assert_eq!(bfs.cost(&2), Some(1));
//! assert_eq!(bfs.cost(&3), Some(2));
//! assert_eq!(bfs.cost(&4), Some(3));
//! ```
//!
//! ## 2次元グリッド上のBFS
//! ```
//! use reprol::bfs::Bfs;
//!
//! let h = 5;
//! let w = 5;
//! let grid = vec![
//!     b"#####",
//!     b"#...#",
//!     b"#.###",
//!     b"#.#.#",
//!     b"#####"
//! ];
//!
//! let bfs = Bfs::new(
//!     h * w,
//!     &[1, 1],
//!     |[i, j]| i * w + j,
//!     |&[i, j]| {
//!         [
//!             [0, 1],
//!             [1, 0],
//!             [0, 1usize.wrapping_neg()],
//!             [1usize.wrapping_neg(), 0],
//!         ]
//!         .into_iter()
//!         .map(move |[di, dj]| [i.wrapping_add(di), j.wrapping_add(dj)])
//!         .filter(|&[ni, nj]| ni < h && nj < w && grid[ni][nj] == b'.')
//!     },
//! );
//!
//! assert_eq!(bfs.cost(&[1, 1]), Some(0));
//! assert_eq!(bfs.cost(&[3, 1]), Some(2));
//! assert_eq!(bfs.cost(&[0, 0]), None);
//! assert_eq!(bfs.cost(&[3, 3]), None);
//! ```
//! ## 01-BFS
//! 辺のコストが0または1である重み付きグラフに対して，
//! 01-BFSで頂点0を始点とする最短距離を計算する．
//! ```
//! use reprol::bfs::Bfs;
//! let graph = vec![
//!     vec![(1, 1), (3, 0)],
//!     vec![(0, 1), (2, 1)],
//!     vec![(1, 1), (4, 1)],
//!     vec![(0, 0), (4, 0)],
//!     vec![(2, 1), (3, 0)],
//! ];
//! let bfs = Bfs::new_01(
//!     graph.len(),
//!     &0,
//!     |&v| v,
//!     |&v| graph[v].iter().cloned(),
//! );
//! assert_eq!(bfs.cost(&0), Some(0));
//! assert_eq!(bfs.cost(&1), Some(1));
//! assert_eq!(bfs.cost(&2), Some(1));
//! assert_eq!(bfs.cost(&3), Some(0));
//! assert_eq!(bfs.cost(&4), Some(0));
//! ```

use std::{collections::VecDeque, fmt::Debug};

/// 経路情報を管理するためのトレイト．
pub trait PathTracker<V> {
    fn new(n: usize) -> Self;

    /// `index`の直前の頂点を返す．
    fn get_previous(&self, index: usize) -> Option<&V>;

    /// `index`の直前の頂点を`v`に更新する．
    fn set_previous(&mut self, index: usize, v: &V);

    /// 始点から`end`までの経路を構築する．
    fn construct_path(
        &self,
        to_index: &impl Fn(&V) -> usize,
        costs: &[Option<usize>],
        end: &V,
    ) -> Option<Vec<V>>;
}

/// 経路を保存する場合に用いる構造体．
/// 各頂点の直前の頂点を保存する．
pub struct WithPath<V> {
    previous: Vec<Option<V>>,
}

impl<V: Clone> PathTracker<V> for WithPath<V> {
    fn new(n: usize) -> Self {
        Self {
            previous: vec![None; n],
        }
    }

    fn get_previous(&self, index: usize) -> Option<&V> {
        self.previous[index].as_ref()
    }

    fn set_previous(&mut self, index: usize, value: &V) {
        self.previous[index] = Some(value.clone());
    }

    fn construct_path(
        &self,
        to_index: &impl Fn(&V) -> usize,
        costs: &[Option<usize>],
        end: &V,
    ) -> Option<Vec<V>> {
        costs[to_index(end)]?;

        let mut v = end;
        let mut path = vec![v];

        while let Some(pv) = self.previous[to_index(v)].as_ref() {
            path.push(pv);
            v = pv;
        }

        Some(path.into_iter().rev().cloned().collect())
    }
}

/// 経路を保存しない場合に用いる構造体(ダミー)．
pub struct NoPath;

impl<V> PathTracker<V> for NoPath {
    #[inline(always)]
    fn new(_: usize) -> Self {
        Self
    }

    #[inline(always)]
    fn get_previous(&self, _: usize) -> Option<&V> {
        None
    }

    #[inline(always)]
    fn set_previous(&mut self, _: usize, _: &V) {}

    #[inline(always)]
    fn construct_path(
        &self,
        _: &impl Fn(&V) -> usize,
        _: &[Option<usize>],
        _: &V,
    ) -> Option<Vec<V>> {
        None
    }
}

/// BFSの本体．
///
/// - `V`: 頂点の型
/// - `I`: `V`をインデックス(`usize`)に変換する関数
/// - `P`: 経路情報に関する構造体(`WithPath` または `NoPath`)
pub struct BfsImpl<V, I, P> {
    start: V,
    to_index: I,
    costs: Vec<Option<usize>>,
    path_tracker: P,
}

impl<V, I, P> BfsImpl<V, I, P>
where
    V: Clone,
    I: Fn(&V) -> usize,
    P: PathTracker<V>,
{
    /// BFSで最短経路を計算する．
    ///
    /// - `n`: 頂点数
    /// - `start`: 始点
    /// - `to_index`: 頂点をインデックス(`[0, n)`)に変換する関数
    /// - `neighbors`: 頂点`v`の隣接頂点のイテレータを返す関数
    pub fn new<E>(n: usize, start: &V, to_index: I, mut neighbors: impl FnMut(&V) -> E) -> Self
    where
        E: Iterator<Item = V>,
    {
        let mut costs = vec![None; n];
        let mut path_tracker = P::new(n);

        let mut queue = VecDeque::new();

        costs[to_index(start)] = Some(0);
        queue.push_back(start.clone());

        while let Some(v) = queue.pop_front() {
            let index_v = to_index(&v);
            let cost_v = costs[index_v].unwrap();

            for nv in neighbors(&v) {
                let index_nv = to_index(&nv);
                let new_cost_nv = cost_v + 1;

                if costs[index_nv].is_none() {
                    costs[index_nv] = Some(new_cost_nv);
                    path_tracker.set_previous(index_nv, &v);
                    queue.push_back(nv);
                }
            }
        }

        Self {
            start: start.clone(),
            to_index,
            costs,
            path_tracker,
        }
    }

    /// 01-BFSで最短経路を計算する．
    ///
    /// 辺のコストは0または1とする．
    ///
    /// - `n`: 頂点数
    /// - `start`: 始点
    /// - `to_index`: 頂点をインデックス(`[0, n)`)に変換する関数
    /// - `neighbors`: 頂点`v`の(隣接頂点, 辺のコスト)のイテレータを返す関数
    pub fn new_01<E>(n: usize, start: &V, to_index: I, mut neighbors: impl FnMut(&V) -> E) -> Self
    where
        E: Iterator<Item = (V, usize)>,
    {
        let mut costs = vec![None; n];
        let mut path_tracker = P::new(n);

        let mut queue = VecDeque::new();

        costs[to_index(start)] = Some(0);
        queue.push_back(start.clone());

        while let Some(v) = queue.pop_front() {
            let index_v = to_index(&v);
            let cost_v = costs[index_v].unwrap();

            for (nv, dcost) in neighbors(&v) {
                assert!(dcost <= 1);
                let index_nv = to_index(&nv);
                let new_cost_nv = cost_v + dcost;

                if !costs[index_nv].is_some_and(|cost_nv| cost_nv <= new_cost_nv) {
                    costs[index_nv] = Some(new_cost_nv);
                    path_tracker.set_previous(index_nv, &v);
                    if dcost == 0 {
                        queue.push_front(nv);
                    } else {
                        queue.push_back(nv);
                    }
                }
            }
        }

        Self {
            start: start.clone(),
            to_index,
            costs,
            path_tracker,
        }
    }

    /// 始点．
    pub fn start(&self) -> &V {
        &self.start
    }

    /// 始点から`v`へのコストを返す．
    pub fn cost(&self, v: &V) -> Option<usize> {
        self.costs[(self.to_index)(v)]
    }
}

impl<V, I> BfsImpl<V, I, WithPath<V>>
where
    V: Clone,
    I: Fn(&V) -> usize,
{
    /// `v`の直前の頂点を返す．
    pub fn previous(&self, v: &V) -> Option<&V> {
        self.path_tracker.get_previous((self.to_index)(v))
    }

    /// 始点から`end`までの経路を構築する．
    pub fn path(&self, end: &V) -> Option<Vec<V>> {
        self.path_tracker
            .construct_path(&self.to_index, &self.costs, end)
    }
}

impl<V, I, P> Debug for BfsImpl<V, I, P>
where
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bfs")
            .field("start", &self.start)
            .field("costs", &self.costs)
            .finish()
    }
}

/// BFS(経路復元なし)
pub type Bfs<V, I> = BfsImpl<V, I, NoPath>;

/// BFS(経路復元あり)
pub type BfsWithPath<V, I> = BfsImpl<V, I, WithPath<V>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_no_path() {
        {
            let start = 0;
            let graph = vec![vec![1], vec![2], vec![3], vec![]];
            let expected = vec![Some(0), Some(1), Some(2), Some(3)];
            let bfs = Bfs::new(graph.len(), &start, |&v| v, |&v| graph[v].iter().cloned());
            for v in 0..graph.len() {
                assert_eq!(bfs.cost(&v), expected[v]);
            }
        }

        {
            let start = 0;
            let graph = vec![vec![1], vec![2], vec![], vec![4], vec![]];
            let expected = vec![Some(0), Some(1), Some(2), None, None];
            let bfs = Bfs::new(graph.len(), &start, |&v| v, |&v| graph[v].iter().cloned());
            for v in 0..graph.len() {
                assert_eq!(bfs.cost(&v), expected[v]);
            }
        }

        {
            let start = 0;
            let graph = vec![vec![1], vec![2], vec![0]];
            let expected = vec![Some(0), Some(1), Some(2)];
            let bfs = Bfs::new(graph.len(), &start, |&v| v, |&v| graph[v].iter().cloned());
            for v in 0..graph.len() {
                assert_eq!(bfs.cost(&v), expected[v]);
            }
        }

        {
            let start = 2;
            let graph = vec![vec![1], vec![2], vec![3], vec![]];
            let expected = vec![None, None, Some(0), Some(1)];
            let bfs = Bfs::new(graph.len(), &start, |&v| v, |&v| graph[v].iter().cloned());
            for v in 0..graph.len() {
                assert_eq!(bfs.cost(&v), expected[v]);
            }
        }

        {
            let start = 0;
            let graph = vec![vec![1, 2], vec![3], vec![3, 4], vec![5], vec![5], vec![]];
            let expected = vec![Some(0), Some(1), Some(1), Some(2), Some(2), Some(3)];
            let bfs = Bfs::new(graph.len(), &start, |&v| v, |&v| graph[v].iter().cloned());
            for v in 0..graph.len() {
                assert_eq!(bfs.cost(&v), expected[v]);
            }
        }

        {
            let start = 0;
            let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![2, 0]];
            let expected = vec![Some(0), Some(1), Some(2), Some(1)];
            let bfs = Bfs::new(graph.len(), &start, |&v| v, |&v| graph[v].iter().cloned());
            for v in 0..graph.len() {
                assert_eq!(bfs.cost(&v), expected[v]);
            }
        }
    }

    #[test]
    fn test_bfs_with_path() {
        {
            let start = 0;
            let graph = vec![vec![1], vec![2], vec![3], vec![]];
            let expected = vec![Some(0), Some(1), Some(2), Some(3)];
            let bfs = BfsWithPath::new(graph.len(), &start, |&v| v, |&v| graph[v].iter().cloned());
            for v in 0..graph.len() {
                assert_eq!(bfs.cost(&v), expected[v]);
            }
            assert_eq!(bfs.path(&3), Some(vec![0, 1, 2, 3]));
        }

        {
            let start = 0;
            let graph = vec![vec![1, 2], vec![3], vec![3, 4], vec![5], vec![5], vec![]];
            let expected = vec![Some(0), Some(1), Some(1), Some(2), Some(2), Some(3)];
            let bfs = BfsWithPath::new(graph.len(), &start, |&v| v, |&v| graph[v].iter().cloned());
            for v in 0..graph.len() {
                assert_eq!(bfs.cost(&v), expected[v]);
            }
            assert_eq!(bfs.path(&4), Some(vec![0, 2, 4]));
        }

        {
            // 0 - 1 - 2
            //     |
            //     3 - 4
            let start = 0;
            let graph = vec![vec![1], vec![0, 2, 3], vec![1], vec![1, 4], vec![3]];
            let expected = vec![
                // (cost, path)
                (Some(0), Some(vec![0])),
                (Some(1), Some(vec![0, 1])),
                (Some(2), Some(vec![0, 1, 2])),
                (Some(2), Some(vec![0, 1, 3])),
                (Some(3), Some(vec![0, 1, 3, 4])),
            ];
            let bfs = BfsWithPath::new(graph.len(), &start, |&v| v, |&v| graph[v].iter().cloned());
            for (v, (cost, path)) in expected.into_iter().enumerate() {
                assert_eq!(bfs.cost(&v), cost);
                assert_eq!(bfs.path(&v), path);
            }
        }

        {
            let graph = vec![vec![1], vec![0, 2], vec![1], vec![]];
            let bfs = BfsWithPath::new(graph.len(), &0, |&v| v, |&v| graph[v].iter().cloned());
            assert_eq!(bfs.path(&3), None);
            assert_eq!(bfs.cost(&3), None);
        }
    }

    #[test]
    fn test_bfs_grid() {
        const DIJ: [[usize; 2]; 4] = [
            [1, 0],
            [0, 1],
            [1usize.wrapping_neg(), 0],
            [0, 1usize.wrapping_neg()],
        ];

        {
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
            let start = [1, 1];

            let bfs = Bfs::new(
                h * w,
                &start,
                |[i, j]| i * w + j,
                |&[i, j]| {
                    DIJ.into_iter()
                        .map(move |[di, dj]| [i.wrapping_add(di), j.wrapping_add(dj)])
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

        {
            let h = 3;
            let w = 4;
            let grid = vec![
                vec!['.', '.', '#', '.'],
                vec!['.', '.', '#', '.'],
                vec!['#', '.', '.', '.'],
            ];
            let start = [0, 0];

            let bfs = Bfs::new(
                h * w,
                &start,
                |[i, j]| i * w + j,
                |&[i, j]| {
                    DIJ.into_iter()
                        .map(move |[di, dj]| [i.wrapping_add(di), j.wrapping_add(dj)])
                        .filter(|&[ni, nj]| ni < h && nj < w && grid[ni][nj] == '.')
                },
            );

            let expected = vec![
                vec![Some(0), Some(1), None, Some(7)],
                vec![Some(1), Some(2), None, Some(6)],
                vec![None, Some(3), Some(4), Some(5)],
            ];

            for i in 0..h {
                for j in 0..w {
                    assert_eq!(bfs.cost(&[i, j]), expected[i][j]);
                }
            }
        }
    }

    #[test]
    fn test_01bfs() {
        {
            let start = 0;
            let graph = vec![vec![(1, 1), (3, 0)], vec![(2, 1)], vec![], vec![(1, 1)]];
            let expected = vec![Some(0), Some(1), Some(2), Some(0)];
            let bfs = Bfs::new_01(graph.len(), &start, |&v| v, |&v| graph[v].iter().cloned());
            for v in 0..graph.len() {
                assert_eq!(bfs.cost(&v), expected[v]);
            }
        }

        {
            let start = 0;
            let graph = vec![vec![(1, 1)], vec![], vec![]];
            let bfs = Bfs::new_01(graph.len(), &start, |&v| v, |&v| graph[v].iter().cloned());
            assert_eq!(bfs.cost(&0), Some(0));
            assert_eq!(bfs.cost(&1), Some(1));
            assert_eq!(bfs.cost(&2), None);
        }

        {
            // 0 - 1 - 2
            //     |
            //     3 - 4
            let start = 0;
            let graph = vec![vec![1], vec![0, 2, 3], vec![1], vec![1, 4], vec![3]];
            let expected = vec![Some(0), Some(1), Some(2), Some(2), Some(3)];
            let bfs = Bfs::new_01(
                graph.len(),
                &start,
                |&v| v,
                |&v| graph[v].iter().cloned().map(|nv| (nv, 1)),
            );
            for v in 0..graph.len() {
                assert_eq!(bfs.cost(&v), expected[v]);
            }
        }
    }
}
