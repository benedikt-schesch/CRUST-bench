
pub fn golay24_encode(data: u16) -> u32 {
let parity = calculate_parity(data);
((data as u32) << 12) | parity
}
pub fn golay24_sdecode(soft_bits: &[f32]) -> u16 {
if soft_bits.len() < 24 {
return 0;
}
let mut codeword: u32 = 0;
for (i, &bit) in soft_bits.iter().take(24).enumerate() {
if bit > 0.0 {
codeword |= 1 << (23 - i);
}
}
(codeword >> 12) as u16
}
pub fn soft_bit_xor(a: f32, b: f32) -> f32 {
let sign = if (a > 0.0) == (b > 0.0) { 1.0 } else { -1.0 };
sign * a.abs().min(b.abs())
}
pub const DECODE_TABLE: [u16; 4096] = [0; 4096];
pub const ENCODE_MATRIX: [u32; 12] = [0; 12];
pub const DECODE_MATRIX: [u32; 12] = [0; 12];
fn calculate_parity(data: u16) -> u32 {
let mut parity: u32 = 0;
let mut temp = data;
for i in 0..12 {
if (temp & 1) != 0 {
parity ^= 0x800 >> i;
}
temp >>= 1;
}
parity & 0xFFF
}
