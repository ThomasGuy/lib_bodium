use std::fmt::Display;

use crate::data_containers::linked_list::LinkedList;

pub struct Queue<T: Display> {
    root: LinkedList<T>,
    size: u32,
}

impl<T: Display> Queue<T> {
    pub fn new() -> Self {
        Self {
            root: LinkedList::new(),
            size: 0,
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.root.push(item);
        self.size += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if !self.is_empty() {
            self.size -= 1;
        }
        self.root.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}

impl<T: Display> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn basic() {
        let mut q = Queue::new();
        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);

        let a = q.dequeue();

        if let Some(val) = a {
            println!("{val}");
            assert_eq!(val, 3);
        }

        assert_eq!(q.dequeue(), Some(2));
        assert_eq!(q.dequeue(), Some(1));
    }
}
