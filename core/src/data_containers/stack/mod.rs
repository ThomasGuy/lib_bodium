use crate::data_containers::linked_list::{IntoIter, Iter, IterMut, LinkedList};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Stack<T> {
    root: LinkedList<T>,
    size: u32,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            root: LinkedList::new(),
            size: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        self.root.push(item);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let item = self.root.pop();
        if item.is_some() {
            self.size -= 1;
        }
        item
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.root.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.root.iter_mut()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

// 1. Implementing IntoIterator for moving ownership (`for item in stack`)
impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.root.into_iter()
    }
}

// 2. Implementing IntoIterator for shared references (`for item in &stack`)
impl<'a, T> IntoIterator for &'a Stack<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// 3. Zero-Allocation Display Trait Implementation
impl<T: Display> Display for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut iter = self.into_iter().peekable();
        while let Some(w) = iter.next() {
            write!(f, "{}", w)?;
            if iter.peek().is_some() {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_push_and_pop() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);

        stack.push(10);
        stack.push(20);
        stack.push(30);

        assert!(!stack.is_empty());
        assert_eq!(stack.size(), 3);

        // LIFO Order check
        assert_eq!(stack.pop(), Some(30));
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.size(), 1);

        assert_eq!(stack.pop(), Some(10));
        assert_eq!(stack.pop(), None); // Completely empty now
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn test_empty_pop_underflow_safety() {
        let mut stack: Stack<i32> = Stack::new();

        // Multiple pops on an empty stack should return None and keep size at 0
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.size(), 0);

        // Pushing after empty pops should function normally
        stack.push(99);
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.pop(), Some(99));
    }

    #[test]
    fn test_stack_iteration() {
        let mut stack = Stack::new();
        stack.push("A");
        stack.push("B");

        // Reference iteration (LIFO order)
        let mut iter = (&stack).into_iter();
        assert_eq!(iter.next(), Some(&"B"));
        assert_eq!(iter.next(), Some(&"A"));
        assert_eq!(iter.next(), None);
    }
}
