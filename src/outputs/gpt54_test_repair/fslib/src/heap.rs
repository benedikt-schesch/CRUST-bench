use std::cmp::Ordering;
use std::collections::HashMap;

const HEAP_INIT_SIZE: usize = 0xff;
const HEAP_RESIZE_FACTOR: usize = 2;

pub type HeapCmp<T> = fn(&T, &T) -> Ordering;

pub struct Heap<T> {
pub n_items: usize,
pub n_max: usize,
pub limit: usize,
pub cmp: HeapCmp<T>,
pub ht: HashMap<T, usize>,
pub items: Vec<T>,
}

pub fn parent(i: usize) -> usize {
((i + 1) >> 1) - 1
}

pub fn left(i: usize) -> usize {
((i + 1) << 1) - 1
}

pub fn right(i: usize) -> usize {
(i + 1) << 1
}

impl<T: Ord + Clone + std::hash::Hash + Eq> Heap<T> {
pub fn new(cmp: HeapCmp<T>, _item_size: usize, init_size: usize, limit: usize) -> Self {
let n_max = if limit != 0 {
limit
} else if init_size == 0 {
HEAP_INIT_SIZE
} else {
init_size
};
Self {
n_items: 0,
n_max,
limit,
cmp,
ht: HashMap::new(),
items: Vec::with_capacity(n_max.max(1)),
}
}

pub fn index(&mut self, _hsh: fn(&T) -> u64, _hcmp: fn(&T, &T) -> bool) {
self.ht.clear();
for (i, item) in self.items.iter().take(self.n_items).enumerate() {
self.ht.insert(item.clone(), i);
}
}

pub fn remove(&mut self) {
self.ht.clear();
self.items.clear();
self.n_items = 0;
}

pub fn pop(&mut self) -> Option<T> {
if self.n_items == 0 {
return None;
}

let item = self.items[0].clone();
self.ht.remove(&item);
self.n_items -= 1;

if self.n_items > 0 {
let last = self.items[self.n_items].clone();
self.items[0] = last.clone();
self.ht.insert(last, 0);
}

self.items.truncate(self.n_items);

if self.n_items > 1 {
self.heapify(0);
}

Some(item)
}

pub fn heapify(&mut self, mut i: usize) {
while i < self.n_items {
let l = left(i);
let r = right(i);
let mut best = i;

if l < self.n_items && (self.cmp)(&self.items[l], &self.items[best]) == Ordering::Less {
best = l;
}
if r < self.n_items && (self.cmp)(&self.items[r], &self.items[best]) == Ordering::Less {
best = r;
}

if best != i {
self.swap_items(i, best);
i = best;
} else {
break;
}
}
}

pub fn insert(&mut self, item: T) {
if self.limit == 0 && self.n_items == self.n_max {
self.n_max = (self.n_max.max(1)) * HEAP_RESIZE_FACTOR;
if self.items.capacity() < self.n_max {
self.items
.reserve(self.n_max.saturating_sub(self.items.capacity()));
}
}

if self.limit == 0 || self.n_items < self.limit {
self.items.push(item.clone());
self.ht.insert(item.clone(), self.n_items);
self.n_items += 1;
let idx = self.n_items - 1;
self.update(item, idx);
} else {
let mut worst_idx = 0usize;
for i in 1..self.n_items {
if (self.cmp)(&self.items[i], &self.items[worst_idx]) == Ordering::Greater {
worst_idx = i;
}
}

if (self.cmp)(&item, &self.items[worst_idx]) == Ordering::Less {
let old = self.items[worst_idx].clone();
self.ht.remove(&old);
self.items[worst_idx] = item.clone();
self.ht.insert(item.clone(), worst_idx);
self.update(item, worst_idx);
self.heapify(worst_idx);
}
}
}

pub fn update(&mut self, item: T, mut i: usize) {
if i >= self.n_items {
return;
}
self.items[i] = item.clone();
self.ht.insert(item, i);

while i > 0 && (self.cmp)(&self.items[i], &self.items[parent(i)]) == Ordering::Less {
let j = parent(i);
self.swap_items(i, j);
i = j;
}
}

pub fn find(&self, item: &T) -> Option<usize> {
self.ht.get(item).copied()
}

fn swap_items(&mut self, i: usize, j: usize) {
self.items.swap(i, j);
let a = self.items[i].clone();
let b = self.items[j].clone();
self.ht.insert(a, i);
self.ht.insert(b, j);
}

fn delete_max(&mut self) {
if self.n_items == 0 {
return;
}

let mut worst_idx = 0usize;
for i in 1..self.n_items {
if (self.cmp)(&self.items[i], &self.items[worst_idx]) == Ordering::Greater {
worst_idx = i;
}
}

let old = self.items[worst_idx].clone();
self.ht.remove(&old);

self.n_items -= 1;
if worst_idx != self.n_items {
let replacement = self.items[self.n_items].clone();
self.items[worst_idx] = replacement.clone();
self.ht.insert(replacement.clone(), worst_idx);
self.items.truncate(self.n_items);
self.update(replacement, worst_idx);
self.heapify(worst_idx);
} else {
self.items.truncate(self.n_items);
}
}
}
