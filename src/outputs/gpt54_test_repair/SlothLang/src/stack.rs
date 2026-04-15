use crate::{parser, slothvm, throw};
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
self.top.take().map(|mut node| {
self.top = node.next.take();
node.data
})
}
pub fn peek(&self, pos: usize) -> Option<i32> {
let mut current = self.top.as_ref();
let mut idx = 0usize;
while let Some(node) = current {
if idx == pos {
return Some(node.data);
}
current = node.next.as_ref();
idx += 1;
}
None
}
pub fn print(&self) {
print!("|");
let mut current = self.top.as_ref();
while let Some(node) = current {
print!("{} ", node.data);
current = node.next.as_ref();
}
println!();
}
}
impl Drop for Stack {
fn drop(&mut self) {
while self.pop().is_some() {}
}
}
