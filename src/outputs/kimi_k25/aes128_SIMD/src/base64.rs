//! Base64 Encoding Module

/// Encode a byte slice to a base64 string
///
/// # Arguments
/// * `input` - The bytes to encode
/// * `len` - The length of input to encode
///
/// # Returns
/// A base64-encoded string
pub fn base64_encode(input: &[u8], len: usize) -> String {
const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
let mut result = String::new();

let input = &input[..len.min(input.len())];

for chunk in input.chunks(3) {
let buf = match chunk.len() {
1 => [chunk[0], 0, 0],
2 => [chunk[0], chunk[1], 0],
3 => [chunk[0], chunk[1], chunk[2]],
_ => unreachable!(),
};

let b = ((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32);

result.push(ALPHABET[((b >> 18) & 0x3F) as usize] as char);
result.push(ALPHABET[((b >> 12) & 0x3F) as usize] as char);

if chunk.len() > 1 {
result.push(ALPHABET[((b >> 6) & 0x3F) as usize] as char);
} else {
result.push('=');
}

if chunk.len() > 2 {
result.push(ALPHABET[(b & 0x3F) as usize] as char);
} else {
result.push('=');
}
}

result
}
