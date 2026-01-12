//! Cartesian Tree
//!
//! 数列`v`に対応するCartesian Treeを構築する．
//! 各ノードは配列のインデックスを持ち，親ノードの値が子ノード以下となるヒープ条件を満たす．
//!
//! # 使用例
//! ```
//! use reprol::ds::cartesian_tree::CartesianTree;
//!
//! let v = vec![3, 1, 4, 1, 5];
//! let tree = CartesianTree::new(&v);
//! assert_eq!(tree.root(), 1);
//! assert_eq!(tree.left(1), Some(0));
//! assert_eq!(tree.right(1), Some(3));
//! ```

/// Cartesian Treeの内部ノード．
/// 親子関係として配列のインデックスを保持する．
#[derive(Debug, Clone, Default)]
struct Node {
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

/// 数列から構築したCartesian Tree．
/// 各ノードは元配列のインデックスを表す．
#[derive(Debug, Clone)]
pub struct CartesianTree {
    nodes: Vec<Node>,
    root: usize,
}

impl CartesianTree {
    /// 数列`v`からCartesian Treeを構築する．
    ///
    /// # Panics
    /// - `v`が空のとき
    pub fn new<T: PartialOrd>(v: &[T]) -> Self {
        assert!(!v.is_empty());

        let n = v.len();
        let mut nodes = vec![Node::default(); n];

        let mut stack: Vec<usize> = Vec::with_capacity(n);

        for i in 0..n {
            let mut p = None;

            while let Some(j) = match stack.last() {
                Some(&j) if v[j] > v[i] => stack.pop(),
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

    /// 根に対応するインデックスを返す．
    pub fn root(&self) -> usize {
        self.root
    }

    /// ノード`v`の親のインデックスを返す．存在しない場合は`None`．
    pub fn parent(&self, v: usize) -> Option<usize> {
        self.nodes[v].parent
    }

    /// ノード`v`の左子のインデックスを返す．存在しない場合は`None`．
    pub fn left(&self, v: usize) -> Option<usize> {
        self.nodes[v].left
    }

    /// ノード`v`の右子のインデックスを返す．存在しない場合は`None`．
    pub fn right(&self, v: usize) -> Option<usize> {
        self.nodes[v].right
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
            let tree = CartesianTree::new(&v);

            assert_eq!(tree.root(), 1);

            assert_eq!(tree.left(1), Some(0));
            assert_eq!(tree.right(1), Some(3));

            assert_eq!(tree.parent(0), Some(1));
            assert_eq!(tree.parent(3), Some(1));

            assert_eq!(tree.left(3), Some(2));
            assert_eq!(tree.right(3), Some(4));

            assert_eq!(tree.parent(2), Some(3));
            assert_eq!(tree.parent(4), Some(3));
        }

        {
            let v = vec![1, 2, 3, 4];
            let tree = CartesianTree::new(&v);

            assert_eq!(tree.root(), 0);

            assert_eq!(tree.parent(1), Some(0));
            assert_eq!(tree.parent(2), Some(1));
            assert_eq!(tree.parent(3), Some(2));

            assert_eq!(tree.left(0), None);
            assert_eq!(tree.left(1), None);
            assert_eq!(tree.left(2), None);
            assert_eq!(tree.left(3), None);

            assert_eq!(tree.right(0), Some(1));
            assert_eq!(tree.right(1), Some(2));
            assert_eq!(tree.right(2), Some(3));
            assert_eq!(tree.right(3), None);
        }

        {
            let v = vec![4, 3, 2, 1];
            let tree = CartesianTree::new(&v);

            assert_eq!(tree.root(), 3);

            assert_eq!(tree.parent(3), None);
            assert_eq!(tree.parent(2), Some(3));
            assert_eq!(tree.parent(1), Some(2));
            assert_eq!(tree.parent(0), Some(1));

            assert_eq!(tree.left(3), Some(2));
            assert_eq!(tree.left(2), Some(1));
            assert_eq!(tree.left(1), Some(0));
            assert_eq!(tree.left(0), None);

            assert_eq!(tree.right(3), None);
            assert_eq!(tree.right(2), None);
            assert_eq!(tree.right(1), None);
            assert_eq!(tree.right(0), None);
        }

        {
            let v = vec![42];
            let tree = CartesianTree::new(&v);

            assert_eq!(tree.root(), 0);
            assert_eq!(tree.parent(0), None);
            assert_eq!(tree.left(0), None);
            assert_eq!(tree.right(0), None);
        }
    }

    #[test]
    fn test_reverse() {
        let v = vec![3, 1, 4, 1, 5];
        let reversed = v.iter().copied().map(Reverse).collect::<Vec<_>>();
        let tree = CartesianTree::new(&reversed);

        assert_eq!(tree.root(), 4);

        assert_eq!(tree.left(4), Some(2));
        assert_eq!(tree.right(4), None);
        assert_eq!(tree.parent(4), None);

        assert_eq!(tree.parent(2), Some(4));
        assert_eq!(tree.left(2), Some(0));
        assert_eq!(tree.right(2), Some(3));

        assert_eq!(tree.parent(0), Some(2));
        assert_eq!(tree.left(0), None);
        assert_eq!(tree.right(0), Some(1));

        assert_eq!(tree.parent(1), Some(0));
        assert_eq!(tree.left(1), None);
        assert_eq!(tree.right(1), None);

        assert_eq!(tree.parent(3), Some(2));
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
                        let v: Vec<$ty> = (0..n)
                            .map(|_| rng.random_range(<$ty>::MIN..=<$ty>::MAX))
                            .collect();
                        let tree = CartesianTree::new(&v);

                        let min_value = *v.iter().min().unwrap();
                        assert_eq!(v[tree.root()], min_value);
                        assert_eq!(tree.parent(tree.root()), None);

                        for i in 0..n {
                            if let Some(p) = tree.parent(i) {
                                assert!(v[p] <= v[i]);
                            }
                            if let Some(l) = tree.left(i) {
                                assert!(v[i] <= v[l]);
                            }
                            if let Some(r) = tree.right(i) {
                                assert!(v[i] <= v[r]);
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
}
