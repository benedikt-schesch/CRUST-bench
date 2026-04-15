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
#[derive(Clone)]
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
let n_node = Node::new_node(key, key_len, value, value_len);
match self.node.take() {
None => {
self.node = Some(n_node);
}
Some(root) => {
let new_root = Node::insert_into(root, n_node);
self.node = Some(new_root);
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
let key_len = min_size(BTREE_KEY_SIZE, key_len);
let key_hash = calc_key_hash(key, key_len);
self.node = Node::delete_root(self.node.clone(), key_hash, key.clone(), key_len);
}
pub fn get_entry_count(&self) -> usize {
match &self.node {
None => 0,
Some(node) => node.get_node_count(),
}
}
pub fn find_entry(&self, key: &Vec<u8>, key_len: usize) -> Option<Value> {
let node = self.node.as_ref()?;
let key_len = min_size(BTREE_KEY_SIZE, key_len);
let key_hash = calc_key_hash(key, key_len);
node.find_value(key_hash, key.clone(), key_len)
}
pub fn free_tree(&mut self) {
self.node = None;
}
}
impl Node {
pub fn new_node(key: Vec<u8>, key_len: usize, value: Vec<u8>, value_len: usize) -> Arc<Self> {
let actual_key_len = min_size(BTREE_KEY_SIZE, key_len);
let mut p_key = [0u8; BTREE_KEY_SIZE];
let copy_key_len = actual_key_len.min(key.len());
p_key[..copy_key_len].copy_from_slice(&key[..copy_key_len]);
let stored_value_len = value_len.min(value.len());
let stored_value = value[..stored_value_len].to_vec();
Arc::new(Node {
key_hash: calc_key_hash(&p_key[..actual_key_len].to_vec(), actual_key_len),
p_key,
key_len: actual_key_len,
value: Value {
value: stored_value,
len: stored_value_len,
},
child_left: None,
child_right: None,
})
}
pub fn add_node(&mut self, n_node: Arc<Node>) {
let inserted = Node::insert_into(Arc::new(self.clone()), n_node);
if let Some(updated) = Arc::into_inner(inserted) {
*self = updated;
}
}
pub fn free_node(&mut self) {
self.child_left = None;
self.child_right = None;
self.value.value.clear();
self.value.len = 0;
self.key_len = 0;
self.key_hash = 0;
self.p_key = [0u8; BTREE_KEY_SIZE];
}
pub fn delete_node(
root: &mut Arc<Node>,
key_hash: u32,
key: Vec<u8>,
key_len: usize,
) -> Option<Arc<Node>> {
Node::delete_root(Some(root.clone()), key_hash, key, key_len)
}
pub fn get_node_count(&self) -> usize {
1 + self
.child_left
.as_ref()
.map_or(0, |n| n.get_node_count())
+ self.child_right.as_ref().map_or(0, |n| n.get_node_count())
}
pub fn list_node_entries(&self, list: &mut EntryList) {
if let Some(left) = &self.child_left {
left.list_node_entries(list);
}
if list.len < list.cap {
list.entries.push(Entry {
key: BTreeKey {
key: self.p_key[..self.key_len].to_vec(),
len: self.key_len,
},
value: Value {
value: self.value.value[..self.value.len.min(self.value.value.len())].to_vec(),
len: self.value.len,
},
});
list.len += 1;
}
if let Some(right) = &self.child_right {
right.list_node_entries(list);
}
}
pub fn find_value(&self, key_hash: u32, key: Vec<u8>, key_len: usize) -> Option<Value> {
let key_len = min_size(BTREE_KEY_SIZE, key_len);
let cmp_len = key_len.min(key.len());
if self.key_hash == key_hash && self.p_key[..cmp_len] == key[..cmp_len] {
return Some(self.value.clone());
}
if key_hash > self.key_hash {
return self
.child_right
.as_ref()
.and_then(|right| right.find_value(key_hash, key, key_len));
}
self.child_left
.as_ref()
.and_then(|left| left.find_value(key_hash, key, key_len))
}
fn insert_into(root: Arc<Node>, n_node: Arc<Node>) -> Arc<Node> {
if n_node.key_hash > root.key_hash {
let new_right = match &root.child_right {
None => Some(n_node),
Some(r) => Some(Node::insert_into(r.clone(), n_node)),
};
Arc::new(Node {
key_hash: root.key_hash,
p_key: root.p_key,
key_len: root.key_len,
value: root.value.clone(),
child_left: root.child_left.clone(),
child_right: new_right,
})
} else if n_node.key_hash == root.key_hash
&& root.p_key[..n_node.key_len.min(root.key_len)]
== n_node.p_key[..n_node.key_len.min(root.key_len)]
&& n_node.key_len == root.key_len
{
Arc::new(Node {
key_hash: root.key_hash,
p_key: root.p_key,
key_len: root.key_len,
value: n_node.value.clone(),
child_left: root.child_left.clone(),
child_right: root.child_right.clone(),
})
} else {
let new_left = match &root.child_left {
None => Some(n_node),
Some(l) => Some(Node::insert_into(l.clone(), n_node)),
};
Arc::new(Node {
key_hash: root.key_hash,
p_key: root.p_key,
key_len: root.key_len,
value: root.value.clone(),
child_left: new_left,
child_right: root.child_right.clone(),
})
}
}
fn delete_root(
root: Option<Arc<Node>>,
key_hash: u32,
key: Vec<u8>,
key_len: usize,
) -> Option<Arc<Node>> {
let root = root?;
let key_len = min_size(BTREE_KEY_SIZE, key_len);
let cmp_len = key_len.min(key.len());
if key_hash < root.key_hash {
return Some(Arc::new(Node {
key_hash: root.key_hash,
p_key: root.p_key,
key_len: root.key_len,
value: root.value.clone(),
child_left: Node::delete_root(root.child_left.clone(), key_hash, key, key_len),
child_right: root.child_right.clone(),
}));
} else if key_hash > root.key_hash {
return Some(Arc::new(Node {
key_hash: root.key_hash,
p_key: root.p_key,
key_len: root.key_len,
value: root.value.clone(),
child_left: root.child_left.clone(),
child_right: Node::delete_root(root.child_right.clone(), key_hash, key, key_len),
}));
}
if root.p_key[..cmp_len] != key[..cmp_len] || root.key_len != key_len {
return Some(root);
}
if root.child_left.is_none() {
return root.child_right.clone();
}
if root.child_right.is_none() {
return root.child_left.clone();
}
let successor = Node::min_node(root.child_right.as_ref().cloned());
if let Some(temp) = successor {
let new_right = Node::delete_root(
root.child_right.clone(),
temp.key_hash,
temp.p_key[..temp.key_len].to_vec(),
temp.key_len,
);
return Some(Arc::new(Node {
key_hash: temp.key_hash,
p_key: temp.p_key,
key_len: temp.key_len,
value: temp.value.clone(),
child_left: root.child_left.clone(),
child_right: new_right,
}));
}
Some(root)
}
fn min_node(node: Option<Arc<Node>>) -> Option<Arc<Node>> {
let mut current = node?;
loop {
match &current.child_left {
Some(left) => current = left.clone(),
None => return Some(current),
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
let lim = key_len.min(key.len());
for i in 0..lim {
key_sum = key_sum % u32::MAX;
key_sum = key_sum.wrapping_add(((key[i] as u32).wrapping_mul((i as u32) + 1)) % u32::MAX);
}
key_sum
}
pub fn btree_malloc<T: Any>(_: usize) -> T {
panic!("btree_malloc is not needed in safe Rust")
}
pub fn btree_free<T: Any>(_: &T) {}
