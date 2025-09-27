use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
    marker::PhantomData,
};

pub trait Queue {
    fn new() -> Self;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn push(&mut self, v: usize);
    fn pop(&mut self) -> Option<usize>;
}

// 辞書順最小用
pub struct Ordered(BinaryHeap<Reverse<usize>>);

impl Queue for Ordered {
    #[inline(always)]
    fn new() -> Self {
        Self(BinaryHeap::new())
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline(always)]
    fn push(&mut self, v: usize) {
        self.0.push(Reverse(v));
    }

    #[inline(always)]
    fn pop(&mut self) -> Option<usize> {
        self.0.pop().map(|Reverse(v)| v)
    }
}

// 辞書順最小保証しない用
pub struct Unordered(VecDeque<usize>);

impl Queue for Unordered {
    #[inline(always)]
    fn new() -> Self {
        Self(VecDeque::new())
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline(always)]
    fn push(&mut self, v: usize) {
        self.0.push_back(v);
    }

    #[inline(always)]
    fn pop(&mut self) -> Option<usize> {
        self.0.pop_front()
    }
}

pub struct ToplogicalSortImpl<Q> {
    order: Option<Vec<usize>>,
    is_unique: bool,
    phantom: PhantomData<Q>,
}

impl<Q: Queue> ToplogicalSortImpl<Q> {
    pub fn new(graph: &[Vec<usize>]) -> Self {
        let n = graph.len();

        let mut indegrees = vec![0; n];

        for v in graph {
            for &nv in v {
                indegrees[nv] += 1;
            }
        }

        let mut que = Q::new();

        for (i, indegree) in indegrees.iter().enumerate() {
            if indegree == &0 {
                que.push(i);
            }
        }

        let mut order = Vec::with_capacity(n);
        let mut is_unique = true;

        while let Some(v) = que.pop() {
            if que.len() >= 1 {
                is_unique = false;
            }
            order.push(v);

            for &nv in &graph[v] {
                indegrees[nv] -= 1;
                if indegrees[nv] == 0 {
                    que.push(nv);
                }
            }
        }

        let order = (order.len() == n).then_some(order);

        Self {
            order,
            is_unique,
            phantom: PhantomData,
        }
    }

    /// トポロジカル順序
    pub fn order(&self) -> Option<&Vec<usize>> {
        self.order.as_ref()
    }

    /// トポロジカル順序が一意かどうか
    /// トポロジカル順序が存在しない場合はfalse
    pub fn is_unique(&self) -> bool {
        self.is_unique
    }
}

/// 辞書順最小のものを求めるトポロジカルソート
pub type ToplogicalSort = ToplogicalSortImpl<Ordered>;

/// 辞書順最小を保証しないトポロジカルソート
pub type ToplogicalSortUnordered = ToplogicalSortImpl<Unordered>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topological_sort() {
        // TODO: is_uniqueのテスト

        let g = vec![vec![1], vec![2], vec![], vec![1, 4], vec![5], vec![2]];
        let t = ToplogicalSort::new(&g);
        assert_eq!(t.order(), Some(&vec![0, 3, 1, 4, 5, 2]));

        let g = vec![vec![1], vec![2], vec![3], vec![1, 4], vec![5], vec![2]];
        let t = ToplogicalSort::new(&g);
        assert_eq!(t.order(), None);
    }
}
