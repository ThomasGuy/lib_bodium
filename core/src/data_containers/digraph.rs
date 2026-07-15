use crate::data_containers::Bag;

/// A Directed Graph (Digraph) structural container representing one-way vertices.
#[derive(Debug, Clone)]
pub struct Digraph {
    vertices: usize,
    edges: usize,
    adj: Vec<Bag<usize>>, // Array of Bags holding directional target indices
}

impl Digraph {
    /// Initializes an empty Digraph with V vertices and 0 edges
    pub fn new(vertices: usize) -> Self {
        let mut adj = Vec::with_capacity(vertices);
        for _ in 0..vertices {
            adj.push(Bag::new());
        }
        Self {
            vertices,
            edges: 0,
            adj,
        }
    }

    /// 🚀 THE CORE DIFFERENCE: Adds a directed edge from v to w (v -> w)
    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.adj[v].add(w); // Push w into v's directional bag
        self.edges += 1; // Note: We do NOT push v into w's bag!
    }

    pub fn vertices(&self) -> usize {
        self.vertices
    }

    pub fn edges(&self) -> usize {
        self.edges
    }

    /// Returns a shared reference iterator over all vertices adjacent from v
    pub fn adj(&self, v: usize) -> &Bag<usize> {
        &self.adj[v]
    }

    /// 🔄 Returns the reverse view of this digraph.
    /// Inverting all edges (w -> v) is crucial for advanced connectivity checks!
    pub fn reverse(&self) -> Self {
        let mut rev = Self::new(self.vertices);
        for v in 0..self.vertices {
            for &w in &self.adj[v] {
                rev.add_edge(w, v); // Flip the direction
            }
        }
        rev
    }
}
