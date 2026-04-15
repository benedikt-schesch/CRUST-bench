//! MaxHeap module for the Megalania library.

/// A max heap data structure for managing elements with priority.
pub struct MaxHeap<T> {
data: Vec<T>,
comparator: Box<dyn Fn(&T, &T) -> i32>,
}

impl<T> MaxHeap<T> {
/// Creates a new max heap with the specified capacity and comparator.
pub fn new(capacity: usize, comparator: Box<dyn Fn(&T, &T) -> i32>) -> Self {
Self {
data: Vec::with_capacity(capacity),
comparator,
}
}

/// Creates a new max heap with the specified capacity and comparator.
pub fn with_capacity(capacity: usize, comparator: Box<dyn Fn(&T, &T) -> i32>) -> Self {
Self::new(capacity, comparator)
}

/// Inserts an element onto the heap.
pub fn insert(&mut self, item: T) -> bool {
self.data.push(item);
self.heapify_up(self.data.len() - 1);
true
}

/// Pops the maximum element from the heap.
pub fn pop(&mut self) -> Option<T> {
if self.data.is_empty() {
return None;
}
let last = self.data.len() - 1;
self.data.swap(0, last);
let max = self.data.pop();
if !self.data.is_empty() {
self.heapify_down(0);
}
max
}

/// Returns true if the heap is empty.
pub fn is_empty(&self) -> bool {
self.data.is_empty()
}

/// Returns the number of elements in the heap.
pub fn len(&self) -> usize {
self.data.len()
}

/// Returns the number of elements in the heap (alias for len).
pub fn count(&self) -> usize {
self.len()
}

/// Returns a reference to the maximum element.
pub fn maximum(&self) -> Option<&T> {
self.data.first()
}

/// Removes the maximum element from the heap.
pub fn remove_maximum(&mut self) -> bool {
self.pop().is_some()
}

/// Updates the maximum element (re-heapifies).
pub fn update_maximum(&mut self) -> bool {
if self.data.is_empty() {
return false;
}
self.heapify_down(0);
true
}

fn heapify_up(&mut self, mut index: usize) {
while index > 0 {
let parent = (index - 1) / 2;
if (self.comparator)(&self.data[index], &self.data[parent]) > 0 {
self.data.swap(index, parent);
index = parent;
} else {
break;
}
}
}

fn heapify_down(&mut self, mut index: usize) {
let len = self.data.len();
loop {
let left = 2 * index + 1;
let right = 2 * index + 2;
let mut largest = index;

if left < len && (self.comparator)(&self.data[left], &self.data[largest]) > 0 {
largest = left;
}
if right < len && (self.comparator)(&self.data[right], &self.data[largest]) > 0 {
largest = right;
}

if largest != index {
self.data.swap(index, largest);
index = largest;
} else {
break;
}
}
}
}

impl<T: Ord> Default for MaxHeap<T> {
fn default() -> Self {
Self {
data: Vec::new(),
comparator: Box::new(|a: &T, b: &T| {
if a > b { 1 }
else if a < b { -1 }
else { 0 }
}),
}
}
}
