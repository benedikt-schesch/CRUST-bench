use crate::compiler::Pos;
use crate::vector::{vector_back_ptr_or_null, vector_empty, vector_pop, vector_push, Vector};
use lazy_static::lazy_static;
use std::sync::Mutex;
#[derive(Debug, Default, Clone)]
pub struct NodeBinded {
pub owner: Option<Box<Node>>,
pub function: Option<Box<Node>>,
}
#[derive(Debug, Default, Clone)]
pub struct Node {
pub r#type: i32,
pub flags: i32,
pub pos: Pos,
pub binded: NodeBinded,
pub cval: Option<char>,
pub sval: Option<String>,
pub inum: Option<u32>,
pub lnum: Option<u64>,
pub llnum: Option<u64>,
}
lazy_static! {
static ref NODES: Mutex<Vec<Node>> = Mutex::new(Vec::new());
}
lazy_static! {
static ref NODE_VECTOR: Mutex<Option<Vector>> = Mutex::new(None);
}
lazy_static! {
static ref NODE_VECTOR_ROOT: Mutex<Option<Vector>> = Mutex::new(None);
}
fn encode_index(idx: u64) -> [u8; 8] {
idx.to_le_bytes()
}
fn decode_index(bytes: &[u8]) -> Option<u64> {
if bytes.len() < 8 {
return None;
}
let mut arr = [0u8; 8];
arr.copy_from_slice(&bytes[..8]);
Some(u64::from_le_bytes(arr))
}
pub fn node_set_vector(vec: Vector, root_vec: Vector) {
*NODE_VECTOR.lock().unwrap() = Some(vec);
*NODE_VECTOR_ROOT.lock().unwrap() = Some(root_vec);
}
pub fn node_push(node: &Node) {
let mut nodes = NODES.lock().unwrap();
nodes.push(node.clone());
let idx = (nodes.len() - 1) as u64;
drop(nodes);
if let Some(ref mut vec) = *NODE_VECTOR.lock().unwrap() {
vector_push(vec, &encode_index(idx));
}
}
pub fn node_peek_or_null() -> Option<Node> {
let mut guard = NODE_VECTOR.lock().unwrap();
let vec = guard.as_mut()?;
let bytes = vector_back_ptr_or_null(vec)?;
let idx = decode_index(bytes)? as usize;
let nodes = NODES.lock().unwrap();
nodes.get(idx).cloned()
}
pub fn node_peek() -> Node {
node_peek_or_null().unwrap_or_default()
}
pub fn node_pop() -> Node {
let popped = node_peek();
let popped_idx = {
let mut vec_guard = NODE_VECTOR.lock().unwrap();
if let Some(ref mut vec) = *vec_guard {
let idx = vector_back_ptr_or_null(vec).and_then(|b| decode_index(b));
vector_pop(vec);
idx
} else {
None
}
};
let root_idx = {
let mut root_guard = NODE_VECTOR_ROOT.lock().unwrap();
if let Some(ref mut root_vec) = *root_guard {
vector_back_ptr_or_null(root_vec).and_then(|b| decode_index(b))
} else {
None
}
};
if popped_idx.is_some() && popped_idx == root_idx {
let mut root_guard = NODE_VECTOR_ROOT.lock().unwrap();
if let Some(ref mut root_vec) = *root_guard {
vector_pop(root_vec);
}
}
popped
}
pub fn node_create(template: &Node) -> Node {
let node = template.clone();
node_push(&node);
node
}
