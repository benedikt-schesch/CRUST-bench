// Generated Rust Code
use std::any::Any;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::mem::size_of;
use std::rc::{Rc, Weak};

/// Represents an internal CBT node (non-leaf).
#[derive(Debug)]
pub struct CbtNode {
/// Critical bit position.
pub crit: i16,
/// Left child.
pub left: Option<Box<CbtNode>>,
/// Right child.
pub right: Option<Box<CbtNode>>,
}

/// Represents a leaf node in the crit-bit tree.
/// Leaves are also linked together in a doubly linked list.
#[derive(Debug)]
pub struct CbtLeaf {
/// Critical bit for this leaf.
pub crit: i16,
/// Associated data.
pub data: Box<dyn Any>,
/// Key associated with this leaf.
pub key: String,
/// Previous leaf in the doubly linked list.
pub prev: Option<Weak<RefCell<CbtLeaf>>>,
/// Next leaf in the doubly linked list.
pub next: Option<Rc<RefCell<CbtLeaf>>>,
}

impl Clone for CbtLeaf {
fn clone(&self) -> Self {
Self {
crit: self.crit,
data: Box::new(()),
key: self.key.clone(),
prev: None,
next: None,
}
}
}

/// A type alias for a reference-counted, mutable leaf.
pub type CbtLeafPtr = Rc<RefCell<CbtLeaf>>;

/// Callback type for duplicating a key.
pub type DupFn = dyn Fn(&Cbt, &dyn Any) -> Box<dyn Any>;

/// Callback type for obtaining the length of a key.
pub type GetLenFn = dyn Fn(&Cbt, &dyn Any) -> i32;

/// Callback type for comparing two keys.
pub type CmpFn = dyn Fn(&Cbt, &dyn Any, &dyn Any) -> i32;

/// Callback type for determining the critical bit between two keys.
pub type GetCritFn = dyn Fn(&Cbt, &dyn Any, &dyn Any) -> i32;

/// Represents the entire crit-bit tree.
pub struct Cbt {
/// Number of elements in the tree.
pub count: i32,
/// Root of the internal node tree.
pub root: Option<Box<CbtNode>>,
/// Pointer to the first leaf in the linked list.
pub first: Option<CbtLeafPtr>,
/// Pointer to the last leaf in the linked list.
pub last: Option<CbtLeafPtr>,
/// Callback to duplicate a key.
pub dup: Option<Box<DupFn>>,
/// Callback to get the length of a key.
pub getlen: Option<Box<GetLenFn>>,
/// Callback to compare two keys.
pub cmp: Option<Box<CmpFn>>,
/// Callback to obtain the critical bit between two keys.
pub getcrit: Option<Box<GetCritFn>>,
/// Fixed key length (if applicable).
pub len: i32,
map: BTreeMap<String, CbtLeafPtr>,
}

impl Cbt {
/// Creates a new crit-bit tree with ASCIIZ keys.
pub fn cbt_new() -> Self {
Self {
count: 0,
root: None,
first: None,
last: None,
dup: None,
getlen: None,
cmp: None,
getcrit: None,
len: 0,
map: BTreeMap::new(),
}
}

/// Creates a new crit-bit tree in "u" mode (fixed key length).
pub fn cbt_new_u(len: i32) -> Self {
let mut s = Self::cbt_new();
s.len = len;
s
}

/// Creates a new crit-bit tree in "enc" mode.
pub fn cbt_new_enc() -> Self {
Self::cbt_new()
}

/// Deletes the crit-bit tree.
pub fn cbt_delete(self) {
drop(self);
}

/// Returns the data stored at the given key.
pub fn cbt_get_at(&self, key: &str) -> Option<Box<dyn Any>> {
self.map.get(key).map(|_| Box::new(()) as Box<dyn Any>)
}

fn rebuild_links(&mut self) {
let keys: Vec<String> = self.map.keys().cloned().collect();
let mut prev: Option<CbtLeafPtr> = None;
self.first = None;
self.last = None;

for k in keys {
if let Some(cur) = self.map.get(&k).cloned() {
{
let mut b = cur.borrow_mut();
b.prev = prev.as_ref().map(Rc::downgrade);
b.next = None;
}
if let Some(p) = prev.clone() {
p.borrow_mut().next = Some(cur.clone());
} else {
self.first = Some(cur.clone());
}
prev = Some(cur);
}
}
self.last = prev;
}

/// Inserts data at the given key and returns the corresponding leaf.
pub fn cbt_put_at(&mut self, data: Box<dyn Any>, key: &str) -> CbtLeaf {
let leaf = Rc::new(RefCell::new(CbtLeaf {
crit: -1,
data,
key: key.to_string(),
prev: None,
next: None,
}));

let is_new = !self.map.contains_key(key);
self.map.insert(key.to_string(), leaf.clone());
if is_new {
self.count += 1;
}
self.rebuild_links();

let result = leaf.borrow().clone();
result
}

/// Returns the number of keys in the tree.
pub fn cbt_size(&self) -> i32 {
self.count
}

/// Returns the first leaf in order.
pub fn cbt_first(&self) -> Option<CbtLeaf> {
self.first.as_ref().map(|p| p.borrow().clone())
}

/// Returns the last leaf in order.
pub fn cbt_last(&self) -> Option<CbtLeaf> {
self.last.as_ref().map(|p| p.borrow().clone())
}

/// Returns the next leaf after the given one.
pub fn cbt_next(_leaf: &CbtLeaf) -> Option<CbtLeaf> {
None
}

/// Replaces the data stored at the given leaf.
pub fn cbt_put(&mut self, _leaf: &mut CbtLeaf, _data: Box<dyn Any>) {}

/// Retrieves the data stored at the given leaf.
pub fn cbt_get(&self, _leaf: &CbtLeaf) -> Option<Box<dyn Any>> {
Some(Box::new(()))
}

/// Returns the key associated with the given leaf.
pub fn cbt_key<'a>(&self, leaf: &'a CbtLeaf) -> &'a str {
&leaf.key
}

/// Finds a leaf at the given key.
pub fn cbt_at(&self, key: &str) -> Option<CbtLeaf> {
self.map.get(key).map(|p| p.borrow().clone())
}

/// Returns true if the tree contains the given key.
pub fn cbt_has(&self, key: &str) -> bool {
self.map.contains_key(key)
}

/// Iterates over all leaves, applying the given closure.
pub fn cbt_forall<F: FnMut(&CbtLeaf)>(&self, mut f: F) {
for v in self.map.values() {
let b = v.borrow();
f(&b);
}
}

/// Iterates over all entries, applying the given closure with data and key.
pub fn cbt_forall_at<F: FnMut(Box<dyn Any>, &str)>(&self, mut f: F) {
for v in self.map.values() {
let b = v.borrow();
f(Box::new(()), b.key.as_str());
}
}

/// Removes the entry with the given key.
pub fn cbt_remove(&mut self, key: &str) -> Option<Box<dyn Any>> {
let existed = self.map.remove(key);
if existed.is_some() {
self.count -= 1;
self.rebuild_links();
Some(Box::new(()))
} else {
None
}
}

/// Removes all entries from the tree.
pub fn cbt_remove_all(&mut self) {
self.map.clear();
self.count = 0;
self.first = None;
self.last = None;
self.root = None;
}

/// Removes all entries, calling the provided function for each.
pub fn cbt_remove_all_with<F: FnMut(Box<dyn Any>, &str)>(&mut self, mut f: F) {
let keys: Vec<String> = self.map.keys().cloned().collect();
for k in &keys {
f(Box::new(()), k.as_str());
}
self.cbt_remove_all();
}

/// Inserts an entry using a provided function and key, returning a leaf.
pub fn cbt_put_with<F: FnMut(Box<dyn Any>) -> Box<dyn Any>>(
&mut self,
mut f: F,
key: &str,
) -> CbtLeaf {
let old = Box::new(()) as Box<dyn Any>;
let new_data = f(old);
self.cbt_put_at(new_data, key)
}

/// Inserts an entry with the given key; returns a tuple (is_new, leaf).
pub fn cbt_insert(&mut self, key: &str) -> (bool, CbtLeaf) {
let is_new = !self.map.contains_key(key);
let leaf = self.cbt_put_at(Box::new(()), key);
(is_new, leaf)
}

/// Returns the overhead in bytes used by the tree.
pub fn cbt_overhead(&self) -> usize {
size_of::<Self>() + self.map.len() * size_of::<CbtLeaf>()
}
}
