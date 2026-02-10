use crate::linked_list::{IntoIter, Iter, IterMut, List};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Stack<T: Display> {
    root: List<T>,
    size: u32,
}

impl<T: Display> Stack<T> {
    pub fn new() -> Self {
        Stack {
            root: List::new(),
            size: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        self.root.push(item);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if !self.is_empty() {
            self.size -= 1;
        }
        self.root.pop()
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
    }

    pub fn into_it(self) -> IntoIter<T> {
        self.root.into_it()
    }

    pub fn it(&self) -> Iter<'_, T> {
        self.root.it()
    }

    pub fn it_mut(&mut self) -> IterMut<'_, T> {
        self.root.it_mut()
    }
}

impl<T: Display> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}
