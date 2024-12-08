use std::{cmp::Reverse, collections::BinaryHeap, ops::Add};

/// ダイクストラで最小コストを計算する
/// V: 頂点の型
/// C: コストの型
/// I: 頂点からインデックスへの写像
pub struct Dijkstra<V, C, I> {
    /// 頂点数(状態数)
    n: usize,

    /// 始点頂点
    start: V,

    /// 状態からインデックスへの写像
    /// 例: H*Wの2次元グリッドのマス(i, j) -> i*w+j
    to_index: I,

    /// costs[v]: 頂点startからvへの最短経路のコスト
    /// startから到達不可能である場合はNone
    costs: Vec<Option<C>>,

    /// previous_vertices[v]: 最短経路においてvの直前に訪問する頂点
    previous_vertices: Vec<Option<V>>,
}

impl<V, C, I> Dijkstra<V, C, I>
where
    V: Clone + Ord,
    C: Clone + Ord + Add<Output = C>,
    I: Fn(&V) -> usize,
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
        let mut previous_vertices = vec![None; n];

        let mut heap = BinaryHeap::new();

        costs[to_index(&start)] = Some(zero.clone());
        heap.push(Reverse((zero.clone(), start.clone())));

        while let Some(Reverse((cost, v))) = heap.pop() {
            if let Some(min_distance) = &costs[to_index(&v)] {
                if min_distance < &cost {
                    continue;
                }
            }

            for (nv, dcost) in neighbors(&v) {
                let new_cost = cost.clone() + dcost;

                if let Some(min_cost) = &costs[to_index(&nv)] {
                    if min_cost <= &new_cost {
                        continue;
                    }
                }

                costs[to_index(&nv)] = Some(new_cost.clone());
                previous_vertices[to_index(&nv)] = Some(v.clone());
                heap.push(Reverse((new_cost, nv)));
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
        &self.costs[(self.to_index)(v)]
    }

    /// 頂点endへの最短経路を構築する
    /// startとendを含む
    pub fn construct_path(&self, end: &V) -> Option<Vec<V>> {
        if self.costs[(self.to_index)(end)].is_none() {
            return None;
        }

        let mut res = vec![end.clone()];

        while let Some(p) = &self.previous_vertices[(self.to_index)(res.last().unwrap())] {
            res.push(p.clone());
        }

        res.reverse();

        Some(res)
    }
}

/// 隣接リスト表現のグラフからダイクストラによって最短経路を計算する
pub fn dijkstra_adjacencies(
    g: &[Vec<(usize, u64)>],
    start: usize,
) -> Dijkstra<usize, u64, impl Fn(&usize) -> usize> {
    Dijkstra::new(
        g.len(),
        &start,
        &0,
        |&v: &usize| v,
        |&v| g[v].iter().cloned(),
    )
}

// TODO: パス復元のテストを書く
#[cfg(test)]
mod tests {
    use super::dijkstra_adjacencies;

    #[test]
    fn test_dijkstra() {
        fn costs(g: &[Vec<(usize, u64)>], start: usize) -> Vec<Option<u64>> {
            let dijkstra = dijkstra_adjacencies(&g, start);
            (0..g.len()).map(|v| dijkstra.cost(&v).clone()).collect()
        }

        // (graph, start, expected)
        // test 6-10: グラフは同じ．startが異なる
        let testcases = vec![
            // test 1
            (
                vec![
                    vec![(1, 2)],
                    vec![(2, 3), (4, 9)],
                    vec![(4, 4)],
                    vec![(0, 1)],
                    vec![],
                ],
                0,
                vec![Some(0), Some(2), Some(5), None, Some(9)],
            ),
            // test 2
            (
                vec![vec![(1, 2)], vec![(2, 3)], vec![], vec![(4, 1)], vec![]],
                0,
                vec![Some(0), Some(2), Some(5), None, None],
            ),
            // test 3
            (
                vec![
                    vec![(1, 10), (2, 1)],
                    vec![(3, 2)],
                    vec![(1, 1), (3, 4)],
                    vec![],
                ],
                0,
                vec![Some(0), Some(2), Some(1), Some(4)],
            ),
            // test 4
            (
                vec![vec![(1, 1)], vec![(2, 1)], vec![(0, 1)]],
                0,
                vec![Some(0), Some(1), Some(2)],
            ),
            // test 5
            (
                vec![
                    vec![(1, 2), (2, 5)],
                    vec![(2, 2), (3, 1)],
                    vec![(3, 3), (4, 1)],
                    vec![(4, 2)],
                    vec![],
                ],
                0,
                vec![Some(0), Some(2), Some(4), Some(3), Some(5)],
            ),
            // test 6
            (
                vec![
                    vec![(1, 2)],
                    vec![(2, 3), (4, 9)],
                    vec![(4, 4)],
                    vec![(0, 1)],
                    vec![],
                ],
                0,
                vec![Some(0), Some(2), Some(5), None, Some(9)],
            ),
            // test 7
            (
                vec![
                    vec![(1, 2)],
                    vec![(2, 3), (4, 9)],
                    vec![(4, 4)],
                    vec![(0, 1)],
                    vec![],
                ],
                1,
                vec![None, Some(0), Some(3), None, Some(7)],
            ),
            // test 8
            (
                vec![
                    vec![(1, 2)],
                    vec![(2, 3), (4, 9)],
                    vec![(4, 4)],
                    vec![(0, 1)],
                    vec![],
                ],
                2,
                vec![None, None, Some(0), None, Some(4)],
            ),
            // test 9
            (
                vec![
                    vec![(1, 2)],
                    vec![(2, 3), (4, 9)],
                    vec![(4, 4)],
                    vec![(0, 1)],
                    vec![],
                ],
                3,
                vec![Some(1), Some(3), Some(6), Some(0), Some(10)],
            ),
            // test 10
            (
                vec![
                    vec![(1, 2)],
                    vec![(2, 3), (4, 9)],
                    vec![(4, 4)],
                    vec![(0, 1)],
                    vec![],
                ],
                4,
                vec![None, None, None, None, Some(0)],
            ),
        ];

        for (g, start, expected) in testcases {
            assert_eq!(costs(&g, start), expected);
        }
    }
}
