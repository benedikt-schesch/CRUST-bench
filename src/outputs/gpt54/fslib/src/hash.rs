use std::collections::LinkedList;
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Default)]
pub struct Bucket<K, V> {
items: LinkedList<HashItem<K, V>>,
}

struct HashItem<K, V> {
key: K,
value: V,
}

pub struct HashTable<K, V, F>
where
K: Eq + Hash + Clone,
V: Clone,
F: Fn(&K) -> usize,
{
pub buckets: Vec<Bucket<K, V>>,
pub hash_f: F,
pub n_items: usize,
pub _marker: PhantomData<K>,
}

impl<K, V, F> HashTable<K, V, F>
where
K: Eq + Hash + Clone,
V: Clone,
F: Fn(&K) -> usize,
{
pub fn new(hash_f: F, size: usize) -> Self {
let mut buckets = Vec::with_capacity(size);
for _ in 0..size {
buckets.push(Bucket {
items: LinkedList::new(),
});
}
Self {
buckets,
hash_f,
n_items: 0,
_marker: PhantomData,
}
}

pub fn insert(&mut self, key: K, value: V) {
let bnum = (self.hash_f)(&key) % self.buckets.len();
for item in self.buckets[bnum].items.iter_mut() {
if item.key == key {
item.value = value;
return;
}
}
self.buckets[bnum].items.push_back(HashItem { key, value });
self.n_items += 1;
if (self.n_items as f32) / (self.buckets.len() as f32) > 0.75 {
self.resize();
}
}

pub fn get(&self, key: &K) -> Option<&V> {
let bnum = (self.hash_f)(key) % self.buckets.len();
for item in self.buckets[bnum].items.iter() {
if &item.key == key {
return Some(&item.value);
}
}
None
}

pub fn remove(&mut self, key: &K) {
let bnum = (self.hash_f)(key) % self.buckets.len();
let mut new_list = LinkedList::new();
let mut removed = false;
while let Some(item) = self.buckets[bnum].items.pop_front() {
if !removed && &item.key == key {
removed = true;
self.n_items = self.n_items.saturating_sub(1);
} else {
new_list.push_back(item);
}
}
self.buckets[bnum].items = new_list;
}

pub fn resize(&mut self) {
let old_buckets = std::mem::take(&mut self.buckets);
let new_size = if old_buckets.is_empty() { 1 } else { old_buckets.len() * 2 };
let mut buckets = Vec::with_capacity(new_size);
for _ in 0..new_size {
buckets.push(Bucket {
items: LinkedList::new(),
});
}
self.buckets = buckets;
self.n_items = 0;

for bucket in old_buckets {
for item in bucket.items {
self.insert(item.key, item.value);
}
}
}
}
