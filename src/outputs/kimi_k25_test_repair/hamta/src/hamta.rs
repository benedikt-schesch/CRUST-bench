use std::boxed::Box;
pub const FNV_PRIME: u64 = 1099511628211;
pub const FNV_BASE: u64 = 14695981039346656037;
pub const HAMT_NODE_T_FLAG: u32 = 1;
pub const KEY_VALUE_T_FLAG: u32 = 0;
pub const CHUNK_SIZE: usize = 6;
pub type HashFn<T> = fn(&T) -> u32;
pub type EqualsFn<T> = fn(&T, &T) -> bool;
pub type StrFn<T> = fn(&T) -> String;
pub struct KeyValue<T, U> {
pub key: T,
pub value: U,
}
impl<T, U> KeyValue<T, U> {
}
pub enum HamtNode<T, U> {
Leaf(Option<KeyValue<T, U>>),
Sub(SubNode<T, U>),
}
impl<T, U> HamtNode<T, U> {
pub fn hamt_node_search(&self, hash: u32, lvl: i32, key: &T, equals_fn: EqualsFn<T>) -> Option<&KeyValue<T, U>> {
match self {
HamtNode::Leaf(opt_kv) => {
if let Some(kv) = opt_kv {
if equals_fn(&kv.key, key) {
return Some(kv);
}
}
None
}
HamtNode::Sub(sub) => {
let symbol = hamt_get_symbol(hash, lvl) as u32;
let shifted = sub.bitmap >> symbol;
if (shifted & 1) != 0 {
let child_position = (shifted >> 1).count_ones() as usize;
if let Some(child) = sub.children.get(child_position) {
return child.hamt_node_search(hash, lvl + 1, key, equals_fn);
}
}
None
}
}
}
pub fn hamt_node_insert(&mut self, hash: u32, lvl: i32, key: T, value: U, hash_fn: HashFn<T>, equals_fn: EqualsFn<T>) -> (bool, Option<KeyValue<T, U>>) {
if (lvl as usize) * CHUNK_SIZE > 32 {
panic!("HAMT level too deep");
}
match self {
HamtNode::Leaf(opt_kv) => {
if let Some(existing) = opt_kv {
if equals_fn(&existing.key, &key) {
let old = std::mem::replace(opt_kv, Some(KeyValue { key, value }));
return (false, old);
} else {
let taken_kv = opt_kv.take().unwrap();
let existing_hash = hash_fn(&taken_kv.key);
let existing_symbol = hamt_get_symbol(existing_hash, lvl) as u32;
let new_sub = SubNode {
bitmap: 1 << existing_symbol,
children: vec![HamtNode::Leaf(Some(taken_kv))],
};
*self = HamtNode::Sub(new_sub);
return self.hamt_node_insert(hash, lvl, key, value, hash_fn, equals_fn);
}
} else {
*opt_kv = Some(KeyValue { key, value });
return (true, None);
}
}
HamtNode::Sub(sub) => {
let symbol = hamt_get_symbol(hash, lvl) as u32;
let shifted = sub.bitmap >> symbol;
if (shifted & 1) != 0 {
let child_position = (shifted >> 1).count_ones() as usize;
return sub.children[child_position].hamt_node_insert(hash, lvl + 1, key, value, hash_fn, equals_fn);
} else {
let child_position = (shifted >> 1).count_ones() as usize;
sub.bitmap |= 1 << symbol;
sub.children.insert(child_position, HamtNode::Leaf(Some(KeyValue { key, value })));
return (true, None);
}
}
}
}
pub fn hamt_node_remove(&mut self, hash: u32, lvl: i32, key: &T, equals_fn: EqualsFn<T>) -> Option<KeyValue<T, U>> {
if let HamtNode::Sub(sub) = self {
let symbol = hamt_get_symbol(hash, lvl) as u32;
let shifted = sub.bitmap >> symbol;
if (shifted & 1) == 0 {
return None;
}
let child_position = (shifted >> 1).count_ones() as usize;
let is_matching_leaf = if let HamtNode::Leaf(Some(kv)) = &sub.children[child_position] {
equals_fn(&kv.key, key)
} else {
false
};
if is_matching_leaf {
sub.bitmap &= !(1 << symbol);
let removed = sub.children.remove(child_position);
match sub.children.len() {
0 => {
*self = HamtNode::Leaf(None);
}
1 => {
if let HamtNode::Leaf(_) = &sub.children[0] {
let only = sub.children.remove(0);
*self = only;
}
}
_ => {}
}
if let HamtNode::Leaf(Some(kv)) = removed {
return Some(kv);
}
} else {
let result = sub.children[child_position].hamt_node_remove(hash, lvl + 1, key, equals_fn);
if result.is_some() {
if sub.children.len() == 1 {
if let HamtNode::Leaf(_) = &sub.children[0] {
let only = sub.children.remove(0);
*self = only;
}
}
}
return result;
}
}
None
}
pub fn hamt_node_destroy(&mut self, key_dtor: fn(&T), val_dtor: fn(&U)) {
match self {
HamtNode::Leaf(opt_kv) => {
if let Some(kv) = opt_kv.take() {
key_dtor(&kv.key);
val_dtor(&kv.value);
}
}
HamtNode::Sub(sub) => {
for child in &mut sub.children {
child.hamt_node_destroy(key_dtor, val_dtor);
}
}
}
}
pub fn hamt_node_print(&self, lvl: i32, str_fn_key: StrFn<T>, str_fn_val: StrFn<U>) {
let indent = "  ".repeat(lvl as usize);
match self {
HamtNode::Leaf(opt_kv) => {
if let Some(kv) = opt_kv {
println!("{}{} -> {}", indent, str_fn_key(&kv.key), str_fn_val(&kv.value));
} else {
println!("{}<empty>", indent);
}
}
HamtNode::Sub(sub) => {
println!("{}bitmap: {:08x}", indent, sub.bitmap);
for child in &sub.children {
child.hamt_node_print(lvl + 1, str_fn_key, str_fn_val);
}
}
}
}
}
pub struct SubNode<T, U> {
pub bitmap: u32,
pub children: Vec<HamtNode<T, U>>,
}
pub struct Hamt<T, U> {
pub root: Option<Box<HamtNode<T, U>>>,
pub size: i32,
pub hash_fn: HashFn<T>,
pub equals_fn: EqualsFn<T>,
}
impl<T, U> Hamt<T, U> {
pub fn new_hamt(hash_fn: HashFn<T>, equals_fn: EqualsFn<T>) -> Self {
Hamt {
root: None,
size: 0,
hash_fn,
equals_fn,
}
}
pub fn hamt_size(&self) -> i32 {
self.size
}
pub fn hamt_set(&mut self, key: T, value: U, old: &mut Option<KeyValue<T, U>>) -> bool {
if self.size == 0 {
self.root = Some(Box::new(HamtNode::Leaf(Some(KeyValue { key, value }))));
self.size = 1;
*old = None;
return true;
}
let hash = (self.hash_fn)(&key);
if let Some(root) = &mut self.root {
let (inserted, prev) = root.hamt_node_insert(hash, 0, key, value, self.hash_fn, self.equals_fn);
if inserted {
self.size += 1;
*old = None;
true
} else {
*old = prev;
false
}
} else {
unreachable!()
}
}
pub fn hamt_search(&self, key: &T) -> Option<&KeyValue<T, U>> {
if self.size == 0 {
return None;
}
let hash = (self.hash_fn)(key);
self.root.as_ref()?.hamt_node_search(hash, 0, key, self.equals_fn)
}
pub fn hamt_remove(&mut self, key: &T, removed: &mut Option<KeyValue<T, U>>) -> bool {
*removed = None;
if self.size == 0 {
return false;
}
if self.size == 1 {
if let Some(root) = &self.root {
if let HamtNode::Leaf(Some(kv)) = root.as_ref() {
if (self.equals_fn)(&kv.key, key) {
let kv = if let Some(root_box) = self.root.take() {
if let HamtNode::Leaf(Some(kv)) = *root_box {
kv
} else {
unreachable!()
}
} else {
unreachable!()
};
self.size = 0;
*removed = Some(kv);
return true;
} else {
return false;
}
}
}
}
let hash = (self.hash_fn)(key);
if let Some(root) = &mut self.root {
let result = root.hamt_node_remove(hash, 0, key, self.equals_fn);
let found = result.is_some();
if found {
self.size -= 1;
if self.size == 0 {
self.root = None;
}
*removed = result;
}
found
} else {
false
}
}
pub fn hamt_destroy(mut self, key_dtor: fn(&T), val_dtor: fn(&U)) {
if let Some(root) = &mut self.root {
root.hamt_node_destroy(key_dtor, val_dtor);
}
}
pub fn hamt_print(&self, str_fn_key: StrFn<T>, str_fn_val: StrFn<U>) {
if self.size == 0 {
println!("{{}}");
} else if let Some(root) = &self.root {
root.hamt_node_print(0, str_fn_key, str_fn_val);
}
println!("---\n");
}
}
pub fn hamt_int_hash(key: &i32) -> u32 {
let bytes = key.to_ne_bytes();
hamt_fnv1_hash(&bytes)
}
pub fn hamt_str_hash(key: &str) -> u32 {
hamt_fnv1_hash(key.as_bytes())
}
pub fn hamt_int_equals(a: &i32, b: &i32) -> bool {
a == b
}
pub fn hamt_str_equals(a: &str, b: &str) -> bool {
a == b
}
pub fn hamt_fnv1_hash(key: &[u8]) -> u32 {
let mut hash = FNV_BASE;
for &byte in key {
hash = hash.wrapping_mul(FNV_PRIME);
hash ^= byte as u64;
}
hash as u32
}
pub fn hamt_get_symbol(hash: u32, lvl: i32) -> i32 {
let left = (lvl as u32) * (CHUNK_SIZE as u32);
let left_plus_chunk = left + (CHUNK_SIZE as u32);
let right = if left_plus_chunk > 32 {
0
} else {
32 - left_plus_chunk
};
let symbol = hash << left;
((symbol >> (right + left)) & 0x3F) as i32
}
