use std::cmp;
pub fn hecc_encode(input: &[u8]) -> Vec<u8> {
let mut output = Vec::new();
let mut chunks = input.chunks_exact(15);
for chunk in &mut chunks {
let mut arr = [0u8; 15];
arr.copy_from_slice(chunk);
hecc_encode_impl(&arr, &mut output);
}
let remainder = chunks.remainder();
if !remainder.is_empty() {
let r = remainder.len();
let mut arr = [0u8; 15];
arr[..r].copy_from_slice(remainder);
let start_len = output.len();
hecc_encode_impl(&arr, &mut output);
output.drain(start_len + r..start_len + 15);
}
output
}
fn hecc_encode_impl(input: &[u8; 15], output: &mut Vec<u8>) {
let mut p1: u8 = 0;
let mut p2: u8 = 0;
let mut power2: usize = 4;
output.extend_from_slice(input);
let mut j = 0;
for i in 3.. {
if power2 == i {
power2 *= 2;
continue;
}
if j >= 120 {
break;
}
let byte_idx = j / 8;
let bit_idx = j % 8;
let bit = (input[byte_idx] >> bit_idx) & 1;
if bit_idx == 0 {
p2 ^= input[byte_idx].count_ones() as u8 & 1;
}
if bit == 1 {
p1 ^= i as u8;
}
j += 1;
}
p2 ^= p1.count_ones() as u8 & 1;
let check_byte = p1 | (p2 << 7);
output.push(check_byte);
}
pub fn hecc_decode(input: &[u8]) -> Option<Vec<u8>> {
if input.len() % 16 == 1 {
return None;
}
let mut output = Vec::new();
let mut chunks = input.chunks_exact(16);
for chunk in &mut chunks {
let mut arr = [0u8; 16];
arr.copy_from_slice(chunk);
if !hecc_decode_impl(&arr, &mut output) {
return None;
}
}
let remainder = chunks.remainder();
if !remainder.is_empty() {
let r = remainder.len();
let mut arr = [0u8; 16];
arr[..r-1].copy_from_slice(&remainder[..r-1]);
arr[15] = remainder[r-1];
let mut temp_output = Vec::new();
if !hecc_decode_impl(&arr, &mut temp_output) {
return None;
}
output.extend_from_slice(&temp_output[..r-1]);
}
Some(output)
}
fn hecc_decode_impl(input: &[u8; 16], output: &mut Vec<u8>) -> bool {
let check = input[15];
let data_bytes = &input[0..15];
let mut p1: u8 = 0;
let mut p2: u8 = 0;
let mut power2: usize = 4;
let mut j = 0;
for i in 3.. {
if power2 == i {
power2 *= 2;
continue;
}
if j >= 120 {
break;
}
let byte_idx = j / 8;
let bit_idx = j % 8;
let bit = (data_bytes[byte_idx] >> bit_idx) & 1;
if bit_idx == 0 {
p2 ^= data_bytes[byte_idx].count_ones() as u8 & 1;
}
if bit == 1 {
p1 ^= i as u8;
}
j += 1;
}
p1 ^= check & 0x7f;
if p1 != 0 {
p2 ^= check.count_ones() as u8 & 1;
if p2 == 0 {
return false;
}
if p1 & (p1 - 1) != 0 { 
let log_val = log2uint8(p1);
let data_idx = (p1 - log_val - 2) as usize;
let byte_idx = data_idx / 8;
let bit_idx = data_idx % 8;
let mut corrected = [0u8; 15];
corrected.copy_from_slice(data_bytes);
corrected[byte_idx] ^= 1 << bit_idx;
output.extend_from_slice(&corrected);
return true;
}
}
output.extend_from_slice(data_bytes);
true
}
pub fn log2uint8(x: u8) -> u8 {
7 - x.leading_zeros() as u8
}
