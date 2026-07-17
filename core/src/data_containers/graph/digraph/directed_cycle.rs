use crate::data_containers::{DiGraph, Stack};

pub struct DirectedCycle {
    marked: Vec<bool>,
    edge_to: Vec<Option<usize>>,
    on_stack: Vec<bool>,
    cycle: Option<Stack<usize>>,
}

impl DirectedCycle {
    pub(super) fn new(g: &DiGraph) -> Self {
        let mut detector = Self {
            marked: vec![false; g.vertices()],
            edge_to: vec![None; g.vertices()],
            on_stack: vec![false; g.vertices()],
            cycle: None,
        };

        for v in 0..g.vertices() {
            if !detector.marked[v] && detector.cycle.is_none() {
                detector.dfs(g, v);
            }
        }

        detector
    }

    fn dfs(&mut self, g: &DiGraph, v: usize) {
        self.marked[v] = true;
        self.on_stack[v] = true;

        for &w in g.adj(v) {
            if self.cycle.is_some() {
                return;
            }

            if !self.marked[w] {
                self.edge_to[w] = Some(v);
                self.dfs(g, w);
            } else if self.on_stack[w] {
                let mut cycle_stack = Stack::new();
                let mut current = v;

                while Some(current) != Some(w) {
                    if let Some(parent) = self.edge_to[current] {
                        cycle_stack.push(current);
                        current = parent;
                    } else {
                        break;
                    }
                }
                cycle_stack.push(w);
                cycle_stack.push(v);
                self.cycle = Some(cycle_stack);
            }
        }

        self.on_stack[v] = false;
    }

    // pub(super) fn has_cycle(&self) -> bool {
    //     self.cycle.is_some()
    // }

    pub(super) fn cycle(&self) -> Option<&Stack<usize>> {
        self.cycle.as_ref()
    }
}
