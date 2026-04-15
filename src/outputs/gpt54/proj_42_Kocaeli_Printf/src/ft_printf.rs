use std::any::Any;
use std::io::{self, Write};

// Constants
pub const HEXALOW: &str = "0123456789abcdef";
pub const DECIMAL: &str = "0123456789";
pub const HEXAUP: &str = "0123456789ABCDEF";
pub const LOCATION: i32 = 2;

// Function Declarations
pub fn writeuint(mut n: u64, len: &mut i32) -> i32 {
let decimal = DECIMAL.as_bytes();
let mut arr: [u8; 20] = [0; 20];
let mut i: usize = 0;

if n == 0 {
return writechar('0', len);
}

while n != 0 {
arr[i] = decimal[(n % 10) as usize];
i += 1;
n /= 10;
}

while i > 0 {
i -= 1;
if writechar(arr[i] as char, len) == -1 {
return -1;
}
}
1
}

pub fn format(args: &[Box<dyn Any>], c: char, len: &mut i32) -> i32 {
let arg = if args.is_empty() { None } else { Some(&args[0]) };

match c {
'c' => {
if let Some(a) = arg {
if let Some(v) = a.downcast_ref::<char>() {
return writechar(*v, len);
}
if let Some(v) = a.downcast_ref::<i32>() {
return writechar(char::from_u32(*v as u32).unwrap_or('\0'), len);
}
if let Some(v) = a.downcast_ref::<u8>() {
return writechar(*v as char, len);
}
}
-1
}
's' => {
if let Some(a) = arg {
if let Some(v) = a.downcast_ref::<String>() {
return writestring(v.as_str(), len);
}
if let Some(v) = a.downcast_ref::<&str>() {
return writestring(v, len);
}
}
writestring("(null)", len)
}
'd' | 'i' => {
if let Some(a) = arg {
if let Some(v) = a.downcast_ref::<i32>() {
return writeint(*v, len);
}
if let Some(v) = a.downcast_ref::<i16>() {
return writeint(*v as i32, len);
}
if let Some(v) = a.downcast_ref::<i64>() {
return writeint(*v as i32, len);
}
}
-1
}
'u' => {
if let Some(a) = arg {
if let Some(v) = a.downcast_ref::<u32>() {
return writeuint(*v as u64, len);
}
if let Some(v) = a.downcast_ref::<u64>() {
return writeuint(*v, len);
}
if let Some(v) = a.downcast_ref::<usize>() {
return writeuint(*v as u64, len);
}
}
-1
}
'p' => {
if let Some(a) = arg {
if let Some(v) = a.downcast_ref::<*const std::ffi::c_void>() {
return writepoint(*v, len);
}
if let Some(v) = a.downcast_ref::<usize>() {
let ptr = *v as *const std::ffi::c_void;
return writepoint(ptr, len);
}
}
if LOCATION == 2 {
writestring("(nil)", len)
} else {
writepoint(std::ptr::null(), len)
}
}
'x' | 'X' => {
if let Some(a) = arg {
if let Some(v) = a.downcast_ref::<u32>() {
return writehex(*v as u64, c, len);
}
if let Some(v) = a.downcast_ref::<u64>() {
return writehex(*v, c, len);
}
if let Some(v) = a.downcast_ref::<usize>() {
return writehex(*v as u64, c, len);
}
}
-1
}
'%' => writechar('%', len),
_ => -1,
}
}

pub fn writechar(c: char, len: &mut i32) -> i32 {
*len += 1;
let mut stdout = io::stdout();
let mut buf = [0u8; 4];
let s = c.encode_utf8(&mut buf);
match stdout.write_all(s.as_bytes()) {
Ok(_) => 1,
Err(_) => -1,
}
}

pub fn writepoint(n: *const std::ffi::c_void, len: &mut i32) -> i32 {
let mut arr: [u8; 32] = [0; 32];
let mut i: usize = 0;
let mut nb = n as usize as u64;
let hex = HEXALOW.as_bytes();

if LOCATION == 2 && n.is_null() {
return writestring("(nil)", len);
}
if writestring("0x", len) == -1 {
return -1;
}
if nb == 0 {
return writechar('0', len);
}
while nb != 0 {
arr[i] = hex[(nb % 16) as usize];
i += 1;
nb /= 16;
}
while i > 0 {
i -= 1;
if writechar(arr[i] as char, len) == -1 {
return -1;
}
}
1
}

pub fn writeint(mut n: i32, len: &mut i32) -> i32 {
let decimal = DECIMAL.as_bytes();
let mut arr: [u8; 10] = [0; 10];
let mut i: usize = 0;

if n == 0 {
return writechar('0', len);
}
if n == i32::MIN {
return writestring("-2147483648", len);
}
if n < 0 {
if writechar('-', len) == -1 {
return -1;
}
n = -n;
}
while n != 0 {
arr[i] = decimal[(n % 10) as usize];
i += 1;
n /= 10;
}
while i > 0 {
i -= 1;
if writechar(arr[i] as char, len) == -1 {
return -1;
}
}
1
}

pub fn writestring(s: &str, len: &mut i32) -> i32 {
for ch in s.chars() {
if writechar(ch, len) == -1 {
return -1;
}
}
1
}

pub fn writehex(mut n: u64, c: char, len: &mut i32) -> i32 {
let mut arr: [u8; 16] = [0; 16];
let mut i: usize = 0;
let hex = if c == 'x' {
HEXALOW.as_bytes()
} else {
HEXAUP.as_bytes()
};

if n == 0 {
arr[i] = b'0';
i += 1;
}
while n != 0 {
arr[i] = hex[(n % 16) as usize];
i += 1;
n /= 16;
}
while i > 0 {
i -= 1;
if writechar(arr[i] as char, len) == -1 {
return -1;
}
}
1
}

#[macro_export]
macro_rules! ft_printf {
($fmt:expr, $($arg:expr),*) => {{
let args_vec: Vec<Box<dyn std::any::Any>> = vec![$(Box::new($arg) as Box<dyn std::any::Any>),*];
let mut len = 0;
let mut chars = $fmt.chars();
let mut arg_index: usize = 0;
while let Some(c) = chars.next() {
if c == '%' {
if let Some(next_c) = chars.next() {
if next_c == '%' {
if crate::ft_printf::writechar('%', &mut len) == -1 {
return -1;
}
} else {
if arg_index >= args_vec.len() {
return -1;
}
if crate::ft_printf::format(&args_vec[arg_index..arg_index + 1], next_c, &mut len) == -1 {
return -1;
}
arg_index += 1;
}
}
} else if crate::ft_printf::writechar(c, &mut len) == -1 {
return -1;
}
}
len
}};
}
