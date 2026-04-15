use Bostree::test_tree_sanity;
use Bostree::bostree::BOSTree;
fn main() {
let mut tree = BOSTree::new();
let names = vec!["alice", "bob", "charlie", "david", "eve"];
for name in &names {
let _node = tree.bostree_insert(name.to_string(), None);
}
let w: u32 = 2;
let node = tree.bostree_select(w as usize);
if let Some(n) = node {
println!("Node at index {}: {}", w, n.borrow().key);
}
test_tree_sanity(&tree);
}
