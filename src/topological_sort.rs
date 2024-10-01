use std::{cmp::Reverse, collections::BinaryHeap};

/// トポロジカルソート
/// 辞書順最小の頂点番号配列を返す
/// グラフが閉路を含む場合はNoneを返す
/// https://zenn.dev/reputeless/books/standard-cpp-for-competitive-programming/viewer/topological-sort
pub fn topological_sort(graph: &[Vec<usize>]) -> Option<Vec<usize>> {
    let mut indegrees = vec![0; graph.len()];

    for v in graph {
        for &to in v {
            indegrees[to] += 1;
        }
    }

    let mut heap = BinaryHeap::new();

    for i in 0..graph.len() {
        if indegrees[i] == 0 {
            heap.push(Reverse(i));
        }
    }

    let mut res = vec![];

    while let Some(Reverse(from)) = heap.pop() {
        res.push(from);

        for &to in &graph[from] {
            indegrees[to] -= 1;
            if indegrees[to] == 0 {
                heap.push(Reverse(to));
            }
        }
    }

    if res.len() == graph.len() {
        Some(res)
    } else {
        None
    }
}
