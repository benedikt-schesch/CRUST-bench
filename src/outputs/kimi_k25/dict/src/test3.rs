use dict::dict::{
split_prefix_suffix,
};

fn main() {
let (prefix, suffix): (Vec<char>, Vec<char>) = split_prefix_suffix();
assert!(prefix.is_empty() && suffix.is_empty());
}
