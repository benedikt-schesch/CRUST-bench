use std::fs::File;
use std::io::{Read, Write};
pub struct IntVec;
pub struct TrustedParser {
f_out: File,
f: File,
}
impl TrustedParser {
pub fn tp_end(&mut self) {}
pub fn output_literal_buffer(&self) {}
pub fn append_integer(&self) {}
pub fn tp_init(filename: &str, out: File) -> Self {
let f = File::open(filename).unwrap_or_else(|_| File::create(filename).unwrap());
Self { f_out: out, f }
}
pub fn tp_parse(&mut self, sig: &mut Option<Vec<u8>>) -> bool {
let mut buf = Vec::new();
let _ = self.f.read_to_end(&mut buf);
*sig = Some(vec![0u8; 16]);
let _ = self.f_out.write_all(&[0u8; 16]);
true
}
pub fn process(&mut self, _c: char) -> bool {
false
}
}
