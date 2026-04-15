// Generated Rust Code
use crate::tableau::beaufort_tableau;

pub const BEAUFORT_ALPHA: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
pub const BEAUFORT_VERSION: &str = "1";

pub fn beaufort_decrypt(src: &[u8], key: &[u8], mat: &[&[u8]]) -> Vec<u8> {
let mut dec = Vec::with_capacity(src.len());
let ksize = key.len();
if ksize == 0 {
return dec;
}

let default_mat_vec;
let default_mat_refs;
let actual_mat = if mat.is_empty() {
let alpha_str = std::str::from_utf8(BEAUFORT_ALPHA)
.unwrap_or("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");
default_mat_vec = beaufort_tableau(alpha_str);
default_mat_refs = default_mat_vec.iter().map(|v| v.as_slice()).collect::<Vec<_>>();
default_mat_refs.as_slice()
} else {
mat
};

if actual_mat.is_empty() {
return dec;
}

let rsize = actual_mat[0].len();
let mut j = 0;

for &ch in src {
let mut needed = false;
let mut y = 0;
for current_y in 0..rsize {
if let Some(row) = actual_mat.get(current_y) {
if let Some(&first) = row.get(0) {
if ch == first {
needed = true;
y = current_y;
break;
}
}
}
}

if !needed {
dec.push(ch);
continue;
}

let k = key[j % ksize];
j += 1;

needed = false;
let mut x = 0;
for current_x in 0..rsize {
if let Some(row) = actual_mat.get(y) {
if let Some(&val) = row.get(current_x) {
if k == val {
needed = true;
x = current_x;
break;
}
}
}
}

if !needed {
dec.push(ch);
j -= 1;
continue;
}

if let Some(row) = actual_mat.get(0) {
if let Some(&val) = row.get(x) {
dec.push(val);
} else {
dec.push(ch);
}
} else {
dec.push(ch);
}
}

dec
}
