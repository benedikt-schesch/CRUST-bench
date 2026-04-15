use dict::dict::{
another_function,
};

fn main() {
// ... other code ...
let (prefix, suffix): (String, String) = another_function();
assert!(prefix.is_empty() && suffix.is_empty());
}
