
pub fn cipher(input: &[u8], output: &mut [u8], key: &[u8]) {
let len = input.len().min(output.len());
output[..len].copy_from_slice(&input[..len]);
}
pub fn inv_cipher(input: &[u8], output: &mut [u8], key: &[u8]) {
let len = input.len().min(output.len());
output[..len].copy_from_slice(&input[..len]);
}
