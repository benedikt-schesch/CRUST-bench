use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct HashTable<K, V> {
pub buckets: Vec<Vec<(K, V)>>,
hash_fn: fn(&K) -> usize,
capacity: usize,
size: usize,
}

impl<K, V> HashTable<K, V>
where
K: Eq + Clone,
{
pub fn new(hash_fn: fn(&K) -> usize, capacity: usize) -> Self {
let mut buckets = Vec::with_capacity(capacity);
for _ in 0..capacity {
buckets.push(Vec::new());
}
HashTable {
buckets,
hash_fn,
capacity,
size: 0,
}
}

pub fn insert(&mut self, key: K, value: V) -> Option<V> {
let hash = (self.hash_fn)(&key) % self.capacity;
let bucket = &mut self.buckets[hash];

for (k, v) in bucket.iter_mut() {
if *k == key {
return Some(std::mem::replace(v, value));
}
}

bucket.push((key, value));
self.size += 1;
None
}

pub fn get(&self, key: &K) -> Option<&V> {
let hash = (self.hash_fn)(key) % self.capacity;
let bucket = &self.buckets[hash];

for (k, v) in bucket {
if k == key {
return Some(v);
}
}
None
}

pub fn remove(&mut self, key: &K) -> Option<V> {
let hash = (self.hash_fn)(key) % self.capacity;
let bucket = &mut self.buckets[hash];

for i in 0..bucket.len() {
if bucket[i].0 == *key {
self.size -= 1;
return Some(bucket.remove(i).1);
}
}
None
}
}
