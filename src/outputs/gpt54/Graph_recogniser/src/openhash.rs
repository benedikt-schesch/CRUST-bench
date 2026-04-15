// Generated Rust Code
use crate::check;
use crate::hash::{compare_keys, hash_by_power, rehash, POWER, REHASHER};
use crate::log::{LogType, Logger};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

const LOAD_FACTOR: f32 = 0.6;
#[cfg(debug_assertions)]
const EPS: f32 = 1e-3;

#[derive(Clone)]
pub struct OpenEntry {
key: Option<&'static str>,
data: Option<&'static str>,
}

pub struct OpenHashTable {
cur_size: usize,
max_size: usize,
arr: Vec<OpenEntry>,
}

impl OpenHashTable {
pub fn new(initial_size: usize) -> Arc<RwLock<Self>> {
let _phantom_use: Option<(HashMap<(), ()>, Arc<Logger>, LogType)> = None;
check!(initial_size != 0);
check!(initial_size as u32 != POWER && initial_size as u32 != REHASHER);

let arr = vec![
OpenEntry {
key: None,
data: None,
};
initial_size
];

Arc::new(RwLock::new(Self {
cur_size: 0,
max_size: initial_size,
arr,
}))
}

fn query(&self, key: &str) -> usize {
check!(!key.is_empty());
let mut h = hash_by_power(key, POWER);
loop {
let idx = (h as usize) % self.max_size;
let entry = &self.arr[idx];
match entry.key {
None => return idx,
Some(cur_key) if compare_keys(cur_key, key) == std::cmp::Ordering::Equal => {
return idx;
}
_ => {
h = rehash(h);
}
}
}
}

pub fn insert(&mut self, key: &'static str, data: &'static str) {
#[cfg(debug_assertions)]
check!((self.cur_size as f32 / self.max_size as f32) < LOAD_FACTOR + EPS);

if (self.cur_size + 1) as f32 / self.max_size as f32 > LOAD_FACTOR {
self.max_size *= 2;
let old_arr = self.arr.clone();
self.arr = vec![
OpenEntry {
key: None,
data: None,
};
self.max_size
];

for entry in old_arr {
if let Some(cur_key) = entry.key {
let idx = self.query(cur_key);
self.arr[idx] = entry;
}
}
}

let cell_idx = self.query(key);
if self.arr[cell_idx].key.is_none() {
self.cur_size += 1;
}
self.arr[cell_idx].key = Some(key);
self.arr[cell_idx].data = Some(data);
}

pub fn find(&self, key: &str) -> Option<&'static str> {
let cell_idx = self.query(key);
match self.arr[cell_idx].key {
Some(stored_key) if compare_keys(stored_key, key) == std::cmp::Ordering::Equal => {
self.arr[cell_idx].data
}
_ => None,
}
}

pub fn free_open_hash_table(&mut self) {
self.arr.clear();
self.cur_size = 0;
self.max_size = 0;
}
}
