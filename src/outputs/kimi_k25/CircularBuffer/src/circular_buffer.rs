use std::collections::VecDeque;

pub struct CircularBuffer<T> {
buffer: VecDeque<T>,
capacity: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
EmptyBuffer,
FullBuffer,
}

impl<T> CircularBuffer<T> {
pub fn new(capacity: usize) -> Self {
CircularBuffer {
buffer: VecDeque::with_capacity(capacity),
capacity,
}
}

pub fn write(&mut self, element: T) -> Result<(), Error> {
if self.buffer.len() == self.capacity {
Err(Error::FullBuffer)
} else {
self.buffer.push_back(element);
Ok(())
}
}

pub fn read(&mut self) -> Result<T, Error> {
self.buffer.pop_front().ok_or(Error::EmptyBuffer)
}

pub fn clear(&mut self) {
self.buffer.clear();
}

pub fn overwrite(&mut self, element: T) {
if self.buffer.len() == self.capacity {
self.buffer.pop_front();
}
self.buffer.push_back(element);
}
}

// Implementation for byte-specific operations
impl CircularBuffer<u8> {
pub fn push(&mut self, data: &[u8], len: usize) -> Result<(), Error> {
if len > data.len() {
return Err(Error::FullBuffer);
}
if self.buffer.len() + len > self.capacity {
return Err(Error::FullBuffer);
}
for i in 0..len {
self.buffer.push_back(data[i]);
}
Ok(())
}

pub fn pop(&mut self, len: usize, buf: &mut [u8]) -> Result<usize, Error> {
if self.buffer.is_empty() {
return Err(Error::EmptyBuffer);
}
let to_read = std::cmp::min(len, buf.len());
let mut count = 0;
for i in 0..to_read {
if let Some(byte) = self.buffer.pop_front() {
buf[i] = byte;
count += 1;
} else {
break;
}
}
Ok(count)
}

// Renamed from `read` to `read_bytes` to avoid conflict with generic `read` method
pub fn read_bytes(&mut self, len: usize, buf: &mut [u8]) -> Result<usize, Error> {
self.pop(len, buf)
}

pub fn get_data_size(&self) -> usize {
self.buffer.len()
}
}
