pub fn leftpad(str: &str, padding: &str, min_len: usize, dest: &mut [u8]) -> usize {
let str_len = str.len();

// Default padding to " " if empty (equivalent to C: if (!padding || !padding[0]) padding = " ";)
let padding = if padding.is_empty() { " " } else { padding };
let padding_bytes = padding.as_bytes();

// Calculate number of padding characters needed
let npad = if str_len < min_len { min_len - str_len } else { 0 };

// If dest is empty, return total length needed (equivalent to C: if (!dest || !dest_sz) return str_len + npad;)
if dest.is_empty() {
return str_len + npad;
}

let dest_sz = dest.len();
let mut dest_len = 0;
let mut pad_idx = 0;

// Fill with padding characters, cycling through padding string
while dest_len < npad && dest_len < dest_sz - 1 {
dest[dest_len] = padding_bytes[pad_idx];
dest_len += 1;
pad_idx += 1;
if pad_idx >= padding_bytes.len() {
pad_idx = 0;
}
}

// Copy original string
let str_bytes = str.as_bytes();
let mut str_idx = 0;
while str_idx < str_len && dest_len < dest_sz - 1 {
dest[dest_len] = str_bytes[str_idx];
dest_len += 1;
str_idx += 1;
}

// Null terminate
dest[dest_len] = 0;

dest_len
}
