pub fn pad_buffer(input: &[u8], input_len: usize, output: &mut Vec<u8>, output_len: &mut usize) {
let block_size = 16usize;
let effective_len = input_len.min(input.len());
let padded_len = ((effective_len / block_size) + 1) * block_size;
let pad_value = (padded_len - effective_len) as u8;
output.clear();
output.extend_from_slice(&input[..effective_len]);
output.resize(padded_len, pad_value);
if effective_len < padded_len {
for b in &mut output[effective_len..padded_len] {
*b = pad_value;
}
}
*output_len = padded_len;
}
pub fn remove_padding(input: &[u8], input_len: usize) -> usize {
let effective_len = input_len.min(input.len());
if effective_len == 0 {
return 0;
}
let pad_value = input[effective_len - 1] as usize;
if !(1..=16).contains(&pad_value) {
return effective_len;
}
let start = effective_len.saturating_sub(pad_value);
for &b in &input[start..effective_len] {
if b as usize != pad_value {
return effective_len;
}
}
effective_len - pad_value
}
