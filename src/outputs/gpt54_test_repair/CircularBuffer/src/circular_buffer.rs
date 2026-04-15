use std::vec::Vec;

pub struct CircularBuffer {
size: usize,
data_size: usize,
tail_offset: usize,
head_offset: usize,
buffer: Vec<u8>,
}

impl CircularBuffer {
pub fn new(size: usize) -> Self {
let mut buf = CircularBuffer {
size,
data_size: 0,
tail_offset: usize::MAX,
head_offset: usize::MAX,
buffer: vec![0; size],
};
buf.reset();
buf
}

pub fn pop(&mut self, length: usize, data_out: &mut [u8]) -> usize {
self.inter_read(length, data_out, true)
}

pub fn read(&self, length: usize, data_out: &mut [u8]) -> usize {
let mut clone = CircularBuffer {
size: self.size,
data_size: self.data_size,
tail_offset: self.tail_offset,
head_offset: self.head_offset,
buffer: self.buffer.clone(),
};
clone.inter_read(length, data_out, false)
}

pub fn get_size(&self) -> usize {
self.size
}

pub fn print(&self, hex: bool) {
let c_size = self.get_size();
let mut out = String::new();
for i in 0..c_size {
let c = if self.get_data_size() == 0 {
b'_'
} else if self.tail_offset < self.head_offset {
if i > self.tail_offset && i < self.head_offset {
b'_'
} else {
self.buffer[i]
}
} else if i > self.tail_offset || i < self.head_offset {
b'_'
} else {
self.buffer[i]
};
if hex {
out.push_str(&format!("{:02X}|", c));
} else {
out.push(c as char);
out.push('|');
}
}
println!(
"CircularBuffer: {} <size {} dataSize:{}>",
out,
self.get_size(),
self.get_data_size()
);
}

pub fn free(self) {}

pub fn reset(&mut self) {
self.head_offset = usize::MAX;
self.tail_offset = usize::MAX;
self.data_size = 0;
}

pub fn get_capacity(&self) -> usize {
self.size
}

pub fn push(&mut self, src: &[u8], length: usize) {
if length == 0 || self.size == 0 {
return;
}

let available = src.len().min(length);
if available == 0 {
return;
}

let mut writable_len = available;
let mut start = 0usize;

if writable_len > self.size {
start = writable_len - self.size;
writable_len = self.size;
}

let psrc = &src[start..start + writable_len];

if self.data_size == 0 {
self.head_offset = 0;
self.tail_offset = writable_len - 1;

let first = writable_len.min(self.size);
self.buffer[..first].copy_from_slice(&psrc[..first]);
self.data_size = writable_len;
return;
}

let old_data_size = self.data_size;

for &byte in psrc {
let next_tail = if self.tail_offset == usize::MAX {
0
} else {
(self.tail_offset + 1) % self.size
};

self.buffer[next_tail] = byte;
self.tail_offset = next_tail;

if self.data_size < self.size {
self.data_size += 1;
} else if self.head_offset != usize::MAX {
self.head_offset = (self.head_offset + 1) % self.size;
}
}

if old_data_size == 0 && self.head_offset == usize::MAX {
self.head_offset = 0;
}
}

pub fn inter_read(&mut self, length: usize, data_out: &mut [u8], reset_head: bool) -> usize {
if self.data_size == 0 || length == 0 {
return 0;
}

let rd_len = length.min(self.data_size);
let copy_len = rd_len.min(data_out.len());

if copy_len > 0 {
if self.head_offset + copy_len <= self.size {
data_out[..copy_len]
.copy_from_slice(&self.buffer[self.head_offset..self.head_offset + copy_len]);
} else {
let first_len = self.size - self.head_offset;
let second_len = copy_len - first_len;

data_out[..first_len]
.copy_from_slice(&self.buffer[self.head_offset..self.head_offset + first_len]);
data_out[first_len..copy_len].copy_from_slice(&self.buffer[..second_len]);
}
}

if reset_head {
if rd_len == self.data_size {
self.reset();
} else {
self.head_offset = (self.head_offset + rd_len) % self.size;
self.data_size -= rd_len;
}
}

rd_len
}

pub fn get_data_size(&self) -> usize {
self.data_size
}
}
