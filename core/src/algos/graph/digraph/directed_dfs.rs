use crate::data_containers::{DiGraph, Stack};

pub struct DirectedDFS {
    marked: Vec<bool>,
    edge_to: Vec<Option<usize>>,
    source_vertex: usize,
}

impl DirectedDFS {
    pub fn new(g: &DiGraph, source_vertex: usize) -> Self {
        let mut dfs = Self {
            marked: vec![false; g.vertices()],
            edge_to: vec![None; g.vertices()],
            source_vertex,
        };
        dfs.build(g);
        dfs
    }

    fn build(&mut self, g: &DiGraph) {
        self.dfs(g, self.source_vertex);
    }

    fn dfs(&mut self, g: &DiGraph, v: usize) {
        self.marked[v] = true;

        // 🚀 Loops strictly over neighbors reachable FROM v (v -> w)
        for &w in g.adj(v) {
            if !self.marked[w] {
                self.edge_to[w] = Some(v);
                self.dfs(g, w);
            }
        }
    }

    pub fn has_path_to(&self, v: usize) -> bool {
        self.marked[v]
    }

    pub fn path_to(&self, v: usize) -> Option<Stack<usize>> {
        if !self.has_path_to(v) {
            return None;
        }
        let mut path = Stack::<usize>::new();
        let mut current = v;

        while let Some(parent) = self.edge_to[current] {
            path.push(current);
            current = parent;
        }

        path.push(current);
        Some(path)
    }
}
