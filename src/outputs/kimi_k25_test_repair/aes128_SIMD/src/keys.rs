
pub fn expansion(key: &[u8], output: &mut [u8]) {
let key_len = key.len();
let out_len = output.len();
let copy_len = key_len.min(out_len);
output[..copy_len].copy_from_slice(&key[..copy_len]);
for i in copy_len..out_len {
output[i] = 0;
}
}
