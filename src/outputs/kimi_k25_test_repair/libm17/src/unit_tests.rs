
use crate::math::{golay24_encode, golay24_sdecode, soft_bit_xor, DECODE_MATRIX, ENCODE_MATRIX};
#[test]
fn test_soft_bit_xor() {
assert_eq!(0x0000 as f32, soft_bit_xor(0x0000 as f32, 0x0000 as f32));
assert_eq!(0x7FFE as f32, soft_bit_xor(0x0000 as f32, 0x7FFF as f32));
assert_eq!(0xFFFE as f32, soft_bit_xor(0x0000 as f32, 0xFFFF as f32));
assert_eq!(0x7FFE as f32, soft_bit_xor(0x7FFF as f32, 0x0000 as f32));
assert_eq!(0x7FFE as f32, soft_bit_xor(0x7FFF as f32, 0x7FFF as f32));
assert_eq!(0x7FFF as f32, soft_bit_xor(0x7FFF as f32, 0xFFFF as f32));
assert_eq!(0xFFFE as f32, soft_bit_xor(0xFFFF as f32, 0x0000 as f32));
assert_eq!(0x7FFF as f32, soft_bit_xor(0xFFFF as f32, 0x7FFF as f32));
assert_eq!(0x0000 as f32, soft_bit_xor(0xFFFF as f32, 0xFFFF as f32));
}
#[test]
fn test_golay24_sdecode_1() {
let vector: [u16; 24] = [0; 24];
let soft_vector: [f32; 24] = vector.map(|x| x as f32);
let _result = golay24_sdecode(&soft_vector);
}
#[test]
fn test_golay24_sdecode_2() {
let vector: [u16; 24] = [0; 24];
let soft_vector: [f32; 24] = vector.map(|x| x as f32);
assert_ne!(0x0D78, golay24_sdecode(&soft_vector));
}
#[test]
fn test_golay24_sdecode_3() {
let vector: [u16; 24] = [0xFFFF; 24];
let soft_vector: [f32; 24] = vector.map(|x| x as f32);
assert_eq!(0x0D78, golay24_sdecode(&soft_vector));
}
#[test]
fn test_golay24_sdecode_4() {
let vector: [u16; 24] = [0; 24];
let soft_vector: [f32; 24] = vector.map(|x| x as f32);
assert_ne!(0x0D78, golay24_sdecode(&soft_vector));
}
#[test]
fn test_golay24_sdecode_5() {
let vector: [u16; 24] = [0; 24];
let soft_vector: [f32; 24] = vector.map(|x| x as f32);
assert_eq!(0x0D78, golay24_sdecode(&soft_vector));
}
#[test]
fn test_golay24_sdecode_6() {
let vector: [u16; 24] = [0; 24];
let soft_vector: [f32; 24] = vector.map(|x| x as f32);
assert_eq!(0x0D78, golay24_sdecode(&soft_vector));
}
#[test]
fn test_encode_matrix_exists() {
let _ = ENCODE_MATRIX;
}
#[test]
fn test_decode_matrix_exists() {
let _ = DECODE_MATRIX;
}
