pub fn beaufort_tableau(alpha: &str) -> Vec<Vec<u8>> {
let size = alpha.len();
if size == 0 {
return Vec::new();
}
let alpha_bytes = alpha.as_bytes();
let mut mat: Vec<Vec<u8>> = Vec::with_capacity(size);
for y in 0..size {
let mut row: Vec<u8> = Vec::with_capacity(size);
for x in 0..size {
let j = size - x;
let idx = (j + y) % size;
row.push(alpha_bytes[idx]);
}
mat.push(row);
}
mat
}
