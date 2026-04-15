pub const AMP_VERSION: i16 = 1;

pub struct Amp {
pub version: i16,
pub argc: i16,
pub buf: String,
}

impl Amp {
fn read_u32_be(buf: &[u8]) -> u32 {
((buf[0] as u32) << 24)
| ((buf[1] as u32) << 16)
| ((buf[2] as u32) << 8)
| (buf[3] as u32)
}

pub fn decode(&mut self, buf: &str) {
let bytes = buf.as_bytes();
if bytes.is_empty() {
self.version = 0;
self.argc = 0;
self.buf.clear();
return;
}

self.version = (bytes[0] >> 4) as i16;
self.argc = (bytes[0] & 0x0f) as i16;
self.buf = String::from_utf8_lossy(&bytes[1..]).into_owned();
}

pub fn decode_arg(&mut self) -> &str {
let bytes = self.buf.as_bytes();
if bytes.len() < 4 {
self.buf.clear();
return "";
}

let len = Self::read_u32_be(&bytes[..4]) as usize;
if bytes.len() < 4 + len {
self.buf.clear();
return "";
}

let arg = String::from_utf8_lossy(&bytes[4..4 + len]).into_owned();
let remaining = String::from_utf8_lossy(&bytes[4 + len..]).into_owned();

self.buf = remaining;
Box::leak(arg.into_boxed_str())
}
}

pub fn amp_encode(argv: &[&str]) -> String {
let mut out = Vec::<u8>::new();
let argc = argv.len() as u8;
out.push(((AMP_VERSION as u8) << 4) | (argc & 0x0f));

for arg in argv {
let bytes = arg.as_bytes();
let len = bytes.len() as u32;
out.push(((len >> 24) & 0xff) as u8);
out.push(((len >> 16) & 0xff) as u8);
out.push(((len >> 8) & 0xff) as u8);
out.push((len & 0xff) as u8);
out.extend_from_slice(bytes);
}

String::from_utf8_lossy(&out).into_owned()
}
