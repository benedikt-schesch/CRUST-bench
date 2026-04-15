
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};
pub const RANDOM_SIZE: usize = 16;
pub fn generate_random_data(buffer: &mut [u8]) {
let charset = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
let nanos = SystemTime::now()
.duration_since(UNIX_EPOCH)
.map(|d| d.as_nanos())
.unwrap_or(0);
let mut state = nanos as u64 ^ 0x9E37_79B9_7F4A_7C15;
for byte in buffer.iter_mut() {
state ^= state << 7;
state ^= state >> 9;
state = state
.wrapping_mul(6364136223846793005)
.wrapping_add(1);
*byte = charset[(state as usize) % charset.len()];
}
}
pub fn generate_hash(data: &[u8], hash: &mut [u8]) {
let mut out = [0u8; 32];
for i in 0..4 {
let mut hasher = DefaultHasher::new();
i.hash(&mut hasher);
data.hash(&mut hasher);
let part = hasher.finish().to_le_bytes();
let start = i * 8;
let end = start + 8;
out[start..end].copy_from_slice(&part);
}
let len = hash.len().min(out.len());
hash[..len].copy_from_slice(&out[..len]);
}
