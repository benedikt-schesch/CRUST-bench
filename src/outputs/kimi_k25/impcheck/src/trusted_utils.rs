use std::io::{Read, Write};

pub const SIG_SIZE_BYTES: usize = 64;

pub const TRUSTED_CHK_INIT: u8 = 0;
pub const TRUSTED_CHK_LOAD: u8 = 1;
pub const TRUSTED_CHK_END_LOAD: u8 = 2;
pub const TRUSTED_CHK_TERMINATE: u8 = 3;
pub const TRUSTED_CHK_CLS_PRODUCE: u8 = 4;
pub const TRUSTED_CHK_CLS_IMPORT: u8 = 5;
pub const TRUSTED_CHK_CLS_DELETE: u8 = 6;
pub const TRUSTED_CHK_VALIDATE_SAT: u8 = 7;
pub const TRUSTED_CHK_VALIDATE_UNSAT: u8 = 8;

pub fn trusted_utils_read_int<R: Read>(reader: &mut R) -> i32 {
let mut buf = [0u8; 4];
reader.read_exact(&mut buf).expect("Failed to read int");
i32::from_le_bytes(buf)
}

pub fn trusted_utils_write_char<W: Write>(val: u8, writer: &mut W) {
writer.write_all(&[val]).expect("Failed to write char");
}

pub fn trusted_utils_write_int<W: Write>(val: i32, writer: &mut W) {
writer.write_all(&val.to_le_bytes()).expect("Failed to write int");
}

pub fn trusted_utils_write_sig<W: Write>(sig: &[u8], writer: &mut W) {
writer.write_all(sig).expect("Failed to write sig");
}

pub fn trusted_utils_write_ints<W: Write>(vals: &[i32], len: u64, writer: &mut W) {
for i in 0..len as usize {
if i < vals.len() {
trusted_utils_write_int(vals[i], writer);
}
}
}

pub fn trusted_utils_write_ul<W: Write>(val: u64, writer: &mut W) {
writer.write_all(&val.to_le_bytes()).expect("Failed to write ul");
}

pub fn trusted_utils_write_uls<W: Write>(vals: &[u64], len: u64, writer: &mut W) {
for i in 0..len as usize {
if i < vals.len() {
trusted_utils_write_ul(vals[i], writer);
}
}
}

pub fn trusted_utils_write_bool<W: Write>(val: bool, writer: &mut W) {
let byte = if val { 1u8 } else { 0u8 };
writer.write_all(&[byte]).expect("Failed to write bool");
}

pub fn trusted_utils_read_sig<R: Read>(buf: &mut [u8], reader: &mut R) {
reader.read_exact(buf).expect("Failed to read sig");
}
