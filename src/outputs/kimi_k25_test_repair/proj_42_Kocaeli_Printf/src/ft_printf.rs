use std::io::{self, Write};
use std::ffi::c_void;
pub const HEXALOW: char = 'x';
pub const HEXAUP: char = 'X';
pub const DECIMAL: char = 'd';
pub const LOCATION: char = 'p';
pub fn writeint(n: i32, len: &mut usize) -> usize {
let s = n.to_string();
match io::stdout().write_all(s.as_bytes()) {
Ok(_) => {
*len += s.len();
1
}
Err(_) => 0,
}
}
pub fn writeuint(n: u32, len: &mut usize) -> usize {
let s = n.to_string();
match io::stdout().write_all(s.as_bytes()) {
Ok(_) => {
*len += s.len();
1
}
Err(_) => 0,
}
}
pub fn writehex(n: usize, case: char, len: &mut usize) -> usize {
let s = if case == 'X' {
format!("{:X}", n)
} else {
format!("{:x}", n)
};
match io::stdout().write_all(s.as_bytes()) {
Ok(_) => {
*len += s.len();
1
}
Err(_) => 0,
}
}
pub fn writepoint(p: *const c_void, len: &mut usize) -> usize {
let s = format!("{:p}", p);
match io::stdout().write_all(s.as_bytes()) {
Ok(_) => {
*len += s.len();
1
}
Err(_) => 0,
}
}
pub fn writechar(c: char, len: &mut usize) -> usize {
let mut buf = [0u8; 4];
let s = c.encode_utf8(&mut buf);
match io::stdout().write_all(s.as_bytes()) {
Ok(_) => {
*len += s.len();
1
}
Err(_) => 0,
}
}
pub fn writestring(s: &str, len: &mut usize) -> usize {
match io::stdout().write_all(s.as_bytes()) {
Ok(_) => {
*len += s.len();
1
}
Err(_) => 0,
}
}
