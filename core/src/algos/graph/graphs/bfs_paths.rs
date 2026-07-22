use crate::data_containers::{Graph, Stack};
use std::collections::VecDeque;
use std::fmt::Display;

pub struct BreadthFirstPaths {
    marked: Vec<bool>,
    edge_to: Vec<Option<usize>>,
    source: usize,
    total_vertices: usize,
}

impl BreadthFirstPaths {
    pub fn new(g: &Graph, source: usize) -> Self {
        Self {
            marked: vec![false; g.vertex()],
            edge_to: vec![None; g.vertex()],
            source,
            total_vertices: g.vertex(),
        }
    }

    pub fn build(&mut self, g: &Graph) {
        self.bfs(g, self.source);
    }

    fn bfs(&mut self, g: &Graph, s: usize) {
        let mut queue = VecDeque::<usize>::new();
        self.marked[s] = true;
        queue.push_back(s);

        while let Some(v) = queue.pop_front() {
            for w in g.adj(v) {
                if !self.marked[*w] {
                    self.edge_to[*w] = Some(v);
                    self.marked[*w] = true;
                    queue.push_back(*w);
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

        let mut path = Stack::new();
        let mut current = v;

        // 🚀 Walk backwards along the parent links until we hit the source vertex
        // (The source vertex remains `None` in the edge_to array!)
        while let Some(parent) = self.edge_to[current] {
            path.push(current);
            current = parent; // Step back to the next parent index
        }

        // Push the final source vertex onto the stack
        path.push(current);
        Some(path)
    }
}

impl Display for BreadthFirstPaths {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Shortest path from source vertex {}", self.source)?;
        for v in 0..self.total_vertices {
            if self.has_path_to(v) {
                write!(f, "{} to {} : ", self.source, v)?;
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
