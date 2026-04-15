use crate::{parser, throw, slothvm};
pub struct ListNode {
pub data: i32,
pub next: Option<Box<ListNode>>,
}
pub struct Stack {
pub top: Option<Box<ListNode>>,
pub bottom: Option<Box<ListNode>>,
}
impl Stack {
pub fn new() -> Self {
Stack {
top: None,
bottom: None,
}
}
pub fn push(&mut self, x: i32) {
let new_node = Box::new(ListNode {
data: x,
next: self.top.take(),
});
self.top = Some(new_node);
}
pub fn is_empty(&self) -> bool {
self.top.is_none()
}
pub fn pop(&mut self) -> Option<i32> {
if let Some(mut node) = self.top.take() {
self.top = node.next.take();
Some(node.data)
} else {
None
}
}
pub fn peek(&self, mut pos: usize) -> Option<i32> {
let mut current = &self.top;
while pos > 0 {
if let Some(node) = current {
current = &node.next;
pos -= 1;
} else {
return None;
}
}
current.as_ref().map(|node| node.data)
}
pub fn print(&self) {
print!("|");
let mut current = &self.top;
while let Some(node) = current {
print!("{} ", node.data);
current = &node.next;
}
println!();
}
}
impl Drop for Stack {
fn drop(&mut self) {
while let Some(_) = self.pop() {}
}
}
