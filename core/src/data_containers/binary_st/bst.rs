use super::Tree;
use super::node::Node;
use std::fmt::Display;

#[derive(Debug)]
pub struct BinarySearchTree<K, V> {
    root: Tree<K, V>,
}

impl<K, V> Default for BinarySearchTree<K, V>
where
    K: Ord + Clone + Display,
    V: Clone + Display,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> BinarySearchTree<K, V>
where
    K: Ord + Clone + Display,
    V: Clone + Display,
{
    pub fn new() -> Self {
        BinarySearchTree { root: Tree::new() }
    }

    pub fn put(&mut self, key: K, val: V) {
        // Pass as references matching your optimized Tree signature
        self.root.put(&key, &val);
    }

    // 🚀 Fixed: Returns a read-only reference directly from memory,
    // saving immense CPU cycles by skipping deep structural clones!
    pub fn get(&self, key: &K) -> Option<&Node<K, V>> {
        self.root.get(key)
    }

    pub fn delete(&mut self, key: K) {
        // Pass as reference matching your optimized Tree signature
        self.root.remove(&key);
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
    }

    pub fn contains(&self, key: K) -> bool {
        // Pass as reference matching your optimized Tree signature
        self.root.contains(&key)
    }

    pub fn size(&self) -> u32 {
        self.root.size()
    }

    // pub fn nodes(&self) -> Vec<&Node<K, V>> {
    //     let mut queue: Vec<&Node<K, V>> = Vec::new();
    //     self.root.nodes(&mut queue)
    // }

    pub fn keys(&self) -> Vec<K> {
        let mut queue: Vec<K> = Vec::new();
        self.root.keys(&mut queue)
    }

    pub fn iter_keys(&self) -> std::vec::IntoIter<K> {
        let mut queue: Vec<K> = Vec::new();
        self.root.keys(&mut queue).into_iter()
    }

    pub fn floor(&self, key: K) -> Option<K> {
        // Pass as reference matching your optimized Tree signature
        self.root.floor(&key)
    }

    // 🚀 Fixed: Returns a clean reference matching Tree::select
    pub fn select(&self, x: u32) -> Option<&Node<K, V>> {
        self.root.select(x)
    }

    pub fn rank(&self, x: K) -> u32 {
        // Pass as reference matching your optimized Tree signature
        self.root.rank(&x)
    }
}
