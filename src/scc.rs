/// componentsには各強連結成分がはいる
/// componentsはトポロジカル順になっている
pub struct Scc {
    n: usize,
    graph: Vec<Vec<usize>>,
    rev_graph: Vec<Vec<usize>>,
    components: Vec<Vec<usize>>,
    component_ids: Vec<usize>,
    has_solved: bool,
}

impl Scc {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            graph: vec![vec![]; n],
            rev_graph: vec![vec![]; n],
            components: vec![],
            component_ids: vec![0; n],
            has_solved: false,
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        debug_assert!(!self.has_solved);
        debug_assert!(u < self.n);
        debug_assert!(v < self.n);
        self.graph[u].push(v);
        self.rev_graph[v].push(u);
    }

    pub fn num_components(&self) -> usize {
        debug_assert!(self.has_solved);
        self.components.len()
    }

    pub fn component(&self, id: usize) -> &[usize] {
        debug_assert!(self.has_solved);
        debug_assert!(id < self.components.len());
        &self.components[id]
    }

    pub fn components(&self) -> &[Vec<usize>] {
        debug_assert!(self.has_solved);
        &self.components
    }

    pub fn solve(&mut self) {
        debug_assert!(!self.has_solved);

        self.has_solved = true;

        let order = {
            let mut order = vec![];
            let mut visited = vec![false; self.n];
            for v in 0..self.n {
                if !visited[v] {
                    dfs(&self.graph, &mut visited, &mut order, v);
                }
            }
            order
        };

        let mut component_id = 0;

        let mut visited = vec![false; self.n];
        for i in (0..self.n).rev() {
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
