use crate::data_containers::bag::Bag;
use std::fmt::Display;
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum GraphError {
    #[error("Missing data on edge {edge} (Expected vertex coordinates)")]
    NoData { edge: i32 },
    #[error(
        "Vertex index out of bounds on edge {edge}: Point {vertex} exceeds graph boundary of {total_vertices}"
    )]
    OutOfBounds {
        edge: i32,
        vertex: i32,
        total_vertices: i32,
    },
}

/*
    A graph is a set of vertices and a collection of edges that each connect a
    pair of vertices.
*/
#[derive(Clone)]
pub struct Graph {
    pub vertex: i32, // verticies
    edges: i32,      // edges
    adj: Vec<Bag<i32>>,
}

impl Graph {
    pub fn new(vertex: i32) -> Self {
        let adj = vec![Bag::<i32>::new(); vertex as usize];
        Self {
            vertex,
            edges: 0,
            adj,
        }
    }
    /// Safely builds your graph from a raw integer iterator stream.
    pub fn build(
        &mut self,
        expected_edges: i32,
        mut iter: std::vec::IntoIter<i32>,
    ) -> Result<(), GraphError> {
        for idx in 0..expected_edges {
            let current_edge = idx + 1;
            // 1. Safely extract your vertex pairs
            let v = iter
                .next()
                .ok_or(GraphError::NoData { edge: current_edge })?;
            let w = iter
                .next()
                .ok_or(GraphError::NoData { edge: current_edge })?;

            // 2. Prevent Out of Bounds Panics before hitting the vector array
            if v >= self.vertex || v < 0 {
                return Err(GraphError::OutOfBounds {
                    edge: current_edge,
                    vertex: v,
                    total_vertices: self.vertex,
                });
            }
            if w >= self.vertex || w < 0 {
                return Err(GraphError::OutOfBounds {
                    edge: current_edge,
                    vertex: w,
                    total_vertices: self.vertex,
                });
            }

            self.adj[v as usize].add(w);
            self.adj[w as usize].add(v);
            self.edges += 1;
        }

        Ok(())
    }

    pub fn adj(&self, v: i32) -> &Bag<i32> {
        &self.adj[v as usize]
    }

    pub fn edges(&self) -> i32 {
        self.edges
    }
}

// 4. Zero-allocation string formatter streaming directly to the buffer output
impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "A Graph has {} vertices, {} edges",
            self.vertex, self.edges
        )?;
        for v in 0..self.vertex {
            write!(f, "{} : ", v)?;
            writeln!(f, "{}", self.adj(v))?; // Automatically matches your Bag's Display format
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn basic() {}
}
