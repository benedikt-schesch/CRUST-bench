use crate::aes::{NB, NR, NK};
use crate::aes::RCON;
use crate::aes::SBOX;

pub fn add(state: &mut [[u8; NB]; 4], round_key: &[[u8; NB]; 4]) {
for i in 0..4 {
for j in 0..NB {
state[i][j] ^= round_key[i][j];
}
}
}

pub fn expansion(key: &[u8; 4 * NK], w: &mut [u8; 4 * NB * (NR + 1)]) {
for b in w.iter_mut() {
*b = 0;
}

w[..(4 * NK)].copy_from_slice(&key[..(4 * NK)]);

let total_words = NB * (NR + 1);
let mut i = NK;

while i < total_words {
let mut temp = [0u8; 4];
for j in 0..4 {
temp[j] = w[4 * (i - 1) + j];
}

if i % NK == 0 {
temp.rotate_left(1);
for j in 0..4 {
temp[j] = SBOX[temp[j] as usize];
}
temp[0] ^= RCON[i / NK];
} else if NK > 6 && (i % NK == 4) {
for j in 0..4 {
temp[j] = SBOX[temp[j] as usize];
}
}

for j in 0..4 {
w[4 * i + j] = w[4 * (i - NK) + j] ^ temp[j];
}

i += 1;
}
}
