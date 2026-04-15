
use std::fs::File;
use std::io::Write;
#[derive(Debug, Clone)]
pub struct CsvField {
pub data: String,
pub len: usize,
}
impl CsvField {
pub fn new() -> Self {
let len = 10;
Self {
data: String::new(),
len,
}
}
pub fn reset(&mut self) {
self.data.clear();
}
pub fn set(&mut self, buf: &str, buf_start_idx: usize, len: usize) {
const STR_MEM_PAD: usize = 10;
if len + 1 > self.len {
self.len = len + 1 + STR_MEM_PAD;
}
let start = buf_start_idx.min(buf.len());
let end = start.saturating_add(len).min(buf.len());
self.data = buf[start..end].to_string();
}
pub fn append(&mut self, buf: &str, buf_start_idx: usize, buflen: usize) {
const STR_MEM_PAD: usize = 10;
let origflen = self.data.len();
if origflen + buflen + 1 > self.len {
self.len = self.len + buflen + STR_MEM_PAD;
}
let start = buf_start_idx.min(buf.len());
let end = start.saturating_add(buflen).min(buf.len());
self.data.push_str(&buf[start..end]);
}
pub fn print_to_file(&self, fp: &mut File) {
let _ = writeln!(fp, "[{}:{}]", self.data, self.len as i32);
}
}
