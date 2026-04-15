// Generated Rust Code
pub const BEAUFORT_ALPHA: &[u8] =
b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
pub const BEAUFORT_VERSION: &str = "1";

pub fn beaufort_encrypt(src: &[u8], key: &[u8], mat: &[&[u8]]) -> Vec<u8> {
if mat.is_empty() || key.is_empty() || mat[0].is_empty() {
return src.to_vec();
}

let rsize = mat[0].len();
let ksize = key.len();
let mut enc = Vec::with_capacity(src.len());
let mut j = 0usize;

for &ch in src {
let mut needed = false;
let mut x = 0usize;
let y = 0usize;

while x < rsize {
if x < mat[y].len() && ch == mat[y][x] {
needed = true;
break;
}
x += 1;
}

if !needed {
enc.push(ch);
continue;
}

let k = key[j % ksize];
j += 1;

let mut found = false;
let mut y2 = 0usize;
while y2 < rsize {
if y2 < mat.len() && x < mat[y2].len() && k == mat[y2][x] {
found = true;
break;
}
y2 += 1;
}

if !found {
enc.push(ch);
j -= 1;
continue;
}

enc.push(mat[y2][0]);
}

enc
}
