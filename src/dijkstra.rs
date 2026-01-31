//! ダイクストラ(Dijkstra)
//!
//! ダイクストラ法で単一始点最短経路を計算する．
//!
//! # 使用例
//! ```
//! use reprol::dijkstra::Dijkstra;
//! let start = 0;
//! let graph = vec![
//!     vec![(1, 2)],
//!     vec![(2, 3), (4, 9)],
//!     vec![(4, 4)],
//!     vec![(0, 1)],
//!     vec![],
//! ];
//! let dijkstra = Dijkstra::new(
//!     graph.len(),
//!     &start,
//!     &0,
//!     |&v| v,
//!     |&v| graph[v].iter().cloned(),
//! );
//! assert_eq!(dijkstra.cost(&0), Some(&0));
//! assert_eq!(dijkstra.cost(&1), Some(&2));
//! assert_eq!(dijkstra.cost(&2), Some(&5));
//! assert_eq!(dijkstra.cost(&3), None);
//! assert_eq!(dijkstra.cost(&4), Some(&9));
//! ```

use std::{cmp::Reverse, collections::BinaryHeap, fmt::Debug, ops::Add};

use crate::path_tracker::{NoPath, PathTracker, WithPath};

/// ダイクストラの本体．
///
/// - `V`: 頂点の型
/// - `C`: コストの型
/// - `I`: `V`をインデックス(`usize`)に変換する関数
/// - `P`: 経路情報に関する構造体(`WithPath` または `NoPath`)
pub struct DijkstraImpl<V, C, I, P> {
    start: V,
    to_index: I,
    costs: Vec<Option<C>>,
    path_tracker: P,
}

impl<V, C, I, P> DijkstraImpl<V, C, I, P>
where
    V: Clone + Ord,
    C: Clone + Ord + Add<Output = C>,
    I: Fn(&V) -> usize,
    P: PathTracker<V>,
{
    pub fn new<E>(
        n: usize,
        start: &V,
        zero: &C,
        to_index: I,
        mut neighbors: impl FnMut(&V) -> E,
    ) -> Self
    where
        E: Iterator<Item = (V, C)>,
    {
        let mut costs = vec![None; n];
        let mut path_tracker = P::new(n);

        let mut heap = BinaryHeap::new();

        costs[to_index(start)] = Some(zero.clone());
        heap.push((Reverse(zero.clone()), start.clone()));

        while let Some((Reverse(cost), v)) = heap.pop() {
            let index_v = to_index(&v);
            if costs[index_v].as_ref().is_some_and(|cost_v| cost_v < &cost) {
                continue;
            }

            for (nv, dcost) in neighbors(&v) {
                let index_nv = to_index(&nv);
                let new_cost_nv = cost.clone() + dcost;

                if costs[index_nv]
                    .as_ref()
                    .is_none_or(|cost_nv| &new_cost_nv < cost_nv)
                {
                    costs[index_nv] = Some(new_cost_nv.clone());
                    path_tracker.set_previous(index_nv, &v);
                    heap.push((Reverse(new_cost_nv), nv));
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
    pub fn cost(&self, v: &V) -> Option<&C> {
        self.costs[(self.to_index)(v)].as_ref()
    }
}

impl<V, C, I> DijkstraImpl<V, C, I, WithPath<V>>
where
    V: Clone + Ord,
    C: Clone + Ord + Add<Output = C>,
    I: Fn(&V) -> usize,
{
    /// `v`の直前の頂点を返す．
    pub fn previous(&self, v: &V) -> Option<&V> {
        self.path_tracker.get_previous((self.to_index)(v))
    }

    /// 始点から`end`までの経路を構築する．
    pub fn path(&self, end: &V) -> Option<Vec<V>> {
        self.cost(end)?;
        Some(self.path_tracker.construct_path(&self.to_index, end))
    }
}

impl<V, C, I, P> Debug for DijkstraImpl<V, C, I, P>
where
    V: Debug,
    C: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Dijkstra")
            .field("start", &self.start)
            .field("costs", &self.costs)
            .finish()
    }
}

/// ダイクストラ(経路復元なし)
pub type Dijkstra<V, C, I> = DijkstraImpl<V, C, I, NoPath>;

/// ダイクストラ(経路復元あり)
pub type DijkstraWithPath<V, C, I> = DijkstraImpl<V, C, I, WithPath<V>>;

#[cfg(test)]
mod tests {
    use std::{fmt::Debug, ops::Add};

    use super::*;

    struct CostTestCase<'a, T> {
        graph: Vec<Vec<(usize, T)>>,
        start: usize,
        expected_costs: Vec<Option<&'a T>>,
    }

    impl<'a, T> CostTestCase<'a, T>
    where
        T: Debug + Clone + Ord + Add<Output = T>,
    {
        fn test(&self, zero: T) {
            assert_eq!(self.graph.len(), self.expected_costs.len());
            let dijkstra = Dijkstra::new(
                self.graph.len(),
                &self.start,
                &zero,
                |&v| v,
                |&v| self.graph[v].iter().cloned(),
            );
            for v in 0..self.graph.len() {
                assert_eq!(dijkstra.cost(&v), self.expected_costs[v]);
            }
        }
    }

    #[test]
    fn test_no_path() {
        // case1:
        CostTestCase {
            graph: vec![
                vec![(1, 2)],
                vec![(2, 3), (4, 9)],
                vec![(4, 4)],
                vec![(0, 1)],
                vec![],
            ],
            start: 0,
            expected_costs: vec![Some(&0), Some(&2), Some(&5), None, Some(&9)],
        }
        .test(0i32);

        // case 2: case 1と同じグラフ
        CostTestCase {
            graph: vec![
                vec![(1, 2)],
                vec![(2, 3), (4, 9)],
                vec![(4, 4)],
                vec![(0, 1)],
                vec![],
            ],
            start: 1,
            expected_costs: vec![None, Some(&0), Some(&3), None, Some(&7)],
        }
        .test(0i32);

        // case 3:
        CostTestCase {
            graph: vec![
                vec![(1, 2)],
                vec![(2, 3), (2, 7)],
                vec![(2, 2)],
                vec![(4, 1)],
                vec![],
            ],
            start: 0,
            expected_costs: vec![Some(&0), Some(&2), Some(&5), None, None],
        }
        .test(0u32);

        // case 4:
        CostTestCase {
            graph: vec![
                vec![(1, 10), (2, 1)],
                vec![(3, 2)],
                vec![(1, 1), (3, 4)],
                vec![],
            ],
            start: 0,
            expected_costs: vec![Some(&0), Some(&2), Some(&1), Some(&4)],
        }
        .test(0u64);

        // case 5:
        CostTestCase {
            graph: vec![vec![(1, 1)], vec![(2, 1)], vec![(0, 1)]],
            start: 0,
            expected_costs: vec![Some(&0), Some(&1), Some(&2)],
        }
        .test(0u64);

        // case 6:
        CostTestCase {
            graph: vec![
                vec![(1, 2), (2, 5)],
                vec![(2, 2), (3, 1)],
                vec![(3, 3), (4, 1)],
                vec![(4, 2)],
                vec![],
            ],
            start: 0,
            expected_costs: vec![Some(&0), Some(&2), Some(&4), Some(&3), Some(&5)],
        }
        .test(0i64);

        // case 7:
        CostTestCase {
            graph: vec![
                vec![(1, 2)],
                vec![(2, 3), (4, 9)],
                vec![(4, 4)],
                vec![(0, 1)],
                vec![],
            ],
            start: 2,
            expected_costs: vec![None, None, Some(&0), None, Some(&4)],
        }
        .test(0i64);

        // case 8: case 7と同じグラフ
        CostTestCase {
            graph: vec![
                vec![(1, 2)],
                vec![(2, 3), (4, 9)],
                vec![(4, 4)],
                vec![(0, 1)],
                vec![],
            ],
            start: 3,
            expected_costs: vec![Some(&1), Some(&3), Some(&6), Some(&0), Some(&10)],
        }
        .test(0i64);
    }

    #[test]
    fn test_path() {
        {
            let graph = vec![
                vec![(1, 2)],
                vec![(2, 3), (4, 9)],
                vec![(4, 4)],
                vec![(0, 1)],
                vec![],
            ];
            let start = 0;
            let dijkstra = DijkstraWithPath::new(
                graph.len(),
                &start,
                &0i32,
                |&v| v,
                |&v| graph[v].iter().cloned(),
            );
            assert_eq!(dijkstra.path(&4), Some(vec![0, 1, 2, 4]));
            assert_eq!(dijkstra.path(&3), None);
        }

        {
            let graph = vec![
                vec![(1, 2)],
                vec![(2, 3), (4, 9)],
                vec![(4, 4)],
                vec![(0, 1)],
                vec![],
            ];
            let start = 3;
            let dijkstra = DijkstraWithPath::new(
                graph.len(),
                &start,
                &0i32,
                |&v| v,
                |&v| graph[v].iter().cloned(),
            );
            assert_eq!(dijkstra.path(&4), Some(vec![3, 0, 1, 2, 4]));
        }
    }
}
