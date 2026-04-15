
pub type Hash = u32;
pub type Key = &'static str;
pub const EMPTY_KEY: Option<Key> = None;
pub type Data = &'static str;
pub const EMPTY_DATA: Option<Data> = None;
pub const POWER: Hash = 131;
pub const ALTERNATIVE_POWER: Hash = 171;
pub const REHASHER: Hash = 718841;
pub fn hash_by_power(key: &str, power: Hash) -> Hash {
let mut res: Hash = 0;
for b in key.bytes() {
res = res.wrapping_mul(power).wrapping_add(b as Hash);
}
res
}
pub fn hash(key: &str) -> Hash {
hash_by_power(key, POWER)
}
pub fn alternative_hash(key: &str) -> Hash {
hash_by_power(key, ALTERNATIVE_POWER)
}
pub fn compare_keys(k1: &str, k2: &str) -> std::cmp::Ordering {
k1.cmp(k2)
}
pub fn rehash(h: Hash) -> Hash {
h.wrapping_add(REHASHER)
}
