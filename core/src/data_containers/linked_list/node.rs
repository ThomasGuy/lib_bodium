use super::Link;

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Node<T> {
    pub(crate) item: T,
    pub(crate) next: Link<T>,
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "Node {{ item: {}, next: {} }}",
            self.item,
            self.next.as_ref().unwrap().item
        )
    }
}
