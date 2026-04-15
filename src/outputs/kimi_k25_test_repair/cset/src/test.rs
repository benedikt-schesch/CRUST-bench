use cset::Cset;
fn main() {
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
assert!(set1.remove(20));
assert!(!set1.contains(&20));
assert_eq!(set1.len(), 2);
let set2 = set1.clone();
assert_eq!(set2.len(), 2);
let mut set3 = Cset::new();
set3.insert(30);
set3.insert(40);
set3.insert(50);
let mut union_set = Cset::new();
union_set.union(&set1, &set3);
assert_eq!(union_set.len(), 4); 
let mut intersect_set = Cset::new();
intersect_set.intersect(&set1, &set3);
assert_eq!(intersect_set.len(), 1); 
let mut diff_set = Cset::new();
diff_set.difference(&set1, &set3);
assert_eq!(diff_set.len(), 1); 
let sym_diff = set1.symmetric_difference(&set3);
assert_eq!(sym_diff.len(), 3); 
let mut small = Cset::new();
small.insert(10);
assert!(small.is_subset(&set1));
assert!(set1.is_superset(&small));
let mut other = Cset::new();
other.insert(100);
other.insert(200);
assert!(set1.is_disjoint(&other));
let mut found = false;
for value in set1.iter() {
if *value == 10 {
found = true;
}
assert_ne!(*value, 99);
}
assert!(found);
set1.clear();
assert!(set1.is_empty());
println!("All tests passed!");
}
