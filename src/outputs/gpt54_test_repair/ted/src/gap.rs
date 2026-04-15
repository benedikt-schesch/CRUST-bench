
pub const MEM_ERROR: i32 = 128;
#[derive(Clone)]
pub struct GapBuffer {
pub buffer: Vec<char>,
pub str_len: usize,
pub gap_len: usize,
pub gap_loc: usize,
}
impl GapBuffer {
fn resize_buffer(&mut self, new_capacity: usize) -> i32 {
let buffer_size = self.str_len + self.gap_len;
let final_capacity = if new_capacity < buffer_size {
buffer_size
} else {
new_capacity
};
let gap_size = self.gap_len + (final_capacity - buffer_size);
let mut new_buffer = vec!['\0'; final_capacity];
for i in 0..self.gap_loc {
new_buffer[i] = self.buffer[i];
}
let tail_len = self.str_len.saturating_sub(self.gap_loc);
let old_tail_start = self.gap_loc + self.gap_len;
let new_tail_start = self.gap_loc + gap_size;
for i in 0..tail_len {
new_buffer[new_tail_start + i] = self.buffer[old_tail_start + i];
}
self.buffer = new_buffer;
self.gap_len = gap_size;
0
}
pub fn create(capacity: usize) -> Self {
Self {
buffer: vec!['\0'; capacity],
str_len: 0,
gap_len: capacity,
gap_loc: 0,
}
}
pub fn destroy(self) {}
pub fn insert_char(&mut self, ch: char) -> i32 {
if self.gap_len <= 1 {
let current_cap = self.gap_len + self.str_len;
let new_cap = if current_cap == 0 { 1 } else { current_cap * 2 };
let err = self.resize_buffer(new_cap);
if err != 0 {
return err;
}
}
self.buffer[self.gap_loc] = ch;
self.str_len += 1;
self.gap_loc += 1;
self.gap_len -= 1;
0
}
pub fn backspace(&mut self) {
if self.gap_loc > 0 {
self.gap_loc -= 1;
self.gap_len += 1;
self.str_len -= 1;
}
}
pub fn move_gap(&mut self, location: usize) -> i32 {
let location = location.min(self.str_len);
let capacity = self.gap_len + self.str_len;
let mut new_buffer = vec!['\0'; capacity];
if location < self.gap_loc {
for i in 0..location {
new_buffer[i] = self.buffer[i];
}
let moved = self.gap_loc - location;
for i in 0..moved {
new_buffer[location + self.gap_len + i] = self.buffer[location + i];
}
let tail_len = self.str_len.saturating_sub(self.gap_loc);
for i in 0..tail_len {
new_buffer[location + self.gap_len + moved + i] =
self.buffer[self.gap_loc + self.gap_len + i];
}
} else {
for i in 0..self.gap_loc {
new_buffer[i] = self.buffer[i];
}
let moved = location - self.gap_loc;
for i in 0..moved {
new_buffer[self.gap_loc + i] = self.buffer[self.gap_loc + self.gap_len + i];
}
let tail_len = self.str_len.saturating_sub(location);
for i in 0..tail_len {
new_buffer[location + self.gap_len + i] =
self.buffer[self.gap_loc + self.gap_len + moved + i];
}
}
self.buffer = new_buffer;
self.gap_loc = location;
0
}
pub fn get_string(&self) -> String {
let mut s = String::with_capacity(self.str_len);
for i in 0..self.gap_loc {
s.push(self.buffer[i]);
}
for i in 0..(self.str_len.saturating_sub(self.gap_loc)) {
s.push(self.buffer[self.gap_loc + self.gap_len + i]);
}
s
}
pub fn split(&mut self) -> Self {
let capacity = self.gap_len + self.str_len;
let second_half_of_str_len = self.str_len.saturating_sub(self.gap_loc);
let mut new_gap_buffer = Self::create(capacity);
let start = capacity - second_half_of_str_len;
for i in 0..second_half_of_str_len {
new_gap_buffer.buffer[start + i] = self.buffer[self.gap_loc + self.gap_len + i];
}
new_gap_buffer.str_len = second_half_of_str_len;
new_gap_buffer.gap_loc = 0;
new_gap_buffer.gap_len = capacity - second_half_of_str_len;
self.str_len = self.gap_loc;
self.gap_len = capacity - self.str_len;
new_gap_buffer
}
pub fn create_from_string(s: &str, gap_len: usize) -> Self {
if s.is_empty() {
Self::create(gap_len)
} else {
let chars: Vec<char> = s.chars().collect();
let s_len = chars.len();
let capacity = s_len + gap_len;
let mut new_buffer = Self::create(capacity);
for (i, ch) in chars.iter().enumerate() {
new_buffer.buffer[i] = *ch;
}
new_buffer.str_len = s_len;
new_buffer.gap_loc = s_len;
new_buffer.gap_len = gap_len;
new_buffer
}
}
pub fn char_at(&self, i: usize) -> char {
if self.str_len == 0 || i >= self.str_len {
'\0'
} else if i < self.gap_loc {
self.buffer[i]
} else {
self.buffer[i + self.gap_len]
}
}
}
