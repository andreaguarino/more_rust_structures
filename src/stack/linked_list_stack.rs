use crate::stack::Stack;

#[derive(Clone)]
struct Node<T: Clone> {
    item: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Clone)]
struct LinkedListStack<T: Clone> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: Clone> Stack<T> for LinkedListStack<T> {
    fn push(&mut self, item: T) -> () {
        let head = self.head.clone();
        let new_head = Node { item, next: head };
        self.head = Some(Box::new(new_head));
        self.size += 1;
    }

    fn pop(&mut self) -> T {
        let head = match self.head.clone() {
            Some(h) => h,
            None => panic!("Cannot call pop on an empty stack"),
        };
        self.head = (*head).next;
        self.size -= 1;
        return (*head).item;
    }

    fn peek(&self) -> T {
        return match self.head.clone() {
            Some(h) => (*h).item,
            None => panic!("Cannot call peek on an empty stack"),
        };
    }

    fn count(&self) -> usize {
        return self.size;
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::{linked_list_stack::LinkedListStack, Stack};

    #[test]
    fn stack_push_pop() {
        let mut stack = LinkedListStack::<u32> {
            head: None,
            size: 0,
        };
        stack.push(42);
        assert_eq!(stack.count(), 1);
        assert_eq!(stack.peek(), 42);
        let item = stack.pop();
        assert_eq!(item, 42);
        assert_eq!(stack.count(), 0);
    }
}
