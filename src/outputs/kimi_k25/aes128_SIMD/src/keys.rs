//! AES Key Expansion Module

/// Expand the cipher key into round keys for AES-128
///
/// # Arguments
/// * `key` - The original cipher key (16 bytes for AES-128)
/// * `output` - The output buffer to write expanded key schedule (176 bytes for AES-128)
pub fn expansion(key: &[u8], output: &mut [u8]) {
// AES-128 key expansion produces 11 round keys (44 words / 176 bytes)
// This is a stub implementation that copies the key and fills the rest with zeros
// Real implementation would perform the AES key schedule algorithm
let key_len = key.len();
let out_len = output.len();

// Copy key to beginning of output
let copy_len = key_len.min(out_len);
output[..copy_len].copy_from_slice(&key[..copy_len]);

// Fill remainder with zeros
for i in copy_len..out_len {
output[i] = 0;
}
}
