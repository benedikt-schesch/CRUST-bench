use std::cmp::Ordering;
pub struct Heap<T> {
data: Vec<T>,
cmp: fn(&T, &T) -> Ordering,
pub n_items: usize,
pub n_max: usize,
}
impl<T> Heap<T> {
pub fn new(cmp: fn(&T, &T) -> Ordering, _elem_size: usize, capacity: usize, _initial: usize) -> Self {
Heap {
data: Vec::with_capacity(capacity),
cmp,
n_items: 0,
n_max: capacity,
}
}
pub fn push(&mut self, item: T) {
self.data.push(item);
self.n_items += 1;
let mut i = self.data.len() - 1;
while i > 0 {
let parent = (i - 1) / 2;
if (self.cmp)(&self.data[i], &self.data[parent]) == Ordering::Less {
self.data.swap(i, parent);
i = parent;
} else {
break;
}
}
}
pub fn pop(&mut self) -> Option<T> {
if self.data.is_empty() {
return None;
}
let last = self.data.len() - 1;
self.data.swap(0, last);
let result = self.data.pop();
self.n_items -= 1;
let mut i = 0;
let len = self.data.len();
while i < len {
let left = 2 * i + 1;
let right = 2 * i + 2;
let mut smallest = i;
if left < len && (self.cmp)(&self.data[left], &self.data[smallest]) == Ordering::Less {
smallest = left;
}
if right < len && (self.cmp)(&self.data[right], &self.data[smallest]) == Ordering::Less {
smallest = right;
}
if smallest != i {
self.data.swap(i, smallest);
i = smallest;
} else {
break;
}
}
result
}
pub fn remove(&mut self) -> Option<T> {
self.pop()
}
pub fn insert(&mut self, item: T) {
self.push(item);
}
pub fn is_empty(&self) -> bool {
self.data.is_empty()
}
pub fn len(&self) -> usize {
self.data.len()
}
pub fn update(&mut self, item: T, _index: usize) {
self.push(item);
}
pub fn find(&self, item: &T) -> Option<usize>
where T: PartialEq,
{
self.data.iter().position(|x| x == item)
}
}
