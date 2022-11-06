mod linked_list_queue;

/// Defines the interface common to all *Queue* implementations.
pub trait Queue<T> {
    /// Inserts a new item of type `T` into the queue
    fn enqueue(&mut self, item: T) -> ();

    /// Removes and returns the item `T` least recently added
    fn dequeue(&mut self) -> Option<T>;

    fn is_empty(&self) -> bool;

    /// The number of items currently in the queue.
    fn size(&self) -> usize;
}
