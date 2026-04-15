
#[derive(Debug, Clone)]
pub struct Utf8Validity {
pub valid: bool,
pub valid_upto: usize,
}
#[derive(Debug, Clone)]
pub struct OwnedUtf8String {
pub str: String,
pub byte_len: usize,
}
#[derive(Debug, Clone)]
pub struct Utf8Char {
pub str: String,
pub byte_len: u8,
}
#[derive(Debug, Clone)]
pub struct Utf8CharValidity {
pub valid: bool,
pub next_offset: usize,
}
#[derive(Debug, Clone)]
pub struct Utf8String {
pub str: String,
pub byte_len: usize,
}
#[derive(Debug, Clone)]
pub struct Utf8CharIter {
pub str: String,
}
pub fn validate_utf8_char(bytes: &[u8], offset: usize) -> Utf8CharValidity {
if offset >= bytes.len() {
return Utf8CharValidity {
valid: false,
next_offset: offset,
};
}
let b0 = bytes[offset];
if (b0 & 0b10000000) == 0b00000000 {
return Utf8CharValidity {
valid: true,
next_offset: offset + 1,
};
}
if offset + 1 < bytes.len() {
let b1 = bytes[offset + 1];
if (b0 & 0b11100000) == 0b11000000 && (b1 & 0b11000000) == 0b10000000 {
if (b0 & 0b00011111) < 0b00000010 {
return Utf8CharValidity {
valid: false,
next_offset: offset,
};
}
return Utf8CharValidity {
valid: true,
next_offset: offset + 2,
};
}
}
if offset + 2 < bytes.len() {
let b1 = bytes[offset + 1];
let b2 = bytes[offset + 2];
if (b0 & 0b11110000) == 0b11100000
&& (b1 & 0b11000000) == 0b10000000
&& (b2 & 0b11000000) == 0b10000000
{
if (b0 & 0b00001111) == 0b00000000 && (b1 & 0b00111111) < 0b00100000 {
return Utf8CharValidity {
valid: false,
next_offset: offset,
};
}
if b0 == 0b11101101 && b1 >= 0b10100000 && b1 <= 0b10111111 {
return Utf8CharValidity {
valid: false,
next_offset: offset,
};
}
return Utf8CharValidity {
valid: true,
next_offset: offset + 3,
};
}
}
if offset + 3 < bytes.len() {
let b1 = bytes[offset + 1];
let b2 = bytes[offset + 2];
let b3 = bytes[offset + 3];
if (b0 & 0b11111000) == 0b11110000
&& (b1 & 0b11000000) == 0b10000000
&& (b2 & 0b11000000) == 0b10000000
&& (b3 & 0b11000000) == 0b10000000
{
if (b0 & 0b00000111) == 0b00000000 && (b1 & 0b00111111) < 0b00010000 {
return Utf8CharValidity {
valid: false,
next_offset: offset,
};
}
return Utf8CharValidity {
valid: true,
next_offset: offset + 4,
};
}
}
Utf8CharValidity {
valid: false,
next_offset: offset,
}
}
pub fn validate_utf8(bytes: &[u8]) -> Utf8Validity {
let mut offset = 0;
while offset < bytes.len() {
let char_validity = validate_utf8_char(bytes, offset);
if char_validity.valid {
offset = char_validity.next_offset;
} else {
return Utf8Validity {
valid: false,
valid_upto: offset,
};
}
}
Utf8Validity {
valid: true,
valid_upto: offset,
}
}
pub fn make_utf8_string(bytes: &[u8]) -> Utf8String {
let validity = validate_utf8(bytes);
if validity.valid {
let s = String::from_utf8(bytes.to_vec()).unwrap();
Utf8String {
str: s,
byte_len: validity.valid_upto,
}
} else {
Utf8String {
str: String::new(),
byte_len: 0,
}
}
}
pub fn make_utf8_string_lossy(bytes: &[u8]) -> OwnedUtf8String {
let mut buffer: Vec<u8> = Vec::new();
let mut offset = 0;
while offset < bytes.len() {
let char_validity = validate_utf8_char(bytes, offset);
if char_validity.valid {
let char_len = char_validity.next_offset - offset;
buffer.extend_from_slice(&bytes[offset..offset + char_len]);
offset = char_validity.next_offset;
} else {
buffer.push(0xEF);
buffer.push(0xBF);
buffer.push(0xBD);
offset += 1;
}
}
let byte_len = buffer.len();
OwnedUtf8String {
str: String::from_utf8(buffer).unwrap(),
byte_len,
}
}
pub fn as_utf8_string(owned_str: &OwnedUtf8String) -> Utf8String {
Utf8String {
str: owned_str.str.clone(),
byte_len: owned_str.byte_len,
}
}
pub fn free_owned_utf8_string(owned_str: &mut OwnedUtf8String) {
owned_str.str.clear();
owned_str.byte_len = 0;
}
pub fn make_utf8_char_iter(ustr: Utf8String) -> Utf8CharIter {
Utf8CharIter { str: ustr.str }
}
pub fn is_utf8_char_boundary(bytes: &[u8]) -> bool {
if bytes.is_empty() {
return true;
}
let b = bytes[0];
(b <= 0b01111111) || (b >= 0b11000000)
}
pub fn slice_utf8_string(ustr: Utf8String, byte_index: usize, byte_len: usize) -> Utf8String {
let total_len = ustr.byte_len;
let mut start = byte_index;
if start > total_len {
start = total_len;
}
let mut end = start + byte_len;
if end > total_len {
end = total_len;
}
let bytes = ustr.str.as_bytes();
let start_ok = if start == bytes.len() {
true
} else {
is_utf8_char_boundary(&bytes[start..])
};
let end_ok = if end == bytes.len() {
true
} else {
is_utf8_char_boundary(&bytes[end..])
};
if start_ok && end_ok {
let sliced = &ustr.str[start..end];
Utf8String {
str: sliced.to_string(),
byte_len: end - start,
}
} else {
Utf8String {
str: String::new(),
byte_len: 0,
}
}
}
pub fn next_utf8_char(iter: &mut Utf8CharIter) -> Utf8Char {
if iter.str.is_empty() {
return Utf8Char {
str: String::new(),
byte_len: 0,
};
}
let bytes = iter.str.as_bytes();
let mut end = 1;
while end < bytes.len() && !is_utf8_char_boundary(&bytes[end..]) {
end += 1;
}
let char_bytes = &bytes[0..end];
let char_str = String::from_utf8_lossy(char_bytes).to_string();
let byte_len = char_bytes.len() as u8;
iter.str = iter.str[end..].to_string();
Utf8Char {
str: char_str,
byte_len,
}
}
pub fn nth_utf8_char(ustr: Utf8String, char_index: usize) -> Utf8Char {
let mut iter = make_utf8_char_iter(ustr);
let mut remaining = char_index;
loop {
let ch = next_utf8_char(&mut iter);
if ch.byte_len == 0 {
return Utf8Char {
str: String::new(),
byte_len: 0,
};
}
if remaining == 0 {
return ch;
}
remaining -= 1;
}
}
pub fn utf8_char_count(ustr: Utf8String) -> usize {
let mut iter = make_utf8_char_iter(ustr);
let mut count = 0;
loop {
let ch = next_utf8_char(&mut iter);
if ch.byte_len == 0 {
break;
}
count += 1;
}
count
}
pub fn unicode_code_point(uchar: Utf8Char) -> u32 {
let bytes = uchar.str.as_bytes();
match uchar.byte_len {
1 => (bytes[0] & 0b01111111) as u32,
2 => {
((bytes[0] & 0b00011111) as u32) << 6 | ((bytes[1] & 0b00111111) as u32)
}
3 => {
((bytes[0] & 0b00001111) as u32) << 12
| ((bytes[1] & 0b00111111) as u32) << 6
| ((bytes[2] & 0b00111111) as u32)
}
4 => {
((bytes[0] & 0b00000111) as u32) << 18
| ((bytes[1] & 0b00111111) as u32) << 12
| ((bytes[2] & 0b00111111) as u32) << 6
| ((bytes[3] & 0b00111111) as u32)
}
_ => 0,
}
}
