
pub mod clhash {
pub struct ClhashKey {
data: [u64; 4],
}
impl ClhashKey {
pub fn new(data: [u64; 4]) -> Self {
Self { data }
}
}
pub fn get_random_key_for_clhash() -> ClhashKey {
ClhashKey::new([
0x1234_5678_9ABC_DEF0,
0x0FED_CBA9_8765_4321,
0xAABB_CCDD_EEFF_0011,
0x1122_3344_5566_7788,
])
}
pub fn clhash(key: &ClhashKey, data: &[u8]) -> u64 {
let mut hash: u64 = 0;
for (i, &byte) in data.iter().enumerate() {
hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
if i % 8 == 0 {
hash = hash.wrapping_add(key.data[(i / 8) % 4]);
}
}
hash ^ key.data[0]
}
}
