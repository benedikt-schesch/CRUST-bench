
use crate::check;
use crate::hash::{alternative_hash, compare_keys, hash_by_power, ALTERNATIVE_POWER, POWER};
use crate::log::{LogType, Logger};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
const LOAD_FACTOR: f32 = 1.0;
#[cfg(debug_assertions)]
const EPS: f32 = 1e-3;
#[derive(Clone)]
pub struct CuckooEntry {
key: Option<&'static str>,
data: Option<&'static str>,
marker: u32,
}
pub struct CuckooHashTable {
cur_size: u32,
cur_marker: u32,
max_size: u32,
first_arr: Vec<CuckooEntry>,
second_arr: Vec<CuckooEntry>,
}
impl CuckooHashTable {
pub fn new(initial_size: usize) -> Arc<RwLock<Self>> {
let _phantom_use: Option<(HashMap<(), ()>, Arc<Logger>, LogType)> = None;
check!(initial_size >= 2);
check!(initial_size as u32 != POWER && initial_size as u32 != ALTERNATIVE_POWER);
let size = initial_size / 2;
let empty = CuckooEntry {
key: None,
data: None,
marker: 0,
};
Arc::new(RwLock::new(Self {
cur_size: 0,
cur_marker: 0,
max_size: size as u32,
first_arr: vec![empty.clone(); size],
second_arr: vec![empty; size],
}))
}
pub fn insert(&mut self, mut key: &'static str, mut data: &'static str) {
self.cur_marker = self.cur_marker.wrapping_add(1);
if 1.0 + self.cur_size as f32 > self.max_size as f32 * LOAD_FACTOR {
self.resize();
}
let first_idx = (hash_by_power(key, POWER) % self.max_size) as usize;
if self.first_arr[first_idx].key.is_none() {
self.first_arr[first_idx].key = Some(key);
self.first_arr[first_idx].data = Some(data);
self.cur_size += 1;
return;
} else {
check!(
compare_keys(self.first_arr[first_idx].key.unwrap_or(""), key)
!= std::cmp::Ordering::Equal
);
}
let second_idx = (alternative_hash(key) % self.max_size) as usize;
if self.second_arr[second_idx].key.is_none() {
self.second_arr[second_idx].key = Some(key);
self.second_arr[second_idx].data = Some(data);
self.cur_size += 1;
return;
} else {
check!(
compare_keys(self.second_arr[second_idx].key.unwrap_or(""), key)
!= std::cmp::Ordering::Equal
);
}
self.cur_size += 1;
loop {
{
let entry = &mut self.first_arr[first_idx];
let old_key = entry.key.replace(key).expect("occupied entry expected");
let old_data = entry.data.replace(data).expect("occupied entry expected");
key = old_key;
data = old_data;
entry.marker = self.cur_marker;
}
let second_idx = (alternative_hash(key) % self.max_size) as usize;
if self.second_arr[second_idx].marker == self.cur_marker {
self.refill();
self.insert(key, data);
break;
}
if self.second_arr[second_idx].key.is_none() {
self.second_arr[second_idx].key = Some(key);
self.second_arr[second_idx].data = Some(data);
break;
} else {
check!(
compare_keys(self.second_arr[second_idx].key.unwrap_or(""), key)
!= std::cmp::Ordering::Equal
);
}
{
let entry = &mut self.second_arr[second_idx];
let old_key = entry.key.replace(key).expect("occupied entry expected");
let old_data = entry.data.replace(data).expect("occupied entry expected");
key = old_key;
data = old_data;
entry.marker = self.cur_marker;
}
let first_idx = (hash_by_power(key, POWER) % self.max_size) as usize;
if self.first_arr[first_idx].marker == self.cur_marker {
self.refill();
self.insert(key, data);
break;
}
if self.first_arr[first_idx].key.is_none() {
self.first_arr[first_idx].key = Some(key);
self.first_arr[first_idx].data = Some(data);
break;
} else {
check!(
compare_keys(self.first_arr[first_idx].key.unwrap_or(""), key)
!= std::cmp::Ordering::Equal
);
}
}
}
pub fn find(&self, key: &str) -> Option<&'static str> {
let first_idx = (hash_by_power(key, POWER) % self.max_size) as usize;
let first = &self.first_arr[first_idx];
if let Some(k) = first.key {
if compare_keys(k, key) == std::cmp::Ordering::Equal {
return first.data;
}
}
let second_idx = (hash_by_power(key, ALTERNATIVE_POWER) % self.max_size) as usize;
let second = &self.second_arr[second_idx];
if let Some(k) = second.key {
if compare_keys(k, key) == std::cmp::Ordering::Equal {
return second.data;
}
}
None
}
fn resize(&mut self) {
self.recreate(self.max_size * 2);
}
fn refill(&mut self) {
self.recreate(self.max_size + 1);
}
fn recreate(&mut self, new_size: u32) {
check!(new_size > 0);
#[cfg(debug_assertions)]
check!(new_size as f32 * LOAD_FACTOR + EPS > self.cur_size as f32);
let old_first = self.first_arr.clone();
let old_second = self.second_arr.clone();
let empty = CuckooEntry {
key: None,
data: None,
marker: 0,
};
self.cur_size = 0;
self.cur_marker = 0;
self.max_size = new_size;
self.first_arr = vec![empty.clone(); new_size as usize];
self.second_arr = vec![empty; new_size as usize];
for entry in old_first {
if let (Some(k), Some(d)) = (entry.key, entry.data) {
self.insert(k, d);
}
}
for entry in old_second {
if let (Some(k), Some(d)) = (entry.key, entry.data) {
self.insert(k, d);
}
}
}
fn get_first_entry(&self, _key: &str) -> &mut CuckooEntry {
panic!("get_first_entry is not used in this safe Rust implementation")
}
fn get_second_entry(&self, _key: &str) -> &mut CuckooEntry {
panic!("get_second_entry is not used in this safe Rust implementation")
}
fn try_to_store(
&mut self,
_key: &'static str,
_data: &'static str,
_entry: &mut CuckooEntry,
) -> bool {
panic!("try_to_store is not used in this safe Rust implementation")
}
fn swap_key_data_entry(
&mut self,
_key: &mut &'static str,
_data: &mut &'static str,
_entry: &mut CuckooEntry,
) {
panic!("swap_key_data_entry is not used in this safe Rust implementation")
}
fn free_cukoo_hash_table(self) {
drop(self);
}
}
