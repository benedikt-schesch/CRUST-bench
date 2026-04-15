use std::collections::VecDeque;

pub struct Queue<T> {
pub items: VecDeque<T>,
}

impl<T> Queue<T> {
pub fn new() -> Self {
Queue { items: VecDeque::new() }
}

pub fn push(&mut self, item: T) {
self.items.push_back(item);
}

pub fn pop(&mut self) -> Option<T> {
self.items.pop_front()
}

// Aliases for test compatibility
pub fn enqueue(&mut self, item: T) {
self.push(item);
}

pub fn dequeue(&mut self) -> Option<T> {
self.pop()
}

pub fn is_empty(&self) -> bool {
self.items.is_empty()
}

pub fn len(&self) -> usize {
self.items.len()
}
}
