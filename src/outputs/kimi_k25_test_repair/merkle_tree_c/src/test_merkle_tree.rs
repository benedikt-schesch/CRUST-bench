
use crate::merkle_tree::*;
use crate::blake2b::Blake2b;
fn test_merge(_: Option<&mut ()>, left: &CbmtNode, right: &CbmtNode) -> CbmtNode {
let mut hasher = Blake2b::new();
let mut data = Vec::new();
data.extend_from_slice(&left.bytes);
data.extend_from_slice(&right.bytes);
let hash = hasher.hash(&data);
let mut result = CbmtNode::default();
let len = std::cmp::min(CBMT_NODE_SIZE, hash.len());
result.bytes[..len].copy_from_slice(&hash[..len]);
result
}
fn main() {
let mut tree = CbmtTree::new();
let leaves = CbmtLeaves {
nodes: vec![
CbmtNode { bytes: [1, 0, 0, 0] },
CbmtNode { bytes: [2, 0, 0, 0] },
],
};
let ret = cbmt_build_merkle_tree(&mut tree, &leaves, test_merge);
assert!(ret);
let root = cbmt_tree_root(&tree);
assert_ne!(root.bytes, [0; 4]);
let root_opt = cbmt_build_merkle_root(&leaves, test_merge);
assert!(root_opt.is_some());
assert_eq!(root_opt.unwrap().bytes, root.bytes);
let indices = CbmtIndices {
values: vec![0],
capacity: 1,
};
let proof = cbmt_tree_build_proof(&tree, &indices);
assert!(proof.is_some());
let proof_leaves = CbmtLeaves {
nodes: vec![leaves.nodes[0]],
};
let ret = cbmt_proof_verify(&proof.unwrap(), &root, &proof_leaves, test_merge);
assert!(ret);
let mut tree2 = CbmtTree::new();
let leaves2 = CbmtLeaves {
nodes: vec![
CbmtNode { bytes: [1, 0, 0, 0] },
CbmtNode { bytes: [2, 0, 0, 0] },
CbmtNode { bytes: [3, 0, 0, 0] },
CbmtNode { bytes: [4, 0, 0, 0] },
],
};
let ret = cbmt_build_merkle_tree(&mut tree2, &leaves2, test_merge);
assert!(ret);
let indices2 = CbmtIndices {
values: vec![0, 2],
capacity: 2,
};
let proof2 = cbmt_tree_build_proof(&tree2, &indices2);
assert!(proof2.is_some());
let proof_leaves2 = CbmtLeaves {
nodes: vec![leaves2.nodes[0], leaves2.nodes[2]],
};
let root2 = cbmt_tree_root(&tree2);
let ret = cbmt_proof_verify(&proof2.unwrap(), &root2, &proof_leaves2, test_merge);
assert!(ret);
println!("All tests passed!");
}
