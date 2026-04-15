
use std::any::Any;
pub struct Blt {
root: Option<Box<Node>>,
size: usize,
}
struct Node {
key: String,
value: Box<dyn Any>,
left: Option<Box<Node>>,
right: Option<Box<Node>>,
}
pub struct BltIt {
pub key: String,
}
impl Blt {
pub fn blt_new() -> Self {
Blt { root: None, size: 0 }
}
pub fn blt_put(&mut self, key: &str, value: Box<dyn Any>) -> Option<Box<dyn Any>> {
let old = Self::put_recursive(&mut self.root, key.to_string(), value);
if old.is_none() {
self.size += 1;
}
old
}
fn put_recursive(node: &mut Option<Box<Node>>, key: String, value: Box<dyn Any>) -> Option<Box<dyn Any>> {
match node {
None => {
*node = Some(Box::new(Node {
key,
value,
left: None,
right: None,
}));
None
}
Some(n) => {
if key < n.key {
Self::put_recursive(&mut n.left, key, value)
} else if key > n.key {
Self::put_recursive(&mut n.right, key, value)
} else {
Some(std::mem::replace(&mut n.value, value))
}
}
}
}
pub fn blt_size(&self) -> usize {
self.size
}
pub fn blt_first(&self) -> Option<BltIt> {
let mut current = &self.root;
while let Some(ref n) = current {
if n.left.is_none() {
return Some(BltIt { key: n.key.clone() });
}
current = &n.left;
}
None
}
pub fn blt_last(&self) -> Option<BltIt> {
let mut current = &self.root;
while let Some(ref n) = current {
if n.right.is_none() {
return Some(BltIt { key: n.key.clone() });
}
current = &n.right;
}
None
}
pub fn blt_next(&self, current: &BltIt) -> Option<BltIt> {
let key = &current.key;
let mut succ = None;
let mut node = &self.root;
while let Some(ref n) = node {
if key < &n.key {
succ = Some(&n.key);
node = &n.left;
} else if key > &n.key {
node = &n.right;
} else {
if let Some(ref right) = n.right {
let mut min = right;
while let Some(ref left) = min.left {
min = left;
}
return Some(BltIt { key: min.key.clone() });
}
break;
}
}
succ.map(|k| BltIt { key: k.clone() })
}
pub fn blt_prev(&self, current: &BltIt) -> Option<BltIt> {
let key = &current.key;
let mut pred = None;
let mut node = &self.root;
while let Some(ref n) = node {
if key > &n.key {
pred = Some(&n.key);
node = &n.right;
} else if key < &n.key {
node = &n.left;
} else {
if let Some(ref left) = n.left {
let mut max = left;
while let Some(ref right) = max.right {
max = right;
}
return Some(BltIt { key: max.key.clone() });
}
break;
}
}
pred.map(|k| BltIt { key: k.clone() })
}
pub fn blt_ceil(&self, key: &str) -> Option<BltIt> {
let mut ceil = None;
let mut node = &self.root;
while let Some(ref n) = node {
if key == n.key {
return Some(BltIt { key: n.key.clone() });
} else if key < &n.key {
ceil = Some(&n.key);
node = &n.left;
} else {
node = &n.right;
}
}
ceil.map(|k| BltIt { key: k.clone() })
}
pub fn blt_floor(&self, key: &str) -> Option<BltIt> {
let mut floor = None;
let mut node = &self.root;
while let Some(ref n) = node {
if key == n.key {
return Some(BltIt { key: n.key.clone() });
} else if key > &n.key {
floor = Some(&n.key);
node = &n.right;
} else {
node = &n.left;
}
}
floor.map(|k| BltIt { key: k.clone() })
}
pub fn blt_forall<F>(&self, mut f: F)
where
F: FnMut(&BltIt) -> i32,
{
Self::inorder(&self.root, &mut f);
}
fn inorder<F>(node: &Option<Box<Node>>, f: &mut F) -> i32
where
F: FnMut(&BltIt) -> i32,
{
if let Some(ref n) = node {
if Self::inorder(&n.left, f) == 0 {
return 0;
}
let it = BltIt { key: n.key.clone() };
if f(&it) == 0 {
return 0;
}
if Self::inorder(&n.right, f) == 0 {
return 0;
}
}
1
}
pub fn blt_allprefixed<F>(&self, prefix: &str, mut f: F)
where
F: FnMut(&BltIt) -> i32,
{
Self::inorder_prefixed(&self.root, prefix, &mut f);
}
fn inorder_prefixed<F>(node: &Option<Box<Node>>, prefix: &str, f: &mut F) -> i32
where
F: FnMut(&BltIt) -> i32,
{
if let Some(ref n) = node {
if Self::inorder_prefixed(&n.left, prefix, f) == 0 {
return 0;
}
if n.key.starts_with(prefix) {
let it = BltIt { key: n.key.clone() };
if f(&it) == 0 {
return 0;
}
}
if Self::inorder_prefixed(&n.right, prefix, f) == 0 {
return 0;
}
}
1
}
}
