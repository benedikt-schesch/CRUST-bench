use dict::dict::{
example_function,
};

fn main() {
let (prefix, suffix): (Vec<u8>, Vec<u8>) = example_function();
assert!(prefix.is_empty() && suffix.is_empty());
}
