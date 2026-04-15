// Generated Rust Code
pub type Utf8 = u8;
pub type Utf16 = u16;
pub type Utf32 = u32;
pub type Latin1 = u8;
pub type Ascii = u8;

fn utf_is_le() -> bool {
cfg!(target_endian = "little")
}

fn utf_is_be() -> bool {
cfg!(target_endian = "big")
}

fn utf_swap_uint16(n: u16) -> u16 {
((n & 0x00ff) << 8) | ((n & 0xff00) >> 8)
}

#[allow(dead_code)]
fn utf_swap_uint32(n: u32) -> u32 {
((n & 0x000000ff) << 24)
| ((n & 0x0000ff00) << 8)
| ((n & 0x00ff0000) >> 8)
| ((n & 0xff000000) >> 24)
}

pub fn utf8_validate(data: &[Utf8]) -> bool {
let len = data.len();
let mut pos: usize = 0;
while pos < len {
let mut next_pos = pos + 16;
if next_pos <= len {
let mut all_ascii = true;
for item in data.iter().take(next_pos).skip(pos) {
if item & 0x80 != 0 {
all_ascii = false;
break;
}
}
if all_ascii {
pos = next_pos;
continue;
}
}
let mut word = data[pos];
while word < 0b10000000 {
pos += 1;
if pos == len {
return true;
}
word = data[pos];
}
if (word & 0b1110_0000) == 0b1100_0000 {
next_pos = pos + 2;
if next_pos > len {
return false;
}
if (data[pos + 1] & 0b1100_0000) != 0b1000_0000 {
return false;
}
let code_point =
(((word & 0b0001_1111) as u32) << 6) | ((data[pos + 1] & 0b0011_1111) as u32);
if code_point < 0x80 || 0x7ff < code_point {
return false;
}
} else if (word & 0b1111_0000) == 0b1110_0000 {
next_pos = pos + 3;
if next_pos > len {
return false;
}
if (data[pos + 1] & 0b1100_0000) != 0b1000_0000 {
return false;
}
if (data[pos + 2] & 0b1100_0000) != 0b1000_0000 {
return false;
}
let code_point = (((word & 0b0000_1111) as u32) << 12)
| (((data[pos + 1] & 0b0011_1111) as u32) << 6)
| ((data[pos + 2] & 0b0011_1111) as u32);
if code_point < 0x800
|| 0xffff < code_point
|| (0xd7ff < code_point && code_point < 0xe000)
{
return false;
}
} else if (word & 0b1111_1000) == 0b1111_0000 {
next_pos = pos + 4;
if next_pos > len {
return false;
}
if (data[pos + 1] & 0b1100_0000) != 0b1000_0000 {
return false;
}
if (data[pos + 2] & 0b1100_0000) != 0b1000_0000 {
return false;
}
if (data[pos + 3] & 0b1100_0000) != 0b1000_0000 {
return false;
}
let code_point = (((word & 0b0000_0111) as u32) << 18)
| (((data[pos + 1] & 0b0011_1111) as u32) << 12)
| (((data[pos + 2] & 0b0011_1111) as u32) << 6)
| ((data[pos + 3] & 0b0011_1111) as u32);
if code_point <= 0xffff || 0x10ffff < code_point {
return false;
}
} else {
return false;
}
pos = next_pos;
}
true
}

pub fn utf8_length_from_utf16le(data: &[Utf16]) -> usize {
let mut counter = 0usize;
for &item in data {
let word = if utf_is_be() { utf_swap_uint16(item) } else { item };
if word <= 0x7f {
counter += 1;
} else if word <= 0x7ff {
counter += 2;
} else if word <= 0xd7ff || word >= 0xe000 {
counter += 3;
} else {
counter += 2;
}
}
counter
}

pub fn utf8_length_from_utf32(data: &[Utf32]) -> usize {
let mut counter = 0usize;
for &v in data {
counter += 1;
counter += usize::from(v > 0x7f);
counter += usize::from(v > 0x7ff);
counter += usize::from(v > 0xffff);
}
counter
}

pub fn utf8_length_from_latin1(data: &[Latin1]) -> usize {
let mut counter = data.len();
for &b in data {
counter += (b >> 7) as usize;
}
counter
}

pub fn utf8_convert_to_utf16le(data: &[Utf8], result: &mut [Utf16]) -> usize {
let len = data.len();
let mut pos = 0usize;
let mut out = 0usize;
while pos < len {
if pos + 8 <= len {
let mut all_ascii = true;
for item in data.iter().skip(pos).take(8) {
if item & 0x80 != 0 {
all_ascii = false;
break;
}
}
if all_ascii {
let final_pos = pos + 8;
while pos < final_pos {
result[out] = if utf_is_be() {
utf_swap_uint16(data[pos] as u16)
} else {
data[pos] as u16
};
out += 1;
pos += 1;
}
continue;
}
}
let leading_byte = data[pos];
if leading_byte < 0b1000_0000 {
result[out] = if utf_is_be() {
utf_swap_uint16(leading_byte as u16)
} else {
leading_byte as u16
};
out += 1;
pos += 1;
} else if (leading_byte & 0b1110_0000) == 0b1100_0000 {
if pos + 1 >= len {
break;
}
let mut code_point = (((leading_byte & 0b0001_1111) as u16) << 6)
| ((data[pos + 1] & 0b0011_1111) as u16);
if utf_is_be() {
code_point = utf_swap_uint16(code_point);
}
result[out] = code_point;
out += 1;
pos += 2;
} else if (leading_byte & 0b1111_0000) == 0b1110_0000 {
if pos + 2 >= len {
break;
}
let mut code_point = (((leading_byte & 0b0000_1111) as u16) << 12)
| (((data[pos + 1] & 0b0011_1111) as u16) << 6)
| ((data[pos + 2] & 0b0011_1111) as u16);
if utf_is_be() {
code_point = utf_swap_uint16(code_point);
}
result[out] = code_point;
out += 1;
pos += 3;
} else if (leading_byte & 0b1111_1000) == 0b1111_0000 {
if pos + 3 >= len {
break;
}
let mut code_point = (((leading_byte & 0b0000_0111) as u32) << 18)
| (((data[pos + 1] & 0b0011_1111) as u32) << 12)
| (((data[pos + 2] & 0b0011_1111) as u32) << 6)
| ((data[pos + 3] & 0b0011_1111) as u32);
code_point -= 0x10000;
let mut high_surrogate = 0xd800u16 + ((code_point >> 10) as u16);
let mut low_surrogate = 0xdc00u16 + ((code_point & 0x3ff) as u16);
if utf_is_be() {
high_surrogate = utf_swap_uint16(high_surrogate);
low_surrogate = utf_swap_uint16(low_surrogate);
}
result[out] = high_surrogate;
result[out + 1] = low_surrogate;
out += 2;
pos += 4;
} else {
return 0;
}
}
out
}

pub fn utf8_convert_to_utf32(data: &[Utf8], result: &mut [Utf32]) -> usize {
let len = data.len();
let mut pos = 0usize;
let mut out = 0usize;
while pos < len {
if pos + 16 <= len {
let mut all_ascii = true;
for item in data.iter().skip(pos).take(16) {
if item & 0x80 != 0 {
all_ascii = false;
break;
}
}
if all_ascii {
let final_pos = pos + 16;
while pos < final_pos {
result[out] = data[pos] as u32;
out += 1;
pos += 1;
}
continue;
}
}
let leading_byte = data[pos];
if leading_byte < 0b1000_0000 {
result[out] = leading_byte as u32;
out += 1;
pos += 1;
} else if (leading_byte & 0b1110_0000) == 0b1100_0000 {
if pos + 1 >= len {
return 0;
}
if (data[pos + 1] & 0b1100_0000) != 0b1000_0000 {
return 0;
}
let code_point = (((leading_byte & 0b0001_1111) as u32) << 6)
| ((data[pos + 1] & 0b0011_1111) as u32);
if code_point < 0x80 || 0x7ff < code_point {
return 0;
}
result[out] = code_point;
out += 1;
pos += 2;
} else if (leading_byte & 0b1111_0000) == 0b1110_0000 {
if pos + 2 >= len {
return 0;
}
if (data[pos + 1] & 0b1100_0000) != 0b1000_0000 {
return 0;
}
if (data[pos + 2] & 0b1100_0000) != 0b1000_0000 {
return 0;
}
let code_point = (((leading_byte & 0b0000_1111) as u32) << 12)
| (((data[pos + 1] & 0b0011_1111) as u32) << 6)
| ((data[pos + 2] & 0b0011_1111) as u32);
if code_point < 0x800
|| 0xffff < code_point
|| (0xd7ff < code_point && code_point < 0xe000)
{
return 0;
}
result[out] = code_point;
out += 1;
pos += 3;
} else if (leading_byte & 0b1111_1000) == 0b1111_0000 {
if pos + 3 >= len {
return 0;
}
if (data[pos + 1] & 0b1100_0000) != 0b1000_0000 {
return 0;
}
if (data[pos + 2] & 0b1100_0000) != 0b1000_0000 {
return 0;
}
if (data[pos + 3] & 0b1100_0000) != 0b1000_0000 {
return 0;
}
let code_point = (((leading_byte & 0b0000_0111) as u32) << 18)
| (((data[pos + 1] & 0b0011_1111) as u32) << 12)
| (((data[pos + 2] & 0b0011_1111) as u32) << 6)
| ((data[pos + 3] & 0b0011_1111) as u32);
if code_point <= 0xffff || 0x10ffff < code_point {
return 0;
}
result[out] = code_point;
out += 1;
pos += 4;
} else {
return 0;
}
}
out
}

pub fn utf8_convert_to_latin1(data: &[Utf8], result: &mut [Latin1]) -> usize {
let len = data.len();
let mut pos = 0usize;
let mut out = 0usize;
while pos < len {
if pos + 16 <= len {
let mut all_ascii = true;
for item in data.iter().skip(pos).take(16) {
if item & 0x80 != 0 {
all_ascii = false;
break;
}
}
if all_ascii {
let final_pos = pos + 16;
while pos < final_pos {
result[out] = data[pos];
out += 1;
pos += 1;
}
continue;
}
}
let leading_byte = data[pos];
if leading_byte < 0b1000_0000 {
result[out] = leading_byte;
out += 1;
pos += 1;
} else if (leading_byte & 0b1110_0000) == 0b1100_0000 {
if pos + 1 >= len {
return 0;
}
if (data[pos + 1] & 0b1100_0000) != 0b1000_0000 {
return 0;
}
let code_point = (((leading_byte & 0b0001_1111) as u32) << 6)
| ((data[pos + 1] & 0b0011_1111) as u32);
if code_point < 0x80 || 0xff < code_point {
return 0;
}
result[out] = code_point as u8;
out += 1;
pos += 2;
} else {
return 0;
}
}
out
}

pub fn utf16le_validate(data: &[Utf16]) -> bool {
let len = data.len();
let mut pos = 0usize;
while pos < len {
let mut word = if utf_is_be() {
utf_swap_uint16(data[pos])
} else {
data[pos]
};
if (word & 0xf800) == 0xd800 {
if pos + 1 >= len {
return false;
}
let mut diff = word.wrapping_sub(0xd800);
if diff > 0x03ff {
return false;
}
word = if utf_is_be() {
utf_swap_uint16(data[pos + 1])
} else {
data[pos + 1]
};
diff = word.wrapping_sub(0xdc00);
if diff > 0x03ff {
return false;
}
pos += 2;
} else {
pos += 1;
}
}
true
}

pub fn utf16_length_from_utf8(data: &[Utf8]) -> usize {
let mut counter = 0usize;
for &b in data {
if (b as i8) > -65 {
counter += 1;
}
if b >= 240 {
counter += 1;
}
}
counter
}

pub fn utf16_length_from_utf32(data: &[Utf32]) -> usize {
let mut counter = 0usize;
for &v in data {
counter += 1;
counter += usize::from(v > 0xffff);
}
counter
}

pub fn utf16_length_from_latin1(data: &[Latin1]) -> usize {
data.len()
}

pub fn utf16le_convert_to_utf8(data: &[Utf16], result: &mut [Utf8]) -> usize {
let len = data.len();
let mut pos = 0usize;
let mut out = 0usize;
while pos < len {
if pos + 4 <= len {
let mut all_ascii = true;
for item in data.iter().skip(pos).take(4) {
let v = if utf_is_be() {
utf_swap_uint16(*item)
} else {
*item
};
if (v & 0xff80) != 0 {
all_ascii = false;
break;
}
}
if all_ascii {
let final_pos = pos + 4;
while pos < final_pos {
result[out] = if utf_is_be() {
utf_swap_uint16(data[pos]) as u8
} else {
data[pos] as u8
};
out += 1;
pos += 1;
}
continue;
}
}
let mut word = if utf_is_be() {
utf_swap_uint16(data[pos])
} else {
data[pos]
};
if (word & 0xff80) == 0 {
result[out] = word as u8;
out += 1;
pos += 1;
} else if (word & 0xf800) == 0 {
result[out] = ((word >> 6) as u8) | 0b1100_0000;
result[out + 1] = ((word & 0b11_1111) as u8) | 0b1000_0000;
out += 2;
pos += 1;
} else if (word & 0xf800) != 0xd800 {
result[out] = ((word >> 12) as u8) | 0b1110_0000;
result[out + 1] = (((word >> 6) & 0b11_1111) as u8) | 0b1000_0000;
result[out + 2] = ((word & 0b11_1111) as u8) | 0b1000_0000;
out += 3;
pos += 1;
} else {
let diff = word.wrapping_sub(0xd800);
if pos + 1 >= len {
return 0;
}
word = if utf_is_be() {
utf_swap_uint16(data[pos + 1])
} else {
data[pos + 1]
};
let value = ((diff as u32) << 10) + (word.wrapping_sub(0xdc00) as u32) + 0x10000;
result[out] = ((value >> 18) as u8) | 0b1111_0000;
result[out + 1] = (((value >> 12) & 0b11_1111) as u8) | 0b1000_0000;
result[out + 2] = (((value >> 6) & 0b11_1111) as u8) | 0b1000_0000;
result[out + 3] = ((value & 0b11_1111) as u8) | 0b1000_0000;
out += 4;
pos += 2;
}
}
out
}

pub fn utf16le_convert_to_utf32(data: &[Utf16], result: &mut [Utf32]) -> usize {
let len = data.len();
let mut pos = 0usize;
let mut out = 0usize;
while pos < len {
let mut word = if utf_is_be() {
utf_swap_uint16(data[pos])
} else {
data[pos]
};
if (word & 0xf800) != 0xd800 {
result[out] = word as u32;
out += 1;
pos += 1;
} else {
let diff = word.wrapping_sub(0xd800);
if diff > 0x03ff {
return 0;
}
if pos + 1 >= len {
return 0;
}
word = if utf_is_be() {
utf_swap_uint16(data[pos + 1])
} else {
data[pos + 1]
};
let value = ((diff as u32) << 10) + (word.wrapping_sub(0xdc00) as u32) + 0x10000;
let diff2 = word.wrapping_sub(0xdc00);
if diff2 > 0x03ff {
return 0;
}
result[out] = value;
out += 1;
pos += 2;
}
}
out
}

pub fn utf16le_convert_to_latin1(data: &[Utf16], result: &mut [Latin1]) -> usize {
let mut pos = 0usize;
let mut overflow: u16 = 0;
let mut out = 0usize;
while pos < data.len() {
let word = if utf_is_be() {
utf_swap_uint16(data[pos])
} else {
data[pos]
};
overflow |= word;
result[out] = (word & 0xff) as u8;
out += 1;
pos += 1;
}
if (overflow & 0xff00) != 0 {
return 0;
}
out
}

pub fn utf32_validate(data: &[Utf32]) -> bool {
let mut pos = 0usize;
while pos < data.len() {
let word = data[pos];
if word > 0x10ffff || (0xd800..=0xdfff).contains(&word) {
return false;
}
pos += 1;
}
true
}

pub fn utf32_length_from_utf8(data: &[Utf8]) -> usize {
let mut counter = 0usize;
for &b in data {
if (b as i8) > -65 {
counter += 1;
}
}
counter
}

pub fn utf32_length_from_utf16le(data: &[Utf16]) -> usize {
let mut counter = 0usize;
for &item in data {
let word = if utf_is_be() { utf_swap_uint16(item) } else { item };
counter += usize::from((word & 0xfc00) != 0xdc00);
}
counter
}

pub fn utf32_length_from_latin1(data: &[Latin1]) -> usize {
data.len()
}

pub fn utf32_convert_to_utf8(data: &[Utf32], result: &mut [Utf8]) -> usize {
let len = data.len();
let mut pos = 0usize;
let mut out = 0usize;
while pos < len {
if pos + 2 <= len {
let v0 = data[pos];
let v1 = data[pos + 1];
if (v0 & 0xffffff80) == 0 && (v1 & 0xffffff80) == 0 {
result[out] = v0 as u8;
result[out + 1] = v1 as u8;
out += 2;
pos += 2;
continue;
}
}
let word = data[pos];
if (word & 0xffffff80) == 0 {
result[out] = word as u8;
out += 1;
pos += 1;
} else if (word & 0xfffff800) == 0 {
result[out] = ((word >> 6) as u8) | 0b1100_0000;
result[out + 1] = ((word & 0b11_1111) as u8) | 0b1000_0000;
out += 2;
pos += 1;
} else if (word & 0xffff0000) == 0 {
if (0xd800..=0xdfff).contains(&word) {
return 0;
}
result[out] = ((word >> 12) as u8) | 0b1110_0000;
result[out + 1] = (((word >> 6) & 0b11_1111) as u8) | 0b1000_0000;
result[out + 2] = ((word & 0b11_1111) as u8) | 0b1000_0000;
out += 3;
pos += 1;
} else {
if word > 0x10ffff {
return 0;
}
result[out] = ((word >> 18) as u8) | 0b1111_0000;
result[out + 1] = (((word >> 12) & 0b11_1111) as u8) | 0b1000_0000;
result[out + 2] = (((word >> 6) & 0b11_1111) as u8) | 0b1000_0000;
result[out + 3] = ((word & 0b11_1111) as u8) | 0b1000_0000;
out += 4;
pos += 1;
}
}
out
}

pub fn utf32_convert_to_utf16le(data: &[Utf32], result: &mut [Utf16]) -> usize {
let mut pos = 0usize;
let mut out = 0usize;
while pos < data.len() {
let mut word = data[pos];
if (word & 0xffff0000) == 0 {
if (0xd800..=0xdfff).contains(&word) {
return 0;
}
result[out] = if utf_is_be() {
utf_swap_uint16(word as u16)
} else {
word as u16
};
out += 1;
} else {
if word > 0x10ffff {
return 0;
}
word -= 0x10000;
let mut high_surrogate = 0xd800u16 + ((word >> 10) as u16);
let mut low_surrogate = 0xdc00u16 + ((word & 0x3ff) as u16);
if utf_is_be() {
high_surrogate = utf_swap_uint16(high_surrogate);
low_surrogate = utf_swap_uint16(low_surrogate);
}
result[out] = high_surrogate;
result[out + 1] = low_surrogate;
out += 2;
}
pos += 1;
}
out
}

pub fn utf32_convert_to_latin1(data: &[Utf32], result: &mut [Latin1]) -> usize {
let mut pos = 0usize;
let mut overflow: u32 = 0;
let mut out = 0usize;
while pos < data.len() {
let word = data[pos];
overflow |= word;
result[out] = (word & 0xff) as u8;
out += 1;
pos += 1;
}
if (overflow & 0xffffff00) != 0 {
return 0;
}
out
}

pub fn latin1_length_from_utf8(data: &[Utf8]) -> usize {
let mut counter = 0usize;
for &b in data {
if (b as i8) > -65 {
counter += 1;
}
}
counter
}

pub fn latin1_length_from_utf16le(data: &[Utf16]) -> usize {
data.len()
}

pub fn latin1_length_from_utf32(data: &[Utf32]) -> usize {
data.len()
}

pub fn latin1_convert_to_utf8(data: &[Latin1], result: &mut [Utf8]) -> usize {
let len = data.len();
let mut pos = 0usize;
let mut out = 0usize;
while pos < len {
if pos + 16 <= len {
let mut all_ascii = true;
for item in data.iter().skip(pos).take(16) {
if item & 0x80 != 0 {
all_ascii = false;
break;
}
}
if all_ascii {
let final_pos = pos + 16;
while pos < final_pos {
result[out] = data[pos];
out += 1;
pos += 1;
}
continue;
}
}
let byte = data[pos];
if (byte & 0x80) == 0 {
result[out] = byte;
out += 1;
pos += 1;
} else {
result[out] = (byte >> 6) | 0b1100_0000;
result[out + 1] = (byte & 0b11_1111) | 0b1000_0000;
out += 2;
pos += 1;
}
}
out
}

pub fn latin1_convert_to_utf16le(data: &[Latin1], result: &mut [Utf16]) -> usize {
let mut pos = 0usize;
let mut out = 0usize;
while pos < data.len() {
result[out] = if utf_is_be() {
utf_swap_uint16(data[pos] as u16)
} else {
data[pos] as u16
};
out += 1;
pos += 1;
}
out
}

pub fn latin1_convert_to_utf32(data: &[Latin1], result: &mut [Utf32]) -> usize {
let mut out = 0usize;
for &b in data {
result[out] = b as u32;
out += 1;
}
out
}

pub fn ascii_validate(data: &[Ascii]) -> bool {
let len = data.len();
let mut pos = 0usize;
while pos + 16 <= len {
for item in data.iter().skip(pos).take(16) {
if (item & 0x80) != 0 {
return false;
}
}
pos += 16;
}
while pos < len {
if data[pos] >= 0b1000_0000 {
return false;
}
pos += 1;
}
true
}

#[allow(dead_code)]
fn _utf_is_le_for_parity() -> bool {
utf_is_le()
}
