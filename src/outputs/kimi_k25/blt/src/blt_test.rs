use crate::blt::{Blt, BltIt};

struct ArrayData {
p: Vec<String>,
}

fn main() {
let mut tree = Blt::blt_new();

// Insert some test data into the tree
tree.blt_put("alpha", Box::new(1));
tree.blt_put("beta", Box::new(2));
tree.blt_put("gamma", Box::new(3));

let arr = ArrayData {
p: vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()],
};

let mut count = 0;
let mut n = 0;

tree.blt_forall(|it: &BltIt| {
while n + 1 < arr.p.len() && arr.p[n] == arr.p[n + 1] {
n += 1;
}
// Check bounds and process
if n >= arr.p.len() {
return 0; // Stop iteration if we've exceeded array bounds
}
// Verify match between tree key and array element
if it.key == arr.p[n] {
println!("Matched: {}", it.key);
}
n += 1;
count += 1;
1 // Return 1 to continue iteration, 0 to stop
});

println!("Total items processed: {}", count);
}
