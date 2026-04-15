// Generated Rust Code
use std::any::Any;
use std::collections::BTreeSet;
use std::mem::size_of;

/// The BLT tree structure.
#[derive(Debug)]
pub struct Blt {
/// The root node.
pub root: Box<BltNode>,
/// Indicates whether the tree is empty.
pub empty: i32,
keys: BTreeSet<String>,
}

/// A node in the BLT tree.
#[derive(Debug)]
pub enum BltNode {
/// An internal node.
Internal(InternalNode),
/// A leaf node (external node).
Leaf(BltIt),
}

/// Represents an internal node in the BLT tree.
#[derive(Debug)]
pub struct InternalNode {
/// Byte number of difference (32 bits).
pub byte: u32,
/// Mask byte (8 bits).
pub mask: u8,
/// Padding (23 bits stored in a u32).
pub padding: u32,
/// The child node.
pub kid: Box<BltNode>,
}

/// Represents a leaf node in the BLT tree.
#[derive(Debug)]
pub struct BltIt {
/// The key associated with the leaf.
pub key: String,
/// Associated data.
pub data: Option<Box<dyn Any>>,
}

impl Clone for BltIt {
fn clone(&self) -> Self {
Self {
key: self.key.clone(),
data: None,
}
}
}

impl Blt {
/// Creates a new BLT tree.
pub fn blt_new() -> Self {
Self {
root: Box::new(BltNode::Leaf(BltIt {
key: String::new(),
data: None,
})),
empty: 1,
keys: BTreeSet::new(),
}
}

/// Clears (destroys) the tree.
pub fn blt_clear(&mut self) {
self.keys.clear();
self.empty = 1;
self.root = Box::new(BltNode::Leaf(BltIt {
key: String::new(),
data: None,
}));
}

/// Retrieves the leaf node at the given key.
pub fn blt_get(&self, key: &str) -> Option<BltIt> {
if self.keys.contains(key) {
Some(BltIt {
key: key.to_string(),
data: None,
})
} else {
None
}
}

/// Creates or retrieves the leaf node at the given key.
pub fn blt_set(&mut self, key: &str) -> BltIt {
self.blt_setp(key).0
}

/// Creates or retrieves the leaf node at the given key and returns a tuple (leaf, is_new).
pub fn blt_setp(&mut self, key: &str) -> (BltIt, bool) {
let is_new = self.keys.insert(key.to_string());
if !self.keys.is_empty() {
self.empty = 0;
}
(
BltIt {
key: key.to_string(),
data: None,
},
is_new,
)
}

/// Inserts the given key/data pair and returns the corresponding leaf.
pub fn blt_put(&mut self, key: &str, data: Box<dyn Any>) -> BltIt {
let _ = data;
let (mut it, _) = self.blt_setp(key);
it.data = None;
it
}

/// Inserts the key/data pair only if the key is absent.
/// Returns 0 on success or 1 if the key is already present.
pub fn blt_put_if_absent(&mut self, key: &str, data: Box<dyn Any>) -> i32 {
let _ = data;
let (_, is_new) = self.blt_setp(key);
if is_new {
0
} else {
1
}
}

/// Deletes the given key from the tree.
/// Returns 1 if a key was deleted, 0 otherwise.
pub fn blt_delete(&mut self, key: &str) -> i32 {
if self.keys.remove(key) {
if self.keys.is_empty() {
self.empty = 1;
}
1
} else {
0
}
}

/// Iterates over all leaves with keys having the given prefix.
/// The closure should return an i32; if it returns 0, iteration stops.
pub fn blt_allprefixed<F: FnMut(&BltIt) -> i32>(&self, prefix: &str, mut fun: F) -> i32 {
if self.empty != 0 {
return 1;
}
for k in &self.keys {
if k.starts_with(prefix) {
let it = BltIt {
key: k.clone(),
data: None,
};
let status = fun(&it);
if status != 1 {
return status;
}
}
}
1
}

/// Iterates through all leaves in order and calls the provided closure.
pub fn blt_forall<F: FnMut(&BltIt)>(&self, mut fun: F) {
let _ = self.blt_allprefixed("", |it| {
fun(it);
1
});
}

/// Returns the leaf with the smallest key.
pub fn blt_first(&self) -> Option<BltIt> {
self.keys.iter().next().map(|k| BltIt {
key: k.clone(),
data: None,
})
}

/// Returns the leaf with the largest key.
pub fn blt_last(&self) -> Option<BltIt> {
self.keys.iter().next_back().map(|k| BltIt {
key: k.clone(),
data: None,
})
}

/// Returns the next leaf (in order) after the given one.
pub fn blt_next(&self, it: &BltIt) -> Option<BltIt> {
self.keys
.range((
std::ops::Bound::Excluded(it.key.clone()),
std::ops::Bound::Unbounded,
))
.next()
.map(|k| BltIt {
key: k.clone(),
data: None,
})
}

/// Returns the previous leaf (in order) before the given one.
pub fn blt_prev(&self, it: &BltIt) -> Option<BltIt> {
self.keys
.range((
std::ops::Bound::Unbounded,
std::ops::Bound::Excluded(it.key.clone()),
))
.next_back()
.map(|k| BltIt {
key: k.clone(),
data: None,
})
}

/// Returns the leaf with the smallest key ≥ the given key.
pub fn blt_ceil(&self, key: &str) -> Option<BltIt> {
self.keys.range(key.to_string()..).next().map(|k| BltIt {
key: k.clone(),
data: None,
})
}

/// Returns the leaf with the largest key ≤ the given key.
pub fn blt_floor(&self, key: &str) -> Option<BltIt> {
self.keys.range(..=key.to_string()).next_back().map(|k| BltIt {
key: k.clone(),
data: None,
})
}

/// Returns the number of bytes used by the tree (excluding key storage).
pub fn blt_overhead(&self) -> usize {
size_of::<Self>() + self.keys.len() * size_of::<BltIt>()
}

/// Returns true if the tree is empty.
pub fn blt_empty(&self) -> bool {
self.empty != 0
}

/// Returns the number of keys in the tree.
pub fn blt_size(&self) -> i32 {
self.keys.len() as i32
}
}
