//! 幅優先探索 / Breadth First Search(BFS)
//!
//! BFSで単一始点最短経路を計算する．
//! 01-BFSも実装．
//!
//! # 使い方
//! ```
//! use reprol::bfs::Bfs;
//! let n = 5;
//! let start = 0;
//! let graph = vec![vec![1], vec![2], vec![3], vec![], vec![]];
//! let bfs = Bfs::new(n, &start, |&v| v, |&v| graph[v].iter().cloned());
//! assert_eq!(bfs.cost(&0), Some(0));
//! assert_eq!(bfs.cost(&1), Some(1));
//! assert_eq!(bfs.cost(&2), Some(2));
//! assert_eq!(bfs.cost(&3), Some(3));
//! assert_eq!(bfs.cost(&4), None);
//! ```
//!
//! # NOTE
//! - コードの重複が多いかも

use std::{collections::VecDeque, fmt::Debug};

/// 経路復元用の構造体．
/// 各頂点の直前の頂点を記録する．
pub struct WithPath<V> {
    previous: Vec<Option<V>>,
}

/// 経路復元を行わない場合に用いる構造体．
pub struct NoPath;

/// BFSの結果を保持する構造体．
///
/// - `V`: 頂点の型
/// - `I`: `V`を`usize`に変換する関数
/// - `P`: 経路情報に関する構造体(`WithPath` または `NoPath`)
pub struct Bfs<V, I, P> {
    /// 始点
    start: V,

    /// 頂点をインデックスに変換する関数
    to_index: I,

    /// 各頂点のコスト
    costs: Vec<Option<usize>>,

    /// 経路情報
    previous: P,
}

/// 経路復元を行わない場合の実装
impl<V, I> Bfs<V, I, NoPath>
where
    V: Clone,
    I: Fn(&V) -> usize,
{
    /// BFSを用いて最短経路を計算する．
    /// 経路復元は行わない．
    pub fn new<E>(n: usize, start: &V, to_index: I, mut neighbors: impl FnMut(&V) -> E) -> Self
    where
        E: Iterator<Item = V>,
    {
        let mut costs = vec![None; n];

        let mut que = VecDeque::new();

        costs[to_index(&start)] = Some(0);
        que.push_back(start.clone());

        while let Some(v) = que.pop_front() {
            let index_v = to_index(&v);
            let cost_v = costs[index_v].unwrap();

            for nv in neighbors(&v) {
                let index_nv = to_index(&nv);
                let new_cost_nv = cost_v + 1;

                match costs[index_nv] {
                    Some(_) => {}
                    _ => {
                        costs[index_nv] = Some(new_cost_nv);
                        que.push_back(nv);
                    }
                }
            }
        }

        Self {
            start: start.clone(),
            to_index,
            costs,
            previous: NoPath,
        }
    }

    /// 01-BFSを用いて最短経路を計算する．
    /// 辺のコストは0または1とする．
    /// 経路復元は行わない．
    pub fn new_01<E>(n: usize, start: &V, to_index: I, mut neighbors: impl FnMut(&V) -> E) -> Self
    where
        E: Iterator<Item = (V, usize)>,
    {
        let mut costs = vec![None; n];

        let mut queue = VecDeque::new();

        costs[to_index(&start)] = Some(0);
        queue.push_back(start.clone());

        while let Some(v) = queue.pop_front() {
            let index_v = to_index(&v);
            let cost_v = costs[index_v].unwrap();

            for (nv, dcost) in neighbors(&v) {
                assert!(dcost <= 1);
                let index_nv = to_index(&nv);
                let new_cost_nv = cost_v + dcost;

                match costs[index_nv] {
                    Some(cost_nv) if cost_nv <= new_cost_nv => {}
                    _ => {
                        costs[index_nv] = Some(new_cost_nv);
                        if dcost == 0 {
                            queue.push_front(nv);
                        } else {
                            queue.push_back(nv);
                        }
                    }
                }
            }
        }

        Self {
            start: start.clone(),
            to_index,
            costs,
            previous: NoPath,
        }
    }
}

/// 経路復元を行う場合の実装
impl<V, I> Bfs<V, I, WithPath<V>>
where
    V: Clone,
    I: Fn(&V) -> usize,
{
    /// BFSを用いて最短経路を計算する．
    /// 経路復元可能．
    pub fn new_with_path<E>(
        n: usize,
        start: &V,
        to_index: I,
        mut neighbors: impl FnMut(&V) -> E,
    ) -> Self
    where
        E: Iterator<Item = V>,
    {
        let mut costs = vec![None; n];
        let mut previous = vec![None; n];

        let mut que = VecDeque::new();

        costs[to_index(&start)] = Some(0);
        que.push_back(start.clone());

        while let Some(v) = que.pop_front() {
            let index_v = to_index(&v);
            let cost_v = costs[index_v].unwrap();

            for nv in neighbors(&v) {
                let index_nv = to_index(&nv);
                let new_cost_nv = cost_v + 1;

                match costs[index_nv] {
                    Some(_) => {}
                    _ => {
                        costs[index_nv] = Some(new_cost_nv);
                        previous[index_nv] = Some(v.clone());
                        que.push_back(nv);
                    }
                }
            }
        }

        Self {
            start: start.clone(),
            to_index,
            costs,
            previous: WithPath { previous },
        }
    }

    /// 01-BFSを用いて最短経路を計算する．
    /// 辺のコストは0または1とする．
    /// 経路復元可能．
    pub fn new_01_with_path<E>(
        n: usize,
        start: &V,
        to_index: I,
        mut neighbors: impl FnMut(&V) -> E,
    ) -> Self
    where
        E: Iterator<Item = (V, usize)>,
    {
        let mut costs = vec![None; n];
        let mut previous = vec![None; n];

        let mut queue = VecDeque::new();

        costs[to_index(&start)] = Some(0);
        queue.push_back(start.clone());

        while let Some(v) = queue.pop_front() {
            let index_v = to_index(&v);
            let cost_v = costs[index_v].unwrap();

            for (nv, dcost) in neighbors(&v) {
                assert!(dcost <= 1);
                let index_nv = to_index(&nv);
                let new_cost_nv = cost_v + dcost;

                match costs[index_nv] {
                    Some(cost_nv) if cost_nv <= new_cost_nv => {}
                    _ => {
                        costs[index_nv] = Some(new_cost_nv);
                        previous[index_nv] = Some(v.clone());
                        if dcost == 0 {
                            queue.push_front(nv);
                        } else {
                            queue.push_back(nv);
                        }
                    }
                }
            }
        }

        Self {
            start: start.clone(),
            to_index,
            costs,
            previous: WithPath { previous },
        }
    }

    /// `v`の直前に通過する頂点を返す．
    pub fn previous(&self, v: &V) -> Option<&V> {
        self.previous.previous[(self.to_index)(v)].as_ref()
    }

    /// 始点から`end`までの経路を構築して返す．
    pub fn path(&self, end: &V) -> Option<Vec<V>> {
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

impl<V, I, P> Bfs<V, I, P>
where
    I: Fn(&V) -> usize,
{
    /// 始点．
    pub fn start(&self) -> &V {
        &self.start
    }

    /// 始点から`v`へのコストを返す．
    pub fn cost(&self, v: &V) -> Option<usize> {
        self.costs[(self.to_index)(v)]
    }
}

impl<V, I, P> Debug for Bfs<V, I, P>
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_no_path() {
        // [0]──▶[1]──▶[2]──▶[3]
        let start = 0;
        let graph = vec![vec![1], vec![2], vec![3], vec![]];
        let expected = vec![Some(0), Some(1), Some(2), Some(3)];
        let bfs = Bfs::new(graph.len(), &start, |&v| v, |&v| graph[v].iter().cloned());
        for v in 0..graph.len() {
            assert_eq!(bfs.cost(&v), expected[v]);
        }

        let start = 0;
        let graph = vec![vec![1], vec![2], vec![], vec![4], vec![]];
        let expected = vec![Some(0), Some(1), Some(2), None, None];
        let bfs = Bfs::new(graph.len(), &start, |&v| v, |&v| graph[v].iter().cloned());
        for v in 0..graph.len() {
            assert_eq!(bfs.cost(&v), expected[v]);
        }

        let start = 0;
        let graph = vec![vec![1], vec![2], vec![0]];
        let expected = vec![Some(0), Some(1), Some(2)];
        let bfs = Bfs::new(graph.len(), &start, |&v| v, |&v| graph[v].iter().cloned());
        for v in 0..graph.len() {
            assert_eq!(bfs.cost(&v), expected[v]);
        }

        let start = 2;
        let graph = vec![vec![1], vec![2], vec![3], vec![]];
        let expected = vec![None, None, Some(0), Some(1)];
        let bfs = Bfs::new(graph.len(), &start, |&v| v, |&v| graph[v].iter().cloned());
        for v in 0..graph.len() {
            assert_eq!(bfs.cost(&v), expected[v]);
        }

        //        ┌──────▶[3]──▶[5]
        //        │             ▲
        // [0]──▶[1]            │
        //  │                   │
        //  └────▶[2]──▶[4]─────┘
        //           │
        //           └──▶[3]
        let start = 0;
        let graph = vec![vec![1, 2], vec![3], vec![3, 4], vec![5], vec![5], vec![]];
        let expected = vec![Some(0), Some(1), Some(1), Some(2), Some(2), Some(3)];
        let bfs = Bfs::new(graph.len(), &start, |&v| v, |&v| graph[v].iter().cloned());
        for v in 0..graph.len() {
            assert_eq!(bfs.cost(&v), expected[v]);
        }
    }

    #[test]
    fn test_bfs_with_path() {
        // [0]──▶[1]──▶[2]──▶[3]
        let start = 0;
        let graph = vec![vec![1], vec![2], vec![3], vec![]];
        let expected = vec![Some(0), Some(1), Some(2), Some(3)];
        let bfs = Bfs::new_with_path(graph.len(), &start, |&v| v, |&v| graph[v].iter().cloned());
        for v in 0..graph.len() {
            assert_eq!(bfs.cost(&v), expected[v]);
        }
        assert_eq!(bfs.path(&3), Some(vec![0, 1, 2, 3]));

        //        ┌──────▶[3]──▶[5]
        //        │             ▲
        // [0]──▶[1]            │
        //  │                   │
        //  └────▶[2]──▶[4]─────┘
        //           │
        //           └──▶[3]
        let start = 0;
        let graph = vec![vec![1, 2], vec![3], vec![3, 4], vec![5], vec![5], vec![]];
        let expected = vec![Some(0), Some(1), Some(1), Some(2), Some(2), Some(3)];
        let bfs = Bfs::new_with_path(graph.len(), &start, |&v| v, |&v| graph[v].iter().cloned());
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

    //     #[test]
    //     fn test_01bfs() {
    //         // TODO:01-BFS特有のテストを追加する
    //         // 以下はBFSのテストの使い回し

    //         let start = 0;
    //         let graph = vec![vec![1], vec![2], vec![3], vec![]];
    //         let expected = vec![Some(0), Some(1), Some(2), Some(3)];
    //         let bfs = Bfs::new_01(
    //             graph.len(),
    //             &start,
    //             |&v| v,
    //             |&v| graph[v].iter().map(|&nv| (nv, 1)),
    //         );
    //         for v in 0..graph.len() {
    //             assert_eq!(bfs.cost(&v), expected[v]);
    //         }

    //         let start = 0;
    //         let graph = vec![vec![1], vec![2], vec![], vec![4], vec![]];
    //         let expected = vec![Some(0), Some(1), Some(2), None, None];
    //         let bfs = Bfs::new_01(
    //             graph.len(),
    //             &start,
    //             |&v| v,
    //             |&v| graph[v].iter().map(|&nv| (nv, 1)),
    //         );
    //         for v in 0..graph.len() {
    //             assert_eq!(bfs.cost(&v), expected[v]);
    //         }

    //         let start = 0;
    //         let graph = vec![vec![1], vec![2], vec![0]];
    //         let expected = vec![Some(0), Some(1), Some(2)];
    //         let bfs = Bfs::new_01(
    //             graph.len(),
    //             &start,
    //             |&v| v,
    //             |&v| graph[v].iter().map(|&nv| (nv, 1)),
    //         );
    //         for v in 0..graph.len() {
    //             assert_eq!(bfs.cost(&v), expected[v]);
    //         }

    //         let start = 2;
    //         let graph = vec![vec![1], vec![2], vec![3], vec![]];
    //         let expected = vec![None, None, Some(0), Some(1)];
    //         let bfs = Bfs::new_01(
    //             graph.len(),
    //             &start,
    //             |&v| v,
    //             |&v| graph[v].iter().map(|&nv| (nv, 1)),
    //         );
    //         for v in 0..graph.len() {
    //             assert_eq!(bfs.cost(&v), expected[v]);
    //         }

    //         let start = 0;
    //         let graph = vec![vec![1, 2], vec![3], vec![3, 4], vec![5], vec![5], vec![]];
    //         let expected = vec![Some(0), Some(1), Some(1), Some(2), Some(2), Some(3)];
    //         let bfs = Bfs::new_01(
    //             graph.len(),
    //             &start,
    //             |&v| v,
    //             |&v| graph[v].iter().map(|&nv| (nv, 1)),
    //         );
    //         for v in 0..graph.len() {
    //             assert_eq!(bfs.cost(&v), expected[v]);
    //         }
    //     }
}
