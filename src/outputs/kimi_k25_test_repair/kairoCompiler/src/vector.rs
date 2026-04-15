
pub struct Vector<T> {
data: Vec<T>,
}
impl<T> Vector<T> {
pub fn new() -> Self {
Self { data: Vec::new() }
}
pub fn with_capacity(capacity: usize) -> Self {
Self {
data: Vec::with_capacity(capacity),
}
}
pub fn push(&mut self, item: T) {
self.data.push(item);
}
pub fn get(&self, index: usize) -> Option<&T> {
self.data.get(index)
}
pub fn len(&self) -> usize {
self.data.len()
}
pub fn is_empty(&self) -> bool {
self.data.is_empty()
}
}
