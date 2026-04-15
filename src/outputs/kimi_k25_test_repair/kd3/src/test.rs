
use crate::kdtree::{KDTree, KDTreeIterator, Point};
fn main() {
let mut tree = KDTree::new();
let mut x = vec![0.0, 1.0, 2.0, 3.0, 4.0];
let mut y = vec![0.0, 1.0, 2.0, 3.0, 4.0];
let mut z = vec![0.0, 1.0, 2.0, 3.0, 4.0];
tree.build(&mut x, &mut y, &mut z, 5);
let mut iter = KDTreeIterator::new();
tree.search(&mut iter, -10.0, 0.0, 0.0, 9.999);
println!("Found {} points in radius 9.999", iter.size);
let mut iter = KDTreeIterator::new();
tree.search(&mut iter, 0.0, 0.0, 0.0, 0.499);
println!("Found {} points in radius 0.499", iter.size);
let mut iter = KDTreeIterator::new();
tree.search(&mut iter, 0.5, 0.5, 0.5, 0.5);
println!("Found {} points near (0.5, 0.5, 0.5)", iter.size);
let mut iter = KDTreeIterator::new();
tree.search(&mut iter, 0.5, 0.5, 0.5, 100.0);
println!("Found {} points in radius 100.0", iter.size);
let mut iter = KDTreeIterator::new();
tree.search(&mut iter, 0.5, 0.5, 0.0, 0.5);
println!("Found {} points at (0.5, 0.5, 0.0)", iter.size);
let mut iter = KDTreeIterator::new();
tree.search(&mut iter, 0.5, 0.5, 1.0, 0.5);
println!("Found {} points at (0.5, 0.5, 1.0)", iter.size);
let mut content: Vec<Point> = Vec::new();
let mut iter = KDTreeIterator::new();
tree.search(&mut iter, 0.0, 0.0, 0.0, 10.0);
while let Some(point) = iter.get_next() {
content.push(point);
}
println!("Collected {} points in content vector", content.len());
let space_iter = tree.search_space(0.0, 2.0, 0.0, 2.0, 0.0, 2.0);
println!("Found {} points in space [0,2]x[0,2]x[0,2]", space_iter.size);
}
