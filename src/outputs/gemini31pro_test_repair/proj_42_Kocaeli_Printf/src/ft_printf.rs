// Generated Rust Code

pub const HEXALOW: &str = "0123456789abcdef";
pub const DECIMAL: &str = "0123456789";
pub const HEXAUP: &str = "0123456789ABCDEF";
pub const LOCATION: &str = "0123456789abcdef";

pub fn writeint<T: std::fmt::Display, U: std::ops::DerefMut<Target = i32>>(_n: T, mut _len: U) -> i32 {
let s = format!("{}", _n);
print!("{}", s);
*_len += s.len() as i32;
1
}

pub fn writeuint<T: std::fmt::Display, U: std::ops::DerefMut<Target = i32>>(_n: T, mut _len: U) -> i32 {
let s = format!("{}", _n);
print!("{}", s);
*_len += s.len() as i32;
1
}

pub fn writehex<T: std::fmt::LowerHex + std::fmt::UpperHex, U: std::fmt::Display, V: std::ops::DerefMut<Target = i32>>(_n: T, _base: U, mut _len: V) -> i32 {
let base = format!("{}", _base);
let s = if base == "x" {
format!("{:x}", _n)
} else {
format!("{:X}", _n)
};
print!("{}", s);
*_len += s.len() as i32;
1
}

pub fn writepoint<T: std::fmt::Pointer, U: std::ops::DerefMut<Target = i32>>(_n: T, mut _len: U) -> i32 {
let s = format!("{:p}", _n);
print!("{}", s);
*_len += s.len() as i32;
1
}

pub fn writechar<T: std::fmt::Display, U: std::ops::DerefMut<Target = i32>>(_c: T, mut _len: U) -> i32 {
let s = format!("{}", _c);
print!("{}", s);
*_len += s.len() as i32;
1
}

pub fn writestring<T: std::fmt::Display, U: std::ops::DerefMut<Target = i32>>(_s: T, mut _len: U) -> i32 {
let s = format!("{}", _s);
print!("{}", s);
*_len += s.len() as i32;
1
}
