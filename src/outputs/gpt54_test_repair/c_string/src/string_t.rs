
pub const STRING_T_INDEXES_BUFFER_SIZE: usize = 512;
pub const STRING_T_SPACE_CHARS_ARR: &str = " \t\n\r";
pub type BoolT = bool;
#[derive(Clone)]
pub struct StringT {
pub bytes: Vec<u8>,
pub size: usize,
}
pub type StringTArray = Vec<StringT>;
pub fn new_string(size: usize) -> StringT {
StringT {
bytes: vec![0; size],
size,
}
}
pub fn new_string_from_bytes(bytes: &str) -> StringT {
StringT {
bytes: bytes.as_bytes().to_vec(),
size: bytes.len(),
}
}
pub fn string_free(_str: StringT) {}
pub fn string_len(str_: &StringT) -> usize {
str_.size
}
pub fn string_bytes(str_: &StringT) -> &str {
std::str::from_utf8(&str_.bytes).unwrap_or("")
}
pub fn string_eq(left: &StringT, right: &StringT) -> BoolT {
left.size == right.size && left.bytes == right.bytes
}
pub fn string_copy(str_: &StringT) -> StringT {
str_.clone()
}
pub fn string_concat(first: &StringT, second: &StringT) -> StringT {
let mut bytes = Vec::with_capacity(first.size + second.size);
bytes.extend_from_slice(&first.bytes);
bytes.extend_from_slice(&second.bytes);
StringT {
size: bytes.len(),
bytes,
}
}
pub fn string_substr(str_: &StringT, pos: usize, len: usize) -> StringT {
let end = pos.saturating_add(len).min(str_.bytes.len());
let start = pos.min(end);
let bytes = str_.bytes[start..end].to_vec();
StringT {
size: bytes.len(),
bytes,
}
}
pub fn string_startswith(str_: &StringT, prefix: &str) -> BoolT {
if str_.size < prefix.len() {
return false;
}
let exp_prefix = new_string_from_bytes(prefix);
let str_prefix = string_substr(str_, 0, exp_prefix.size);
string_eq(&str_prefix, &exp_prefix)
}
pub fn string_endswith(str_: &StringT, suffix: &str) -> BoolT {
if str_.size < suffix.len() {
return false;
}
let exp_suffix = new_string_from_bytes(suffix);
let str_suffix = string_substr(str_, str_.size - exp_suffix.size, exp_suffix.size);
string_eq(&str_suffix, &exp_suffix)
}
pub fn string_find(str_: &StringT, chars: &str) -> Option<usize> {
let chars_str = new_string_from_bytes(chars);
if chars_str.size == 0 {
return Some(0);
}
if chars_str.size > str_.size {
return None;
}
for pos in 0..str_.size {
let sub_str = string_substr(str_, pos, chars_str.size);
if string_eq(&sub_str, &chars_str) {
return Some(pos);
}
}
None
}
pub fn string_strip(str_: &StringT) -> StringT {
let mut start_pos = 0usize;
while start_pos < str_.size && string_t_is_space_char(str_.bytes[start_pos]) {
start_pos += 1;
}
let mut end_pos = str_.size as isize - 1;
while end_pos >= 0 && string_t_is_space_char(str_.bytes[end_pos as usize]) {
end_pos -= 1;
}
if start_pos as isize >= end_pos {
return string_copy(str_);
}
string_substr(
str_,
start_pos,
(end_pos as usize).saturating_sub(start_pos) + 1,
)
}
pub fn string_split(str_: &StringT, arr_size: &mut usize) -> StringTArray {
let mut result: StringTArray = Vec::new();
if str_.size == 0 {
result.push(string_copy(str_));
*arr_size = 1;
return result;
}
let mut start_pos = 0usize;
let mut pos = 0usize;
while pos < str_.size {
if string_t_is_space_char(str_.bytes[pos]) {
result.push(string_substr(str_, start_pos, pos - start_pos));
while pos < str_.size && string_t_is_space_char(str_.bytes[pos]) {
pos += 1;
}
start_pos = pos;
} else {
pos += 1;
}
}
if pos != start_pos {
result.push(string_substr(str_, start_pos, pos - start_pos));
}
*arr_size = result.len();
result
}
pub fn string_split_by(str_: &StringT, arr_size: &mut usize, split_chars: &str) -> StringTArray {
let mut result: StringTArray = Vec::new();
let split_str = new_string_from_bytes(split_chars);
if str_.size <= split_str.size {
result.push(string_copy(str_));
*arr_size = 1;
return result;
}
let mut start_pos = 0usize;
let mut pos = 0usize;
while pos < str_.size - split_str.size {
let sub_str = string_substr(str_, pos, split_str.size);
if string_eq(&sub_str, &split_str) {
result.push(string_substr(str_, start_pos, pos - start_pos));
start_pos = pos + split_str.size;
pos += split_str.size;
} else {
pos += 1;
}
}
if pos != start_pos {
result.push(string_substr(str_, start_pos, pos + 1 - start_pos));
}
*arr_size = result.len();
result
}
pub fn string_join_arr(str_arr: &StringTArray, arr_size: usize, space_chars: &str) -> StringT {
if arr_size == 0 {
return new_string(0);
}
let mut total_size = space_chars.len() * (arr_size - 1);
for item in str_arr.iter().take(arr_size) {
total_size += item.size;
}
let mut bytes = Vec::with_capacity(total_size);
for (idx, item) in str_arr.iter().take(arr_size).enumerate() {
bytes.extend_from_slice(&item.bytes);
if idx != arr_size - 1 {
bytes.extend_from_slice(space_chars.as_bytes());
}
}
StringT {
size: bytes.len(),
bytes,
}
}
pub fn string_t_is_space_char(byte: u8) -> BoolT {
STRING_T_SPACE_CHARS_ARR.as_bytes().contains(&byte)
}
