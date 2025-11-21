use crate::bag::Bag;
use std::{fmt::Display, vec::IntoIter};

pub struct Graph {
    pub v: i32,
    e: i32,
    adj: Vec<Bag<i32>>,
}

impl Graph {
    pub fn new(v: i32) -> Self {
        let adj = vec![Bag::<i32>::new(); v as usize];
        Self { v, e: 0, adj }
    }

    pub fn build(&mut self, mut iter: IntoIter<i32>) {
        let edge = iter.next().unwrap();
        for _ in 0..edge {
            let v = iter.next().unwrap();
            let w = iter.next().unwrap();
            self.add_edge(v, w);
        }
    }

    fn add_edge(&mut self, v: i32, w: i32) {
        self.adj[v as usize].add(w);
        self.adj[w as usize].add(v);
        self.e += 1;
    }

    pub fn adj_ref(&self, v: i32) -> Bag<i32> {
        self.adj[v as usize].clone()
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = format!("{} vertices, {} edges\n", self.v, self.e);
        for v in 0..self.v {
            s = format!("{}{} : ", s, v);
            s = format!("{}{}", s, self.adj_ref(v).to_string());
        }
        writeln!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn basic() {}
}
