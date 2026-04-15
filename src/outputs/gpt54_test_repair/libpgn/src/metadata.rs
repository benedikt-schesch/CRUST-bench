use std::collections::HashMap;
use crate::utils::buffer::PgnBuffer;
use crate::utils::cursor::pgn_cursor_skip_newline;
const PGN_METADATA_INITIAL_SIZE: usize = 8;
const PGN_METADATA_GROW_SIZE: usize = 8;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PgnMetadataItem {
key: PgnBuffer,
value: PgnBuffer,
}
impl PgnMetadataItem {
pub fn new() -> Self {
Self {
key: PgnBuffer::new(),
value: PgnBuffer::new(),
}
}
}
#[derive(Debug)]
pub struct PgnMetadata {
items: HashMap<String, String>,
}
impl PgnMetadata {
pub fn new() -> Self {
let _ = PGN_METADATA_INITIAL_SIZE;
let _ = PGN_METADATA_GROW_SIZE;
Self {
items: HashMap::new(),
}
}
pub fn from_string_with_consumption(s: &str, consumed: &mut usize) -> Self {
let bytes = s.as_bytes();
let mut cursor = 0usize;
if bytes.get(cursor).copied() != Some(b'[') {
return Self::new();
}
let mut metadata = Self::new();
let mut key_buffer = PgnBuffer::new();
let mut value_buffer = PgnBuffer::new();
loop {
if bytes.get(cursor).copied() != Some(b'[') {
break;
}
cursor += 1;
while bytes.get(cursor).copied() != Some(b' ') {
key_buffer.append(bytes[cursor] as char);
cursor += 1;
}
key_buffer.append_null_terminator();
cursor += 1;
assert_eq!(bytes.get(cursor).copied(), Some(b'"'));
cursor += 1;
while bytes.get(cursor).copied() != Some(b'"') {
value_buffer.append(bytes[cursor] as char);
cursor += 1;
}
value_buffer.append_null_terminator();
metadata.insert(key_buffer.as_str(), value_buffer.as_str());
key_buffer.reset();
value_buffer.reset();
cursor += 1;
assert_eq!(bytes.get(cursor).copied(), Some(b']'));
cursor += 1;
pgn_cursor_skip_newline(s, &mut cursor);
}
*consumed += cursor;
metadata
}
pub fn from_string(s: &str) -> Self {
let mut consumed = 0;
Self::from_string_with_consumption(s, &mut consumed)
}
pub fn insert(&mut self, key: &str, value: &str) {
self.items.insert(key.to_string(), value.to_string());
}
pub fn get(&self, key: &str) -> Option<&str> {
self.items.get(key).map(String::as_str)
}
pub fn delete(&mut self, key: &str) {
self.items.remove(key);
}
}
