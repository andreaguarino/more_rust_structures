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
    /// Push item of type `T` into the head of the linked list
    /// Note that `self.head` needs to cloned in order to perform a move.
    /// This has to be done explicitely, because `Box` types don't implement the Copy trait
    /// Time and space complexity: O(1) - however it may depend on the clone
    fn push(&mut self, item: T) -> () {
        let head = self.head.clone();
        let new_head = Node { item, next: head };
        self.head = Some(Box::new(new_head));
        self.size += 1;
    }

    /// Returns item reference by `head` from the linked list
    /// Update `head` to point to the second element in the linked list
    /// Time and space complexity: O(1) - however it may depend on the clone
    fn pop(&mut self) -> T {
        let previous_head = match self.head.clone() {
            Some(h) => h,
            None => panic!("Cannot call pop on an empty stack"),
        };
        self.head = (*previous_head).next;
        self.size -= 1;
        (*previous_head).item
    }

    /// Time and space complexity: O(1) - however it may depend on the clone
    fn peek(&self) -> T {
        match self.head.clone() {
            Some(h) => (*h).item,
            None => panic!("Cannot call peek on an empty stack"),
        }
    }
    /// Time and space complexity: O(1)
    fn count(&self) -> usize {
        self.size
    }
}

impl<T: Clone> LinkedListStack<T> {
    fn new() -> Self {
        LinkedListStack::<T> {
            head: None,
            size: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::{linked_list_stack::LinkedListStack, Stack};

    #[test]
    fn stack_push_pop() {
        let mut stack = LinkedListStack::new();
        stack.push(42);
        assert_eq!(stack.count(), 1);
        assert_eq!(stack.peek(), 42);
        let item = stack.pop();
        assert_eq!(item, 42);
        assert_eq!(stack.count(), 0);
    }
}
