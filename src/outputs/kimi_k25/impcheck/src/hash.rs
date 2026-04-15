use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct HashTable<K: Hash + PartialEq + 'static> {
buckets: Vec<Vec<(K, Box<i32>)>>,
pub capacity: usize,
pub size: u64,
last_found: Option<(usize, usize)>,
}

impl<K: Hash + PartialEq + 'static> HashTable<K> {
pub fn new(capacity: usize) -> Self {
let capacity = if capacity == 0 { 1 } else { capacity };
let mut buckets = Vec::with_capacity(capacity);
for _ in 0..capacity {
buckets.push(Vec::new());
}
Self {
buckets,
capacity,
size: 0,
last_found: None,
}
}

fn hash_index(&self, key: &K) -> usize {
let mut hasher = DefaultHasher::new();
key.hash(&mut hasher);
(hasher.finish() as usize) % self.capacity
}

pub fn hash_table_insert(&mut self, key: K, value: Box<i32>) -> bool {
let index = self.hash_index(&key);
let bucket = &mut self.buckets[index];

// Check if key already exists
for (k, _) in bucket.iter() {
if k == &key {
return false;
}
}

bucket.push((key, value));
self.size += 1;
true
}

pub fn hash_table_find(&mut self, key: K) -> Option<&Box<i32>> {
let index = self.hash_index(&key);
let bucket = &self.buckets[index];

for (i, (k, v)) in bucket.iter().enumerate() {
if k == &key {
self.last_found = Some((index, i));
return Some(v);
}
}
self.last_found = None;
None
}

pub fn hash_table_delete(&mut self, key: K) -> bool {
let index = self.hash_index(&key);
let bucket = &mut self.buckets[index];

if let Some(pos) = bucket.iter().position(|(k, _)| k == &key) {
bucket.remove(pos);
self.size -= 1;
// Update last_found if necessary
if let Some((last_bucket, last_pos)) = self.last_found {
if last_bucket == index {
if last_pos == pos {
self.last_found = None;
} else if last_pos > pos {
self.last_found = Some((last_bucket, last_pos - 1));
}
}
}
true
} else {
false
}
}

pub fn hash_table_delete_last_found(&mut self) -> bool {
if let Some((bucket_idx, entry_idx)) = self.last_found {
if bucket_idx < self.buckets.len() && entry_idx < self.buckets[bucket_idx].len() {
self.buckets[bucket_idx].remove(entry_idx);
self.size -= 1;
self.last_found = None;
true
} else {
false
}
} else {
false
}
}

pub fn hash_table_free(&mut self) {
self.buckets.clear();
self.buckets.shrink_to_fit();
self.size = 0;
self.capacity = 0;
self.last_found = None;
}
}
