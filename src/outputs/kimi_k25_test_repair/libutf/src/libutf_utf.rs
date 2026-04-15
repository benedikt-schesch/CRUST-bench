
pub fn validate_utf8(input: &[u8]) -> bool {
utf8_validate(input)
}
pub fn utf8_validate<T: AsRef<[u8]>>(input: T) -> bool {
std::str::from_utf8(input.as_ref()).is_ok()
}
pub fn utf16le_validate<T: AsRef<[u8]>>(input: T) -> bool {
let input = input.as_ref();
if input.len() % 2 != 0 {
return false;
}
let mut i = 0;
while i < input.len() {
let code_unit = u16::from_le_bytes([input[i], input[i + 1]]);
if (0xD800..=0xDBFF).contains(&code_unit) {
if i + 3 >= input.len() {
return false;
}
let next_unit = u16::from_le_bytes([input[i + 2], input[i + 3]]);
if !(0xDC00..=0xDFFF).contains(&next_unit) {
return false;
}
i += 4;
} else if (0xDC00..=0xDFFF).contains(&code_unit) {
return false;
} else {
i += 2;
}
}
true
}
pub fn utf16le_convert_to_utf8<T: AsRef<[u8]>>(input: T, output: &mut Vec<u8>) -> usize {
let input = input.as_ref();
if input.len() % 2 != 0 {
return 0;
}
let mut bytes_written = 0;
let mut i = 0;
while i < input.len() {
let code_unit = u16::from_le_bytes([input[i], input[i + 1]]);
let c = if (0xD800..=0xDBFF).contains(&code_unit) {
if i + 3 >= input.len() {
return bytes_written;
}
let next_unit = u16::from_le_bytes([input[i + 2], input[i + 3]]);
if !(0xDC00..=0xDFFF).contains(&next_unit) {
return bytes_written;
}
i += 4;
let c = ((code_unit as u32 - 0xD800) << 10) + (next_unit as u32 - 0xDC00) + 0x10000;
char::from_u32(c)
} else if (0xDC00..=0xDFFF).contains(&code_unit) {
return bytes_written;
} else {
i += 2;
char::from_u32(code_unit as u32)
};
if let Some(c) = c {
let mut buf = [0; 4];
let s = c.encode_utf8(&mut buf);
output.extend_from_slice(s.as_bytes());
bytes_written += s.len();
} else {
return bytes_written;
}
}
bytes_written
}
pub fn utf8_convert_to_utf16le<T: AsRef<[u8]>>(input: T, output: &mut Vec<u8>) -> usize {
let input = input.as_ref();
let s = match std::str::from_utf8(input) {
Ok(s) => s,
Err(_) => return 0,
};
let mut bytes_written = 0;
for c in s.chars() {
let mut buf = [0; 2];
let encoded = c.encode_utf16(&mut buf);
for unit in encoded {
let bytes = (*unit).to_le_bytes();
output.push(bytes[0]);
output.push(bytes[1]);
bytes_written += 2;
}
}
bytes_written
}
pub fn utf16_length_from_utf8<T: AsRef<[u8]>>(input: T) -> usize {
let input = input.as_ref();
match std::str::from_utf8(input) {
Ok(s) => s.encode_utf16().count(),
Err(_) => 0,
}
}
