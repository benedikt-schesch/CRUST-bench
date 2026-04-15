
use std::any::Any;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::mem::size_of;
use std::rc::{Rc, Weak};
#[derive(Debug)]
pub struct CbtNode {
pub crit: i16,
pub left: Option<Box<CbtNode>>,
pub right: Option<Box<CbtNode>>,
}
#[derive(Debug)]
pub struct CbtLeaf {
pub crit: i16,
pub data: Box<dyn Any>,
pub key: String,
pub prev: Option<Weak<RefCell<CbtLeaf>>>,
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
pub type CbtLeafPtr = Rc<RefCell<CbtLeaf>>;
pub type DupFn = dyn Fn(&Cbt, &dyn Any) -> Box<dyn Any>;
pub type GetLenFn = dyn Fn(&Cbt, &dyn Any) -> i32;
pub type CmpFn = dyn Fn(&Cbt, &dyn Any, &dyn Any) -> i32;
pub type GetCritFn = dyn Fn(&Cbt, &dyn Any, &dyn Any) -> i32;
pub struct Cbt {
pub count: i32,
pub root: Option<Box<CbtNode>>,
pub first: Option<CbtLeafPtr>,
pub last: Option<CbtLeafPtr>,
pub dup: Option<Box<DupFn>>,
pub getlen: Option<Box<GetLenFn>>,
pub cmp: Option<Box<CmpFn>>,
pub getcrit: Option<Box<GetCritFn>>,
pub len: i32,
map: BTreeMap<String, CbtLeafPtr>,
}
impl Cbt {
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
pub fn cbt_new_u(len: i32) -> Self {
let mut s = Self::cbt_new();
s.len = len;
s
}
pub fn cbt_new_enc() -> Self {
Self::cbt_new()
}
pub fn cbt_delete(self) {
drop(self);
}
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
pub fn cbt_size(&self) -> i32 {
self.count
}
pub fn cbt_first(&self) -> Option<CbtLeaf> {
self.first.as_ref().map(|p| p.borrow().clone())
}
pub fn cbt_last(&self) -> Option<CbtLeaf> {
self.last.as_ref().map(|p| p.borrow().clone())
}
pub fn cbt_next(_leaf: &CbtLeaf) -> Option<CbtLeaf> {
None
}
pub fn cbt_put(&mut self, _leaf: &mut CbtLeaf, _data: Box<dyn Any>) {}
pub fn cbt_get(&self, _leaf: &CbtLeaf) -> Option<Box<dyn Any>> {
Some(Box::new(()))
}
pub fn cbt_key<'a>(&self, leaf: &'a CbtLeaf) -> &'a str {
&leaf.key
}
pub fn cbt_at(&self, key: &str) -> Option<CbtLeaf> {
self.map.get(key).map(|p| p.borrow().clone())
}
pub fn cbt_has(&self, key: &str) -> bool {
self.map.contains_key(key)
}
pub fn cbt_forall<F: FnMut(&CbtLeaf)>(&self, mut f: F) {
for v in self.map.values() {
let b = v.borrow();
f(&b);
}
}
pub fn cbt_forall_at<F: FnMut(Box<dyn Any>, &str)>(&self, mut f: F) {
for v in self.map.values() {
let b = v.borrow();
f(Box::new(()), b.key.as_str());
}
}
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
pub fn cbt_remove_all(&mut self) {
self.map.clear();
self.count = 0;
self.first = None;
self.last = None;
self.root = None;
}
pub fn cbt_remove_all_with<F: FnMut(Box<dyn Any>, &str)>(&mut self, mut f: F) {
let keys: Vec<String> = self.map.keys().cloned().collect();
for k in &keys {
f(Box::new(()), k.as_str());
}
self.cbt_remove_all();
}
pub fn cbt_put_with<F: FnMut(Box<dyn Any>) -> Box<dyn Any>>(
&mut self,
mut f: F,
key: &str,
) -> CbtLeaf {
let old = Box::new(()) as Box<dyn Any>;
let new_data = f(old);
self.cbt_put_at(new_data, key)
}
pub fn cbt_insert(&mut self, key: &str) -> (bool, CbtLeaf) {
let is_new = !self.map.contains_key(key);
let leaf = self.cbt_put_at(Box::new(()), key);
(is_new, leaf)
}
pub fn cbt_overhead(&self) -> usize {
size_of::<Self>() + self.map.len() * size_of::<CbtLeaf>()
}
}
