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
let new_node = Box::new(QueueNode {
value,
next: None,
});
if self.is_empty() {
self.head = Some(new_node);
} else {
let mut current = self.head.as_mut().unwrap();
while current.next.is_some() {
current = current.next.as_mut().unwrap();
}
current.next = Some(new_node);
}
self.size += 1;
}
pub fn pop(&mut self) -> Option<T> {
if self.is_empty() {
return None;
}
let mut head_node = self.head.take()?;
self.head = head_node.next.take();
self.size -= 1;
if self.size == 0 {
self.tail = None;
}
Some(head_node.value)
}
pub fn front(&self) -> Option<&T> {
self.head.as_ref().map(|node| &node.value)
}
pub fn back(&self) -> Option<&T> {
if self.is_empty() {
return None;
}
let mut current = self.head.as_ref()?;
while current.next.is_some() {
current = current.next.as_ref()?;
}
Some(&current.value)
}
pub fn free(&mut self) {
self.head = None;
self.tail = None;
self.size = 0;
}
}
