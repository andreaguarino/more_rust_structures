use crate::queue::Queue;
use std::{cell::RefCell, rc::Rc};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    item: T,
    next: Link<T>,
}

struct LinkedListQueue<T> {
    first: Link<T>,
    last: Link<T>,
    size: usize,
}

impl<T> Queue<T> for LinkedListQueue<T> {
    fn enqueue(&mut self, item: T) -> () {
        let enqueued_element = Rc::new(RefCell::new(Node { item, next: None }));
        self.first = match self.first.take() {
            None => Some(enqueued_element.clone()),
            first => first,
        };
        self.last = match self.last.take() {
            None => Some(enqueued_element),
            Some(last) => {
                last.borrow_mut().next = Some(enqueued_element);
                Some(last)
            }
        };
        self.size += 1;
    }

    fn dequeue(&mut self) -> Option<T> {
        let last_element = match self.first.take() {
            None => None,
            Some(last) => {
                let node = Rc::try_unwrap(last).ok().unwrap().into_inner();
                Some(node)
            }
        };
        // TODO: Update self.first to last_element.next
        self.size -= 1;
        last_element.map(|last| last.item)
    }

    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl<T> LinkedListQueue<T> {
    fn new() -> Self {
        LinkedListQueue {
            first: None,
            last: None,
            size: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::queue::{linked_list_queue::LinkedListQueue, Queue};

    #[test]
    fn basic_queue() {
        let mut queue = LinkedListQueue::new();
        queue.enqueue(42);
        assert_eq!(queue.size, 1);
    }
}
