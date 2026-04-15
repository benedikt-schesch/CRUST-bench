//! Test binary for kdtree module

use crate::kdtree::{KDTree, KDTreeIterator, Point};

fn main() {
let mut tree = KDTree::new();

// Sample data
let mut x = vec![0.0, 1.0, 2.0, 3.0, 4.0];
let mut y = vec![0.0, 1.0, 2.0, 3.0, 4.0];
let mut z = vec![0.0, 1.0, 2.0, 3.0, 4.0];

tree.build(&mut x, &mut y, &mut z, 5);

// Test 1: Search with radius
let mut iter = KDTreeIterator::new();
tree.search(&mut iter, -10.0, 0.0, 0.0, 9.999);
println!("Found {} points in radius 9.999", iter.size);

// Test 2: Search with smaller radius
let mut iter = KDTreeIterator::new();
tree.search(&mut iter, 0.0, 0.0, 0.0, 0.499);
println!("Found {} points in radius 0.499", iter.size);

// Test 3: Search at specific point
let mut iter = KDTreeIterator::new();
tree.search(&mut iter, 0.5, 0.5, 0.5, 0.5);
println!("Found {} points near (0.5, 0.5, 0.5)", iter.size);

// Test 4: Search with large radius
let mut iter = KDTreeIterator::new();
tree.search(&mut iter, 0.5, 0.5, 0.5, 100.0);
println!("Found {} points in radius 100.0", iter.size);

// Test 5: Search at (0.5, 0.5, 0.0) with radius 0.5
let mut iter = KDTreeIterator::new();
tree.search(&mut iter, 0.5, 0.5, 0.0, 0.5);
println!("Found {} points at (0.5, 0.5, 0.0)", iter.size);

// Test 6: Search at (0.5, 0.5, 1.0) with radius 0.5
let mut iter = KDTreeIterator::new();
tree.search(&mut iter, 0.5, 0.5, 1.0, 0.5);
println!("Found {} points at (0.5, 0.5, 1.0)", iter.size);

// Test 7: Iterate through results and store in content vector
let mut content: Vec<Point> = Vec::new();
let mut iter = KDTreeIterator::new();
tree.search(&mut iter, 0.0, 0.0, 0.0, 10.0);

while let Some(point) = iter.get_next() {
content.push(point);
}

println!("Collected {} points in content vector", content.len());

// Test 8: Search space
let space_iter = tree.search_space(0.0, 2.0, 0.0, 2.0, 0.0, 2.0);
println!("Found {} points in space [0,2]x[0,2]x[0,2]", space_iter.size);
}
