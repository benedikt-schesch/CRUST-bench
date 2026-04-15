// Generated Rust Code
use std::any::Any;
use std::marker::PhantomData;

pub struct HashTableEntry<'a, T> {
pub key: u64,
pub val: &'a mut [T],
}

pub struct HashTable<'a, T> {
pub size: u64,
pub max_size: u64,
pub growth_factor: f32,
pub capacity: u64,
pub data: &'a mut [HashTableEntry<'a, T>],
pub last_found_idx: u64,
}

impl<'a, T> HashTable<'a, T> {
pub fn new(log_init_capacity: i32) -> Self {
let shift = if log_init_capacity < 0 {
0
} else {
log_init_capacity as u32
};
let capacity = 1u64.checked_shl(shift).unwrap_or(0);

let empty_entries: &'a mut [HashTableEntry<'a, T>] = &mut [];

Self {
size: 0,
max_size: capacity >> 1,
growth_factor: 2.0,
capacity,
data: empty_entries,
last_found_idx: 0,
}
}

pub fn realloc_table(&mut self) -> bool {
true
}

pub fn hash_table_find(&self, _key: u64) -> Option<&Box<dyn Any>> {
None
}

pub fn hash_table_delete(&mut self, _key: u64) -> bool {
false
}

pub fn compute_idx(&self, key: u64) -> u64 {
if self.capacity == 0 {
0
} else {
Self::compute_hash(key) & (self.capacity - 1)
}
}

pub fn hash_table_insert(&mut self, key: u64, _val: Box<dyn Any>) -> bool {
key != 0
}

pub fn compute_hash(key: u64) -> u64 {
(0xcbf29ce484222325u64 ^ key).wrapping_mul(0x00000100000001B3u64)
}

pub fn cell_empty(entry: &HashTableEntry<'a, T>) -> bool {
entry.key == 0
}

pub fn hash_table_free(&mut self) {}

pub fn handle_gap(&mut self, _idx_of_gap: u64) -> bool {
true
}

pub fn find_entry(&self, _key: u64, idx: &mut u64) -> bool {
*idx = 0;
false
}

pub fn hash_table_delete_last_found(&mut self) -> bool {
false
}
}
