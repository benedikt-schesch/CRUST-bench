
pub fn xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect()
}
pub fn rot_word(word: &[u8]) -> [u8; 4] {
[word[1], word[2], word[3], word[0]]
}
pub fn sub_byte(byte: u8) -> u8 {
byte
}
