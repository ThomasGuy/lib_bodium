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
    fn _min_ref(&self) -> Option<&Node<K, V>> {
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

    pub(crate) fn keys(&self, que: &mut Vec<K>) -> Vec<K> {
        if let Some(node) = &self.0 {
            node.left.keys(que);
            que.push(node.key.clone());
            node.right.keys(que);
        }
        que.clone()
    }

    /// Collects all nodes in-order via shared memory references
    pub(crate) fn nodes<'a>(&'a self, que: &mut Vec<&'a Node<K, V>>) -> Vec<&'a Node<K, V>> {
        if let Some(node) = &self.0 {
            node.left.nodes(que);
            que.push(node.as_ref()); // Pushes a lightweight reference instead of cloning!
            node.right.nodes(que);
        }
        que.clone()
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

#[cfg(test)]
mod tests {
    use crate::data_containers::binary_tree;
    use pretty_assertions::{self, assert_eq};

    #[test]
    fn put_get_size_contains_delete() {
        let mut bst = binary_tree::BinarySearchTree::new();
        bst.put(18, "Tom");
        bst.put(25, "Na");
        bst.put(16, "Simon");
        bst.put(17, "Adrian");
        bst.put(9, "Bella");
        bst.put(12, "Sally");
        bst.put(14, "Jim");
        bst.put(7, "Ed");
        bst.put(11, "Nick");

        assert_eq!(bst.size(), 9 as u32);
        assert_eq!(bst.get(&17).unwrap().val, "Adrian");
        assert_eq!(bst.size(), bst.get(&18).unwrap().node_count);
        assert_eq!(bst.select(4).unwrap().key, 14);
        assert!(bst.contains(9));
        assert_ne!(bst.contains(50), true);
        bst.delete(9);
        assert_ne!(bst.contains(9), true);
        assert_eq!(bst.size(), 8 as u32);
        assert_eq!(bst.keys(), [7, 11, 12, 14, 16, 17, 18, 25]);
        assert_eq!(bst.floor(15).unwrap(), 14);
        assert_ne!(bst.is_empty(), true);
        assert_eq!(bst.is_empty(), false);
        bst.delete(12);
        bst.delete(7);
        assert_eq!(bst.size(), 6 as u32);
        assert_eq!(bst.keys(), [11, 14, 16, 17, 18, 25]);
    }
}
