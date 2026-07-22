use std::fmt::Display;

use crate::data_containers::{DiGraph, Stack};

pub struct DirectedBFS {
    total_vertices: usize,
    marked: Vec<bool>,
    edge_to: Vec<Option<usize>>,
    source_vertex: usize,
}

impl DirectedBFS {
    pub fn new(g: &DiGraph, source_vertex: usize) -> Self {
        let mut bfs = Self {
            total_vertices: g.vertices(),
            marked: vec![false; g.vertices()],
            edge_to: vec![None; g.vertices()],
            source_vertex,
        };
        bfs.build(g);
        bfs
    }

    pub fn build(&mut self, g: &DiGraph) {
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

impl Display for DirectedBFS {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Shortest path from source vertex {}", self.source_vertex)?;
        for v in 0..self.total_vertices {
            if self.has_path_to(v) {
                write!(f, "{} to {} : ", self.source_vertex, v)?;
                if let Some(data) = self.path_to(v) {
                    let mut iter = data.into_iter().peekable();
                    while let Some(x) = iter.next() {
                        write!(f, "{}", x)?;
                        if iter.peek().is_some() {
                            write!(f, "-")?;
                        }
                    }
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
