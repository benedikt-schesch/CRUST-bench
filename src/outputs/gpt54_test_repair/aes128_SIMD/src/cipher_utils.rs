use crate::aes::NB;
use crate::aes::RSBOX;
use crate::aes::SBOX;
pub fn g_mult(first: u8, second: u8) -> u8 {
let mut first_mut = first;
let mut second_mut = second;
let mut p = 0u8;
for _ in 0..8 {
if (second_mut & 1) != 0 {
p ^= first_mut;
}
let hi_bit_set = first_mut & 0x80;
first_mut <<= 1;
if hi_bit_set != 0 {
first_mut ^= 0x1b;
}
second_mut >>= 1;
}
p
}
pub fn sub(state: &mut [[u8; NB]; 4]) {
for row in state.iter_mut().take(4) {
for cell in row.iter_mut().take(NB) {
*cell = SBOX[*cell as usize];
}
}
}
pub fn inv_sub(state: &mut [[u8; NB]; 4]) {
for row in state.iter_mut().take(4) {
for cell in row.iter_mut().take(NB) {
*cell = RSBOX[*cell as usize];
}
}
}
