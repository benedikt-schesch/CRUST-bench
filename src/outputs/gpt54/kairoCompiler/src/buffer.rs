pub const BUFFER_REALLOC_AMOUNT: usize = 2000;

#[derive(Debug, Default, Clone)]
pub struct Buffer {
pub data: Vec<u8>,
pub rindex: usize,
pub len: usize,
pub msize: usize,
}

pub fn buffer_create() -> Buffer {
Buffer {
data: vec![0; BUFFER_REALLOC_AMOUNT],
rindex: 0,
len: 0,
msize: BUFFER_REALLOC_AMOUNT,
}
}

pub fn buffer_read(buffer: &mut Buffer) -> char {
if buffer.rindex >= buffer.len {
return '\u{FFFF}';
}
let c = buffer.data[buffer.rindex] as char;
buffer.rindex += 1;
c
}

pub fn buffer_peek(buffer: &Buffer) -> char {
if buffer.rindex >= buffer.len {
return '\u{FFFF}';
}
buffer.data[buffer.rindex] as char
}

pub fn buffer_extend(buffer: &mut Buffer, size: usize) {
buffer.msize += size;
if buffer.data.len() < buffer.msize {
buffer.data.resize(buffer.msize, 0);
}
}

pub fn buffer_printf(buffer: &mut Buffer, fmt: &str) {
let bytes = fmt.as_bytes();
let index = buffer.len;
let len = 2048usize;
buffer_extend(buffer, len);
let actual_len = bytes.len();
let needed = index + actual_len + 1;
if buffer.data.len() < needed {
buffer.data.resize(needed, 0);
buffer.msize = buffer.data.len();
}
for (i, b) in bytes.iter().enumerate() {
buffer.data[index + i] = *b;
}
buffer.data[index + actual_len] = 0;
buffer.len += actual_len;
}

pub fn buffer_printf_no_terminator(buffer: &mut Buffer, fmt: &str) {
let bytes = fmt.as_bytes();
let index = buffer.len;
let len = 2048usize;
buffer_extend(buffer, len);
let actual_len = bytes.len();
let needed = index + actual_len + 1;
if buffer.data.len() < needed {
buffer.data.resize(needed, 0);
buffer.msize = buffer.data.len();
}
for (i, b) in bytes.iter().enumerate() {
buffer.data[index + i] = *b;
}
buffer.data[index + actual_len] = 0;
buffer.len += actual_len;
}

pub fn buffer_write(buffer: &mut Buffer, c: char) {
buffer_need(buffer, 1);
if buffer.len >= buffer.data.len() {
buffer.data.resize(buffer.len + 1, 0);
buffer.msize = buffer.data.len();
}
buffer.data[buffer.len] = c as u8;
buffer.len += 1;
}

pub fn buffer_need(buffer: &mut Buffer, size: usize) {
if buffer.msize <= buffer.len + size {
buffer_extend(buffer, size + BUFFER_REALLOC_AMOUNT);
}
}

pub fn buffer_ptr(buffer: &Buffer) -> &[u8] {
&buffer.data[..buffer.len.min(buffer.data.len())]
}

pub fn buffer_free(_buffer: Buffer) {}
