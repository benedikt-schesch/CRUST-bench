//! Hash table implementation for the lambda calculus evaluator.

use crate::common::AstNode;
use std::collections::HashMap;

/// Hash table structure
pub struct HashTable {
map: HashMap<String, AstNode>,
}

impl HashTable {
/// Create a new hash table
pub fn new() -> Self {
HashTable {
map: HashMap::new(),
}
}

/// Insert a key-value pair
pub fn insert(&mut self, key: &str, value: AstNode) {
self.map.insert(key.to_string(), value);
}

/// Get a value by key
pub fn get(&self, key: &str) -> Option<&AstNode> {
self.map.get(key)
}

/// Check if key exists
pub fn contains(&self, key: &str) -> bool {
self.map.contains_key(key)
}
}

impl Default for HashTable {
fn default() -> Self {
Self::new()
}
}
