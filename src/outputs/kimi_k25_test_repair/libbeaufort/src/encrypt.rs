use crate::tableau::beaufort_tableau;
use crate::BEAUFORT_ALPHA;
pub fn beaufort_encrypt(src: &[u8], key: &[u8], mat: &[&[u8]]) -> Vec<u8> {
if key.is_empty() {
return src.to_vec();
}
if mat.is_empty() {
let alpha_str = std::str::from_utf8(BEAUFORT_ALPHA).expect("BEAUFORT_ALPHA is valid UTF-8");
let owned_mat = beaufort_tableau(alpha_str);
let mat_refs: Vec<&[u8]> = owned_mat.iter().map(|v| v.as_slice()).collect();
return beaufort_encrypt_internal(src, key, &mat_refs);
}
beaufort_encrypt_internal(src, key, mat)
}
fn beaufort_encrypt_internal(src: &[u8], key: &[u8], mat: &[&[u8]]) -> Vec<u8> {
if mat.is_empty() {
return src.to_vec();
}
let rsize = mat[0].len();
let ksize = key.len();
let mut result: Vec<u8> = Vec::with_capacity(src.len());
let mut j: usize = 0;
for &ch in src {
let mut x = 0;
let mut found = false;
for col_idx in 0..rsize {
if mat[0][col_idx] == ch {
x = col_idx;
found = true;
break;
}
}
if !found {
result.push(ch);
continue;
}
let k = key[j % ksize];
j += 1;
let mut y = 0;
found = false;
for row_idx in 0..rsize {
if mat[row_idx][x] == k {
y = row_idx;
found = true;
break;
}
}
if !found {
result.push(ch);
j -= 1;
continue;
}
result.push(mat[y][0]);
}
result
}
