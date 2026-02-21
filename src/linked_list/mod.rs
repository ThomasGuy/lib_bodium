pub(crate) mod node;
use node::Node;

use std::fmt::Display;

#[derive(Debug, Clone)]
pub(crate) struct List<T: Display> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T: Display> List<T> {
    pub(crate) fn new() -> Self {
        List { head: None }
    }

    pub(crate) fn push(&mut self, item: T) {
        let first = Box::new(Node {
            item,
            next: self.head.take(),
        });
        self.head = Some(first);
    }

    pub(crate) fn pop(&mut self) -> Option<T> {
        self.head.take().map(|box_node| {
            self.head = box_node.next;
            box_node.item
        })
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // pub fn peek(&self) -> Option<&T> {
    //     self.head.as_ref().map(|node| &node.item)
    // }

    // pub fn peek_mut(&mut self) -> Option<&mut T> {
    //     self.head.as_mut().map(|node| &mut node.item)
    // }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T: Display> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

impl<T: Display> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct IntoIter<T: Display>(List<T>);

impl<T: Display> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T: Display> {
    next: Option<&'a Node<T>>,
}

impl<'a, T: Display> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.item
        })
    }
}

pub struct IterMut<'a, T: Display> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T: Display> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.item
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;
    use pretty_assertions::{self, assert_eq};

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
