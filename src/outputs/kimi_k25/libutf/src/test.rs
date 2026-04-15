use crate::libutf_string::Utf8String;
use crate::libutf_utf;

fn main() {
// Line 6: Test Utf8String creation
let mut string = Utf8String::new();
drop(&mut string);

// Line 15: Test Utf8String creation
let mut string = Utf8String::new();
drop(&mut string);

// Line 24: Test Utf8String creation
let mut string = Utf8String::new();
drop(&mut string);

// Line 33: Test Utf8String creation
let mut string = Utf8String::new();
drop(&mut string);

// Line 44: Test utf16le_convert_to_utf8 with proper type conversion
// Convert &[u16] to Vec<u8> to satisfy AsRef<[u8]> trait bound
let utf16_data: &[u16] = &[0x0048, 0x0065, 0x006C, 0x006C, 0x006F]; // "Hello" in UTF-16
let utf16_bytes: Vec<u8> = utf16_data.iter().flat_map(|&x| x.to_le_bytes()).collect();
let mut result = Vec::new();
let written = libutf_utf::utf16le_convert_to_utf8(&utf16_bytes, &mut result);
assert_eq!(written, 5);
assert_eq!(result, b"Hello");

// Line 59: Test utf8_convert_to_utf16le with correct output type (Vec<u8>, not Vec<u16>)
let string = "Test";
let mut result: Vec<u8> = Vec::new(); // Fixed: changed from Vec<u16> to Vec<u8>
let written = libutf_utf::utf8_convert_to_utf16le(string, &mut result);
assert_eq!(written, 8); // 4 chars * 2 bytes each

// Line 110: Test utf16le_validate with invalid input (lone high surrogate)
// Convert &[u16] to Vec<u8> to satisfy AsRef<[u8]> trait bound
let invalid_utf16: &[u16] = &[0xD800]; // Invalid: lone high surrogate
let invalid_bytes: Vec<u8> = invalid_utf16.iter().flat_map(|&x| x.to_le_bytes()).collect();
assert!(!libutf_utf::utf16le_validate(&invalid_bytes));

// Line 144: Test utf16le_validate with valid input
// Convert &[u16] to Vec<u8> to satisfy AsRef<[u8]> trait bound
let valid_utf16: &[u16] = &[0x0041, 0x0042, 0x0043]; // "ABC" in UTF-16
let valid_bytes: Vec<u8> = valid_utf16.iter().flat_map(|&x| x.to_le_bytes()).collect();
assert!(libutf_utf::utf16le_validate(&valid_bytes));
}
