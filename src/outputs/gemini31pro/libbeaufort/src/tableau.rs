// Generated Rust Code
pub const BEAUFORT_ALPHA: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
pub const BEAUFORT_VERSION: &str = "1";

pub fn beaufort_tableau(alpha: &str) -> Vec<Vec<u8>> {
let alpha_bytes = alpha.as_bytes();
let size = alpha_bytes.len();
let mut mat = Vec::with_capacity(size);

for y in 0..size {
let mut row = Vec::with_capacity(size);
let mut j = size;
for _ in 0..size {
row.push(alpha_bytes[(j + y) % size]);
j -= 1;
}
mat.push(row);
}

mat
}
