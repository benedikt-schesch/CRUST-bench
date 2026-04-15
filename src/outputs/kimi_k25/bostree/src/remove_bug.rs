use Bostree::test_tree_sanity;
use Bostree::bostree::BOSTree;

fn test_tree() -> BOSTree<i32> {
let mut tree = BOSTree::new();
for key in 0..10 {
let _node = tree.bostree_insert(key, None);
}
tree
}

fn main() {
let tree = test_tree();
test_tree_sanity(&tree);
}
