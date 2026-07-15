use super::node::{Link, Node};
use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Tree<K, V>(pub(crate) Link<K, V>);

impl<K, V> Tree<K, V>
where
    K: Ord + Clone + Display,
    V: Clone + Display,
{
    pub(crate) fn new() -> Self {
        Tree(None)
    }

    pub(crate) fn put(&mut self, key: &K, val: &V) {
        if let Some(ref mut node) = self.0 {
            match key.cmp(&node.key) {
                Ordering::Less => {
                    node.left.put(key, val);
                    node.node_count = node.right.size() + node.left.size() + 1;
                }
                Ordering::Greater => {
                    node.right.put(key, val);
                    node.node_count = node.right.size() + node.left.size() + 1;
                }
                Ordering::Equal => node.val = val.clone(),
            }
        } else {
            self.0 = Some(Box::new(Node::new(key.clone(), val.clone(), 1)));
        }
    }

    pub(crate) fn get(&self, key: &K) -> Option<&Node<K, V>> {
        let mut current = self;
        while let Some(node) = &current.0 {
            match key.cmp(&node.key) {
                Ordering::Less => current = &node.left,
                Ordering::Greater => current = &node.right,
                Ordering::Equal => return Some(node),
            }
        }
        None
    }

    // 🚀 Returns a clean reference instead of forcing a deep clone
    pub(crate) fn min_ref(&self) -> Option<&Node<K, V>> {
        let mut current = self;
        while let Some(node) = &current.0 {
            if node.left.0.is_none() {
                return Some(node);
            }
            current = &node.left;
        }
        None
    }

    // 🚀 True Hibbard Helper: Detaches the absolute minimum node from the tree and returns it!
    fn remove_min(&mut self) -> Link<K, V> {
        if let Some(mut node) = self.0.take() {
            if node.left.0.is_none() {
                // No left child! This node is the minimum. Return its right child to repair parent link
                self.0 = node.right.0.take();
                node.node_count = 1;
                Some(node)
            } else {
                // Drill down left branch
                let min_node = node.left.remove_min();
                node.node_count = node.right.size() + node.left.size() + 1;
                self.0 = Some(node);
                min_node
            }
        } else {
            None
        }
    }

    // 🚀 Pristine Hibbard Deletion Engine
    pub(crate) fn remove(&mut self, key: &K) {
        if let Some(mut node) = self.0.take() {
            match key.cmp(&node.key) {
                Ordering::Less => {
                    node.left.remove(key);
                    node.node_count = node.right.size() + node.left.size() + 1;
                    self.0 = Some(node);
                }
                Ordering::Greater => {
                    node.right.remove(key);
                    node.node_count = node.right.size() + node.left.size() + 1;
                    self.0 = Some(node);
                }
                Ordering::Equal => {
                    match (node.left.0.is_none(), node.right.0.is_none()) {
                        (true, true) => self.0 = None,
                        (true, false) => self.0 = node.right.0.take(),
                        (false, true) => self.0 = node.left.0.take(),
                        (false, false) => {
                            // Two children! Find successor (min of right branch)
                            let mut successor = node.right.remove_min().unwrap();
                            successor.left = Tree(node.left.0.take());
                            successor.right = Tree(node.right.0.take());
                            successor.node_count =
                                successor.right.size() + successor.left.size() + 1;
                            self.0 = Some(successor);
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn size(&self) -> u32 {
        if let Some(node) = &self.0 {
            return node.node_count;
        }
        0
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub(crate) fn keys(&self, que: &mut Vec<K>) {
        if let Some(node) = &self.0 {
            node.left.keys(que);
            que.push(node.key.clone());
            node.right.keys(que);
        }
    }

    pub(crate) fn keys_ref<'a>(&'a self, que: &mut Vec<&'a K>) {
        if let Some(node) = &self.0 {
            node.left.keys_ref(que);
            que.push(&node.key);
            node.right.keys_ref(que);
        }
    }

    /// Collects all nodes in-order via shared memory references
    pub(crate) fn nodes<'a>(&'a self, que: &mut Vec<&'a Node<K, V>>) {
        if let Some(node) = &self.0 {
            node.left.nodes(que);
            que.push(node.as_ref()); // Pushes a lightweight reference instead of cloning!
            node.right.nodes(que);
        }
    }

    pub(crate) fn floor(&self, key: &K) -> Option<K> {
        match &self.0 {
            None => None,
            Some(node) => match key.cmp(&node.key) {
                Ordering::Equal => Some(node.key.clone()),
                Ordering::Less => node.left.floor(key),
                Ordering::Greater => match node.right.floor(key) {
                    None => Some(node.key.clone()),
                    Some(result) => Some(result),
                },
            },
        }
    }

    pub(crate) fn contains(&self, key: &K) -> bool {
        let mut current = self;
        while let Some(node) = &current.0 {
            match key.cmp(&node.key) {
                Ordering::Less => current = &node.left,
                Ordering::Greater => current = &node.right,
                Ordering::Equal => return true,
            }
        }
        false
    }

    pub(crate) fn select(&self, k: u32) -> Option<&Node<K, V>> {
        match &self.0 {
            None => None,
            Some(node) => {
                let t = node.left.size();
                match k.cmp(&t) {
                    Ordering::Less => node.left.select(k),
                    Ordering::Greater => node.right.select(k - t - 1),
                    Ordering::Equal => Some(node.as_ref()),
                }
            }
        }
    }

    pub(crate) fn rank(&self, key: &K) -> u32 {
        match &self.0 {
            None => 0,
            Some(node) => match key.cmp(&node.key) {
                Ordering::Less => node.left.rank(key),
                Ordering::Equal => node.left.size(),
                Ordering::Greater => node.left.size() + node.right.rank(key) + 1,
            },
        }
    }
}
