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
        self.graph[u].push(v);
        self.graph[v].push(u);
        self.has_built = false;
    }

    /// 辺(u, v)が橋かどうかを判定する
    /// buildしてから使う
    /// NOTE: uとvを結ぶ辺がないときの挙動が不明
    pub fn is_bridge(&self, mut u: usize, mut v: usize) -> bool {
        assert!(self.has_built);
        if self.order[u] > self.order[v] {
            swap(&mut u, &mut v);
        }
        self.order[u] < self.lowlink[v]
    }

    pub fn articulations(&self) -> impl DoubleEndedIterator<Item = usize> + '_ {
        assert!(self.has_built);
        self.articulations.iter().cloned()
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

#[cfg(test)]
mod tests {
    use super::LowLink;

    #[test]
    fn test_lowlink() {
        let n = 8;
        let edges = vec![
            (0, 2),
            (0, 1),
            (6, 5),
            (6, 7),
            (3, 5),
            (3, 4),
            (2, 3),
            (1, 2),
            (4, 5),
        ];

        let mut lowlink = LowLink::new(n);
        for &(u, v) in &edges {
            lowlink.add_edge(u, v);
        }

        lowlink.build();

        assert_eq!(lowlink.is_bridge(0, 2), false);
        assert_eq!(lowlink.is_bridge(0, 1), false);
        assert_eq!(lowlink.is_bridge(6, 5), true);
        assert_eq!(lowlink.is_bridge(6, 7), true);
        assert_eq!(lowlink.is_bridge(3, 5), false);
        assert_eq!(lowlink.is_bridge(3, 4), false);
        assert_eq!(lowlink.is_bridge(2, 3), true);
        assert_eq!(lowlink.is_bridge(1, 2), false);
        assert_eq!(lowlink.is_bridge(4, 5), false);
    }
}
