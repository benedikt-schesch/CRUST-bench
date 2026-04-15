// Import necessary modules
use crate::closed_syncmers::base_to_bits;
use crate::closed_syncmers::complement_base;
use crate::closed_syncmers::MinimizerResult;

// Function Declarations
pub fn compute_closed_syncmers_naive(
sequence: &str,
seq_len: usize,
k: i32,
s: i32,
results: &mut Vec<MinimizerResult>,
num_results: &mut i32,
) {
*num_results = 0;
results.clear();

if k <= 0 || s <= 0 || k < s || seq_len < k as usize {
return;
}

let k_usize = k as usize;
let s_usize = s as usize;
let bytes = sequence.as_bytes();

if bytes.len() < seq_len {
return;
}

let mask = if 2 * s_usize >= 128 {
u128::MAX
} else {
(1u128 << (2 * s_usize)) - 1
};

for i in 0..=seq_len - k_usize {
let mut min_hash = u128::MAX;
let mut min_pos_in_kmer: usize = 0;

for j in 0..=k_usize - s_usize {
let s_mer_pos = i + j;
let mut hash_fwd: u128 = 0;
let mut hash_rev: u128 = 0;

for kk in 0..s_usize {
let base = base_to_bits(bytes[s_mer_pos + kk] as char);
hash_fwd = (hash_fwd << 2) | base as u128;
}
hash_fwd &= mask;

for kk in 0..s_usize {
let pos = s_mer_pos + s_usize - 1 - kk;
let base = base_to_bits(bytes[pos] as char);
let comp_base = complement_base(base);
hash_rev = (hash_rev << 2) | comp_base as u128;
}
hash_rev &= mask;

let canonical_hash = if hash_fwd < hash_rev { hash_fwd } else { hash_rev };
if canonical_hash < min_hash {
min_hash = canonical_hash;
min_pos_in_kmer = j;
}
}

if min_pos_in_kmer == 0 || min_pos_in_kmer == k_usize - s_usize {
results.push(MinimizerResult {
kmer_position: i,
smer_position: i + min_pos_in_kmer,
minimizer_hash: min_hash,
});
*num_results += 1;
}
}
}
