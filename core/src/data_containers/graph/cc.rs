use super::Graph;

pub struct ConnectedComponents {
    marked: Vec<bool>,
    id: Vec<usize>, // Tracks which component ID each vertex belongs to
    count: usize,   // Total number of connected components discovered
}

impl ConnectedComponents {
    /// Discovers all isolated components inside the graph
    pub(super) fn new(g: &Graph) -> Self {
        let vertex_count = g.vertex() as usize;
        let mut cc = Self {
            marked: vec![false; vertex_count],
            id: vec![0; vertex_count],
            count: 0,
        };

        // Run DFS on every unmarked vertex to find separate clusters
        for v in 0..g.vertex() {
            if !cc.marked[v as usize] {
                cc.dfs(g, v);
                cc.count += 1; // Increment ID tag for the next independent cluster
            }
        }

        cc
    }

    /// Internal recursive DFS worker that tags an entire cluster
    fn dfs(&mut self, g: &Graph, v: i32) {
        self.marked[v as usize] = true;
        self.id[v as usize] = self.count;

        for w in g.adj(v) {
            if !self.marked[*w as usize] {
                self.dfs(g, *w);
            }
        }
    }

    /// 🚀 Groups all vertex indices by their component ID into a list of vectors.
    pub(super) fn groups(&self) -> (i32, Vec<Vec<usize>>) {
        let mut components = vec![Vec::new(); self.count];
        for v in 0..self.marked.len() {
            components[self.id[v]].push(v);
        }

        (self.count as i32, components)
    }
}
