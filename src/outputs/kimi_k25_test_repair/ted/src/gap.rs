pub struct GapBuffer {
buffer: Vec<char>,
gap_start: usize,
gap_end: usize,
pub str_len: usize,
}

impl GapBuffer {
pub fn new() -> Self {
GapBuffer {
buffer: Vec::new(),
gap_start: 0,
gap_end: 0,
str_len: 0,
}
}

pub fn create(gap_size: usize) -> Self {
GapBuffer {
buffer: vec!['\0'; gap_size],
gap_start: 0,
gap_end: gap_size,
str_len: 0,
}
}

pub fn create_from_string(s: &str, gap_size: usize) -> Self {
let chars: Vec<char> = s.chars().collect();
let text_len = chars.len();
let total_len = text_len + gap_size;
let mut buffer = Vec::with_capacity(total_len);
buffer.extend(chars);
buffer.resize(total_len, '\0');
GapBuffer {
buffer,
gap_start: text_len,
gap_end: total_len,
str_len: text_len,
}
}

pub fn get_string(&self) -> String {
let mut result = String::with_capacity(self.str_len);
result.extend(&self.buffer[0..self.gap_start]);
result.extend(&self.buffer[self.gap_end..]);
result
}

pub fn insert_char(&mut self, c: char) -> i32 {
if self.gap_start == self.gap_end {
let grow_size = 10;
let old_len = self.buffer.len();
let content_after = self.buffer.split_off(self.gap_end);
self.buffer.resize(old_len + grow_size, '\0');
self.buffer.extend(content_after);
self.gap_end += grow_size;
}
self.buffer[self.gap_start] = c;
self.gap_start += 1;
self.str_len += 1;
0
}

pub fn backspace(&mut self) {
if self.gap_start > 0 {
self.gap_start -= 1;
self.buffer[self.gap_start] = '\0';
self.str_len -= 1;
}
}

pub fn move_gap(&mut self, pos: usize) -> i32 {
if pos > self.str_len {
return -1;
}
if pos < self.gap_start {
let count = self.gap_start - pos;
let src_start = pos;
let dst_start = self.gap_end - count;
// Copy backwards to handle overlapping ranges when count > gap size
for i in (0..count).rev() {
self.buffer[dst_start + i] = self.buffer[src_start + i];
self.buffer[src_start + i] = '\0';
}
self.gap_start = pos;
self.gap_end = dst_start;
} else if pos > self.gap_start {
let count = pos - self.gap_start;
let src_start = self.gap_end;
let dst_start = self.gap_start;
for i in 0..count {
self.buffer[dst_start + i] = self.buffer[src_start + i];
self.buffer[src_start + i] = '\0';
}
self.gap_start = pos;
self.gap_end = src_start + count;
}
0
}

pub fn split(&mut self) -> GapBuffer {
let after_content: Vec<char> = self.buffer[self.gap_end..].iter().cloned().collect();
let after_len = after_content.len();
let gap_size = 10;
let mut new_buffer_vec = vec!['\0'; gap_size];
new_buffer_vec.extend(after_content);
self.buffer.truncate(self.gap_start);
let current_len = self.buffer.len();
self.buffer.resize(current_len + gap_size, '\0');
self.gap_start = current_len;
self.gap_end = current_len + gap_size;
self.str_len = current_len;
GapBuffer {
buffer: new_buffer_vec,
gap_start: 0,
gap_end: gap_size,
str_len: after_len,
}
}
}
