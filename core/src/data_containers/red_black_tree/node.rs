use crate::data_containers::red_black_tree::Tree;
use std::fmt::Display;

pub type Link<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Colour {
    Red,
    Black,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node<K, V> {
    pub key: K,
    pub val: V,
    pub colour: Colour,
    pub(crate) left: Tree<K, V>,
    pub(crate) right: Tree<K, V>,
    pub(crate) node_count: u32,
}

impl<K, V> Node<K, V>
where
    K: Ord + Clone + Display,
    V: Clone + Display,
{
    pub fn new(key: K, val: V, node_count: u32) -> Self {
        Node {
            key,
            val,
            colour: Colour::Red,
            left: Tree::new(),
            right: Tree::new(),
            node_count,
        }
    }
}

impl<K: Display, V: Display> Display for Node<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Node {{ key: {}, value: {} }}", self.key, self.val)
    }
}
