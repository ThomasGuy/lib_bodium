use super::Link;

use std::fmt::Display;

#[derive(Debug, Clone)]
pub(crate) struct Node<T>
where
    T: Display,
{
    pub(crate) item: T,
    pub(crate) next: Link<T>,
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Node {{ item: {} }}", self.item)
    }
}
