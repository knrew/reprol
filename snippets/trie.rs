const SIGMA: usize = 26;
const OFFSET: usize = b'A' as usize;

#[derive(Debug, Clone)]
struct Node {
    childs: [Option<usize>; SIGMA],
    parent: Option<usize>,
    is_terminal: bool,
}

impl Node {
    fn new() -> Self {
        Self {
            childs: [None; SIGMA],
            parent: None,
            is_terminal: false,
        }
    }
}

#[derive(Debug, Clone)]
struct Trie {
    nodes: Vec<Node>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            nodes: vec![Node::new()],
        }
    }

    fn insert(&mut self, s: &[u8]) {
        let mut v = 0;
        let mut path = vec![v];

        for &c in s {
            let c = c as usize - OFFSET;
            v = match self.nodes[v].childs[c] {
                Some(nv) => nv,
                None => {
                    let new_id = self.nodes.len();
                    self.nodes.push(Node::new());
                    self.nodes[new_id].parent = Some(v);
                    self.nodes[v].childs[c] = Some(new_id);
                    new_id
                }
            };
            path.push(v);
        }

        self.nodes[v].is_terminal = true;
    }
}
