use std::mem::swap;

/// LowLinkを用いて無向グラフの橋を検出する
pub struct LowLink {
    len: usize,

    /// 隣接リスト表現のグラフ
    graph: Vec<Vec<usize>>,

    order: Vec<usize>,

    lowlink: Vec<usize>,

    /// 関節点
    articulations: Vec<usize>,

    has_built: bool,
}

impl LowLink {
    pub fn new(len: usize) -> Self {
        Self {
            len,
            graph: vec![vec![]; len],
            order: vec![],
            lowlink: vec![],
            articulations: vec![],
            has_built: false,
        }
    }

    // 辺(u, v)を追加する
    // 辺(v, u)も自動的に追加される
    pub fn add_edge(&mut self, u: usize, v: usize) {
        debug_assert!(u < self.len);
        debug_assert!(v < self.len);
        self.graph[u].push(v);
        self.graph[v].push(u);
        self.has_built = false;
    }

    /// 辺(u, v)が橋かどうかを判定する
    /// build()を読んでから使う
    /// TODO: uとvを結ぶ辺がないときの挙動が不明
    pub fn is_bridge(&self, mut u: usize, mut v: usize) -> bool {
        debug_assert!(self.has_built);
        if self.order[u] > self.order[v] {
            swap(&mut u, &mut v);
        }
        self.order[u] < self.lowlink[v]
    }

    pub fn build(&mut self) {
        self.order = vec![0; self.len];
        self.lowlink = vec![0; self.len];
        self.articulations = vec![];
        self.has_built = true;

        let mut k = 0;
        let mut visited = vec![false; self.len];

        for v in 0..self.len {
            if !visited[v] {
                self.dfs(&mut visited, v, v, 0, &mut k);
            }
        }
    }

    fn dfs(&mut self, visited: &mut [bool], root: usize, v: usize, pv: usize, k: &mut usize) {
        visited[v] = true;

        self.order[v] = *k;
        *k += 1;
        self.lowlink[v] = self.order[v];

        let mut is_articulation = false;
        let mut num_childs = 0;

        for i in 0..self.graph[v].len() {
            let nv = self.graph[v][i];

            if !visited[nv] {
                num_childs += 1;

                self.dfs(visited, root, nv, v, k);

                self.lowlink[v] = self.lowlink[v].min(self.lowlink[nv]);

                if v != root && self.order[v] <= self.lowlink[nv] {
                    is_articulation = true;
                }
            } else if v == root || nv != pv {
                self.lowlink[v] = self.lowlink[v].min(self.order[nv]);
            }
        }

        if v == root && num_childs > 1 {
            is_articulation = true;
        }

        if is_articulation {
            self.articulations.push(v);
        }
    }
}
