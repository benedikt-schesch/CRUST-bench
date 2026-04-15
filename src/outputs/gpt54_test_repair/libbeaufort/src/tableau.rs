
pub fn beaufort_tableau(alpha: &str) -> Vec<Vec<u8>> {
let bytes = alpha.as_bytes();
let size = bytes.len();
let mut mat: Vec<Vec<u8>> = Vec::with_capacity(size);
let mut y = 0usize;
while y < size {
let mut row = Vec::with_capacity(size);
let mut x = 0usize;
let mut j = size;
while x < size {
row.push(bytes[(j + y) % size]);
x += 1;
j -= 1;
}
mat.push(row);
y += 1;
}
mat
}
