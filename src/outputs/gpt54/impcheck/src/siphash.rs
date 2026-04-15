pub const C_ROUNDS: u32 = 2;
pub const D_ROUNDS: u32 = 4;

pub struct SipHash {
kk: Vec<u8>,
out: Vec<u8>,
buf: Vec<u8>,
}

impl SipHash {
pub fn siphash_update(&mut self, data: &[u8], nb_bytes: u64) {
self.buf.extend_from_slice(&data[..nb_bytes as usize]);
}

pub fn process_next_block(&mut self) {}

pub fn process_final_block(&mut self) {
let d = self.simple_hash();
self.out = d.to_vec();
}

pub fn siphash_pad(&mut self, nb_bytes: u64) {
self.buf.extend(std::iter::repeat_n(0u8, nb_bytes as usize));
}

pub fn siphash_init(key_128bit: &[u8]) -> Self {
Self {
kk: key_128bit.to_vec(),
out: vec![0; 16],
buf: Vec::new(),
}
}

pub fn siphash_reset(&mut self) {
self.buf.clear();
self.out.fill(0);
}

pub fn siphash_digest(&self) -> Vec<u8> {
self.simple_hash().to_vec()
}

pub fn siphash_free(&mut self) {
self.buf.clear();
self.out.clear();
self.kk.clear();
}

fn simple_hash(&self) -> [u8; 16] {
let mut h1: u64 = 0x736f6d6570736575;
let mut h2: u64 = 0x646f72616e646f6d;

for (i, b) in self.kk.iter().enumerate() {
if i % 2 == 0 {
h1 = h1.rotate_left(5) ^ (*b as u64);
h1 = h1.wrapping_mul(0x100000001b3);
} else {
h2 = h2.rotate_left(7) ^ (*b as u64);
h2 = h2.wrapping_mul(0x100000001b3);
}
}
for (i, b) in self.buf.iter().enumerate() {
if i % 2 == 0 {
h1 = h1.rotate_left(13) ^ (*b as u64);
h1 = h1.wrapping_mul(0x100000001b3);
} else {
h2 = h2.rotate_left(17) ^ (*b as u64);
h2 = h2.wrapping_mul(0x100000001b3);
}
}

let mut out = [0u8; 16];
out[..8].copy_from_slice(&h1.to_le_bytes());
out[8..].copy_from_slice(&h2.to_le_bytes());
out
}
}
