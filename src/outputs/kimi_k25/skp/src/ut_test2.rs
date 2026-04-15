use skp::skp::skp_;

fn main() {
let from: &[u8] = b"hello world test";
let to: &[u8] = b"world";
let _end: &[u8] = b"test";

// Line 21: sub-slice operation
let len = (&from[1..]).len().saturating_sub(to.len());
assert_eq!(len, 11);

// Line 26
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 11);

// Line 32
let from: &[u8] = b"abcdef";
let to: &[u8] = b"abc";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 3);

// Line 37
let from: &[u8] = b"test";
let to: &[u8] = b"test";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 0);

// Line 42
let from: &[u8] = b"short";
let to: &[u8] = b"longer";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 0);

// Line 48
let from: &[u8] = b"prefix";
let to: &[u8] = b"pre";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 3);

// Line 53
let from: &[u8] = b"suffix";
let to: &[u8] = b"fix";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 3);

// Line 58
let from: &[u8] = b"middle";
let to: &[u8] = b"dd";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 4);

// Line 63: using end variable
let to: &[u8] = b"hello";
let end: &[u8] = b"lo";
let len = to.len().saturating_sub(end.len());
assert_eq!(len, 3);

// Use the imported function
let result = skp_(b"search test", b"test");
assert_eq!(result, Some(7));

println!("All tests passed!");
}
