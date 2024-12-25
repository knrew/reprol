/// componentsには各強連結成分がはいる
/// componentsはトポロジカル順になっている
pub struct Scc {
    len: usize,
    graph: Vec<Vec<usize>>,
    rev_graph: Vec<Vec<usize>>,
    components: Vec<Vec<usize>>,
    component_ids: Vec<usize>,
    has_built: bool,
}

impl Scc {
    pub fn new(n: usize) -> Self {
        Self {
            len: n,
            graph: vec![vec![]; n],
            rev_graph: vec![vec![]; n],
            components: vec![],
            component_ids: vec![0; n],
            has_built: false,
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        assert!(u < self.len);
        assert!(v < self.len);
        self.graph[u].push(v);
        self.rev_graph[v].push(u);
        self.has_built = false;
    }

    pub fn components(&self) -> impl DoubleEndedIterator<Item = &Vec<usize>> {
        assert!(self.has_built);
        self.components.iter()
    }

    pub fn component(&self, id: usize) -> impl DoubleEndedIterator<Item = usize> + '_ {
        assert!(self.has_built);
        assert!(id < self.components.len());
        self.components[id].iter().cloned()
    }

    /// 分解された連結成分の個数
    pub fn num_components(&self) -> usize {
        assert!(self.has_built);
        self.components.len()
    }

    /// 連結成分idに含まれる要素数
    pub fn size(&self, id: usize) -> usize {
        assert!(self.has_built);
        self.components[id].len()
    }

    pub fn build(&mut self) {
        self.components = vec![];
        self.component_ids = vec![0; self.len];
        self.has_built = true;

        let order = {
            let mut order = vec![];
            let mut visited = vec![false; self.len];
            for v in 0..self.len {
                if !visited[v] {
                    dfs(&self.graph, &mut visited, &mut order, v);
                }
            }
            order
        };

        let mut component_id = 0;

        let mut visited = vec![false; self.len];
        for i in (0..self.len).rev() {
            let v = order[i];
            if !visited[v] {
                let mut rev_order = vec![];
                rev_dfs(
                    &self.rev_graph,
                    &mut visited,
                    &mut self.component_ids,
                    &mut rev_order,
                    v,
                    component_id,
                );
                component_id += 1;
                self.components.push(rev_order);
            }
        }
    }
}

fn dfs(graph: &[Vec<usize>], visited: &mut [bool], order: &mut Vec<usize>, v: usize) {
    visited[v] = true;
    for &nv in &graph[v] {
        if !visited[nv] {
            dfs(graph, visited, order, nv);
        }
    }
    order.push(v);
}

fn rev_dfs(
    rev_graph: &[Vec<usize>],
    visited: &mut [bool],
    component_ids: &mut Vec<usize>,
    order: &mut Vec<usize>,
    v: usize,
    component_id: usize,
) {
    visited[v] = true;
    component_ids[v] = component_id;
    for &nv in &rev_graph[v] {
        if !visited[nv] {
            rev_dfs(rev_graph, visited, component_ids, order, nv, component_id);
        }
    }
    order.push(v);
}
