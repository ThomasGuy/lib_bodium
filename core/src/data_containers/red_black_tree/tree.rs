use super::node::{Colour, Link, Node};
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

    pub(crate) fn check_black_height(&self) -> Option<u32> {
        let Some(node) = &self.0 else {
            return Some(1); // Null links are counted as Black (height 1)
        };

        // Recurse down both sides
        let left_height = node.left.check_black_height()?;
        let right_height = node.right.check_black_height()?;

        // If the black heights of subtrees don't match, the invariant is broken!
        if left_height != right_height {
            return None;
        }

        // Add 1 to the height if the current node is Black
        if node.colour == Colour::Black {
            Some(left_height + 1)
        } else {
            Some(left_height)
        }
    }

    pub(crate) fn insert(&mut self, key: K, val: V) {
        self.insert_recursive(key, val);

        // 🚀 Rule: The root of a Red-Black Tree must ALWAYS be Black
        // if let Some(ref mut root_node) = self.0 {
        //     root_node.colour = Colour::Black;
        // }
    }

    fn insert_recursive(&mut self, key: K, val: V) {
        let Some(ref mut node) = self.0 else {
            // 🚀 Base case: Create the new node as RED with a count of 1
            self.0 = Some(Box::new(Node::new(key, val, 1)));
            return;
        };

        // 1. Structural insertion + update parent node counts
        if key < node.key {
            node.left.insert_recursive(key, val);
        } else if key > node.key {
            node.right.insert_recursive(key, val);
        } else {
            node.val = val; // Key exists, just update value (count doesn't change)
            return;
        }

        // 2. Update parent node count (Run once for whichever branch was executed)
        node.node_count = node.left.size() + node.right.size() + 1;

        // 3. Red-Black Fixup Step (Rotations may alter tree structure)
        self.fix_up();

        // 4. Recalculate count again after fix_up in case rotations swapped parents/children
        if let Some(ref mut rotated_node) = self.0 {
            rotated_node.node_count = rotated_node.left.size() + rotated_node.right.size() + 1;
        }
    }

    pub(crate) fn fix_up(&mut self) {
        // --- 1. Right-Leaning Red check ---
        let mut do_rotate_left = false;
        if let Some(node) = &self.0 {
            // node.right and node.left are both Tree wrappers, so they use .is_red() directly
            if node.right.is_red() && !node.left.is_red() {
                do_rotate_left = true;
            }
        }
        if do_rotate_left {
            self.rotate_left();
        }

        // --- 🚀 THE CRITICAL FIX: Fix Right-Leaning Red on the Left Child ---
        // If the left child has a right-red link (a Left-Right grandchild),
        // we must left-rotate the left child FIRST to convert the zig-zag path
        // into a straight left-left path!
        let mut do_rotate_left_child = false;
        #[allow(clippy::collapsible_if)]
        if let Some(node) = &self.0 {
            if let Some(left_node) = &node.left.0 {
                if left_node.right.is_red() && !left_node.left.is_red() {
                    do_rotate_left_child = true;
                }
            }
        }
        #[allow(clippy::collapsible_if)]
        if do_rotate_left_child {
            if let Some(ref mut node) = self.0 {
                node.left.rotate_left(); // Fix the sub-branch layout
            }
        }

        // --- 2. Consecutive Left Red check ---
        let mut do_rotate_right = false;
        #[allow(clippy::collapsible_if)]
        if let Some(node) = &self.0 {
            if node.left.is_red() && self.is_left_left_red() {
                do_rotate_right = true;
            }
        }
        if do_rotate_right {
            self.rotate_right();
        }

        // --- 3. Colour Flip check ---
        let mut do_flip = false;
        #[allow(clippy::collapsible_if)]
        if let Some(node) = &self.0 {
            if node.left.is_red() && node.right.is_red() {
                do_flip = true;
            }
        }
        if do_flip {
            self.flip_colours();
        }
    }

    // Check if the current tree node is Red. If None, it is logically Black.
    fn is_red(&self) -> bool {
        if let Some(node) = &self.0 {
            node.colour == Colour::Red
        } else {
            false
        }
    }

    // Safely get a reference to the left child Tree wrapper
    fn left_child(&self) -> Option<&Tree<K, V>> {
        self.0.as_ref().map(|node| &node.left)
    }

    // Safely check if the left-left grandchild is Red
    fn is_left_left_red(&self) -> bool {
        #[allow(clippy::collapsible_if)]
        if let Some(left) = self.left_child() {
            if let Some(left_left) = left.left_child() {
                return left_left.is_red();
            }
        }
        false
    }
 
    fn flip_colours(&mut self) {
        if let Some(ref mut node) = self.0 {
            node.colour = match node.colour {
                Colour::Red => Colour::Black,
                Colour::Black => Colour::Red,
            };

            // Toggle left child colour
            if let Some(ref mut left_node) = node.left.0 {
                left_node.colour = match left_node.colour {
                    Colour::Red => Colour::Black,
                    Colour::Black => Colour::Red,
                };
            }

            // Toggle right child colour
            if let Some(ref mut right_node) = node.right.0 {
                right_node.colour = match right_node.colour {
                    Colour::Red => Colour::Black,
                    Colour::Black => Colour::Red,
                };
            }
        }
    }

    pub fn rotate_left(&mut self) {
        //  Take ownership of the current root node out of the option slot
        if let Some(mut old_root) = self.0.take() {
            //  Take ownership of its right child node (the one moving up)
            if let Some(mut new_root) = old_root.right.0.take() {
                // 1. Move the new root's left child to become the old root's right child
                old_root.right.0 = new_root.left.0.take();

                // 2. Color updates
                new_root.colour = old_root.colour;
                old_root.colour = Colour::Red;

                // 3. 🚀 CRITICAL FIX: Recalculate sizes for the swapped nodes
                // Update the child (old_root) FIRST so the parent can read its clean size!
                old_root.node_count = old_root.left.size() + old_root.right.size() + 1;
                new_root.node_count = new_root.left.size() + new_root.right.size() + 1;

                // 4. Re-link
                new_root.left.0 = Some(old_root);
                self.0 = Some(new_root);
            } else {
                // Rollback safety fallback if no right child actually existed
                self.0 = Some(old_root);
            }
        }
    }

    pub fn rotate_right(&mut self) {
        // 1. Take ownership of the current root node out of the option slot
        if let Some(mut old_root) = self.0.take() {
            // 2. Take ownership of its left child node (the one moving up)
            if let Some(mut new_root) = old_root.left.0.take() {
                // 1. Structural swap
                old_root.left.0 = new_root.right.0.take();

                // 2. Color updates
                new_root.colour = old_root.colour;
                old_root.colour = Colour::Red;

                // 3. 🚀 CRITICAL FIX: Recalculate sizes for the swapped nodes
                // Update the child (old_root) FIRST so the parent can read its clean size!
                old_root.node_count = old_root.left.size() + old_root.right.size() + 1;
                new_root.node_count = new_root.left.size() + new_root.right.size() + 1;

                // 4. Re-link
                new_root.right.0 = Some(old_root);
                self.0 = Some(new_root);
            } else {
                // Rollback safety fallback if no left child actually existed
                self.0 = Some(old_root);
            }
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

    pub(crate) fn trace_search_path<'a>(&'a self, target: &K, queue: &mut Vec<&'a K>) {
        // 1. Base case: If we hit a None leaf, stop tracking
        let Some(node) = &self.0 else {
            return;
        };
        // 2. Record that we visited this node along the search path
        queue.push(&node.key);
        // 3. Recurse down the appropriate branch based on binary search rules
        match target.cmp(&node.key) {
            Ordering::Less => node.left.trace_search_path(target, queue),
            Ordering::Greater => node.right.trace_search_path(target, queue),
            Ordering::Equal => {} // Target found! Stop recursing and return up the stack
        }
    }
}
