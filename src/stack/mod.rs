mod linked_list_stack;

pub trait Stack<T> {
    fn push(&mut self, item: T) -> ();

    fn pop(&mut self) -> T;

    fn peek(&self) -> T;

    fn count(&self) -> usize;
}
