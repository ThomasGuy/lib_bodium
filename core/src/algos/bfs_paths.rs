use crate::data_containers::{Graph, Stack};
use std::collections::VecDeque;
use std::fmt::Display;

pub struct BreadthFirstPaths {
    marked: Vec<bool>,
    edge_to: Vec<i32>,
    source: i32,
    total_vertices: i32,
}

impl BreadthFirstPaths {
    pub fn new(g: &Graph, source: i32) -> Self {
        Self {
            marked: vec![false; g.vertex() as usize],
            edge_to: vec![-1; g.vertex() as usize],
            source,
            total_vertices: g.vertex(),
        }
    }

    pub fn build(&mut self, g: &Graph) {
        self.bfs(g, self.source);
    }

    fn bfs(&mut self, g: &Graph, s: i32) {
        let mut queue = VecDeque::<i32>::new();
        self.marked[s as usize] = true;
        queue.push_back(s);

        while let Some(v) = queue.pop_front() {
            for w in g.adj(v) {
                if !self.marked[*w as usize] {
                    self.edge_to[*w as usize] = v;
                    self.marked[*w as usize] = true;
                    queue.push_back(*w);
                }
            }
        }
    }

    pub fn has_path_to(&self, v: i32) -> bool {
        self.marked[v as usize]
    }

    pub fn path_to(&self, v: i32) -> Option<Stack<i32>> {
        if !self.has_path_to(v) {
            return None;
        }
        let mut path = Stack::<i32>::new();
        let mut x = v;
        while x != self.source {
            path.push(x);
            x = self.edge_to[x as usize];
        }
        path.push(self.source);
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
