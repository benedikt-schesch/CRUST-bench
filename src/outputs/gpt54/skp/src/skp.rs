// Generated Rust Code
use std::fmt::Arguments;

/// SKP version information.
pub const SKP_VER: u32 = 0x0003001C;
pub const SKP_VER_STR: &str = "0.3.1rc";

/// A loop state used for scanning.
/// (In the C header this is defined only via macros; here we provide a Rust struct.)
#[derive(Debug, Default, Clone)]
pub struct SkpLoop {
pub start: String,
pub to: Option<String>,
pub end: Option<String>,
pub alt: i32,
}

/// Compatibility namespace expected by the test binaries.
/// The original translated tests import `skp::skp::{skp_}` and then call functions
/// through that module path. Re-exporting all public items here preserves that API.
pub mod skp_ {
pub use super::{
chr_cmp, get_close, get_qclose, is_alnum, is_alpha, is_blank, is_break, is_ctrl, is_digit,
is_idchr, is_lower, is_oneof, is_space, is_string, is_upper, is_xdigit, match_pat,
skp_loop_len, skp_next, skp_scan, skptrace, SkpLoop, MATCHED, MATCHED_FAIL,
MATCHED_GOAL, MATCHED_GOALNOT, SKP_VER, SKP_VER_STR,
};
}

/// Returns the “length” from start to to. (This mimics the inline function `skp_loop_len`.)
pub fn skp_loop_len(start: &str, to: &str) -> i32 {
let ret = start.len() as i32 - to.len() as i32;
if (0..=(1 << 16)).contains(&ret) {
ret
} else {
0
}
}

/// Compatibility wrapper used by the translated test binaries.
///
/// It returns:
/// - match result / alt code
/// - remaining source slice after the consumed prefix
/// - remaining pattern slice after the consumed prefix
pub fn skp_<'a>(src: &'a str, pat: &'a str) -> (i32, &'a str, &'a str) {
let mut flg = 0i32;
let (ret, src_end, pat_end) = match_pat(pat, src, &mut flg);
(ret, src_end, pat_end)
}

/// Global variable used in the C code.
/// (In C declared as `volatile int skp_zero;`—here we use a mutable static.)
pub static mut SKP_ZERO: i32 = 0;

/// Trace function (corresponds to the C macro skptrace).
pub fn skptrace(args: Arguments<'_>) {
eprintln!("TRCE: {}", args);
}

fn first_byte(s: &str) -> u8 {
s.as_bytes().first().copied().unwrap_or(0)
}

fn byte_at(s: &str, idx: usize) -> u8 {
s.as_bytes().get(idx).copied().unwrap_or(0)
}

fn slice_from(s: &str, idx: usize) -> &str {
if idx >= s.len() {
""
} else {
&s[idx..]
}
}

fn lower_ascii_u32(c: u32) -> u32 {
if (b'A' as u32) <= c && c <= (b'Z' as u32) {
c + 32
} else {
c
}
}

/// Returns the next Unicode code point from the string `s` (similar to `skp_next` in C).
/// Returns a tuple `(code_point, rest_of_string)`.
pub fn skp_next(s: &str, iso: i32) -> (u32, &str) {
if s.is_empty() {
return (0, s);
}

let bytes = s.as_bytes();
let mut idx = 0usize;
let mut c = bytes[idx] as u32;
idx += 1;

if iso == 0 {
if idx < bytes.len() && (bytes[idx] & 0xC0) == 0x80 {
c = (c << 8) | bytes[idx] as u32;
idx += 1;
if idx < bytes.len() && (bytes[idx] & 0xC0) == 0x80 {
c = (c << 8) | bytes[idx] as u32;
idx += 1;
if idx < bytes.len() && (bytes[idx] & 0xC0) == 0x80 {
c = (c << 8) | bytes[idx] as u32;
idx += 1;
}
}
}
}

if c == 0x0D && idx < bytes.len() && bytes[idx] == 0x0A {
c = 0x0D0A;
idx += 1;
}

(c, slice_from(s, idx))
}

/// Compares two code points. If `fold` is nonzero, performs case-insensitive comparison.
/// (Corresponds to `chr_cmp`.)
pub fn chr_cmp(a: u32, b: u32, fold: i32) -> bool {
let mut aa = a;
let mut bb = b;
if fold != 0 && aa <= 0x7F && bb <= 0x7F {
aa = lower_ascii_u32(aa);
bb = lower_ascii_u32(bb);
}
aa == bb
}

/// Returns true if `c` is a blank character.
/// (Corresponds to `is_blank`.)
pub fn is_blank(c: u32) -> bool {
if c < 0xFF {
return c == 0x20 || c == 0x09;
}
match c & 0xFFFFFF00 {
0x00000000 => c == 0xA0,
0x0000C200 => c == 0xC2A0,
0x00E19A00 => c == 0xE19A80,
0x00E28000 => (0xE28080..=0xE2808A).contains(&c) || c == 0xE280AF,
0x00E38000 => c == 0xE38080,
_ => false,
}
}

/// Returns true if `c` is a line-break character.
/// (Corresponds to `is_break`.)
pub fn is_break(c: u32) -> bool {
if c < 0x0F {
return c == 0x0A || c == 0x0C || c == 0x0D;
}
if c < 0xFF {
return c == 0x85;
}
c == 0x0D0A || c == 0xC285 || c == 0xE280A8 || c == 0xE280A9
}

/// Returns true if `c` is a space (blank or break).
pub fn is_space(c: u32) -> bool {
is_blank(c) || is_break(c)
}

/// Returns true if `c` is a digit.
pub fn is_digit(c: u32) -> bool {
(b'0' as u32) <= c && c <= (b'9' as u32)
}

/// Returns true if `c` is a hexadecimal digit.
pub fn is_xdigit(c: u32) -> bool {
((b'0' as u32) <= c && c <= (b'9' as u32))
|| ((b'A' as u32) <= c && c <= (b'F' as u32))
|| ((b'a' as u32) <= c && c <= (b'f' as u32))
}

/// Returns true if `c` is an uppercase letter.
pub fn is_upper(c: u32) -> bool {
(b'A' as u32) <= c && c <= (b'Z' as u32)
}

/// Returns true if `c` is a lowercase letter.
pub fn is_lower(c: u32) -> bool {
(b'a' as u32) <= c && c <= (b'z' as u32)
}

/// Returns true if `c` is an alphabetic character.
pub fn is_alpha(c: u32) -> bool {
is_upper(c) || is_lower(c)
}

/// Returns true if `c` is a valid identifier character.
pub fn is_idchr(c: u32) -> bool {
is_alpha(c) || is_digit(c) || c == b'_' as u32
}

/// Returns true if `c` is alphanumeric.
pub fn is_alnum(c: u32) -> bool {
is_alpha(c) || is_digit(c)
}

/// Returns true if `c` is a control character.
pub fn is_ctrl(c: u32) -> bool {
c < 0x20 || (0xC280..0xC2A0).contains(&c) || (0x7F..0xA0).contains(&c)
}

/// Returns true if `ch` is one of the characters in `set`. The `iso` flag is used for encoding.
pub fn is_oneof(ch: u32, set: &str, iso: i32) -> bool {
if ch == 0 {
return false;
}

let (mut p_ch, mut s) = skp_next(set, iso);

if p_ch == b']' as u32 {
if ch == b']' as u32 {
return true;
} else {
let t = skp_next(s, iso);
p_ch = t.0;
s = t.1;
}
}

while p_ch != b']' as u32 && p_ch != 0 {
if p_ch == ch {
return true;
}
let q_ch = p_ch;
let t = skp_next(s, iso);
p_ch = t.0;
s = t.1;
if p_ch == b'-' as u32 && first_byte(s) != b']' {
let t2 = skp_next(s, iso);
p_ch = t2.0;
s = t2.1;
if q_ch < ch && ch <= p_ch {
return true;
}
let t3 = skp_next(s, iso);
p_ch = t3.0;
s = t3.1;
}
}

false
}

/// Checks if the string `s` starts with the pattern `p` for `len` characters, using flag `flg`.
pub fn is_string(s: &str, p: &str, len: i32, flg: i32) -> i32 {
let start = s;
let mut s_cur = s;
let mut p_cur = p;
let mut rem = len;
let mut mlen = 0i32;

while rem != 0 {
if first_byte(p_cur) == 0x0E {
return mlen;
}
let (p_chr, p_end) = skp_next(p_cur, flg & 2);
let (s_chr, s_end) = skp_next(s_cur, flg & 2);
if chr_cmp(s_chr, p_chr, flg & 1) {
mlen += (s_cur.len() - s_end.len()) as i32;
rem -= (p_cur.len() - p_end.len()) as i32;
p_cur = p_end;
s_cur = s_end;
} else {
while rem > 0 && first_byte(p_cur) != 0x0E {
p_cur = slice_from(p_cur, 1);
rem -= 1;
}
if rem <= 0 {
return 0;
}
p_cur = slice_from(p_cur, 1);
rem -= 1;
s_cur = start;
mlen = 0;
}
}

mlen
}

/// Given an opening parenthesis code point, returns the corresponding closing code point.
pub fn get_close(open: u32) -> u32 {
match open {
x if x == b'(' as u32 => b')' as u32,
x if x == b'[' as u32 => b']' as u32,
x if x == b'{' as u32 => b'}' as u32,
x if x == b'<' as u32 => b'>' as u32,
_ => 0,
}
}

/// Given a quote character, returns the corresponding closing quote.
pub fn get_qclose(open: u32) -> u32 {
match open {
x if x == b'\'' as u32 => x,
x if x == b'"' as u32 => x,
x if x == b'`' as u32 => x,
_ => 0,
}
}

/// Constants for match results.
pub const MATCHED_FAIL: i32 = 0;
pub const MATCHED: i32 = 1;
pub const MATCHED_GOAL: i32 = 2;
pub const MATCHED_GOALNOT: i32 = 3;

/// Matches the pattern `pat` against source `src` and returns a tuple:
/// `(match_result, src_end, pat_end)`. The flag parameter is passed by mutable reference.
pub fn match_pat<'a>(pat: &'a str, src: &'a str, flg: &mut i32) -> (i32, &'a str, &'a str) {
let mut pat_cur = pat;
let mut ret = MATCHED_FAIL;
let mut match_min: u32 = 1;
let mut match_max: u32 = 1;
let mut match_not: u32 = 0;
let intnumber = false;

let mut s_end = src;
let mut s_tmp;
let mut s_chr;
{
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}

match first_byte(pat_cur) {
b'*' => {
match_min = 0;
match_max = u32::MAX;
pat_cur = slice_from(pat_cur, 1);
}
b'+' => {
match_max = u32::MAX;
pat_cur = slice_from(pat_cur, 1);
}
b'?' => {
match_min = 0;
pat_cur = slice_from(pat_cur, 1);
}
_ => {}
}

if first_byte(pat_cur) == b'!' {
match_not = 1;
pat_cur = slice_from(pat_cur, 1);
}

while first_byte(pat_cur) == b' ' {
pat_cur = slice_from(pat_cur, 1);
}

let op = first_byte(pat_cur);
if !pat_cur.is_empty() {
pat_cur = slice_from(pat_cur, 1);
}

match op {
b'.' => {
if match_not != 0 {
ret = if s_chr == 0 { MATCHED } else { MATCHED_FAIL };
} else {
let mut cnt = 0u32;
while cnt < match_max && s_chr != 0 {
cnt += 1;
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
ret = if cnt >= match_min { MATCHED } else { MATCHED_FAIL };
}
}
b'$' => {
if s_chr == 0 {
ret = MATCHED;
} else {
let mut cnt = 0u32;
while cnt < match_max && s_chr != 0 && ((is_break(s_chr) as u32) != match_not) {
cnt += 1;
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
ret = if cnt >= match_min { MATCHED } else { MATCHED_FAIL };
}
}
b'n' => {
let mut cnt = 0u32;
while cnt < match_max && s_chr != 0 && ((is_break(s_chr) as u32) != match_not) {
cnt += 1;
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
ret = if cnt >= match_min { MATCHED } else { MATCHED_FAIL };
}
b'd' | b'D' if pat == pat_cur => {
let mut cnt = 0u32;
while cnt < match_max && s_chr != 0 && ((is_digit(s_chr) as u32) != match_not) {
cnt += 1;
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
ret = if cnt >= match_min { MATCHED } else { MATCHED_FAIL };
}
b'd' => {
let mut cnt = 0u32;
while cnt < match_max && s_chr != 0 && ((is_digit(s_chr) as u32) != match_not) {
cnt += 1;
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
ret = if cnt >= match_min { MATCHED } else { MATCHED_FAIL };
}
b'x' => {
let mut cnt = 0u32;
while cnt < match_max && s_chr != 0 && ((is_xdigit(s_chr) as u32) != match_not) {
cnt += 1;
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
ret = if cnt >= match_min { MATCHED } else { MATCHED_FAIL };
}
b'a' => {
let mut cnt = 0u32;
while cnt < match_max && s_chr != 0 && ((is_alpha(s_chr) as u32) != match_not) {
cnt += 1;
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
ret = if cnt >= match_min { MATCHED } else { MATCHED_FAIL };
}
b'u' => {
let mut cnt = 0u32;
while cnt < match_max && s_chr != 0 && ((is_upper(s_chr) as u32) != match_not) {
cnt += 1;
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
ret = if cnt >= match_min { MATCHED } else { MATCHED_FAIL };
}
b'l' => {
let mut cnt = 0u32;
while cnt < match_max && s_chr != 0 && ((is_lower(s_chr) as u32) != match_not) {
cnt += 1;
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
ret = if cnt >= match_min { MATCHED } else { MATCHED_FAIL };
}
b's' => {
let mut cnt = 0u32;
while cnt < match_max && s_chr != 0 && ((is_space(s_chr) as u32) != match_not) {
cnt += 1;
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
ret = if cnt >= match_min { MATCHED } else { MATCHED_FAIL };
}
b'w' => {
let mut cnt = 0u32;
while cnt < match_max && s_chr != 0 && ((is_blank(s_chr) as u32) != match_not) {
cnt += 1;
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
ret = if cnt >= match_min { MATCHED } else { MATCHED_FAIL };
}
b'c' => {
let mut cnt = 0u32;
while cnt < match_max && s_chr != 0 && ((is_ctrl(s_chr) as u32) != match_not) {
cnt += 1;
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
ret = if cnt >= match_min { MATCHED } else { MATCHED_FAIL };
}
b'i' => {
let mut cnt = 0u32;
while cnt < match_max && s_chr != 0 && ((is_idchr(s_chr) as u32) != match_not) {
cnt += 1;
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
ret = if cnt >= match_min { MATCHED } else { MATCHED_FAIL };
}
b'@' => {
let mut cnt = 0u32;
while cnt < match_max && s_chr != 0 && ((is_alnum(s_chr) as u32) != match_not) {
cnt += 1;
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
ret = if cnt >= match_min { MATCHED } else { MATCHED_FAIL };
}
b'&' => {
ret = if match_not != 0 {
MATCHED_GOALNOT
} else {
MATCHED_GOAL
};
}
b'[' => {
let mut cnt = 0u32;
while cnt < match_max
&& s_chr != 0
&& ((is_oneof(s_chr, pat_cur, *flg & 2) as u32) != match_not)
{
cnt += 1;
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
ret = if cnt >= match_min { MATCHED } else { MATCHED_FAIL };

if first_byte(pat_cur) == b']' {
pat_cur = slice_from(pat_cur, 1);
}
while !pat_cur.is_empty() && first_byte(pat_cur) != b']' {
pat_cur = slice_from(pat_cur, 1);
}
if !pat_cur.is_empty() {
pat_cur = slice_from(pat_cur, 1);
}
}
b'"' | b'\'' | b'`' => {
let quote = op;
let mut l = 0usize;
while l < pat_cur.len() && byte_at(pat_cur, l) != quote {
l += 1;
}
let ml = if l > 0 {
is_string(s_end, &pat_cur[..l], l as i32, *flg)
} else {
0
};
if l > 0 && ml > 0 {
if match_not == 0 {
s_end = slice_from(s_end, ml as usize);
ret = MATCHED;
}
} else if match_min == 0 || match_not != 0 {
ret = MATCHED;
}
pat_cur = slice_from(pat_cur, l.saturating_add(1));
}
b'C' => {
*flg = (*flg & !1) | (match_not as i32);
ret = MATCHED;
}
b'U' => {
*flg = (*flg & !2) | ((match_not as i32) * 2);
ret = MATCHED;
}
b'S' => {
while is_space(s_chr) {
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
ret = MATCHED;
}
b'W' => {
while is_blank(s_chr) {
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
ret = MATCHED;
}
b'N' => {
while s_chr != 0 && !is_break(s_chr) {
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
if s_chr != 0 {
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
ret = MATCHED;
}
b'I' => {
if is_alpha(s_chr) || s_chr == b'_' as u32 {
loop {
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
if !(is_alnum(s_chr) || s_chr == b'_' as u32) {
break;
}
}
ret = MATCHED;
}
}
b'(' => {
if first_byte(pat_cur) != b')' || s_chr != b'(' as u32 {
ret = MATCHED_FAIL;
} else {
pat_cur = slice_from(pat_cur, 1);
let open = s_chr;
let close = get_close(open);
if close != 0 {
let mut count: i32 = 1;
while s_chr != 0 && count > 0 {
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
if s_chr == open {
count += 1;
}
if s_chr == close {
count -= 1;
}
}
if count == 0 {
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
ret = MATCHED;
}
}
}
}
b'B' => {
let open = s_chr;
let close = get_close(open);
if close != 0 {
let mut count: i32 = 1;
while s_chr != 0 && count > 0 {
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
if s_chr == open {
count += 1;
}
if s_chr == close {
count -= 1;
}
}
if count == 0 {
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
ret = MATCHED;
}
}
}
b'Q' => {
let qclose = get_qclose(s_chr);
if qclose != 0 {
while s_chr != 0 {
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
if s_chr == qclose {
break;
}
if s_chr == b'\\' as u32 {
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
}
if s_chr != 0 {
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
ret = MATCHED;
}
}
}
b'X' => {
if s_chr == b'0' as u32
&& (byte_at(s_end, 1) == b'x' || byte_at(s_end, 1) == b'X')
&& is_xdigit(byte_at(s_end, 2) as u32)
{
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
ret = MATCHED;
}
while is_xdigit(s_chr) {
ret = MATCHED;
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
}
b'D' => {
let _ = intnumber;
if s_chr == b'+' as u32 || s_chr == b'-' as u32 {
loop {
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
if !is_space(s_chr) {
break;
}
}
}
while is_digit(s_chr) {
ret = MATCHED;
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
}
b'F' => {
if s_chr == b'+' as u32 || s_chr == b'-' as u32 {
loop {
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
if !is_space(s_chr) {
break;
}
}
}
while is_digit(s_chr) {
ret = MATCHED;
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
if s_chr == b'.' as u32 {
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
while is_digit(s_chr) {
ret = MATCHED;
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
if ret == MATCHED && (s_chr == b'E' as u32 || s_chr == b'e' as u32) {
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
if s_chr == b'+' as u32 || s_chr == b'-' as u32 {
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
while is_digit(s_chr) {
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
if s_chr == b'.' as u32 {
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
while is_digit(s_chr) {
s_end = s_tmp;
let t = skp_next(s_end, *flg & 2);
s_chr = t.0;
s_tmp = t.1;
}
}
}
_ => {
if op != 0 && chr_cmp(s_chr, op as u32, *flg & 1) {
s_end = s_tmp;
ret = MATCHED;
} else {
ret = MATCHED_FAIL;
}
}
}

(ret, s_end, pat_cur)
}

/// The core scanning function from the C header.
///
/// Returns `(match_result, consumed_source_len, consumed_pattern_len)`.
pub fn skp_scan(src: &str, pat: &str, flg: &mut i32) -> (i32, usize, usize) {
let (ret, src_end, pat_end) = match_pat(pat, src, flg);
(ret, src.len() - src_end.len(), pat.len() - pat_end.len())
}
