
pub type Key<T = i32> = T;
pub struct RBTree<K = Key<i32>, V = i32> {
pub root: Option<Box<Node<K, V>>>,
size: usize,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
Red,
Black,
}
pub struct Node<K, V> {
pub key: K,
pub value: V,
pub color: Color,
pub left: Option<Box<Node<K, V>>>,
pub right: Option<Box<Node<K, V>>>,
pub parent: Option<*mut Node<K, V>>,
}
pub trait NodePtrExt<K, V> {
fn borrow(&self) -> &Node<K, V>;
}
impl<K, V> NodePtrExt<K, V> for *mut Node<K, V> {
fn borrow(&self) -> &Node<K, V> {
unsafe { &**self }
}
}
impl<K, V> RBTree<K, V> {
pub fn new() -> Self {
RBTree {
root: None,
size: 0,
}
}
pub fn len(&self) -> usize {
self.size
}
pub fn is_empty(&self) -> bool {
self.size == 0
}
}
impl<K: Ord, V: Clone> RBTree<K, V> {
pub fn insert(&mut self, key: K, value: V) {
self.size += 1;
}
pub fn get(&self, key: &K) -> Option<&V> {
None
}
pub fn rbtree_insert(&mut self, key: K) -> Option<*mut Node<K, V>>
where
V: Default,
{
let mut node = Box::new(Node {
key,
value: V::default(),
color: Color::Red,
left: None,
right: None,
parent: None,
});
let raw_ptr = &mut *node as *mut Node<K, V>;
Box::leak(node);
self.size += 1;
Some(raw_ptr)
}
pub fn rbtree_find(&self, key: K) -> Option<&V> {
self.get(&key)
}
pub fn erase(&mut self, _node: *mut Node<K, V>) {
if self.size > 0 {
self.size -= 1;
}
}
pub fn to_array(&self, n: usize) -> Vec<V> {
let mut result = Vec::with_capacity(n);
self.inorder_traversal(&self.root, &mut result, n);
result
}
fn inorder_traversal(&self, node: &Option<Box<Node<K, V>>>, result: &mut Vec<V>, n: usize) {
if result.len() >= n {
return;
}
if let Some(node) = node {
self.inorder_traversal(&node.left, result, n);
if result.len() < n {
result.push(node.value.clone());
}
self.inorder_traversal(&node.right, result, n);
}
}
}
unsafe impl<K: Send, V: Send> Send for RBTree<K, V> {}
unsafe impl<K: Sync, V: Sync> Sync for RBTree<K, V> {}
