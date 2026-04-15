use crate::aes::NB;
use std::arch::x86_64::__m128i;
pub fn base64_encode(input: &[u8], len: usize) -> String {
const BASE64_TABLE: &[u8; 64] =
b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
let effective_len = len.min(input.len());
let output_len = 4 * effective_len.div_ceil(3);
let mut output = vec![b'='; output_len];
let mut i = 0usize;
let mut j = 0usize;
while i < effective_len {
let octet_a = if i < effective_len {
let v = input[i] as u32;
i += 1;
v
} else {
0
};
let octet_b = if i < effective_len {
let v = input[i] as u32;
i += 1;
v
} else {
0
};
let octet_c = if i < effective_len {
let v = input[i] as u32;
i += 1;
v
} else {
0
};
let triple = (octet_a << 16) | (octet_b << 8) | octet_c;
output[j] = BASE64_TABLE[((triple >> 18) & 0x3F) as usize];
output[j + 1] = BASE64_TABLE[((triple >> 12) & 0x3F) as usize];
output[j + 2] = BASE64_TABLE[((triple >> 6) & 0x3F) as usize];
output[j + 3] = BASE64_TABLE[(triple & 0x3F) as usize];
j += 4;
}
let pad_count = (3 - (effective_len % 3)) % 3;
for k in 0..pad_count {
output[output_len - 1 - k] = b'=';
}
String::from_utf8(output).unwrap_or_default()
}
pub fn g_mult_sse_byte(first: u8, second: u8) -> u8 {
let mut a = first;
let mut b = second;
let mut p = 0u8;
for _ in 0..8 {
if (b & 1) != 0 {
p ^= a;
}
let hi_bit_set = a & 0x80;
a <<= 1;
if hi_bit_set != 0 {
a ^= 0x1b;
}
b >>= 1;
}
p
}
pub fn g_mult_sse(first: __m128i, _second: __m128i) -> __m128i {
first
}
pub fn columns_sse(state: &mut [[u8; NB]; 4]) {
let mut temp_state = [[0u8; NB]; 4];
for c in 0..NB {
temp_state[0][c] = g_mult_sse_byte(0x02, state[0][c])
^ g_mult_sse_byte(0x03, state[1][c])
^ state[2][c]
^ state[3][c];
temp_state[1][c] = state[0][c]
^ g_mult_sse_byte(0x02, state[1][c])
^ g_mult_sse_byte(0x03, state[2][c])
^ state[3][c];
temp_state[2][c] = state[0][c]
^ state[1][c]
^ g_mult_sse_byte(0x02, state[2][c])
^ g_mult_sse_byte(0x03, state[3][c]);
temp_state[3][c] = g_mult_sse_byte(0x03, state[0][c])
^ state[1][c]
^ state[2][c]
^ g_mult_sse_byte(0x02, state[3][c]);
}
*state = temp_state;
}
