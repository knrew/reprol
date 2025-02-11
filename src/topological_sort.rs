use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

pub struct ToplogicalSort {
    /// トポロジカル順序
    order: Option<Vec<usize>>,

    /// トポロジカル順序が一意かどうか
    is_unique: bool,
}

impl ToplogicalSort {
    /// 隣接リスト表現のグラフをトポロジカルソートする
    /// 複数のトポロジカル順序が存在する場合は辞書順最小のものを求める
    pub fn new(graph: &[Vec<usize>]) -> Self {
        let mut indegrees = vec![0; graph.len()];

        for v in graph {
            for &nv in v {
                indegrees[nv] += 1;
            }
        }

        let mut heap = BinaryHeap::new();

        for i in 0..graph.len() {
            if indegrees[i] == 0 {
                heap.push(Reverse(i));
            }
        }

        let mut order = vec![];
        let mut is_unique = true;

        while let Some(Reverse(v)) = heap.pop() {
            if heap.len() >= 1 {
                is_unique = false;
            }
            order.push(v);

            for &nv in &graph[v] {
                indegrees[nv] -= 1;
                if indegrees[nv] == 0 {
                    heap.push(Reverse(nv));
                }
            }
        }

        let order = if order.len() == graph.len() {
            Some(order)
        } else {
            None
        };

        Self { order, is_unique }
    }

    /// 隣接リスト表現のグラフをトポロジカルソートする
    /// 複数のトポロジカル順序が存在する場合でも辞書順を保証しない
    pub fn new_unstable(graph: &[Vec<usize>]) -> Self {
        let mut indegrees = vec![0; graph.len()];

        for v in graph {
            for &nv in v {
                indegrees[nv] += 1;
            }
        }

        let mut que = VecDeque::new();

        for i in 0..graph.len() {
            if indegrees[i] == 0 {
                que.push_back(i);
            }
        }

        let mut order = vec![];
        let mut is_unique = true;

        while let Some(v) = que.pop_front() {
            if que.len() >= 1 {
                is_unique = false;
            }

            order.push(v);

            for &nv in &graph[v] {
                indegrees[nv] -= 1;
                if indegrees[nv] == 0 {
                    que.push_back(nv);
                }
            }
        }

        let order = if order.len() == graph.len() {
            Some(order)
        } else {
            is_unique = false;
            None
        };

        Self { order, is_unique }
    }

    pub fn order(&self) -> &Option<Vec<usize>> {
        &self.order
    }

    /// トポロジカル順序が一意かどうか
    /// トポロジカル順序が存在しない場合はfalse
    pub fn is_unique(&self) -> bool {
        self.is_unique
    }
}

#[cfg(test)]
mod tests {
    use super::ToplogicalSort;

    #[test]
    fn test_topological_sort() {
        // TODO: is_uniqueのテスト

        let test_cases = vec![
            // test 1
            (
                vec![vec![1], vec![2], vec![], vec![1, 4], vec![5], vec![2]],
                Some(vec![0, 3, 1, 4, 5, 2]),
            ),
            // test 2: 閉路が存在する場合
            (
                vec![vec![1], vec![2], vec![3], vec![1, 4], vec![5], vec![2]],
                None,
            ),
        ];

        for (g, expected) in test_cases {
            let t = ToplogicalSort::new(&g);
            assert_eq!(t.order(), &expected);
        }
    }
}
