//! Cartesian Tree
//!
//! 数列`v`に対応するCartesian Treeを構築する．
//! 数列の最小値のインデックスをrootとする．
//!
//! TODO:
//! - 説明(doc)を追加する．
//! - テストを書く．

#[derive(Debug, Clone)]
struct Node {
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            parent: None,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CartesianTree {
    nodes: Vec<Node>,
    root: usize,
}

impl CartesianTree {
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
            p.map(|p| nodes[p].parent = Some(i));

            stack.push(i);
        }

        for i in 0..stack.len() - 1 {
            nodes[stack[i]].right = Some(stack[i + 1]);
        }

        let root = stack[0];

        Self { nodes, root }
    }

    pub fn root(&self) -> usize {
        self.root
    }

    pub fn parent(&self, i: usize) -> Option<usize> {
        self.nodes[i].parent
    }

    pub fn left(&self, i: usize) -> Option<usize> {
        self.nodes[i].left
    }

    pub fn right(&self, i: usize) -> Option<usize> {
        self.nodes[i].right
    }
}
