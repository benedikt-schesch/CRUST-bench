
pub const AMP_VERSION: i16 = 1;
pub struct Amp {
pub version: i16,
pub argc: i16,
pub buf: String,
pub pos: usize,
}
impl Default for Amp {
fn default() -> Self {
Self {
version: 0,
argc: 0,
buf: String::new(),
pos: 0,
}
}
}
impl Amp {
pub fn new() -> Self {
Self::default()
}
pub fn decode(&mut self, buf: &str) {
if buf.is_empty() {
self.version = 0;
self.argc = 0;
self.buf = String::new();
self.pos = 0;
return;
}
let bytes = buf.as_bytes();
let header = bytes[0];
self.version = ((header >> 4) & 0x0f) as i16;
self.argc = (header & 0x0f) as i16;
self.buf = buf[1..].to_string();
self.pos = 0;
}
pub fn decode_arg(&mut self) -> &str {
let bytes = self.buf.as_bytes();
if self.pos + 4 > bytes.len() {
return "";
}
let len = ((bytes[self.pos] as u32) << 24 |
(bytes[self.pos + 1] as u32) << 16 |
(bytes[self.pos + 2] as u32) << 8 |
(bytes[self.pos + 3] as u32)) as usize;
let start = self.pos + 4;
let end = start + len;
if end > bytes.len() {
return "";
}
self.pos = end;
&self.buf[start..end]
}
}
pub fn amp_encode(argv: &[&str]) -> String {
let argc = argv.len();
if argc > 15 {
panic!("Too many arguments");
}
let mut result = String::new();
let header = ((AMP_VERSION as u8) << 4) | (argc as u8);
result.push(header as char);
for arg in argv {
let len = arg.len() as u32;
result.push(((len >> 24) & 0xff) as u8 as char);
result.push(((len >> 16) & 0xff) as u8 as char);
result.push(((len >> 8) & 0xff) as u8 as char);
result.push((len & 0xff) as u8 as char);
result.push_str(arg);
}
result
}
