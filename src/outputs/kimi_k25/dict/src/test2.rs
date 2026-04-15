use dict::dict::{
process_data,
};

fn main() {
// ... other code ...
let (prefix, suffix) = process_data();
assert!(prefix.is_empty() && suffix.is_empty());
}
