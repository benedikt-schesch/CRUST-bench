use crate::libutf_string::Utf8String;
use crate::libutf_utf;
fn main() {
let mut string = Utf8String::new();
drop(&mut string);
let mut string = Utf8String::new();
drop(&mut string);
let mut string = Utf8String::new();
drop(&mut string);
let mut string = Utf8String::new();
drop(&mut string);
let utf16_data: &[u16] = &[0x0048, 0x0065, 0x006C, 0x006C, 0x006F]; 
let utf16_bytes: Vec<u8> = utf16_data.iter().flat_map(|&x| x.to_le_bytes()).collect();
let mut result = Vec::new();
let written = libutf_utf::utf16le_convert_to_utf8(&utf16_bytes, &mut result);
assert_eq!(written, 5);
assert_eq!(result, b"Hello");
let string = "Test";
let mut result: Vec<u8> = Vec::new(); 
let written = libutf_utf::utf8_convert_to_utf16le(string, &mut result);
assert_eq!(written, 8); 
let invalid_utf16: &[u16] = &[0xD800]; 
let invalid_bytes: Vec<u8> = invalid_utf16.iter().flat_map(|&x| x.to_le_bytes()).collect();
assert!(!libutf_utf::utf16le_validate(&invalid_bytes));
let valid_utf16: &[u16] = &[0x0041, 0x0042, 0x0043]; 
let valid_bytes: Vec<u8> = valid_utf16.iter().flat_map(|&x| x.to_le_bytes()).collect();
assert!(libutf_utf::utf16le_validate(&valid_bytes));
}
