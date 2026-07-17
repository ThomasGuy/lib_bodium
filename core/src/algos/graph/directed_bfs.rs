use crate::data_containers::{DiGraph, Stack};

pub struct DirectedBFS {
    marked: Vec<bool>,
    edge_to: Vec<Option<usize>>,
    source_vertex: usize,
}

impl DirectedBFS {
    pub fn new(g: &DiGraph, source_vertex: usize) -> Self {
        let mut bfs = Self {
            marked: vec![false; g.vertices()],
            edge_to: vec![None; g.vertices()],
            source_vertex,
        };
        bfs.build(g);
        bfs
    }

    fn build(&mut self, g: &DiGraph) {
        // Simple working local queue layout using Vec
        let mut queue = Vec::new();

        self.marked[self.source_vertex] = true;
        queue.push(self.source_vertex);

        while !queue.is_empty() {
            let v = queue.remove(0); // Pop from front to ensure radial expansion

            for &w in g.adj(v) {
                if !self.marked[w] {
                    self.edge_to[w] = Some(v);
                    self.marked[w] = true;
                    queue.push(w);
                }
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
