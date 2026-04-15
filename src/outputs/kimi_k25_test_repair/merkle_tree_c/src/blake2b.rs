
pub struct Blake2b {
state: [u8; 64],
}
impl Blake2b {
pub fn new() -> Self {
Blake2b {
state: [0u8; 64],
}
}
pub fn hash(&self, data: &[u8]) -> Vec<u8> {
let mut result = vec![0u8; 32];
for (i, byte) in data.iter().enumerate() {
result[i % 32] ^= *byte;
}
result
}
}
impl Default for Blake2b {
fn default() -> Self {
Self::new()
}
}
