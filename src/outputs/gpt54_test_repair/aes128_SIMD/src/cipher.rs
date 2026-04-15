use crate::aes::{NB, NR};
use crate::aes::inv_shift;
use crate::aes::shift;
use crate::cipher_utils::inv_sub;
use crate::cipher_utils::sub;
use crate::keys::add;
use crate::matrix::columns;
use crate::matrix::inv_columns;

fn load_round_key(w: &[u8; 4 * NB * (NR + 1)], round: usize) -> [[u8; NB]; 4] {
let mut round_key = [[0u8; NB]; 4];
let base = round * 4 * NB;

for c in 0..NB {
for r in 0..4 {
round_key[r][c] = w[base + 4 * c + r];
}
}

round_key
}

pub fn cipher(in_data: &[u8; 4 * NB], out: &mut [u8; 4 * NB], w: &[u8; 4 * NB * (NR + 1)]) {
let mut state = [[0u8; NB]; 4];

for i in 0..(4 * NB) {
state[i % 4][i / 4] = in_data[i];
}

let round_key = load_round_key(w, 0);
add(&mut state, &round_key);

for round in 1..NR {
sub(&mut state);
shift(&mut state);
columns(&mut state);

let round_key = load_round_key(w, round);
add(&mut state, &round_key);
}

sub(&mut state);
shift(&mut state);

let round_key = load_round_key(w, NR);
add(&mut state, &round_key);

for i in 0..(4 * NB) {
out[i] = state[i % 4][i / 4];
}
}

pub fn inv_cipher(in_data: &[u8; 4 * NB], out: &mut [u8; 4 * NB], w: &[u8; 4 * NB * (NR + 1)]) {
let mut state = [[0u8; NB]; 4];

for i in 0..(4 * NB) {
state[i % 4][i / 4] = in_data[i];
}

let round_key = load_round_key(w, NR);
add(&mut state, &round_key);

for round in (1..NR).rev() {
inv_shift(&mut state);
inv_sub(&mut state);

let round_key = load_round_key(w, round);
add(&mut state, &round_key);

inv_columns(&mut state);
}

inv_shift(&mut state);
inv_sub(&mut state);

let round_key = load_round_key(w, 0);
add(&mut state, &round_key);

for i in 0..(4 * NB) {
out[i] = state[i % 4][i / 4];
}
}
