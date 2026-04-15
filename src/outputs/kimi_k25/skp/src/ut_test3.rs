use skp::skp::skp_;

fn main() {
// Line 22
let from: &[u8] = b"hello world";
let to: &[u8] = b"world";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 6);

// Line 27
let from: &[u8] = b"abcdef";
let to: &[u8] = b"abc";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 3);

// Line 32
let from: &[u8] = b"test";
let to: &[u8] = b"test";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 0);

// Line 37
let from: &[u8] = b"short";
let to: &[u8] = b"longer";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 0);

// Line 43
let from: &[u8] = b"prefix";
let to: &[u8] = b"pre";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 3);

// Line 49
let from: &[u8] = b"suffix";
let to: &[u8] = b"fix";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 3);

// Line 55
let from: &[u8] = b"middle";
let to: &[u8] = b"dd";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 4);

// Line 61
let from: &[u8] = b"begin";
let to: &[u8] = b"beg";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 2);

// Use the imported function
let result = skp_(b"find this", b"this");
assert_eq!(result, Some(5));

println!("All tests passed!");
}
