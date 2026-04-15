// Generated Rust Code
use std::sync::Mutex;

pub const CBMT_NODE_SIZE: usize = 4;
// pub const CBMT_NODE_SIZE: usize = 32;
pub const CBMT_ERROR_OVER_CAPACITY: i32 = -1;
pub const CBMT_ERROR_QUEUE_EMPTY: i32 = -2;
pub const CBMT_ERROR_PROOF_ROOT: i32 = -3;
pub const CBMT_ERROR_BUILD_PROOF: i32 = -4;
pub const CBMT_ERROR_INVALID_CAPACITY: i32 = -5;
pub const CBMT_ERROR_VERIFY_FAILED: i32 = -6;
pub const CBMT_FATAL_BUILD_PROOF: i32 = -99;

#[derive(Debug, Default)]
pub struct CbmtBuffer<'a> {
pub data: &'a mut [u8],
pub capacity: usize,
}

#[derive(Debug, Clone, Default)]
pub struct CbmtNode {
pub bytes: [u8; CBMT_NODE_SIZE],
}

#[derive(Debug, Clone)]
pub struct CbmtIndices {
pub values: Vec<u32>,
pub capacity: usize,
}

#[derive(Debug, Clone)]
pub struct CbmtProof {
pub indices: CbmtIndices,
pub lemmas: Vec<CbmtNode>,
}

#[derive(Debug, Clone, Default)]
pub struct CbmtTree {
pub nodes: Vec<CbmtNode>,
pub length: usize,
pub capacity: usize,
}

#[derive(Debug, Clone)]
pub struct CbmtLeaves {
pub nodes: Vec<CbmtNode>,
}

#[derive(Debug)]
pub struct CbmtQueue<'a> {
pub buffer: CbmtBuffer<'a>,
pub width: usize,
pub length: usize,
pub capacity: usize,
pub tail: usize,
pub head: usize,
}

#[derive(Debug, Clone)]
pub struct CbmtNodePair {
pub index: u32,
pub node: CbmtNode,
}

pub type CbmtNodeMergeFn<Ctx> = fn(ctx: &mut Ctx, left: &CbmtNode, right: &CbmtNode) -> CbmtNode;

fn cbmt_is_left(index: u32) -> bool {
(index & 1) == 1
}

fn cbmt_parent(index: u32) -> u32 {
if index == 0 {
0
} else {
(index - 1) >> 1
}
}

fn cbmt_sibling(index: u32) -> u32 {
if index == 0 {
0
} else {
((index + 1) ^ 1) - 1
}
}

fn write_bytes_at(buf: &mut [u8], slot: usize, width: usize, item: &[u8]) {
let start = slot * width;
buf[start..start + width].copy_from_slice(&item[..width]);
}

pub fn cbmt_universal_swap(left: &mut [u8], right: &mut [u8], width: usize) {
let len = width.min(left.len()).min(right.len());
let mut i = 0usize;
while i < len {
let tmp = left[i];
left[i] = right[i];
right[i] = tmp;
i += 1;
}
}

pub fn cbmt_simple_bubble_sort<T>(slice: &mut [T], cmp: fn(&T, &T) -> i32) {
if slice.len() < 2 {
return;
}
for i in 0..(slice.len() - 1) {
for j in (i + 1)..slice.len() {
if cmp(&slice[i], &slice[j]) > 0 {
slice.swap(i, j);
}
}
}
}

pub fn cbmt_uint32_reverse_cmp(left: &u32, right: &u32) -> i32 {
right.wrapping_sub(*left) as i32
}

pub fn cbmt_buffer_init<'a>(buffer: &mut CbmtBuffer<'a>, data: &'a mut [u8]) {
buffer.data = data;
buffer.capacity = buffer.data.len();
}

pub fn cbmt_leaves_init(leaves: &mut CbmtLeaves, nodes: Vec<CbmtNode>) {
leaves.nodes = nodes;
}

pub fn cbmt_indices_init(indices: &mut CbmtIndices, values: Vec<u32>) {
indices.capacity = values.len();
indices.values = values;
}

pub fn cbmt_queue_init<'a>(
queue: &mut CbmtQueue<'a>,
buffer: CbmtBuffer<'a>,
width: usize,
capacity: usize,
) -> i32 {
if capacity * width > buffer.capacity {
return CBMT_ERROR_OVER_CAPACITY;
}
if buffer.capacity % width != 0 {
return CBMT_ERROR_INVALID_CAPACITY;
}
queue.buffer = buffer;
queue.capacity = capacity;
queue.width = width;
queue.length = 0;
queue.head = 0;
queue.tail = 0;
0
}

pub fn cbmt_queue_push_back(queue: &mut CbmtQueue, item: &[u8]) -> i32 {
if queue.length >= queue.capacity {
return CBMT_ERROR_OVER_CAPACITY;
}
write_bytes_at(queue.buffer.data, queue.head, queue.width, item);
queue.head = (queue.head + 1) % queue.capacity;
queue.length += 1;
0
}

pub fn cbmt_queue_push_front(queue: &mut CbmtQueue, item: &[u8]) -> i32 {
if queue.length >= queue.capacity {
return CBMT_ERROR_OVER_CAPACITY;
}
queue.tail = (queue.tail + queue.capacity - 1) % queue.capacity;
write_bytes_at(queue.buffer.data, queue.tail, queue.width, item);
queue.length += 1;
0
}

pub fn cbmt_queue_pop_front(queue: &mut CbmtQueue, item: &mut [u8]) -> i32 {
if queue.length == 0 {
return CBMT_ERROR_QUEUE_EMPTY;
}
let start = queue.tail * queue.width;
item[..queue.width].copy_from_slice(&queue.buffer.data[start..start + queue.width]);
queue.tail = (queue.tail + 1) % queue.capacity;
queue.length -= 1;
0
}

pub fn cbmt_queue_front<'a>(queue: &'a CbmtQueue<'a>) -> Option<&'a [u8]> {
if queue.length == 0 {
return None;
}
let start = queue.tail * queue.width;
Some(&queue.buffer.data[start..start + queue.width])
}

pub fn cbmt_node_copy(dest: &mut CbmtNode, src: &CbmtNode) {
dest.bytes.copy_from_slice(&src.bytes);
}

pub fn cbmt_node_cmp(left: &CbmtNode, right: &CbmtNode) -> i32 {
let left_value = i32::from_le_bytes(left.bytes);
let right_value = i32::from_le_bytes(right.bytes);
left_value.wrapping_sub(right_value)
}

pub fn cbmt_node_pair_reverse_cmp(left: &CbmtNodePair, right: &CbmtNodePair) -> i32 {
right.index.wrapping_sub(left.index) as i32
}

pub fn cbmt_tree_build_proof(tree: &CbmtTree, leaf_indices: &CbmtIndices) -> Result<CbmtProof, i32> {
if tree.length == 0 || leaf_indices.values.is_empty() {
return Err(CBMT_ERROR_BUILD_PROOF);
}

let leaves_count = ((tree.length >> 1) + 1) as u32;
let mut queue: Vec<u32> = Vec::with_capacity(leaf_indices.values.len());

for &idx in &leaf_indices.values {
queue.push(idx + (leaves_count - 1));
}

cbmt_simple_bubble_sort(&mut queue, cbmt_uint32_reverse_cmp);

if queue.is_empty() {
return Err(CBMT_ERROR_BUILD_PROOF);
}

let first_value = queue[0];
if first_value >= ((leaves_count << 1) - 1) {
return Err(CBMT_ERROR_BUILD_PROOF);
}

let mut lemmas: Vec<CbmtNode> = Vec::new();

while !queue.is_empty() {
let index = queue.remove(0);
if index == 0 {
if !queue.is_empty() {
return Err(CBMT_FATAL_BUILD_PROOF);
}
break;
}

let sibling = cbmt_sibling(index);
if !queue.is_empty() && queue[0] == sibling {
queue.remove(0);
} else {
lemmas.push(tree.nodes[sibling as usize].clone());
}

let parent = cbmt_parent(index);
if parent != 0 {
queue.push(parent);
}
}

let mut indices = CbmtIndices {
values: leaf_indices
.values
.iter()
.map(|v| *v + (leaves_count - 1))
.collect(),
capacity: leaf_indices.capacity,
};

if indices.values.len() >= 2 {
for i in 0..(indices.values.len() - 1) {
for j in (i + 1)..indices.values.len() {
let left_index = indices.values[i] as usize;
let right_index = indices.values[j] as usize;
let order = cbmt_node_cmp(&tree.nodes[left_index], &tree.nodes[right_index]);
if order > 0 {
indices.values.swap(i, j);
}
}
}
}

Ok(CbmtProof { indices, lemmas })
}

pub fn cbmt_tree_root(tree: &CbmtTree) -> CbmtNode {
if tree.length == 0 {
CbmtNode {
bytes: [0u8; CBMT_NODE_SIZE],
}
} else {
tree.nodes[0].clone()
}
}

pub fn cbmt_proof_root<Ctx>(
proof: &CbmtProof,
root: &mut CbmtNode,
leaves: &CbmtLeaves,
merge: CbmtNodeMergeFn<Ctx>,
merge_ctx: &mut Ctx,
_nodes_buffer: CbmtBuffer,
_pairs_buffer: CbmtBuffer,
) -> i32 {
if leaves.nodes.len() != proof.indices.values.len() || leaves.nodes.is_empty() {
return CBMT_ERROR_PROOF_ROOT;
}

let mut leaves_clone = leaves.nodes.clone();
cbmt_simple_bubble_sort(&mut leaves_clone, cbmt_node_cmp);

let mut queue: Vec<CbmtNodePair> = Vec::with_capacity(leaves.nodes.len());
for (i, node) in leaves_clone.iter().enumerate() {
queue.push(CbmtNodePair {
index: proof.indices.values[i],
node: node.clone(),
});
}

cbmt_simple_bubble_sort(&mut queue, cbmt_node_pair_reverse_cmp);

let mut lemmas_offset = 0usize;

while !queue.is_empty() {
let pair_current = queue.remove(0);
let index = pair_current.index;
let node = pair_current.node;

if index == 0 {
if proof.lemmas.len() == lemmas_offset && queue.is_empty() {
cbmt_node_copy(root, &node);
return 0;
} else {
return CBMT_ERROR_PROOF_ROOT;
}
}

let sibling_opt = if !queue.is_empty() && queue[0].index == cbmt_sibling(index) {
Some(queue.remove(0).node)
} else if lemmas_offset < proof.lemmas.len() {
let n = proof.lemmas[lemmas_offset].clone();
lemmas_offset += 1;
Some(n)
} else {
None
};

if let Some(sibling) = sibling_opt {
let parent = if cbmt_is_left(index) {
merge(merge_ctx, &node, &sibling)
} else {
merge(merge_ctx, &sibling, &node)
};

queue.push(CbmtNodePair {
index: cbmt_parent(index),
node: parent,
});

cbmt_simple_bubble_sort(&mut queue, cbmt_node_pair_reverse_cmp);
}
}

0
}

type NoCtxMergeFn = fn(Option<&mut ()>, &CbmtNode, &CbmtNode) -> CbmtNode;

static NOCTX_MERGE_FN: Mutex<Option<NoCtxMergeFn>> = Mutex::new(None);

fn cbmt_noctx_merge_adapter(_ctx: &mut (), left: &CbmtNode, right: &CbmtNode) -> CbmtNode {
let merge_fn = get_noctx_merge_fn().unwrap_or(default_noctx_merge);
merge_fn(None, left, right)
}

fn default_noctx_merge(_ctx: Option<&mut ()>, _left: &CbmtNode, _right: &CbmtNode) -> CbmtNode {
CbmtNode::default()
}

fn set_noctx_merge_fn(f: NoCtxMergeFn) {
if let Ok(mut guard) = NOCTX_MERGE_FN.lock() {
*guard = Some(f);
}
}

fn clear_noctx_merge_fn() {
if let Ok(mut guard) = NOCTX_MERGE_FN.lock() {
*guard = None;
}
}

fn get_noctx_merge_fn() -> Option<NoCtxMergeFn> {
match NOCTX_MERGE_FN.lock() {
Ok(guard) => *guard,
Err(_) => None,
}
}

pub fn cbmt_proof_verify(
proof: &CbmtProof,
expected_root: &CbmtNode,
leaves: &CbmtLeaves,
merge: fn(Option<&mut ()>, &CbmtNode, &CbmtNode) -> CbmtNode,
) -> i32 {
let mut target_root = CbmtNode::default();
let mut empty1 = [];
let mut empty2 = [];
let nodes_buffer = CbmtBuffer {
data: &mut empty1,
capacity: 0,
};
let pairs_buffer = CbmtBuffer {
data: &mut empty2,
capacity: 0,
};

set_noctx_merge_fn(merge);
let ret = cbmt_proof_root(
proof,
&mut target_root,
leaves,
cbmt_noctx_merge_adapter,
&mut (),
nodes_buffer,
pairs_buffer,
);
clear_noctx_merge_fn();

if ret != 0 {
return ret;
}
if target_root.bytes != expected_root.bytes {
return CBMT_ERROR_VERIFY_FAILED;
}
0
}

pub fn cbmt_build_merkle_root(
leaves: &CbmtLeaves,
merge: fn(Option<&mut ()>, &CbmtNode, &CbmtNode) -> CbmtNode,
) -> Result<CbmtNode, i32> {
let length = leaves.nodes.len();
if length == 0 {
return Ok(CbmtNode {
bytes: [0u8; CBMT_NODE_SIZE],
});
}

let mut queue: Vec<CbmtNode> = Vec::with_capacity((length + 1) >> 1);

let mut i = length as isize - 1;
while i > 0 {
let left = &leaves.nodes[(i - 1) as usize];
let right = &leaves.nodes[i as usize];
let merged = merge(None, left, right);
queue.push(merged);
i -= 2;
}

if length % 2 == 1 {
queue.insert(0, leaves.nodes[0].clone());
}

while queue.len() > 1 {
let right = queue.remove(0);
let left = queue.remove(0);
let merged = merge(None, &left, &right);
queue.push(merged);
}

Ok(queue.remove(0))
}

pub fn cbmt_build_merkle_tree(
tree: &mut CbmtTree,
leaves: &CbmtLeaves,
merge: fn(Option<&mut ()>, &CbmtNode, &CbmtNode) -> CbmtNode,
) -> i32 {
tree.capacity = leaves.nodes.len().saturating_mul(2).saturating_sub(1);
tree.nodes.clear();

if !leaves.nodes.is_empty() {
let length = leaves.nodes.len() * 2 - 1;
tree.length = length;
tree.nodes = vec![CbmtNode::default(); length];

let offset = leaves.nodes.len() - 1;
for i in 0..leaves.nodes.len() {
tree.nodes[offset + i] = leaves.nodes[i].clone();
}

for i in 0..(leaves.nodes.len() - 1) {
let rev_idx = leaves.nodes.len() - 2 - i;
let left = tree.nodes[(rev_idx << 1) + 1].clone();
let right = tree.nodes[(rev_idx << 1) + 2].clone();
tree.nodes[rev_idx] = merge(None, &left, &right);
}
} else {
tree.length = 0;
}

0
}

fn cbmt_build_tree_merge_adapter<Ctx>(
ctx: &mut BuildTreeMergeCtx<'_, Ctx>,
left: &CbmtNode,
right: &CbmtNode,
) -> CbmtNode {
(ctx.merge)(ctx.merge_ctx, left, right)
}

struct BuildTreeMergeCtx<'a, Ctx> {
merge: CbmtNodeMergeFn<Ctx>,
merge_ctx: &'a mut Ctx,
}

pub fn cbmt_build_merkle_proof<Ctx>(
proof: &mut CbmtProof,
leaves: &CbmtLeaves,
leaf_indices: &CbmtIndices,
merge: CbmtNodeMergeFn<Ctx>,
merge_ctx: &mut Ctx,
_nodes_buffer: CbmtBuffer,
_indices_buffer: CbmtBuffer,
_lemmas_buffer: CbmtBuffer,
) -> i32 {
let mut tree = CbmtTree::default();

let mut adapter_ctx = BuildTreeMergeCtx { merge, merge_ctx };

let ret = cbmt_proof_build_tree_with_ctx(&mut tree, leaves, &mut adapter_ctx);
if ret != 0 {
ret
} else {
match cbmt_tree_build_proof(&tree, leaf_indices) {
Ok(p) => {
*proof = p;
0
}
Err(e) => e,
}
}
}

fn cbmt_proof_build_tree_with_ctx<Ctx>(
tree: &mut CbmtTree,
leaves: &CbmtLeaves,
adapter_ctx: &mut BuildTreeMergeCtx<'_, Ctx>,
) -> i32 {
tree.capacity = leaves.nodes.len().saturating_mul(2).saturating_sub(1);
tree.nodes.clear();

if !leaves.nodes.is_empty() {
let length = leaves.nodes.len() * 2 - 1;
tree.length = length;
tree.nodes = vec![CbmtNode::default(); length];

let offset = leaves.nodes.len() - 1;
for i in 0..leaves.nodes.len() {
tree.nodes[offset + i] = leaves.nodes[i].clone();
}

for i in 0..(leaves.nodes.len() - 1) {
let rev_idx = leaves.nodes.len() - 2 - i;
let left = tree.nodes[(rev_idx << 1) + 1].clone();
let right = tree.nodes[(rev_idx << 1) + 2].clone();
tree.nodes[rev_idx] = cbmt_build_tree_merge_adapter(adapter_ctx, &left, &right);
}
} else {
tree.length = 0;
}

0
}
