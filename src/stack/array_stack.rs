use crate::stack::Stack;

/// Uses a Vector of a given capacity (16 by default)
struct ArrayStack<T: Clone> {
    array: Vec<Option<T>>,
    count: usize,
    capacity: usize,
}

impl<T: Clone> Stack<T> for ArrayStack<T> {
    /// Pushes item of type `T` at the position pointed by `head`
    /// Capacity will be updated once `capacity == count`
    /// Amortized complexity: O(1)
    fn push(&mut self, item: T) -> () {
        if self.capacity() == self.count {
            self.update_capacity(self.capacity() * 2);
        }
        self.array[self.count] = Some(item);
        self.count += 1;
    }

    /// Pops item of type `T` pointed by `head`
    /// Sets `array[head]` to None
    /// Capacity will be updated once `count` will become equals or less then 25% of the capacity
    /// Amortized complexity: O(1)
    /// Note that element needs to be cloned
    fn pop(&mut self) -> T {
        let head = self.count - 1;
        let value = self.array[head].clone();
        let item = match value {
            None => panic!("Cannot call pop. No value at the head of the stack."),
            Some(item) => item,
        };
        self.array[head] = None;
        self.count -= 1;
        if self.count <= self.capacity() / 4 {
            self.update_capacity(self.capacity() / 2);
        }
        item
    }

    fn peek(&self) -> T {
        match self.array[self.count - 1].clone() {
            None => panic!("Cannot call peek on an empty stack"),
            Some(item) => item,
        }
    }

    fn count(&self) -> usize {
        self.count
    }
}

impl<T: Clone> ArrayStack<T> {
    fn new(initial_capacity: usize) -> Self {
        let mut stack = ArrayStack {
            array: Vec::with_capacity(initial_capacity),
            count: 0,
            capacity: initial_capacity,
        };
        stack.array = vec![None; initial_capacity];
        return stack;
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
    /// Allocates a new vector with the updated capacity
    /// Copies elements to the new vector, while filling with `None` the new added cells
    /// Time and space complexity: O(n)
    fn update_capacity(&mut self, new_capacity: usize) -> () {
        let mut new_array = Vec::with_capacity(new_capacity);
        for i in 0..new_capacity {
            let elem = match self.array.get(i) {
                None => None,
                Some(item) => item.clone(),
            };
            new_array.push(elem);
        }
        self.capacity = new_capacity;
        self.array = new_array;
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::{array_stack::ArrayStack, Stack};

    #[test]
    fn stack_push_pop() {
        let mut stack = ArrayStack::new(16);
        stack.push(42);
        assert_eq!(stack.count(), 1);
        assert_eq!(stack.peek(), 42);
        let item = stack.pop();
        assert_eq!(item, 42);
        assert_eq!(stack.count(), 0);
    }

    #[test]
    fn capacity() {
        let mut stack = ArrayStack::new(2);
        stack.push(0);
        stack.push(1);
        assert_eq!(stack.capacity(), 2);
        assert_eq!(stack.count(), 2);
        stack.push(2);
        assert_eq!(stack.capacity(), 4);
        assert_eq!(stack.count(), 3);
        stack.pop();
        assert_eq!(stack.capacity(), 4);
        stack.pop();
        assert_eq!(stack.capacity(), 2);
    }
}
