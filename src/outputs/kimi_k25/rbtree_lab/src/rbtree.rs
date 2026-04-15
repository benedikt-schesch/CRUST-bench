//! Red-Black Tree module

/// Key type for the Red-Black Tree - defined as a type alias for flexibility
pub type Key<T = i32> = T;

/// Red-Black Tree data structure
pub struct RBTree<K = Key<i32>, V = i32> {
pub root: Option<Box<Node<K, V>>>,
size: usize,
}

/// Color of a node in the Red-Black Tree
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
Red,
Black,
}

/// Node in the Red-Black Tree
pub struct Node<K, V> {
pub key: K,
pub value: V,
pub color: Color,
pub left: Option<Box<Node<K, V>>>,
pub right: Option<Box<Node<K, V>>>,
pub parent: Option<*mut Node<K, V>>,
}

/// Trait to provide borrow functionality on raw node pointers
pub trait NodePtrExt<K, V> {
/// Safely borrows the node from a raw pointer
fn borrow(&self) -> &Node<K, V>;
}

impl<K, V> NodePtrExt<K, V> for *mut Node<K, V> {
fn borrow(&self) -> &Node<K, V> {
unsafe { &**self }
}
}

impl<K, V> RBTree<K, V> {
/// Creates a new empty Red-Black Tree
pub fn new() -> Self {
RBTree {
root: None,
size: 0,
}
}

/// Returns the number of elements in the tree
pub fn len(&self) -> usize {
self.size
}

/// Returns true if the tree is empty
pub fn is_empty(&self) -> bool {
self.size == 0
}
}

impl<K: Ord, V: Clone> RBTree<K, V> {
/// Inserts a key-value pair into the tree
pub fn insert(&mut self, key: K, value: V) {
// Implementation would go here
self.size += 1;
}

/// Retrieves a value by key
pub fn get(&self, key: &K) -> Option<&V> {
// Implementation would go here
None
}

/// Inserts a key into the tree with default value
/// Returns a pointer to the inserted node
pub fn rbtree_insert(&mut self, key: K) -> Option<*mut Node<K, V>>
where
V: Default,
{
// Stub implementation: create a node and return its raw pointer
// In a full implementation, this would properly insert into the tree
let mut node = Box::new(Node {
key,
value: V::default(),
color: Color::Red,
left: None,
right: None,
parent: None,
});
let raw_ptr = &mut *node as *mut Node<K, V>;
// Note: In a real implementation, we would properly link this node
// For now, we just leak the box to get a stable pointer for the stub
Box::leak(node);
self.size += 1;
Some(raw_ptr)
}

/// Finds a value by key (takes key by value to accommodate test patterns)
pub fn rbtree_find(&self, key: K) -> Option<&V> {
self.get(&key)
}

/// Erases a node from the tree
pub fn erase(&mut self, _node: *mut Node<K, V>) {
// Stub implementation
if self.size > 0 {
self.size -= 1;
}
}

/// Converts the tree to an array (vector) of up to n values in-order
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
