use dict::dict::{
find_matches,
};
fn main() {
let (prefix, suffix): (Vec<u8>, Vec<u8>) = find_matches();
assert!(prefix.is_empty() && suffix.is_empty());
}
