use std::fs::File;
use std::io::{Read, Write};
pub const SIG_SIZE_BYTES: usize = 16;
pub type Signature = [u8; SIG_SIZE_BYTES];
pub type U32 = u32;
pub type U64 = u64;
pub type U8 = u8;
pub const TRUSTED_CHK_MAX_BUF_SIZE: usize = 1 << 14;
pub fn trusted_utils_sig_to_str(sig: &[u8], out: &mut String) {
out.clear();
for b in sig.iter().take(SIG_SIZE_BYTES) {
out.push_str(&format!("{:02x}", b));
}
}
pub fn trusted_utils_write_int(i: i32, file: &mut File) {
let _ = file.write_all(&i.to_le_bytes());
}
pub fn trusted_utils_equal_signatures(left: &[u8], right: &[u8]) -> bool {
left.len() >= SIG_SIZE_BYTES
&& right.len() >= SIG_SIZE_BYTES
&& left[..SIG_SIZE_BYTES] == right[..SIG_SIZE_BYTES]
}
pub fn trusted_utils_write_sig(sig: &[u8], file: &mut File) {
let _ = file.write_all(&sig[..SIG_SIZE_BYTES.min(sig.len())]);
}
pub fn exit_oom() {
panic!("allocation failed - terminating");
}
pub fn trusted_utils_try_match_arg<'a>(arg: &'a str, opt: &str, out: &mut Option<&'a str>) {
if let Some(rest) = arg.strip_prefix(opt) {
*out = Some(rest);
}
}
pub fn trusted_utils_read_sig(out_sig: &mut [u8], file: &mut File) {
let mut buf = [0u8; SIG_SIZE_BYTES];
file.read_exact(&mut buf).unwrap();
out_sig[..SIG_SIZE_BYTES].copy_from_slice(&buf);
}
pub fn trusted_utils_read_ul(file: &mut File) -> u64 {
let mut buf = [0u8; 8];
file.read_exact(&mut buf).unwrap();
u64::from_le_bytes(buf)
}
pub fn trusted_utils_log_err(msg: &str) {
println!("c [TRUSTED_CORE] [ERROR] {}", msg);
}
pub fn trusted_utils_copy_bytes(to: &mut [u8], from: &[u8], nb_bytes: u64) {
let n = nb_bytes as usize;
to[..n].copy_from_slice(&from[..n]);
}
pub fn trusted_utils_write_ints(data: &[i32], nb_ints: u64, file: &mut File) {
for v in data.iter().take(nb_ints as usize) {
let _ = file.write_all(&v.to_le_bytes());
}
}
pub fn trusted_utils_read_uls(data: &mut [u64], nb_uls: u64, file: &mut File) {
for slot in data.iter_mut().take(nb_uls as usize) {
*slot = trusted_utils_read_ul(file);
}
}
pub fn trusted_utils_log(msg: &str) {
println!("c [TRUSTED_CORE] {}", msg);
}
pub fn trusted_utils_read_objs(data: &mut [u8], size: usize, nb_objs: usize, file: &mut File) {
let total = size * nb_objs;
file.read_exact(&mut data[..total]).unwrap();
}
pub fn trusted_utils_read_ints(data: &mut [i32], nb_ints: u64, file: &mut File) {
for slot in data.iter_mut().take(nb_ints as usize) {
*slot = trusted_utils_read_int(file);
}
}
pub fn trusted_utils_write_uls(data: &[u64], nb_uls: u64, file: &mut File) {
for v in data.iter().take(nb_uls as usize) {
let _ = file.write_all(&v.to_le_bytes());
}
}
pub fn trusted_utils_realloc<T: Clone + Default>(from: &mut [T], new_size: u64) -> Vec<T> {
let mut v = from.to_vec();
v.resize(new_size as usize, T::default());
v
}
pub fn trusted_utils_write_bool(b: bool, file: &mut File) {
let _ = file.write_all(&[if b { 1 } else { 0 }]);
}
pub fn trusted_utils_try_match_flag(arg: &str, opt: &str, out: &mut bool) {
if arg.starts_with(opt) {
*out = true;
}
}
pub fn trusted_utils_exit_eof() {
std::process::exit(0);
}
pub fn trusted_utils_write_char(c: char, file: &mut File) {
let _ = file.write_all(&[c as u8]);
}
pub fn trusted_utils_calloc<T>(_nb_objs: u64, _size_per_obj: u64) -> Vec<T> {
Vec::new()
}
pub fn trusted_utils_read_bool(file: &mut File) -> bool {
let mut buf = [0u8; 1];
file.read_exact(&mut buf).unwrap();
buf[0] != 0
}
pub fn trusted_utils_read_int(file: &mut File) -> i32 {
let mut buf = [0u8; 4];
file.read_exact(&mut buf).unwrap();
i32::from_le_bytes(buf)
}
pub fn trusted_utils_str_to_sig(s: &str, out: &mut [u8]) -> bool {
if s.len() != SIG_SIZE_BYTES * 2 {
return false;
}
for i in 0..SIG_SIZE_BYTES {
let part = &s[2 * i..2 * i + 2];
match u8::from_str_radix(part, 16) {
Ok(v) => out[i] = v,
Err(_) => return false,
}
}
true
}
pub fn trusted_utils_write_ul(u: u64, file: &mut File) {
let _ = file.write_all(&u.to_le_bytes());
}
pub fn trusted_utils_read_char(file: &mut File) -> i32 {
let mut buf = [0u8; 1];
file.read_exact(&mut buf).unwrap();
buf[0] as i32
}
