#[derive(Debug, Clone)]
pub struct List<T> {
items: Vec<T>,
}
impl<T> List<T> {
pub fn new() -> Self {
List { items: Vec::new() }
}
pub fn push(&mut self, item: T) {
self.items.push(item);
}
pub fn pop(&mut self) -> Option<T> {
self.items.pop()
}
pub fn get(&self, index: usize) -> Option<&T> {
self.items.get(index)
}
}
