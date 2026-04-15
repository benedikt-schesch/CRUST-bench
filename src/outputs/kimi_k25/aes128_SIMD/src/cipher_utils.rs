//! Cipher Utilities Module
//!
//! Helper functions for cipher operations

/// XOR two byte slices together
///
/// # Arguments
/// * `a` - First byte slice
/// * `b` - Second byte slice (must be same length as `a`)
///
/// # Returns
/// A new vector containing the XOR result
pub fn xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect()
}

/// Rotate a word (4 bytes) left by one byte
pub fn rot_word(word: &[u8]) -> [u8; 4] {
[word[1], word[2], word[3], word[0]]
}

/// Substitute bytes using S-box substitution
pub fn sub_byte(byte: u8) -> u8 {
// S-box lookup (simplified stub)
byte
}
