struct QueueNode<T> {
value: T,
next: Option<Box<QueueNode<T>>>,
}
pub struct Queue<T> {
pub head: Option<Box<QueueNode<T>>>,
pub tail: Option<*mut QueueNode<T>>,
pub size: usize,
}
impl<T> Queue<T> {
pub fn new() -> Self {
Queue {
head: None,
tail: None,
size: 0,
}
}
pub fn is_empty(&self) -> bool {
self.size == 0
}
pub fn push(&mut self, value: T) {
let new_node = Box::new(QueueNode { value, next: None });
match self.head.as_mut() {
None => {
self.head = Some(new_node);
}
Some(mut current) => {
while current.next.is_some() {
current = current.next.as_mut().expect("next exists");
}
current.next = Some(new_node);
}
}
self.size += 1;
self.tail = None;
}
pub fn pop(&mut self) -> Option<T> {
if self.is_empty() {
return None;
}
let old_head = self.head.take()?;
let QueueNode { value, next } = *old_head;
self.head = next;
if self.size == 1 {
self.tail = None;
} else {
self.tail = None;
}
self.size -= 1;
Some(value)
}
pub fn front(&self) -> Option<&T> {
self.head.as_ref().map(|node| &node.value)
}
pub fn back(&self) -> Option<&T> {
let mut current = self.head.as_ref()?;
while let Some(ref next) = current.next {
current = next;
}
Some(&current.value)
}
pub fn free(&mut self) {
self.head = None;
self.tail = None;
self.size = 0;
}
}
