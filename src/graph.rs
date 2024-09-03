use std::ops::{Index, IndexMut};

// pub struct UnweightedGraph {
//     g: Vec<Vec<usize>>,
// }

// impl UnweightedGraph {
//     pub fn new(n: usize) -> Self {
//         Self { g: vec![vec![]; n] }
//     }

//     pub fn from_edges(n: usize, edges: &[(usize, usize)], is_directed: bool) -> Self {
//         let mut g = vec![vec![]; n];
//         for &(x, y) in edges {
//             g[x].push(y);
//             if !is_directed {
//                 g[y].push(x);
//             }
//         }

//         Self { g }
//     }
// }

pub struct WeightedGraph<T> {
    num_vertices: usize,
    is_directed: bool,
    g: Vec<Vec<(usize, T)>>,
}

impl<T: Copy> WeightedGraph<T> {
    pub fn new(n: usize, is_directed: bool) -> Self {
        Self {
            num_vertices: n,
            is_directed,
            g: vec![vec![]; n],
        }
    }

    pub fn from_edges(n: usize, edges: &[(usize, usize, T)], is_directed: bool) -> Self {
        let mut g = Self::new(n, is_directed);
        for &(x, y, w) in edges {
            g.add_edge(&(x, y, w));
        }
        g
    }

    pub fn add_edge(&mut self, edge: &(usize, usize, T)) {
        self.g[edge.0].push((edge.1, edge.2));
        if !self.is_directed {
            self.g[edge.1].push((edge.0, edge.2));
        }
    }

    pub fn num_vertices(&self) -> usize {
        self.num_vertices
    }
}

impl<T> Index<usize> for WeightedGraph<T> {
    type Output = [(usize, T)];
    fn index(&self, index: usize) -> &Self::Output {
        &self.g[index]
    }
}

impl<T> IndexMut<usize> for WeightedGraph<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.g[index]
    }
}
