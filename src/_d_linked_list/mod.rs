use std::cell::RefCell;
use std::rc::Rc;

pub(crate) struct Node<T> {
    pub(crate) item: T,
    pub(crate) next: Link<T>,
    pub(crate) prev: Link<T>,
}

impl<T> Node<T> {
    pub(crate) fn new(item: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            item,
            next: None,
            prev: None,
        }))
    }
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct DList<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> DList<T> {
    pub(crate) fn new() -> Self {
        DList {
            head: None,
            tail: None,
        }
    }

    // pub(crate) fn push_front(&mut self, item: T) {
    //     let first = Box::new(Node {
    //         item,
    //         next: self.head.take(),
    //         prev: None,
    //     });
    //     self.head = Some(first);
    // }

    // pub(crate) fn push_back(&mut self, item: T) {
    //     let last = Box::new(Node {
    //         item,
    //         next: None,
    //         prev: self.tail.take(),
    //     });
    //     self.tail = Some(last);
    // }

    // pub(crate) fn pop_front(&mut self) -> Option<T> {
    //     self.head.take().map(|box_node| {
    //         self.head = box_node.next;
    //         if let Some(node) = &mut self.head {
    //             node.prev = None;
    //         }
    //         if self.head.is_none() {
    //             self.tail = None;
    //         }
    //         box_node.item
    //     })
    // }

    // pub(crate) fn pop_back(&mut self) -> Option<T> {
    //     self.tail.take().map(|box_node| {
    //         self.tail = box_node.prev;
    //         if let Some(node) = &mut self.tail {
    //             node.next = None;
    //         }
    //         if self.tail.is_none() {
    //             self.head = None;
    //         }
    //         box_node.item
    //     })
    // }

    pub(crate) fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions;
}
