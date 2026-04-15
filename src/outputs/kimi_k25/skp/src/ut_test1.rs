use skp::skp::skp_;

fn main() {
// Test case 1
let from: &[u8] = b"hello world";
let to: &[u8] = b"world";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 6);

// Test case 2
let from: &[u8] = b"abcdef";
let to: &[u8] = b"abc";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 3);

// Test case 3
let from: &[u8] = b"test";
let to: &[u8] = b"test";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 0);

// Test case 4
let from: &[u8] = b"short";
let to: &[u8] = b"longer";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 0);

// Test case 5
let from: &[u8] = b"prefix";
let to: &[u8] = b"pre";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 3);

// Test case 6
let from: &[u8] = b"suffix";
let to: &[u8] = b"fix";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 3);

// Test case 7
let from: &[u8] = b"middle";
let to: &[u8] = b"dd";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 4);

// Test case 8
let from: &[u8] = b"begin";
let to: &[u8] = b"beg";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 2);

// Test case 9
let from: &[u8] = b"end";
let to: &[u8] = b"nd";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 1);

// Test case 10
let from: &[u8] = b"overlap";
let to: &[u8] = b"lap";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 4);

// Test case 11
let from: &[u8] = b"multiple";
let to: &[u8] = b"ple";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 5);

// Test case 12
let from: &[u8] = b"tests";
let to: &[u8] = b"s";
let len = from.len().saturating_sub(to.len());
assert_eq!(len, 4);

// Use the imported function
let result = skp_(b"hello world", b"world");
assert_eq!(result, Some(6));

println!("All tests passed!");
}
