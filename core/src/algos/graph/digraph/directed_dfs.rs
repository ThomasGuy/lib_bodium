use std::fmt::Display;

use crate::data_containers::{DiGraph, Stack};

pub struct DirectedDFS {
    total_vertices: usize,
    marked: Vec<bool>,
    edge_to: Vec<Option<usize>>,
    source_vertex: usize,
}

impl DirectedDFS {
    pub fn new(g: &DiGraph, source_vertex: usize) -> Self {
        let mut dfs = Self {
            total_vertices: g.vertices(),
            marked: vec![false; g.vertices()],
            edge_to: vec![None; g.vertices()],
            source_vertex,
        };
        dfs.build(g);
        dfs
    }

    pub fn build(&mut self, g: &DiGraph) {
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

impl Display for DirectedDFS {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "Vertices connected to source vertex {}",
            self.source_vertex
        )?;
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
