use std::io::{self, Write};
use std::ffi::c_void;

pub const HEXALOW: char = 'x';
pub const HEXAUP: char = 'X';
pub const DECIMAL: char = 'd';
pub const LOCATION: char = 'p';

/// Writes an integer to stdout, updates `len` with bytes written, and returns 1 on success or 0 on failure
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

/// Writes an unsigned integer to stdout, updates `len` with bytes written, and returns 1 on success or 0 on failure
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

/// Writes a number in hexadecimal format to stdout, updates `len` with bytes written, and returns 1 on success or 0 on failure
/// `case` specifies 'x' for lowercase or 'X' for uppercase hex formatting
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

/// Writes a pointer address to stdout, updates `len` with bytes written, and returns 1 on success or 0 on failure
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

/// Writes a single character to stdout, updates `len` with bytes written, and returns 1 on success or 0 on failure
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

/// Writes a string to stdout, updates `len` with bytes written, and returns 1 on success or 0 on failure
pub fn writestring(s: &str, len: &mut usize) -> usize {
match io::stdout().write_all(s.as_bytes()) {
Ok(_) => {
*len += s.len();
1
}
Err(_) => 0,
}
}
