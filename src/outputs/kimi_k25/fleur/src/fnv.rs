pub const FNV_PRIME: u64 = 1099511628211;
pub const FNV_OFFSET_BASIS: u64 = 14695981039346656037;

pub fn fnv1(buf: &[u8]) -> u64 {
let mut h = FNV_OFFSET_BASIS;
for &byte in buf {
h = h.wrapping_mul(FNV_PRIME);
h ^= byte as u64;
}
h
}

pub fn getDigest(h: u64) -> String {
let bytes = h.to_be_bytes();
format!("{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
bytes[0], bytes[1], bytes[2], bytes[3],
bytes[4], bytes[5], bytes[6], bytes[7])
}
