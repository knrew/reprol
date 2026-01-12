//! Cartesian Tree
//!
//! 数列`v`に対応するCartesian Treeを構築する．
//! 各ノードは配列のインデックスと値を保持し，親ノードの値が子ノード以下となるヒープ条件を満たす．
//!
//! # 使用例
//! ```
//! use reprol::ds::cartesian_tree::CartesianTree;
//!
//! // デフォルトは最小値が根
//! let v = vec![3, 1, 4, 1, 5];
//! let tree = CartesianTree::new(v);
//! assert_eq!(tree.root(), (1, &1));
//! assert_eq!(tree.left(1), Some((0, &3)));
//! assert_eq!(tree.right(1), Some((3, &1)));
//!
//! // 比較子を指定して最大値を根にする例
//! let v = vec![3, 1, 4, 1, 5];
//! let tree = CartesianTree::new_by(v, |a, b| b.cmp(a));
//! assert_eq!(tree.root(), (4, &5));
//! ```
//!
//! # Panics
//! 空の入力を与えた場合，内部のアサーションによりパニックする．

use std::cmp::Ordering;

/// Cartesian Treeの内部ノード．
/// 配列の値と親子関係を保持する．
#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            parent: None,
            left: None,
            right: None,
        }
    }
}

/// 数列から構築したCartesian Tree．
/// 各ノードは元配列のインデックスと値を表す．
#[derive(Debug, Clone)]
pub struct CartesianTree<T> {
    nodes: Vec<Node<T>>,
    root: usize,
}

impl<T> CartesianTree<T> {
    /// 配列`v`から最小ヒープCartesian Treeを構築する．
    pub fn new(v: Vec<T>) -> Self
    where
        T: Ord,
    {
        Self::from_iter(v.into_iter())
    }

    /// 配列`v`と任意の比較子`cmp`からCartesian Treeを構築する．
    /// `cmp`で比較し小さい要素が上位(根側)に配置される．
    pub fn new_by(v: Vec<T>, cmp: impl FnMut(&T, &T) -> Ordering) -> Self {
        Self::from_iter_by(v.into_iter(), cmp)
    }

    /// イテレータと任意の比較子`cmp`からCartesian Treeを構築する．
    /// `cmp`で比較し小さい要素が上位(根側)に配置される．
    pub fn from_iter_by(
        iter: impl IntoIterator<Item = T>,
        mut cmp: impl FnMut(&T, &T) -> Ordering,
    ) -> Self {
        let mut nodes = iter.into_iter().map(Node::new).collect::<Vec<_>>();

        assert!(!nodes.is_empty());

        let n = nodes.len();

        let mut stack: Vec<usize> = Vec::with_capacity(n);

        for i in 0..n {
            let mut p = None;

            while let Some(j) = match stack.last() {
                Some(&j) if cmp(&nodes[i].value, &nodes[j].value).is_lt() => stack.pop(),
                _ => None,
            } {
                nodes[j].right = p;
                p = Some(j);
            }

            nodes[i].left = p;
            nodes[i].parent = stack.last().cloned();
            if let Some(p) = p {
                nodes[p].parent = Some(i);
            }

            stack.push(i);
        }

        for i in 0..stack.len() - 1 {
            nodes[stack[i]].right = Some(stack[i + 1]);
        }

        let root = stack[0];

        Self { nodes, root }
    }

    /// ノード`v`の値を返す．
    pub fn get(&self, v: usize) -> &T {
        &self.nodes[v].value
    }

    /// 根に対応するインデックスと値を返す．
    pub fn root(&self) -> (usize, &T) {
        (self.root, &self.nodes[self.root].value)
    }

    /// ノード`v`の親のインデックスと値を返す．存在しない場合は`None`．
    pub fn parent(&self, v: usize) -> Option<(usize, &T)> {
        self.nodes[v].parent.map(|p| (p, &self.nodes[p].value))
    }

    /// ノード`v`の左子のインデックスと値を返す．存在しない場合は`None`．
    pub fn left(&self, v: usize) -> Option<(usize, &T)> {
        self.nodes[v].left.map(|l| (l, &self.nodes[l].value))
    }

    /// ノード`v`の右子のインデックスと値を返す．存在しない場合は`None`．
    pub fn right(&self, v: usize) -> Option<(usize, &T)> {
        self.nodes[v].right.map(|r| (r, &self.nodes[r].value))
    }
}

impl<T: Ord> From<Vec<T>> for CartesianTree<T> {
    fn from(v: Vec<T>) -> Self {
        Self::from_iter(v.into_iter())
    }
}

impl<T: Ord, const N: usize> From<[T; N]> for CartesianTree<T> {
    fn from(v: [T; N]) -> Self {
        Self::from_iter(v.into_iter())
    }
}

impl<T: Ord> FromIterator<T> for CartesianTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from_iter_by(iter, |x, y| x.cmp(&y))
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Reverse;

    use rand::Rng;

    use super::*;
    use crate::utils::test_utils::initialize_rng;

    #[test]
    fn test() {
        {
            let v = vec![3, 1, 4, 1, 5];
            let tree = CartesianTree::new(v);

            let (root_index, root_value) = tree.root();
            assert_eq!(root_index, 1);
            assert_eq!(*root_value, 1);

            assert_eq!(tree.left(1), Some((0, &3)));
            assert_eq!(tree.right(1), Some((3, &1)));

            assert_eq!(tree.parent(0), Some((1, &1)));
            assert_eq!(tree.parent(3), Some((1, &1)));

            assert_eq!(tree.left(3), Some((2, &4)));
            assert_eq!(tree.right(3), Some((4, &5)));

            assert_eq!(tree.parent(2), Some((3, &1)));
            assert_eq!(tree.parent(4), Some((3, &1)));
        }

        {
            let v = vec![1, 2, 3, 4];
            let tree = CartesianTree::new(v);

            let (root_index, root_value) = tree.root();
            assert_eq!(root_index, 0);
            assert_eq!(*root_value, 1);

            assert_eq!(tree.parent(1), Some((0, &1)));
            assert_eq!(tree.parent(2), Some((1, &2)));
            assert_eq!(tree.parent(3), Some((2, &3)));

            assert_eq!(tree.left(0), None);
            assert_eq!(tree.left(1), None);
            assert_eq!(tree.left(2), None);
            assert_eq!(tree.left(3), None);

            assert_eq!(tree.right(0), Some((1, &2)));
            assert_eq!(tree.right(1), Some((2, &3)));
            assert_eq!(tree.right(2), Some((3, &4)));
            assert_eq!(tree.right(3), None);
        }

        {
            let v = vec![4, 3, 2, 1];
            let tree = CartesianTree::new(v);

            let (root_index, root_value) = tree.root();
            assert_eq!(root_index, 3);
            assert_eq!(*root_value, 1);

            assert_eq!(tree.parent(3), None);
            assert_eq!(tree.parent(2), Some((3, &1)));
            assert_eq!(tree.parent(1), Some((2, &2)));
            assert_eq!(tree.parent(0), Some((1, &3)));

            assert_eq!(tree.left(3), Some((2, &2)));
            assert_eq!(tree.left(2), Some((1, &3)));
            assert_eq!(tree.left(1), Some((0, &4)));
            assert_eq!(tree.left(0), None);

            assert_eq!(tree.right(3), None);
            assert_eq!(tree.right(2), None);
            assert_eq!(tree.right(1), None);
            assert_eq!(tree.right(0), None);
        }

        {
            let v = vec![42];
            let tree = CartesianTree::new(v.clone());

            let (root_index, root_value) = tree.root();
            assert_eq!(root_index, 0);
            assert_eq!(*root_value, 42);
            assert_eq!(tree.parent(0), None);
            assert_eq!(tree.left(0), None);
            assert_eq!(tree.right(0), None);
        }
    }

    #[test]
    fn test_reverse() {
        let v = vec![3, 1, 4, 1, 5];
        let reversed = v.iter().copied().map(Reverse).collect::<Vec<_>>();
        let tree = CartesianTree::new(reversed);

        let (root_index, root_value) = tree.root();
        assert_eq!(root_index, 4);
        assert_eq!(*root_value, Reverse(5));

        assert_eq!(tree.left(4), Some((2, &Reverse(4))));
        assert_eq!(tree.right(4), None);
        assert_eq!(tree.parent(4), None);

        assert_eq!(tree.parent(2), Some((4, &Reverse(5))));
        assert_eq!(tree.left(2), Some((0, &Reverse(3))));
        assert_eq!(tree.right(2), Some((3, &Reverse(1))));

        assert_eq!(tree.parent(0), Some((2, &Reverse(4))));
        assert_eq!(tree.left(0), None);
        assert_eq!(tree.right(0), Some((1, &Reverse(1))));

        assert_eq!(tree.parent(1), Some((0, &Reverse(3))));
        assert_eq!(tree.left(1), None);
        assert_eq!(tree.right(1), None);

        assert_eq!(tree.parent(3), Some((2, &Reverse(4))));
        assert_eq!(tree.left(3), None);
        assert_eq!(tree.right(3), None);
    }

    #[test]
    fn test_random() {
        macro_rules! define_test_function {
            ($name:ident, $ty:ty) => {
                fn $name(rng: &mut impl Rng) {
                    const T: usize = 200;
                    const N_MAX: usize = 30;

                    for _ in 0..T {
                        let n = rng.random_range(1..=N_MAX);
                        let v = (0..n)
                            .map(|_| rng.random_range(<$ty>::MIN..=<$ty>::MAX))
                            .collect::<Vec<_>>();
                        let tree = CartesianTree::new(v.clone());

                        let (root_index, root_value) = tree.root();
                        let min_value = v.iter().min().unwrap();
                        assert_eq!(&v[root_index], min_value);
                        assert_eq!(root_value, min_value);
                        assert_eq!(tree.parent(root_index), None);

                        for i in 0..n {
                            if let Some((_p, parent_value)) = tree.parent(i) {
                                assert!(parent_value <= &v[i]);
                            }
                            if let Some((_l, left_value)) = tree.left(i) {
                                assert!(&v[i] <= left_value);
                            }
                            if let Some((_r, right_value)) = tree.right(i) {
                                assert!(&v[i] <= right_value);
                            }
                        }
                    }
                }
            };
        }

        define_test_function!(test_i64, i64);
        define_test_function!(test_u64, u64);

        let mut rng = initialize_rng();
        test_i64(&mut rng);
        test_u64(&mut rng);
    }

    #[test]
    fn test_custom_comparator_desc() {
        let v = vec![3, 1, 4, 1, 5];
        let tree = CartesianTree::new_by(v, |a, b| b.cmp(a));

        let (root_index, root_value) = tree.root();
        assert_eq!(root_value, &5);
        assert_eq!(root_index, 4);
        assert_eq!(tree.parent(4), None);

        // 親は常に「大きい」側で、子は小さくなる
        for i in 0..tree.nodes.len() {
            if let Some((_p, parent_value)) = tree.parent(i) {
                assert!(parent_value >= tree.get(i));
            }
            if let Some((_l, left_value)) = tree.left(i) {
                assert!(tree.get(i) >= left_value);
            }
            if let Some((_r, right_value)) = tree.right(i) {
                assert!(tree.get(i) >= right_value);
            }
        }
    }

    #[test]
    fn test_all_equal_comparator() {
        let n = 5;
        let v = vec![1; n];
        let tree = CartesianTree::new_by(v, |_a, _b| Ordering::Equal);

        let (root_index, root_value) = tree.root();
        assert_eq!(root_index, 0);
        assert_eq!(*root_value, 1);
        assert_eq!(tree.parent(0), None);

        for i in 0..n {
            if i == 0 {
                assert_eq!(tree.left(i), None);
                assert_eq!(tree.right(i), Some((1, &1)));
                continue;
            }
            assert_eq!(tree.left(i), None);
            if i + 1 < n {
                assert_eq!(tree.right(i), Some((i + 1, &1)));
            } else {
                assert_eq!(tree.right(i), None);
            }
            assert_eq!(tree.parent(i), Some((i - 1, &1)));
        }
    }

    #[test]
    fn test_stateful_comparator_calls() {
        let v = vec![3, 1, 4, 1, 5];
        let mut calls = 0;
        let tree = CartesianTree::from_iter_by(v.clone(), |a, b| {
            calls += 1;
            a.cmp(b)
        });

        assert!(calls >= v.len() - 1);

        let (root_index, root_value) = tree.root();
        let min_value = v.iter().min().unwrap();
        assert_eq!(root_value, min_value);
        assert_eq!(tree.get(root_index), min_value);
    }
}
