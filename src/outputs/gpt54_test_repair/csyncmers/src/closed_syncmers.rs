use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct MinimizerResult {
pub minimizer_hash: u128,
pub kmer_position: usize,
pub smer_position: usize,
}

pub fn compute_closed_syncmers(
sequence_input: &str,
len: i32,
k: i32,
s: i32,
results: &mut Vec<MinimizerResult>,
num_results: &mut i32,
) {
*num_results = 0;
results.clear();

if len < 0 || k <= 0 || s <= 0 || k < s {
return;
}

if len < k {
eprintln!("Sequence length is less than K");
return;
}

let len_usize = len as usize;
let k_usize = k as usize;
let s_usize = s as usize;
let bytes = sequence_input.as_bytes();

if bytes.len() < len_usize {
return;
}

if len_usize < s_usize {
return;
}

let num_s_mers = len_usize - s_usize + 1;
let window_size = k_usize - s_usize + 1;

let mut s_mer_hashes = vec![0u128; num_s_mers];
let mask = if 2 * s_usize >= 128 {
u128::MAX
} else {
(1u128 << (2 * s_usize)) - 1
};

for s_mer_pos in 0..num_s_mers {
let canonical_hash = canonical_smer_hash(bytes, s_mer_pos, s_usize, mask);
s_mer_hashes[s_mer_pos] = canonical_hash;
}

let mut deque: VecDeque<usize> = VecDeque::new();

for i in 0..num_s_mers {
while let Some(&last_idx) = deque.back() {
if s_mer_hashes[last_idx] > s_mer_hashes[i] {
deque.pop_back();
} else {
break;
}
}

deque.push_back(i);

while let Some(&front_idx) = deque.front() {
if front_idx + window_size <= i {
deque.pop_front();
} else {
break;
}
}

if i + 1 >= window_size {
let min_pos = match deque.front() {
Some(&idx) => idx,
None => continue,
};

let kmer_pos = i + 1 - window_size;

if min_pos == kmer_pos || min_pos == kmer_pos + k_usize - s_usize {
add_minimizer(results, num_results, s_mer_hashes[min_pos], kmer_pos, min_pos);
}
}
}
}

fn canonical_smer_hash(bytes: &[u8], start: usize, s_usize: usize, mask: u128) -> u128 {
let mut hash_fwd: u128 = 0;
let mut hash_rev: u128 = 0;

for kk in 0..s_usize {
let base = base_to_bits(bytes[start + kk] as char);
hash_fwd = (hash_fwd << 2) | base as u128;
}
hash_fwd &= mask;

for kk in 0..s_usize {
let pos = start + s_usize - 1 - kk;
let base = base_to_bits(bytes[pos] as char);
let comp_base = complement_base(base);
hash_rev = (hash_rev << 2) | comp_base as u128;
}
hash_rev &= mask;

if hash_fwd < hash_rev {
hash_fwd
} else {
hash_rev
}
}

pub fn base_to_bits(base: char) -> u8 {
match base {
'A' | 'a' => 0,
'C' | 'c' => 1,
'G' | 'g' => 2,
'T' | 't' => 3,
_ => 0,
}
}

pub fn complement_base(base: u8) -> u8 {
3 - base
}

pub fn add_minimizer(
results: &mut Vec<MinimizerResult>,
size: &mut i32,
minimizer_hash: u128,
kmer_position: usize,
smer_position: usize,
) {
results.push(MinimizerResult {
minimizer_hash,
kmer_position,
smer_position,
});
*size += 1;
}
