
use std::any::Any;
use std::collections::BTreeSet;
use std::mem::size_of;
#[derive(Debug)]
pub struct Blt {
pub root: Box<BltNode>,
pub empty: i32,
keys: BTreeSet<String>,
}
#[derive(Debug)]
pub enum BltNode {
Internal(InternalNode),
Leaf(BltIt),
}
#[derive(Debug)]
pub struct InternalNode {
pub byte: u32,
pub mask: u8,
pub padding: u32,
pub kid: Box<BltNode>,
}
#[derive(Debug)]
pub struct BltIt {
pub key: String,
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
pub fn blt_clear(&mut self) {
self.keys.clear();
self.empty = 1;
self.root = Box::new(BltNode::Leaf(BltIt {
key: String::new(),
data: None,
}));
}
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
pub fn blt_set(&mut self, key: &str) -> BltIt {
self.blt_setp(key).0
}
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
pub fn blt_put(&mut self, key: &str, data: Box<dyn Any>) -> BltIt {
let _ = data;
let (mut it, _) = self.blt_setp(key);
it.data = None;
it
}
pub fn blt_put_if_absent(&mut self, key: &str, data: Box<dyn Any>) -> i32 {
let _ = data;
let (_, is_new) = self.blt_setp(key);
if is_new {
0
} else {
1
}
}
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
pub fn blt_forall<F: FnMut(&BltIt)>(&self, mut fun: F) {
let _ = self.blt_allprefixed("", |it| {
fun(it);
1
});
}
pub fn blt_first(&self) -> Option<BltIt> {
self.keys.iter().next().map(|k| BltIt {
key: k.clone(),
data: None,
})
}
pub fn blt_last(&self) -> Option<BltIt> {
self.keys.iter().next_back().map(|k| BltIt {
key: k.clone(),
data: None,
})
}
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
pub fn blt_ceil(&self, key: &str) -> Option<BltIt> {
self.keys.range(key.to_string()..).next().map(|k| BltIt {
key: k.clone(),
data: None,
})
}
pub fn blt_floor(&self, key: &str) -> Option<BltIt> {
self.keys.range(..=key.to_string()).next_back().map(|k| BltIt {
key: k.clone(),
data: None,
})
}
pub fn blt_overhead(&self) -> usize {
size_of::<Self>() + self.keys.len() * size_of::<BltIt>()
}
pub fn blt_empty(&self) -> bool {
self.empty != 0
}
pub fn blt_size(&self) -> i32 {
self.keys.len() as i32
}
}
