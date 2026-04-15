//! Merkle Tree module with CBMT (Compatible Binary Merkle Tree) API

/// Size of a CBMT node in bytes
pub const CBMT_NODE_SIZE: usize = 4;

/// A node in the CBMT tree
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CbmtNode {
/// Node data as fixed-size byte array
pub bytes: [u8; CBMT_NODE_SIZE],
}

impl Default for CbmtNode {
fn default() -> Self {
CbmtNode {
bytes: [0; CBMT_NODE_SIZE],
}
}
}

/// Collection of leaves for CBMT
pub struct CbmtLeaves {
/// Vector of leaf nodes
pub nodes: Vec<CbmtNode>,
}

/// Indices for proof generation
pub struct CbmtIndices {
/// Vector of indices
pub values: Vec<u32>,
/// Capacity
pub capacity: usize,
}

/// Merkle proof structure
pub struct CbmtProof {
/// Proof lemmas (sibling hashes)
pub lemmas: Vec<CbmtNode>,
/// Indices covered by this proof
pub indices: CbmtIndices,
}

/// CBMT Tree structure
pub struct CbmtTree {
/// All nodes in the tree, stored level by level
/// Layout: [leaves..., level1..., level2..., ..., root]
pub nodes: Vec<CbmtNode>,
/// Number of leaves (length)
pub length: usize,
/// Capacity
pub capacity: usize,
}

impl CbmtTree {
/// Create a new empty tree
pub fn new() -> Self {
CbmtTree {
nodes: Vec::new(),
length: 0,
capacity: 0,
}
}
}

impl Default for CbmtTree {
fn default() -> Self {
Self::new()
}
}

/// Merge function type for combining two nodes
pub type MergeFn = fn(Option<&mut ()>, &CbmtNode, &CbmtNode) -> CbmtNode;

/// Build a merkle tree from leaves
///
/// # Arguments
/// * `tree` - The tree structure to populate
/// * `leaves` - The leaf nodes
/// * `merge` - The merge function to combine nodes
///
/// # Returns
/// true on success, false on error
pub fn cbmt_build_merkle_tree(
tree: &mut CbmtTree,
leaves: &CbmtLeaves,
merge: MergeFn,
) -> bool {
if leaves.nodes.is_empty() {
return false;
}

let mut current_level = leaves.nodes.clone();
tree.length = current_level.len();
tree.nodes.clear();

// Store all nodes level by level
tree.nodes.reserve(current_level.len() * 2);

loop {
tree.nodes.extend_from_slice(&current_level);

if current_level.len() <= 1 {
break;
}

let mut next_level = Vec::new();
for i in (0..current_level.len()).step_by(2) {
let left = &current_level[i];
let right = if i + 1 < current_level.len() {
&current_level[i + 1]
} else {
// If odd number of nodes, duplicate the last one
&current_level[i]
};
let parent = merge(None, left, right);
next_level.push(parent);
}
current_level = next_level;
}

tree.capacity = tree.nodes.capacity();

true
}

/// Get the root of the tree
pub fn cbmt_tree_root(tree: &CbmtTree) -> CbmtNode {
tree.nodes.last().copied().unwrap_or_default()
}

/// Build only the merkle root without storing the full tree
pub fn cbmt_build_merkle_root(
leaves: &CbmtLeaves,
merge: MergeFn,
) -> Option<CbmtNode> {
if leaves.nodes.is_empty() {
return None;
}

let mut current_level = leaves.nodes.clone();

while current_level.len() > 1 {
let mut next_level = Vec::new();
for i in (0..current_level.len()).step_by(2) {
let left = &current_level[i];
let right = if i + 1 < current_level.len() {
&current_level[i + 1]
} else {
&current_level[i]
};
let parent = merge(None, left, right);
next_level.push(parent);
}
current_level = next_level;
}

current_level.first().copied()
}

/// Build a proof for the given indices
pub fn cbmt_tree_build_proof(
tree: &CbmtTree,
indices: &CbmtIndices,
) -> Option<CbmtProof> {
if tree.nodes.is_empty() || indices.values.is_empty() {
return None;
}

let mut lemmas = Vec::new();
let mut current_indices: Vec<usize> = indices.values.iter().map(|&i| i as usize).collect();

// Sort and deduplicate indices
current_indices.sort_unstable();
current_indices.dedup();

// Check bounds
if current_indices.last().map_or(0, |&i| i) >= tree.length {
return None;
}

// Calculate level offsets
let mut level_offsets = vec![0usize];
let mut level_size = tree.length;
let mut total = level_size;

while level_size > 1 {
level_size = (level_size + 1) / 2;
level_offsets.push(total);
total += level_size;
}

// Build proof by traversing up the tree
let mut level = 0;
let mut level_len = tree.length;

while level_len > 1 {
let level_start = level_offsets[level];
let next_level_len = (level_len + 1) / 2;
let mut next_indices = Vec::new();

for &idx in &current_indices {
let sibling = if idx % 2 == 0 {
if idx + 1 < level_len {
idx + 1
} else {
idx // Odd number, duplicate last
}
} else {
idx - 1
};

// Add sibling to lemmas if not in current indices
if !current_indices.contains(&sibling) {
let node_idx = level_start + sibling;
if node_idx < tree.nodes.len() {
lemmas.push(tree.nodes[node_idx]);
}
}

next_indices.push(idx / 2);
}

// Deduplicate next_indices
next_indices.sort_unstable();
next_indices.dedup();

current_indices = next_indices;
level_len = next_level_len;
level += 1;
}

Some(CbmtProof {
lemmas,
indices: CbmtIndices {
values: indices.values.clone(),
capacity: indices.capacity,
},
})
}

/// Verify a merkle proof
pub fn cbmt_proof_verify(
proof: &CbmtProof,
root: &CbmtNode,
leaves: &CbmtLeaves,
merge: MergeFn,
) -> bool {
if leaves.nodes.is_empty() || proof.indices.values.len() != leaves.nodes.len() {
return false;
}

// Pair up leaves with their indices
let mut current: Vec<(usize, CbmtNode)> = proof.indices.values.iter()
.zip(leaves.nodes.iter())
.map(|(&idx, &node)| (idx as usize, node))
.collect();

// Sort by index to process left-to-right
current.sort_by_key(|(idx, _)| *idx);

let mut lemma_idx = 0;

while current.len() > 1 {
let mut next = Vec::new();
let mut i = 0;

while i < current.len() {
let (idx, node) = current[i];
let parent_idx = idx / 2;

if idx % 2 == 0 {
// Left child
let (right, consumed_lemma, step) = if i + 1 < current.len() && current[i + 1].0 == idx + 1 {
// Right child is in the current set
(current[i + 1].1, false, 2)
} else if lemma_idx < proof.lemmas.len() {
// Get right child from proof
(proof.lemmas[lemma_idx], true, 1)
} else {
// Odd number of nodes, duplicate left
(node, false, 1)
};

if consumed_lemma {
lemma_idx += 1;
}
i += step;

let parent = merge(None, &node, &right);
next.push((parent_idx, parent));
} else {
// Right child, left should be in proof
if lemma_idx >= proof.lemmas.len() {
return false;
}
let left = proof.lemmas[lemma_idx];
lemma_idx += 1;
i += 1;

let parent = merge(None, &left, &node);
next.push((parent_idx, parent));
}
}

current = next;
}

if current.len() == 1 {
current[0].1 == *root
} else {
false
}
}
