use crate::data_containers::graph::GraphError;
use crate::data_containers::{Bag, Stack};

use super::directed_cycle;

/// A directed graph (or digraph) is a set of vertices and a collection of directed
/// edges. Each directed edge connects an ordered pair of vertices.
#[derive(Debug, Clone)]
pub struct DiGraph {
    vertex: usize,
    edges: usize,
    adj: Vec<Bag<usize>>, // Array of Bags holding directional target indices
}

impl DiGraph {
    /// Initializes an empty Digraph with V vertices and 0 edges
    pub fn new(vertex: usize) -> Self {
        let mut adj = Vec::with_capacity(vertex);
        for _ in 0..vertex {
            adj.push(Bag::new());
        }
        Self {
            vertex,
            edges: 0,
            adj,
        }
    }

    /// Safely builds your digraph from a raw integer iterator stream.
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
            self.edges += 1;
        }

        Ok(())
    }

    /// 🚀 THE CORE DIFFERENCE: Adds a directed edge from v to w (v -> w)
    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.adj[v].add(w); // Push w into v's directional bag
        self.edges += 1; // Note: We do NOT push v into w's bag!
    }

    pub fn vertices(&self) -> usize {
        self.vertex
    }

    pub fn edges(&self) -> usize {
        self.edges
    }

    /// Returns a shared reference iterator over all vertex adjacent from v
    pub fn adj(&self, v: usize) -> &Bag<usize> {
        &self.adj[v]
    }

    /// 🔄 Returns the reverse view of this digraph.
    /// Inverting all edges (w -> v) is crucial for advanced connectivity checks!
    pub fn reverse(&self) -> Self {
        let mut rev = Self::new(self.vertex);
        for v in 0..self.vertex {
            for &w in &self.adj[v] {
                rev.add_edge(w, v); // Flip the direction
            }
        }
        rev
    }

    pub fn cycle(&self) -> Option<Stack<usize>> {
        let detector = directed_cycle::DirectedCycle::new(self);
        detector.cycle().cloned()
    }
}

#[cfg(test)]
mod tests {
    use crate::algos::{DirectedBFS, DirectedDFS};
    use crate::data_containers::DiGraph;
    use pretty_assertions::assert_eq;

    /// Helper to instantiate a safe, acyclic directed graph (DAG)
    /// 0 -> 1 -> 2 -> 3
    /// 0 -> 2
    fn create_safe_digraph() -> DiGraph {
        let mut g = DiGraph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(0, 2);
        g
    }

    #[test]
    fn test_directed_cycle_and_builder_gate() {
        let mut g = create_safe_digraph();

        // 1. Confirm that a standard chronological pipeline has zero cycles
        assert_eq!(
            g.cycle().is_some(),
            false,
            "Acyclic graph should return None"
        );

        // 2. Introduce a backward-pointing edge to create a feedback loop: 3 -> 1
        // Loop sequence: 1 -> 2 -> 3 -> 1
        g.add_edge(3, 1);

        assert_eq!(
            g.cycle().is_some(),
            true,
            "Cycle detector should flag the circular loop"
        );

        // Unpack the deadlock stack to ensure it recorded the correct trap path
        let deadlock = g.cycle().unwrap();
        assert!(deadlock.size() > 0);
    }

    #[test]
    fn test_directed_dfs_one_way_reachability() {
        let g = create_safe_digraph();
        let dfs = DirectedDFS::new(&g, 0);

        // 1. Verify standard forward reachability matches arrows
        assert!(dfs.has_path_to(3));
        assert!(dfs.has_path_to(1));

        // 2. Verify strict asymmetry: Node 3 cannot reach back to Node 0!
        let dfs_reverse = DirectedDFS::new(&g, 3);
        assert_eq!(
            dfs_reverse.has_path_to(0),
            false,
            "One-way arrows must prevent reverse navigation"
        );
    }

    #[test]
    fn test_directed_bfs_shortest_path_guarantee() {
        let g = create_safe_digraph();
        let bfs = DirectedBFS::new(&g, 0);

        // 1. Verify node reachability via radial expansion
        assert!(bfs.has_path_to(2));

        // 2. Shortest Path Proof:
        // DFS might take the long route (0 -> 1 -> 2), but BFS must prioritize
        // the direct shortcut hop edge (0 -> 2), which requires exactly 2 nodes!
        let shortest_path = bfs.path_to(2).unwrap();
        assert_eq!(
            shortest_path.size(),
            2,
            "BFS must discover the direct shortcut path"
        );

        let mut path_order = Vec::new();
        for node in shortest_path {
            path_order.push(node);
        }
        assert_eq!(
            path_order,
            vec![0, 2],
            "Shortest path sequence must be exactly 0 then 2"
        );
    }
}
