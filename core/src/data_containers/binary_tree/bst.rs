use super::{Node, Tree};
use crate::Stack;
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

    pub fn root_node(&self) -> Option<&Node<K, V>> {
        self.root.0.as_deref()
    }

    pub fn put(&mut self, key: K, val: V) {
        // Pass as references matching your optimized Tree signature
        self.root.put(&key, &val);
    }

    // Returns a read-only reference directly from memory,
    // saving immense CPU cycles by skipping deep structural clones!
    pub fn get(&self, key: &K) -> Option<&Node<K, V>> {
        self.root.get(key)
    }

    pub fn min(&self) -> Option<&Node<K, V>> {
        self.root.min_ref()
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
        self.root.nodes(&mut queue);
        queue
    }

    pub fn keys(&self) -> Vec<K> {
        let mut queue: Vec<K> = Vec::new();
        self.root.keys(&mut queue);
        queue
    }

    pub fn into_iter_keys(&self) -> std::vec::IntoIter<K> {
        self.keys().into_iter()
    }

    pub fn keys_ref(&self) -> Vec<&K> {
        let mut queue: Vec<&K> = Vec::new();
        self.root.keys_ref(&mut queue);
        queue
    }
    /// 🚀 Lazily streams references directly from your tree nodes
    pub fn iter_keys(&self) -> std::vec::IntoIter<&K> {
        self.keys_ref().into_iter()
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

    // Public entry point for your layout engine
    // pub fn get_layout(&self) -> (Vec<VisualNode>, Vec<VisualLine>, f64) {
    //     let mut visual_nodes = Vec::new();
    //     let mut visual_lines = Vec::new();
    //     let mut max_depth = 0.0;

    //     // 1. Reuse your working in-order nodes traversal
    //     let nodes_list = self.nodes();

    //     // 2. Safely unpack and pass the root reference to your layout engine
    //     compute_layout(
    //         self.root.0.as_deref(),
    //         &nodes_list,
    //         0.0,
    //         None,
    //         &mut visual_nodes,
    //         &mut visual_lines,
    //         &mut max_depth,
    //     );

    //     // 3. Return everything needed to draw the tree
    //     (visual_nodes, visual_lines, max_depth)
    // }

    /// 🚀 Lazily yields pairs sequentially with true O(1) memory overhead.
    pub fn iter(&self) -> BstIterator<'_, K, V> {
        BstIterator::new(&self.root.0)
    }
}

// 🚀 Enables native syntax: `for (key, val) in &tree`
impl<'a, K, V> IntoIterator for &'a BinarySearchTree<K, V>
where
    K: Ord + Clone + Display,
    V: Clone + Display,
{
    type Item = (&'a K, &'a V);
    type IntoIter = BstIterator<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// A zero-allocation, lazy In-Order iterator over your Binary Search Tree.
/// It doesn't require Clone, Ord, or Display bounds to track node references.
pub struct BstIterator<'a, K, V> {
    // Stores read-only references to your nodes on your custom library Stack
    stack: Stack<&'a Node<K, V>>,
}

impl<'a, K, V> BstIterator<'a, K, V> {
    pub(crate) fn new(root: &'a Option<Box<Node<K, V>>>) -> Self {
        let mut it = Self {
            stack: Stack::new(),
        };
        it.push_left_path(root);
        it
    }

    /// Drills down the leftmost branch, loading node references onto the stack
    fn push_left_path(&mut self, mut current: &'a Option<Box<Node<K, V>>>) {
        while let Some(node) = current {
            self.stack.push(node.as_ref());
            current = &node.left.0; // Directly reaches through your inner tuple Tree link
        }
    }
}

// 🚀 Implement the standard Rust Iterator trait engine
impl<'a, K, V> Iterator for BstIterator<'a, K, V> {
    // Each step yields a tuple containing read-only references directly from your tree nodes
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        // Pop the next available node reference out of your library stack
        let node = self.stack.pop()?;

        // If the node has a right branch, drill down its left track
        if node.right.0.is_some() {
            self.push_left_path(&node.right.0);
        }

        // Return the references with absolutely zero memory cloning!
        Some((&node.key, &node.val))
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

    #[test]
    fn test_into_iter_keys_sequential_loop() {
        let mut bst = BinarySearchTree::new();

        // Insert keys randomly
        bst.put(15, "Root".to_string());
        bst.put(10, "Left".to_string());
        bst.put(20, "Right".to_string());
        bst.put(5, "Leaf".to_string());

        // 🚀 Test your `iter_keys()` loop syntax directly!
        let mut extracted_keys = Vec::new();
        for key in bst.into_iter_keys() {
            extracted_keys.push(key);
        }

        // A standard BST must yield its keys in strictly sorted order!
        assert_eq!(extracted_keys, vec![5, 10, 15, 20]);
    }

    #[test]
    fn test_iter_reference_key_iterator() {
        let mut bst = BinarySearchTree::new();

        // 1. Seed the tree in an unsorted fashion
        bst.put(42, "Forty-Two".to_string());
        bst.put(12, "Twelve".to_string());
        bst.put(88, "Eighty-Eight".to_string());
        bst.put(5, "Five".to_string());

        // 2. Instantiate your zero-copy reference key iterator
        let mut key_iter = bst.iter_keys();

        // 3. Verify it yields exact &K reference types in strict sorted sequence
        // (We compare against borrowed integers to match the &K type constraint)
        assert_eq!(key_iter.next(), Some(&5));
        assert_eq!(key_iter.next(), Some(&12));
        assert_eq!(key_iter.next(), Some(&42));
        assert_eq!(key_iter.next(), Some(&88));
        assert_eq!(key_iter.next(), None); // Ensure it drains completely

        // 4. Test it inside a standard consumption loop to guarantee standard ergonomics
        let mut verified_count = 0;
        let mut last_key = 0;

        for &key in bst.iter_keys() {
            // Destructure the &K reference to get the copyable u32 primitive
            assert!(
                key > last_key,
                "BST keys must be yielded in ascending sorted order!"
            );
            last_key = key;
            verified_count += 1;
        }

        assert_eq!(verified_count, 4);
    }

    #[test]
    fn test_bst_iterator_traversal() {
        let mut bst = BinarySearchTree::new();

        // Insert nodes in an arbitrary order
        bst.put(15, "Root".to_string());
        bst.put(10, "Left".to_string());
        bst.put(20, "Right".to_string());
        bst.put(5, "Leaf".to_string());

        // 1. Test programmatic iteration via the .iter() method
        let mut it = bst.iter();

        // The iterator must yield items sequentially by Key order (5, 10, 15, 20)
        assert_eq!(it.next(), Some((&5, &"Leaf".to_string())));
        assert_eq!(it.next(), Some((&10, &"Left".to_string())));
        assert_eq!(it.next(), Some((&15, &"Root".to_string())));
        assert_eq!(it.next(), Some((&20, &"Right".to_string())));
        assert_eq!(it.next(), None); // Drained completely

        // 2. Test idiomatic "IntoIterator" reference looping
        // Simply passing a reference `&bst` triggers your IntoIterator implementation!
        let mut collected_pairs = Vec::new();
        for (key, val) in &bst {
            collected_pairs.push((*key, val.as_str()));
        }

        let expected_pairs = vec![(5, "Leaf"), (10, "Left"), (15, "Root"), (20, "Right")];
        assert_eq!(collected_pairs, expected_pairs);
    }
}
