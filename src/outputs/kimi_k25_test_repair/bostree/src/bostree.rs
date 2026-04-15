use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::cmp::max;
pub struct BOSNode<K> {
pub key: K,
pub parent_node: Option<Weak<RefCell<BOSNode<K>>>>,
pub left_child_node: Option<Rc<RefCell<BOSNode<K>>>>,
pub right_child_node: Option<Rc<RefCell<BOSNode<K>>>>,
pub depth: u32,
pub left_child_count: u32,
pub right_child_count: u32,
}
pub struct BOSTree<K> {
root: Option<Rc<RefCell<BOSNode<K>>>>,
count: usize,
}
impl<K: Ord + Clone> BOSTree<K> {
pub fn new() -> Self {
BOSTree {
root: None,
count: 0,
}
}
pub fn bostree_new<T, U>(_cmp: T, _arg: U) -> Self {
Self::new()
}
pub fn bostree_node_count(&self) -> u32 {
self.count as u32
}
fn height(node: &Option<Rc<RefCell<BOSNode<K>>>>) -> u32 {
node.as_ref().map_or(0, |n| n.borrow().depth)
}
fn get_count(node: &Option<Rc<RefCell<BOSNode<K>>>>) -> u32 {
node.as_ref().map_or(0, |n| {
let n = n.borrow();
n.left_child_count + n.right_child_count + 1
})
}
fn update_node(node: &Rc<RefCell<BOSNode<K>>>) {
let (left, right) = {
let n = node.borrow();
(n.left_child_node.clone(), n.right_child_node.clone())
};
let mut n = node.borrow_mut();
n.depth = 1 + max(Self::height(&left), Self::height(&right));
n.left_child_count = Self::get_count(&left);
n.right_child_count = Self::get_count(&right);
}
fn rotate_right(y: Rc<RefCell<BOSNode<K>>>) -> Rc<RefCell<BOSNode<K>>> {
let x = y.borrow_mut().left_child_node.take().unwrap();
let t2 = x.borrow_mut().right_child_node.take();
if let Some(ref t2_rc) = t2 {
t2_rc.borrow_mut().parent_node = Some(Rc::downgrade(&y));
}
x.borrow_mut().right_child_node = Some(y.clone());
y.borrow_mut().parent_node = Some(Rc::downgrade(&x));
y.borrow_mut().left_child_node = t2;
Self::update_node(&y);
Self::update_node(&x);
x
}
fn rotate_left(x: Rc<RefCell<BOSNode<K>>>) -> Rc<RefCell<BOSNode<K>>> {
let y = x.borrow_mut().right_child_node.take().unwrap();
let t2 = y.borrow_mut().left_child_node.take();
if let Some(ref t2_rc) = t2 {
t2_rc.borrow_mut().parent_node = Some(Rc::downgrade(&x));
}
y.borrow_mut().left_child_node = Some(x.clone());
x.borrow_mut().parent_node = Some(Rc::downgrade(&y));
x.borrow_mut().right_child_node = t2;
Self::update_node(&x);
Self::update_node(&y);
y
}
fn get_balance(node: &Rc<RefCell<BOSNode<K>>>) -> i32 {
let n = node.borrow();
Self::height(&n.left_child_node) as i32 - Self::height(&n.right_child_node) as i32
}
fn rebalance(node: Rc<RefCell<BOSNode<K>>>) -> Rc<RefCell<BOSNode<K>>> {
Self::update_node(&node);
let balance = Self::get_balance(&node);
if balance > 1 {
let left = node.borrow().left_child_node.as_ref().unwrap().clone();
if Self::get_balance(&left) < 0 {
let new_left = Self::rotate_left(left);
node.borrow_mut().left_child_node = Some(new_left);
}
Self::rotate_right(node)
} else if balance < -1 {
let right = node.borrow().right_child_node.as_ref().unwrap().clone();
if Self::get_balance(&right) > 0 {
let new_right = Self::rotate_right(right);
node.borrow_mut().right_child_node = Some(new_right);
}
Self::rotate_left(node)
} else {
node
}
}
fn insert_recursive(node: Rc<RefCell<BOSNode<K>>>, key: K) -> Rc<RefCell<BOSNode<K>>> {
let node_key = node.borrow().key.clone();
if key < node_key {
if let Some(left) = node.borrow().left_child_node.clone() {
let new_left = Self::insert_recursive(left, key);
node.borrow_mut().left_child_node = Some(new_left);
} else {
let new_node = Rc::new(RefCell::new(BOSNode {
key,
parent_node: Some(Rc::downgrade(&node)),
left_child_node: None,
right_child_node: None,
depth: 1,
left_child_count: 0,
right_child_count: 0,
}));
node.borrow_mut().left_child_node = Some(new_node);
}
} else if key > node_key {
if let Some(right) = node.borrow().right_child_node.clone() {
let new_right = Self::insert_recursive(right, key);
node.borrow_mut().right_child_node = Some(new_right);
} else {
let new_node = Rc::new(RefCell::new(BOSNode {
key,
parent_node: Some(Rc::downgrade(&node)),
left_child_node: None,
right_child_node: None,
depth: 1,
left_child_count: 0,
right_child_count: 0,
}));
node.borrow_mut().right_child_node = Some(new_node);
}
}
Self::rebalance(node)
}
pub fn insert(&mut self, key: K) {
if self.root.is_none() {
self.root = Some(Rc::new(RefCell::new(BOSNode {
key,
parent_node: None,
left_child_node: None,
right_child_node: None,
depth: 1,
left_child_count: 0,
right_child_count: 0,
})));
self.count = 1;
return;
}
let root = self.root.take().unwrap();
let new_root = Self::insert_recursive(root, key);
self.root = Some(new_root);
self.count += 1;
}
pub fn bostree_insert(&mut self, key: K, _arg: Option<()>) -> Option<Rc<RefCell<BOSNode<K>>>> {
if self.bostree_lookup(&key).is_some() {
return None;
}
let key_clone = key.clone();
if self.root.is_none() {
let new_node = Rc::new(RefCell::new(BOSNode {
key,
parent_node: None,
left_child_node: None,
right_child_node: None,
depth: 1,
left_child_count: 0,
right_child_count: 0,
}));
self.root = Some(new_node.clone());
self.count = 1;
return Some(new_node);
}
let root = self.root.take().unwrap();
let new_root = Self::insert_recursive(root, key);
self.root = Some(new_root);
self.count += 1;
self.bostree_lookup(&key_clone)
}
pub fn bostree_select(&self, index: usize) -> Option<Rc<RefCell<BOSNode<K>>>> {
if index >= self.count {
return None;
}
Self::select_recursive(&self.root, index)
}
fn select_recursive(node: &Option<Rc<RefCell<BOSNode<K>>>>, index: usize) -> Option<Rc<RefCell<BOSNode<K>>>> {
let n = node.as_ref()?;
let left_count = n.borrow().left_child_count as usize;
if index < left_count {
Self::select_recursive(&n.borrow().left_child_node, index)
} else if index == left_count {
Some(n.clone())
} else {
Self::select_recursive(&n.borrow().right_child_node, index - left_count - 1)
}
}
pub fn bostree_lookup<Q: ?Sized>(&self, key: &Q) -> Option<Rc<RefCell<BOSNode<K>>>>
where
K: std::borrow::Borrow<Q>,
Q: Ord,
{
Self::lookup_recursive(&self.root, key)
}
fn lookup_recursive<Q: ?Sized>(node: &Option<Rc<RefCell<BOSNode<K>>>>, key: &Q) -> Option<Rc<RefCell<BOSNode<K>>>>
where
K: std::borrow::Borrow<Q>,
Q: Ord,
{
let n = node.as_ref()?;
let borrowed = n.borrow();
let node_key = borrowed.key.borrow();
if key == node_key {
Some(n.clone())
} else if key < node_key {
Self::lookup_recursive(&borrowed.left_child_node, key)
} else {
Self::lookup_recursive(&borrowed.right_child_node, key)
}
}
fn find_min(node: Rc<RefCell<BOSNode<K>>>) -> Rc<RefCell<BOSNode<K>>> {
let mut current = node;
loop {
let next = current.borrow().left_child_node.clone();
if let Some(l) = next {
current = l;
} else {
return current;
}
}
}
fn remove_node(&mut self, node: &Rc<RefCell<BOSNode<K>>>) {
let parent_weak = node.borrow().parent_node.clone();
if let Some(parent_weak) = parent_weak {
let parent = parent_weak.upgrade().unwrap();
let is_left = parent.borrow().left_child_node.as_ref().map_or(false, |l| Rc::ptr_eq(l, node));
let left_child = node.borrow().left_child_node.clone();
let right_child = node.borrow().right_child_node.clone();
if left_child.is_none() && right_child.is_none() {
if is_left {
parent.borrow_mut().left_child_node = None;
} else {
parent.borrow_mut().right_child_node = None;
}
} else if left_child.is_none() {
let right = right_child.unwrap();
right.borrow_mut().parent_node = Some(Rc::downgrade(&parent));
if is_left {
parent.borrow_mut().left_child_node = Some(right);
} else {
parent.borrow_mut().right_child_node = Some(right);
}
} else if right_child.is_none() {
let left = left_child.unwrap();
left.borrow_mut().parent_node = Some(Rc::downgrade(&parent));
if is_left {
parent.borrow_mut().left_child_node = Some(left);
} else {
parent.borrow_mut().right_child_node = Some(left);
}
} else {
let right = right_child.unwrap();
let successor = Self::find_min(right);
let succ_key = successor.borrow().key.clone();
self.remove_node(&successor);
node.borrow_mut().key = succ_key;
return;
}
self.count -= 1;
let mut current = parent;
loop {
let parent_weak = current.borrow().parent_node.clone();
current = Self::rebalance(current);
if let Some(p) = parent_weak {
let parent_rc = p.upgrade().unwrap();
let is_left_child = parent_rc.borrow().left_child_node.as_ref().map_or(false, |l| Rc::ptr_eq(l, &current));
if is_left_child {
parent_rc.borrow_mut().left_child_node = Some(current.clone());
} else {
parent_rc.borrow_mut().right_child_node = Some(current.clone());
}
current = parent_rc;
} else {
self.root = Some(current);
break;
}
}
} else {
let left_child = node.borrow().left_child_node.clone();
let right_child = node.borrow().right_child_node.clone();
if left_child.is_none() && right_child.is_none() {
self.root = None;
} else if left_child.is_none() {
let right = right_child.unwrap();
right.borrow_mut().parent_node = None;
self.root = Some(right);
} else if right_child.is_none() {
let left = left_child.unwrap();
left.borrow_mut().parent_node = None;
self.root = Some(left);
} else {
let right = right_child.unwrap();
let successor = Self::find_min(right);
let succ_key = successor.borrow().key.clone();
self.remove_node(&successor);
node.borrow_mut().key = succ_key;
return;
}
self.count -= 1;
}
}
pub fn bostree_remove(&mut self, node: &Rc<RefCell<BOSNode<K>>>) {
self.remove_node(node);
}
}
pub fn bostree_next_node<K>(node: &Rc<RefCell<BOSNode<K>>>) -> Option<Rc<RefCell<BOSNode<K>>>> {
let borrowed = node.borrow();
if let Some(ref right) = borrowed.right_child_node {
let mut current = right.clone();
loop {
let next_left = current.borrow().left_child_node.clone();
if let Some(l) = next_left {
current = l;
} else {
return Some(current);
}
}
}
drop(borrowed);
let mut current = node.clone();
loop {
let parent_weak = current.borrow().parent_node.clone();
if let Some(parent) = parent_weak {
let parent_rc = parent.upgrade()?;
let is_right_child = parent_rc
.borrow()
.right_child_node
.as_ref()
.map_or(false, |r| Rc::ptr_eq(r, &current));
if is_right_child {
current = parent_rc;
} else {
return Some(parent_rc);
}
} else {
return None;
}
}
}
pub fn bostree_previous_node<K>(node: &Rc<RefCell<BOSNode<K>>>) -> Option<Rc<RefCell<BOSNode<K>>>> {
let borrowed = node.borrow();
if let Some(ref left) = borrowed.left_child_node {
let mut current = left.clone();
loop {
let next_right = current.borrow().right_child_node.clone();
if let Some(r) = next_right {
current = r;
} else {
return Some(current);
}
}
}
drop(borrowed);
let mut current = node.clone();
loop {
let parent_weak = current.borrow().parent_node.clone();
if let Some(parent) = parent_weak {
let parent_rc = parent.upgrade()?;
let is_left_child = parent_rc
.borrow()
.left_child_node
.as_ref()
.map_or(false, |l| Rc::ptr_eq(l, &current));
if is_left_child {
current = parent_rc;
} else {
return Some(parent_rc);
}
} else {
return None;
}
}
}
pub fn bostree_rank<K>(node: &Rc<RefCell<BOSNode<K>>>) -> usize {
let mut rank = node.borrow().left_child_count as usize;
let mut current = node.clone();
loop {
let parent_weak = current.borrow().parent_node.clone();
if let Some(parent) = parent_weak {
let parent_rc = parent.upgrade().unwrap();
let parent_borrowed = parent_rc.borrow();
if parent_borrowed
.right_child_node
.as_ref()
.map_or(false, |r| Rc::ptr_eq(r, &current))
{
rank += parent_borrowed.left_child_count as usize + 1;
}
drop(parent_borrowed);
current = parent_rc;
} else {
break;
}
}
rank
}
