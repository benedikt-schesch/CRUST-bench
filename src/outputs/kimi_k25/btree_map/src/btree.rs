use std::any::Any;
use std::sync::Arc;

pub const BTREE_KEY_SIZE: usize = 10;

#[derive(Clone)]
pub struct BTreeKey {
pub key: Vec<u8>,
pub len: usize,
}

#[derive(Clone)]
pub struct Value {
pub value: Vec<u8>,
pub len: usize,
}

pub struct Entry {
pub key: BTreeKey,
pub value: Value,
}

pub struct EntryList {
pub entries: Vec<Entry>,
pub len: usize,
pub cap: usize,
}

#[derive(Clone)]
pub struct Node {
pub key_hash: u32,
pub p_key: [u8; BTREE_KEY_SIZE],
pub key_len: usize,
pub value: Value,
pub child_left: Option<Arc<Node>>,
pub child_right: Option<Arc<Node>>,
}

pub struct BTree {
pub node: Option<Arc<Node>>,
}

impl BTree {
pub fn new_btree() -> Self {
BTree { node: None }
}

pub fn add_entry(&mut self, key: Vec<u8>, key_len: usize, value: Vec<u8>, value_len: usize) {
let new_node = Node::new_node(key, key_len, value, value_len);
match &mut self.node {
None => {
self.node = Some(new_node);
}
Some(root) => {
Arc::make_mut(root).add_node(new_node);
}
}
}

pub fn list_entries(&self) -> EntryList {
let cap = self.get_entry_count();
let mut list = EntryList {
entries: Vec::with_capacity(cap),
len: 0,
cap,
};
if let Some(node) = &self.node {
node.list_node_entries(&mut list);
}
list
}

pub fn remove_entry(&mut self, key: &Vec<u8>, key_len: usize) {
if let Some(node) = &mut self.node {
let actual_key_len = min_size(BTREE_KEY_SIZE, key_len);
let key_hash = calc_key_hash(key, actual_key_len);
let new_root = Node::delete_node(node, key_hash, key.clone(), actual_key_len);
self.node = new_root;
}
}

pub fn get_entry_count(&self) -> usize {
match &self.node {
None => 0,
Some(node) => node.get_node_count(),
}
}

pub fn find_entry(&self, key: &Vec<u8>, key_len: usize) -> Option<Value> {
match &self.node {
None => None,
Some(node) => {
let actual_key_len = min_size(BTREE_KEY_SIZE, key_len);
let key_hash = calc_key_hash(key, actual_key_len);
node.find_value(key_hash, key.clone(), actual_key_len)
}
}
}

pub fn free_tree(&mut self) {
if let Some(node) = &mut self.node {
Arc::make_mut(node).free_node();
}
self.node = None;
}
}

impl Node {
pub fn new_node(key: Vec<u8>, key_len: usize, value: Vec<u8>, value_len: usize) -> Arc<Self> {
let actual_key_len = min_size(BTREE_KEY_SIZE, key_len);
let mut p_key = [0u8; BTREE_KEY_SIZE];
for i in 0..actual_key_len {
if i < key.len() {
p_key[i] = key[i];
}
}

let actual_value_len = min_size(value.len(), value_len);
let mut value_vec = vec![0u8; actual_value_len];
for i in 0..actual_value_len {
if i < value.len() {
value_vec[i] = value[i];
}
}

Arc::new(Node {
key_hash: calc_key_hash(&key, key_len),
p_key,
key_len: actual_key_len,
value: Value {
value: value_vec,
len: actual_value_len,
},
child_left: None,
child_right: None,
})
}

pub fn add_node(&mut self, n_node: Arc<Node>) {
if n_node.key_hash > self.key_hash {
match &mut self.child_right {
None => {
self.child_right = Some(n_node);
}
Some(right) => {
Arc::make_mut(right).add_node(n_node);
}
}
} else if n_node.key_hash == self.key_hash {
let compare_len = min_size(self.key_len, n_node.key_len);
let keys_equal = self.p_key[..compare_len] == n_node.p_key[..compare_len];
if keys_equal {
self.value = n_node.value.clone();
} else {
match &mut self.child_left {
None => {
self.child_left = Some(n_node);
}
Some(left) => {
Arc::make_mut(left).add_node(n_node);
}
}
}
} else {
match &mut self.child_left {
None => {
self.child_left = Some(n_node);
}
Some(left) => {
Arc::make_mut(left).add_node(n_node);
}
}
}
}

pub fn free_node(&mut self) {
if let Some(left) = &mut self.child_left {
Arc::make_mut(left).free_node();
}
if let Some(right) = &mut self.child_right {
Arc::make_mut(right).free_node();
}
self.child_left = None;
self.child_right = None;
}

pub fn delete_node(root: &mut Arc<Node>, key_hash: u32, key: Vec<u8>, key_len: usize) -> Option<Arc<Node>> {
let actual_key_len = min_size(BTREE_KEY_SIZE, key_len);

if root.key_hash == key_hash {
let compare_len = min_size(root.key_len, actual_key_len);
let keys_equal = root.p_key[..compare_len] == key[..compare_len];

if keys_equal {
let left = root.child_left.clone();
let mut right = root.child_right.clone();

if left.is_none() {
return right;
} else if right.is_none() {
return left;
}

let mut successor = right.clone().unwrap();
while let Some(left_child) = &successor.child_left {
successor = left_child.clone();
}

let succ_hash = successor.key_hash;
let succ_key = successor.p_key.to_vec();
let succ_key_len = successor.key_len;

let mut right_arc = right.unwrap();
let new_right = Node::delete_node(&mut right_arc, succ_hash, succ_key, succ_key_len);

let new_node = Arc::new(Node {
key_hash: successor.key_hash,
p_key: successor.p_key,
key_len: successor.key_len,
value: successor.value.clone(),
child_left: left,
child_right: new_right,
});

return Some(new_node);
}
}

let mut new_node = Node {
key_hash: root.key_hash,
p_key: root.p_key,
key_len: root.key_len,
value: root.value.clone(),
child_left: root.child_left.clone(),
child_right: root.child_right.clone(),
};

if key_hash < root.key_hash {
if let Some(left) = &mut new_node.child_left {
let new_left = Node::delete_node(left, key_hash, key, key_len);
new_node.child_left = new_left;
}
} else if key_hash > root.key_hash {
if let Some(right) = &mut new_node.child_right {
let new_right = Node::delete_node(right, key_hash, key, key_len);
new_node.child_right = new_right;
}
}

Some(Arc::new(new_node))
}

pub fn get_node_count(&self) -> usize {
let left_count = match &self.child_left {
None => 0,
Some(left) => left.get_node_count(),
};
let right_count = match &self.child_right {
None => 0,
Some(right) => right.get_node_count(),
};
1 + left_count + right_count
}

pub fn list_node_entries(&self, list: &mut EntryList) {
if let Some(left) = &self.child_left {
left.list_node_entries(list);
}

if list.len < list.cap {
let entry = Entry {
key: BTreeKey {
key: self.p_key[..self.key_len].to_vec(),
len: self.key_len,
},
value: self.value.clone(),
};
list.entries.push(entry);
list.len += 1;
}

if let Some(right) = &self.child_right {
right.list_node_entries(list);
}
}

pub fn find_value(&self, key_hash: u32, key: Vec<u8>, key_len: usize) -> Option<Value> {
let actual_key_len = min_size(BTREE_KEY_SIZE, key_len);
if self.key_hash == key_hash {
let compare_len = min_size(self.key_len, actual_key_len);
if self.p_key[..compare_len] == key[..compare_len] {
return Some(self.value.clone());
}
}

if key_hash > self.key_hash {
match &self.child_right {
None => None,
Some(right) => right.find_value(key_hash, key, key_len),
}
} else {
match &self.child_left {
None => None,
Some(left) => left.find_value(key_hash, key, key_len),
}
}
}
}

pub fn min_size(a: usize, b: usize) -> usize {
if a < b {
a
} else {
b
}
}

pub fn calc_key_hash(key: &Vec<u8>, key_len: usize) -> u32 {
let mut key_sum: u32 = 0;
let actual_len = min_size(key_len, key.len());
for i in 0..actual_len {
if key_sum == u32::MAX {
key_sum = 0;
}
let byte_val = key[i] as u32;
let multiplier = (i + 1) as u32;
let product = byte_val.wrapping_mul(multiplier);
key_sum = key_sum.wrapping_add(product);
}
key_sum
}

pub fn btree_malloc<T: Any>(_size: usize) -> T {
panic!("btree_malloc should not be used in Rust implementation")
}

pub fn btree_free<T: Any>(_ptr: &T) {
// Memory is managed automatically by Rust
}
