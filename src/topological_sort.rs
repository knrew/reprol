use std::{cmp::Reverse, collections::BinaryHeap};

pub trait TopologicalSort {
    /// トポロジカルソート
    /// 入力は隣接リスト表現のグラフ
    /// 辞書順最小の頂点番号配列を返す
    /// グラフが閉路を含む場合はNoneを返す
    fn topological_sort(&self) -> Option<Vec<usize>>;
}

impl TopologicalSort for Vec<Vec<usize>> {
    fn topological_sort(&self) -> Option<Vec<usize>> {
        topological_sort(self)
    }
}

impl TopologicalSort for [Vec<usize>] {
    fn topological_sort(&self) -> Option<Vec<usize>> {
        topological_sort(self)
    }
}

/// トポロジカルソート
/// 入力は隣接リスト表現のグラフ
/// 辞書順最小の頂点番号配列を返す
/// グラフが閉路を含む場合はNoneを返す
fn topological_sort(graph: &[Vec<usize>]) -> Option<Vec<usize>> {
    let mut degrees = vec![0; graph.len()];

    for v in graph {
        for &nv in v {
            degrees[nv] += 1;
        }
    }

    let mut heap = BinaryHeap::new();

    for i in 0..graph.len() {
        if degrees[i] == 0 {
            heap.push(Reverse(i));
        }
    }

    let mut res = vec![];

    while let Some(Reverse(v)) = heap.pop() {
        res.push(v);

        for &nv in &graph[v] {
            degrees[nv] -= 1;
            if degrees[nv] == 0 {
                heap.push(Reverse(nv));
            }
        }
    }

    if res.len() == graph.len() {
        Some(res)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::topological_sort;

    #[test]
    fn test_topological_sort() {
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
            assert_eq!(topological_sort(&g), expected);
        }
    }
}
