
use crate::max_heap::MaxHeap;
use std::cell::RefCell;
use std::rc::Rc;
fn plain_comparator(a: &u32, b: &u32) -> i32 {
if a > b { 1 }
else if a < b { -1 }
else { 0 }
}
struct HeapData {
data: Vec<u32>,
}
fn main() {
let heap_size = 10;
let mut heap = MaxHeap::new(heap_size, Box::new(plain_comparator));
heap.insert(5u32);
heap.insert(10u32);
if let Some(maximum) = heap.maximum() {
let expected_value = 10u32;
assert_eq!(*maximum, expected_value, "Maximum value unexpected");
}
let backing_store = Rc::new(RefCell::new(HeapData { data: vec![0u32; 10] }));
let comparator = {
let backing_store = backing_store.clone();
move |a: &u32, b: &u32| -> i32 {
if a > b { 1 }
else if a < b { -1 }
else { 0 }
}
};
let mut heap2 = MaxHeap::new(heap_size, Box::new(comparator));
let maximum_val: u32 = 5;
let maximum: &u32 = &maximum_val;
let index_val: u32 = 3;
let index: &u32 = &index_val;
let val = 1u32;
if val < backing_store.borrow().data[*maximum as usize] {
backing_store.borrow_mut().data[*maximum as usize] = val;
}
assert!(*index < heap_size as u32, "Index out of range");
let _ = backing_store.borrow().data[*index as usize];
print!("{} ", backing_store.borrow().data[*index as usize]);
}
