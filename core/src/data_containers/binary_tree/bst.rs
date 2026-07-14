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

    pub fn nodes(&self) -> Vec<&Node<K, V>> {
        let mut queue: Vec<&Node<K, V>> = Vec::new();
        self.root.nodes(&mut queue)
    }

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

#[cfg(test)]
mod tests {
    use super::BinarySearchTree;
    use pretty_assertions::assert_eq;

    // Helper to generate a standardized tree for repetitive test layouts
    fn setup_test_tree() -> BinarySearchTree<u32, String> {
        let mut tree = BinarySearchTree::new();
        tree.put(15, "Root".to_string());
        tree.put(10, "Left Child".to_string());
        tree.put(20, "Right Child".to_string());
        tree.put(5, "Lowest leaf".to_string());
        tree
    }

    #[test]
    fn test_core_crud_operations() {
        let mut tree = BinarySearchTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.size(), 0);

        // Create
        tree.put(42, "Answer".to_string());
        assert!(!tree.is_empty());
        assert_eq!(tree.size(), 1);
        assert!(tree.contains(42));

        // Read
        assert_eq!(tree.get(&42).unwrap().val, "Answer");
        assert_eq!(tree.get(&99), None); // Missing key query

        // Update
        tree.put(42, "New Answer".to_string());
        assert_eq!(tree.get(&42).unwrap().val, "New Answer");
        assert_eq!(tree.size(), 1); // Size shouldn't expand on value overrides
    }

    #[test]
    fn test_in_order_key_sorting() {
        let tree = setup_test_tree();

        // A standard BST must yield its keys in strictly sorted order!
        let sorted_keys = tree.keys();
        assert_eq!(sorted_keys, vec![5, 10, 15, 20]);
    }

    #[test]
    fn test_rank_and_select() {
        let tree = setup_test_tree(); // Keys sorted: 5, 10, 15, 20

        // Select: find key with X elements smaller than it
        assert_eq!(tree.select(0).unwrap().key, 5);
        assert_eq!(tree.select(2).unwrap().key, 15);
        assert_eq!(tree.select(99), None); // Out of bounds select

        // Rank: count elements strictly smaller than the key
        assert_eq!(tree.rank(5), 0);
        assert_eq!(tree.rank(12), 2); // 5 and 10 are smaller than 12
        assert_eq!(tree.rank(30), 4); // All elements are smaller than 30
    }

    #[test]
    fn test_floor_boundary_search() {
        let tree = setup_test_tree(); // Keys: 5, 10, 15, 20

        assert_eq!(tree.floor(12), Some(10)); // Largest key <= 12
        assert_eq!(tree.floor(15), Some(15)); // Exact match boundary
        assert_eq!(tree.floor(25), Some(20)); // Capped at absolute max
        assert_eq!(tree.floor(3), None); // Below absolute minimum
    }

    #[test]
    fn test_hibbard_deletion_scenarios() {
        let mut tree = setup_test_tree(); // Size: 4, Keys: 5, 10, 15, 20

        // Scenario A: Delete a node with only one child (10 has child 5)
        tree.delete(10);
        assert_eq!(tree.size(), 3);
        assert_eq!(tree.keys(), vec![5, 15, 20]);

        // Scenario B: Delete a leaf node (5 has zero children)
        tree.delete(5);
        assert_eq!(tree.size(), 2);
        assert_eq!(tree.keys(), vec![15, 20]);

        // Scenario C: Delete root carrying two valid child subtrees (15)
        let mut tree_complex = setup_test_tree();
        tree_complex.delete(15);
        assert_eq!(tree_complex.size(), 3);
        // The tree structure must seamlessly shift up a successor while staying sorted
        assert_eq!(tree_complex.keys(), vec![5, 10, 20]);
    }
}
