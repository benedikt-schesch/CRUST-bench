
use crate::common::AstNode;
use std::collections::HashMap;
pub struct HashTable {
map: HashMap<String, AstNode>,
}
impl HashTable {
pub fn new() -> Self {
HashTable {
map: HashMap::new(),
}
}
pub fn insert(&mut self, key: &str, value: AstNode) {
self.map.insert(key.to_string(), value);
}
pub fn get(&self, key: &str) -> Option<&AstNode> {
self.map.get(key)
}
pub fn contains(&self, key: &str) -> bool {
self.map.contains_key(key)
}
}
impl Default for HashTable {
fn default() -> Self {
Self::new()
}
}
