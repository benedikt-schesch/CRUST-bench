use std::fs::File;
use std::io::Write;
pub struct Writer {
file: File,
}
impl Writer {
pub fn write_char(&mut self, c_int: i32) {
let _ = self.file.write_all(&[c_int as u8]);
}
pub fn write_int(&mut self, i: i32) {
let _ = self.file.write_all(&i.to_le_bytes());
}
pub fn write_ints(&mut self, data: &[i32], nb_ints: u64) {
for v in data.iter().take(nb_ints as usize) {
let _ = self.file.write_all(&v.to_le_bytes());
}
}
pub fn write_sig(&mut self, sig: &[u8]) {
let _ = self.file.write_all(sig);
}
pub fn writer_init(output_path: &str) -> Self {
Self {
file: File::create(output_path).unwrap(),
}
}
pub fn write_bool(&mut self, b: bool) {
let _ = self.file.write_all(&[if b { 1 } else { 0 }]);
}
pub fn write_uls(&mut self, data: &[u64], nb_uls: u64) {
for v in data.iter().take(nb_uls as usize) {
let _ = self.file.write_all(&v.to_le_bytes());
}
}
pub fn write_ul(&mut self, ul: u64) {
let _ = self.file.write_all(&ul.to_le_bytes());
}
}
