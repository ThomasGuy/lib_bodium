use crate::data_containers::{Graph, Stack};
use std::fmt::Display;

pub struct DepthFirstPaths {
    marked: Vec<bool>,
    edge_to: Vec<Option<usize>>,
    source_vertex: usize,
    count: usize,
    total_vertices: usize, // Track vertex limit for loops/Display
}

impl DepthFirstPaths {
    // 1. Pass the graph as a reference without cloning it into the struct
    pub fn new(g: &Graph, source_vertex: usize) -> Self {
        Self {
            marked: vec![false; g.vertex()],
            edge_to: vec![None; g.vertex()],
            source_vertex,
            count: 0,
            total_vertices: g.vertex(),
        }
    }

    pub fn build(&mut self, g: &Graph) {
        self.dfs(g, self.source_vertex);
    }

    fn dfs(&mut self, g: &Graph, sv: usize) {
        self.marked[sv] = true;
        self.count += 1;

        // Now g is borrowed independently from `self`, making recursion legal!
        for w in g.adj(sv) {
            if !self.marked[*w] {
                self.edge_to[*w] = Some(sv);
                self.dfs(g, *w);
            }
        }
    }

    pub fn has_path_to(&self, v: usize) -> bool {
        self.marked[v]
    }

    /*
       A graph is connected if there is a path from every vertex to every other vertex
       in the graph. A graph that is not connected consists of a set of connected components,
       which are maximal connected subgraphs.
    */
    pub fn is_connected(&self) -> bool {
        self.count == self.total_vertices
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

impl Display for DepthFirstPaths {
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

#[cfg(test)]
mod tests {
    use crate::algos::{BreadthFirstPaths, DepthFirstPaths};
    use crate::data_containers::Graph;

    fn create_test_graph() -> Graph {
        // 0 -> 1-> 3 and 0 -> 2 -> 4 -> 3
        let mut g = Graph::new(5);
        let stream = vec![0, 1, 0, 2, 1, 4, 2, 3, 4, 3];
        g.build(5, stream.into_iter()).unwrap();
        g
    }

    #[test]
    fn test_bfs_shortest_path() {
        let g = create_test_graph();
        let mut bfs = BreadthFirstPaths::new(&g, 0);
        bfs.build(&g);

        assert!(bfs.has_path_to(3));

        // BFS guarantees the shortest path (0-2-3), which is 3 elements long
        let path = bfs.path_to(3).unwrap();
        assert_eq!(path.size(), 3);
    }

    #[test]
    fn test_dfs_connectivity() {
        let g = create_test_graph();
        let mut dfs = DepthFirstPaths::new(&g, 0);
        dfs.build(&g);

        assert!(dfs.is_connected(), "Graph should be fully connected");
        assert!(dfs.has_path_to(3));
    }
}
