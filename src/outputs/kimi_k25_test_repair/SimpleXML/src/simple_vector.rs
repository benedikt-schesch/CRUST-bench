pub struct Vector<T>{
pub capacity: usize,
pub size: usize,
pub data: Vec<T>,
}
impl<T> Vector<T>{
pub fn new(capacity: usize) -> Self {
Vector {
capacity,
size: 0,
data: Vec::with_capacity(capacity),
}
}
pub fn release(&mut self) {
self.data.clear();
self.size = 0;
self.capacity = 0;
}
pub fn size(&self) -> usize {
self.size
}
pub fn push_back(&mut self, value: T) {
if self.size == self.capacity {
self.capacity = if self.capacity == 0 { 1 } else { self.capacity * 2 };
}
self.data.push(value);
self.size += 1;
}
pub fn get_element_at(&self, index: usize) -> Option<&T> {
if index < self.size {
Some(&self.data[index])
} else {
None
}
}
pub fn pop_back(&mut self) -> Option<T> {
if self.size > 0 {
self.size -= 1;
self.data.pop()
} else {
None
}
}
pub fn push_front(&mut self, value: T) {
self.insert_at_index(value, 0);
}
pub fn pop_front(&mut self) -> Option<T> {
self.remove_at_index(0)
}
pub fn top_front(&self) -> Option<&T> {
self.data.first()
}
pub fn top_back(&self) -> Option<&T> {
self.data.last()
}
pub fn insert_at_index(&mut self, value: T, index: usize) {
assert!(index <= self.size, "index out of range");
if self.size == self.capacity {
self.capacity = if self.capacity == 0 { 1 } else { self.capacity * 2 };
}
self.data.insert(index, value);
self.size += 1;
}
pub fn remove_at_index(&mut self, index: usize) -> Option<T> {
if index < self.size {
self.size -= 1;
Some(self.data.remove(index))
} else {
None
}
}
}
impl<T: PartialEq> Vector<T> {
pub fn index_of(&self, value: &T) -> Option<usize> {
self.index_of_with_start(value, 0)
}
pub fn index_of_with_start(&self, value: &T, start: usize) -> Option<usize> {
if start >= self.size {
return None;
}
for i in start..self.size {
if self.data[i] == *value {
return Some(i);
}
}
None
}
}
