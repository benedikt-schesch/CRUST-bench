
pub const BEAUFORT_ALPHA: &[u8] =
b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
pub const BEAUFORT_VERSION: &str = "1";
pub fn beaufort_decrypt(src: &[u8], key: &[u8], mat: &[&[u8]]) -> Vec<u8> {
if mat.is_empty() || key.is_empty() || mat[0].is_empty() {
return src.to_vec();
}
let rsize = mat[0].len();
let ksize = key.len();
let mut dec = Vec::with_capacity(src.len());
let mut j = 0usize;
for &ch in src {
let mut needed = false;
let mut y = 0usize;
while y < rsize {
if y < mat.len() && !mat[y].is_empty() && ch == mat[y][0] {
needed = true;
break;
}
y += 1;
}
if !needed {
dec.push(ch);
continue;
}
let k = key[j % ksize];
j += 1;
let mut found = false;
let mut x = 0usize;
while x < rsize {
if y < mat.len() && x < mat[y].len() && k == mat[y][x] {
found = true;
break;
}
x += 1;
}
if !found {
dec.push(ch);
j -= 1;
continue;
}
dec.push(mat[0][x]);
}
dec
}
