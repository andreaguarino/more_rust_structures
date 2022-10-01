mod array_stack;
mod linked_list_stack;

/// Defines the interface common to all *Stack* implementations.
pub trait Stack<T> {
    /// Push the provided `item` onto the top of the stack.
    /// The item which was on top of the stack before this push is pushed down onto second position.
    fn push(&mut self, item: T) -> ();

    /// Pops the item out from the top of the stack and returns it as a result.
    /// The item which was on second position at the top of the stack goes on the top after the item on top is popped
    /// out.
    fn pop(&mut self) -> T;

    /// Returns the item of type `T` on top of the stack, without popping it out from the stack.
    fn peek(&self) -> T;

    /// The number of items currently in the stack.
    fn count(&self) -> usize;
}
