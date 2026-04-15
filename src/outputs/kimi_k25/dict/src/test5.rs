use dict::dict::{
analyze,
};

fn main() {
let (prefix, suffix): (String, String) = analyze();
assert!(prefix.is_empty() && suffix.is_empty());
}
