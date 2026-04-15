use cset::Cset;

fn main() {
// Test 1: Basic operations
let mut set1 = Cset::new();
assert!(set1.is_empty());
assert_eq!(set1.len(), 0);

set1.insert(10);
set1.insert(20);
set1.insert(30);

assert_eq!(set1.len(), 3);
assert!(set1.contains(&10));
assert!(set1.contains(&20));
assert!(set1.contains(&30));
assert!(!set1.contains(&40));

// Test 2: Remove
assert!(set1.remove(20));
assert!(!set1.contains(&20));
assert_eq!(set1.len(), 2);

// Test 3: Clone
let set2 = set1.clone();
assert_eq!(set2.len(), 2);

// Test 4: Union
let mut set3 = Cset::new();
set3.insert(30);
set3.insert(40);
set3.insert(50);

let mut union_set = Cset::new();
union_set.union(&set1, &set3);
assert_eq!(union_set.len(), 4); // 10, 30, 40, 50

// Test 5: Intersection
let mut intersect_set = Cset::new();
intersect_set.intersect(&set1, &set3);
assert_eq!(intersect_set.len(), 1); // 30

// Test 6: Difference
let mut diff_set = Cset::new();
diff_set.difference(&set1, &set3);
assert_eq!(diff_set.len(), 1); // 10

// Test 7: Symmetric Difference
let sym_diff = set1.symmetric_difference(&set3);
assert_eq!(sym_diff.len(), 3); // 10, 40, 50

// Test 8: Subset/Superset
let mut small = Cset::new();
small.insert(10);
assert!(small.is_subset(&set1));
assert!(set1.is_superset(&small));

// Test 9: Disjoint
let mut other = Cset::new();
other.insert(100);
other.insert(200);
assert!(set1.is_disjoint(&other));

// Test 10: Iterator - Line 82
let mut found = false;
for value in set1.iter() {
if *value == 10 {
found = true;
}
// Fixed: dereference value to compare with i32, or compare with &10
assert_ne!(*value, 99);
}
assert!(found);

// Test 11: Clear
set1.clear();
assert!(set1.is_empty());

println!("All tests passed!");
}
