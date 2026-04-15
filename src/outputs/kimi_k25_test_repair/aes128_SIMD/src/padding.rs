
pub fn pad_buffer(input: &[u8], _input_len: usize, output: &mut Vec<u8>, output_len: &mut usize) {
const BLOCK_SIZE: usize = 16;
output.clear();
output.extend_from_slice(&input[.._input_len.min(input.len())]);
let padding_len = BLOCK_SIZE - (output.len() % BLOCK_SIZE);
let padding_byte = padding_len as u8;
output.resize(output.len() + padding_len, padding_byte);
*output_len = output.len();
}
pub fn remove_padding(buffer: &[u8], length: usize) -> usize {
if length == 0 {
return 0;
}
if let Some(&last) = buffer.get(length - 1) {
let padding_len = last as usize;
if padding_len > 0 && padding_len <= length && padding_len <= 16 {
let is_valid = buffer[length - padding_len..length]
.iter()
.all(|&b| b == last);
if is_valid {
return length - padding_len;
}
}
}
length
}
