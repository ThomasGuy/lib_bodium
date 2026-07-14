// use crate::data_containers::linked_list::{IntoIter, Iter, IterMut, LinkedList};
use super::linked_list::{IntoIter, Iter, IterMut, LinkedList};
use std::fmt::Display;
// use std::iter::IntoIterator;

#[derive(Debug, Clone)]
pub struct Bag<T> {
    root: LinkedList<T>,
    size: u32,
}

impl<T> Bag<T> {
    pub fn new() -> Self {
        Bag {
            root: LinkedList::new(),
            size: 0,
        }
    }

    pub fn add(&mut self, item: T) {
        self.root.push(item);
        self.size += 1;
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.root.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.root.iter_mut()
    }
}

// 1. Implementing IntoIterator for moving ownership (`for item in bag`)
impl<T> IntoIterator for Bag<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.root.into_iter()
    }
}

// 2. Implementing IntoIterator for shared references (`for item in &bag`)
impl<'a, T> IntoIterator for &'a Bag<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Display> Display for Bag<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // 1. Create a peekable iterator from your reference loop layout
        let mut iter = self.into_iter().peekable();

        // 2. Stream items straight to the output buffer
        while let Some(w) = iter.next() {
            write!(f, "{}", w)?;

            // Only add a space if this isn't the final item!
            if iter.peek().is_some() {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}

impl<T: Display> Default for Bag<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::data_containers::bag::Bag;

    use pretty_assertions::{self, assert_eq};

    #[test]
    fn into_iter() {
        let mut list = Bag::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("bag: {}", list.to_string());

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = Bag::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = Bag::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn implicit_reference_into_iter() {
        let mut list = Bag::<i32>::new();
        list.add(10);
        list.add(20);

        // Verifies that `IntoIterator` for `&Bag<T>` allows multiple reference loops
        let mut sum = 0;
        for val in &list {
            // Implicitly calls list.into_iter() on &Bag
            sum += val;
        }
        assert_eq!(sum, 30);

        // This second loop is only possible because the first loop didn't consume `list`!
        assert_eq!(list.size(), 2);
    }
}
