use crate::data_containers::bag::Bag;
use std::fmt::Display;
use thiserror::Error;

use super::cc;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum GraphError {
    #[error("Missing data on edge {edge} (Expected vertex coordinates)")]
    NoData { edge: usize },
    #[error(
        "Vertex index out of bounds on edge {edge}: Point {vertex} exceeds graph boundary of {total_vertices}"
    )]
    OutOfBounds {
        edge: usize,
        vertex: usize,
        total_vertices: usize,
    },
    #[error("Vertex index cannot be negative")]
    NegativeInt,
}

/// An Undirected Graph representation.
/// A graph is a set of vertices and a collection of edges that each connect a
/// pair of vertices.
#[derive(Debug, Clone)]
pub struct Graph {
    /// Total number of vertices allocated in the graph.
    vertex: usize,
    /// Current count of edges connecting vertices.
    edges: usize,
    /// An adjacency list where each index represents a vertex mapping to a
    /// [`Bag`] of its adjacent neighboring vertices.
    adj: Vec<Bag<usize>>,
}

impl Graph {
    pub fn new(vertex: usize) -> Self {
        let adj = vec![Bag::<usize>::new(); vertex];
        Self {
            vertex,
            edges: 0,
            adj,
        }
    }

    /// Safely builds your graph from a raw integer iterator stream.
    pub fn build(
        &mut self,
        expected_edges: usize,
        mut iter: std::vec::IntoIter<i32>,
    ) -> Result<(), GraphError> {
        for idx in 0..expected_edges {
            let current_edge = idx + 1;
            let v_i32 = iter
                .next()
                .ok_or(GraphError::NoData { edge: current_edge })?;
            let v = v_i32.try_into().map_err(|_| GraphError::NegativeInt)?;

            let w_i32 = iter
                .next()
                .ok_or(GraphError::NoData { edge: current_edge })?;
            let w = w_i32.try_into().map_err(|_| GraphError::NegativeInt)?;

            // Prevent Out of Bounds Panics before hitting the vector array
            if v >= self.vertex {
                return Err(GraphError::OutOfBounds {
                    edge: current_edge,
                    vertex: v,
                    total_vertices: self.vertex,
                });
            }
            if w >= self.vertex {
                return Err(GraphError::OutOfBounds {
                    edge: current_edge,
                    vertex: w,
                    total_vertices: self.vertex,
                });
            }

            self.adj[v].add(w);
            self.adj[w].add(v);
            self.edges += 1;
        }

        Ok(())
    }

    pub fn adj(&self, v: usize) -> &Bag<usize> {
        &self.adj[v]
    }

    pub fn vertex(&self) -> usize {
        self.vertex
    }

    pub fn edges(&self) -> usize {
        self.edges
    }

    pub fn get_cc(&self) -> (usize, Vec<Vec<usize>>) {
        cc::ConnectedComponents::new(self).groups()
    }
}

// Zero-allocation string formatter streaming directly to the buffer output
impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "A Graph has {} vertices, {} edges",
            self.vertex, self.edges
        )?;
        for v in 0..self.vertex {
            write!(f, "{} : ", v)?;
            // Automatically matches your Bag's Display format
            writeln!(f, "{}", self.adj(v))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Bring Graph, DepthFirstPaths into scope
    use crate::algos::{BreadthFirstPaths, DepthFirstPaths}; // Ensure BFS is imported
    use pretty_assertions::assert_eq;

    fn create_test_graph() -> Graph {
        // 0 -> 1 -> 4 -> 3
        // 0 -> 2 -> 3
        let mut g = Graph::new(5);
        let stream = vec![0, 1, 0, 2, 1, 4, 2, 3, 4, 3];
        g.build(5, stream.into_iter()).unwrap();
        g
    }

    #[test]
    fn test_dfs_exact_path_reconstruction() {
        let g = create_test_graph();
        let mut dfs = DepthFirstPaths::new(&g, 0);
        dfs.build(&g);

        // 1. Verify basic connectivity reachability flags
        assert!(dfs.has_path_to(3));
        assert!(dfs.has_path_to(4));
        assert_eq!(dfs.is_connected(), true);

        // 2. Dynamic Unpacking: Read whatever valid path your DFS stack actually took
        let path = dfs.path_to(3).unwrap();
        let actual_size = path.size();

        // Ensure it's a valid path size (it will be 3 or 4 depending on adjacency order)
        assert!(
            actual_size == 3 || actual_size == 4,
            "Path size was unexpected: {}",
            actual_size
        );

        let mut path_order = Vec::new();
        for node in path {
            path_order.push(node);
        }

        // 3. Match the assertions dynamically to validate the exact sequence taken
        if actual_size == 3 {
            // If DFS evaluated vertex 2 first, it goes straight to 3
            assert_eq!(path_order, vec![0, 2, 3]);
        } else {
            // If DFS evaluated vertex 1 and vertex 4 first, it takes the long scenic route
            assert_eq!(path_order, vec![0, 1, 4, 3]);
        }
    }

    #[test]
    fn test_bfs_shortest_path_guarantee() {
        let g = create_test_graph();
        let mut bfs = BreadthFirstPaths::new(&g, 0);
        bfs.build(&g);

        assert!(bfs.has_path_to(3));

        // 🚀 BFS Shortest Path Proof:
        // While DFS took the long route (0-1-4-3), BFS expands radially.
        // It guarantees finding the shortest hop path to 3 via 2 (0 -> 2 -> 3),
        // which takes exactly 3 nodes!
        let path = bfs.path_to(3).unwrap();
        assert_eq!(path.size(), 3);

        // Unpack the Stack to verify the shortest path layout (0 -> 2 -> 3)
        let mut path_order = Vec::new();
        for node in path {
            path_order.push(node);
        }
        assert_eq!(path_order, vec![0, 2, 3]);
    }

    #[test]
    fn test_disconnected_graph_boundaries() {
        // Create an explicitly disconnected graph (Vertices 0, 1, 2 connected; 3, 4 isolated)
        let mut g = Graph::new(5);
        let stream = vec![0, 1, 1, 2];
        g.build(2, stream.into_iter()).unwrap();

        let mut dfs = DepthFirstPaths::new(&g, 0);
        dfs.build(&g);

        // 1. Verify connected components behave correctly
        assert_eq!(dfs.is_connected(), false); // Graph is split!
        assert!(dfs.has_path_to(2));

        // 2. Verify isolated boundaries gracefully return None without panicking
        assert_eq!(dfs.has_path_to(3), false);
        assert_eq!(dfs.path_to(3), None); // No path = None stack container
    }
}
