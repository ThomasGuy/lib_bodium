use crate::linked_list::{IntoIter, Iter, IterMut, List};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Bag<T: Display> {
    root: List<T>,
    size: u32,
}

impl<T: Display> Bag<T> {
    pub fn new() -> Self {
        Bag {
            root: List::new(),
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

impl<T> Display for Bag<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s: String = String::new();
        for w in self.it() {
            s = format!("{}{} ", s, w);
        }
        writeln!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use crate::bag::Bag;

    use pretty_assertions::{self, assert_eq};

    #[test]
    fn into_iter() {
        let mut list = Bag::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("bag: {}", list.to_string());

        let mut iter = list.into_it();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = Bag::new();
        list.add(1);
        list.add(2);
        list.add(3);

        let mut iter = list.it();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = Bag::new();
        list.add(1);
        list.add(2);
        list.add(3);

        let mut iter = list.it_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
