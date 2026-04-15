
use crate::gap::GapBuffer;
use crate::gap::MEM_ERROR;
use std::io::BufRead;
use std::io::BufReader;
pub const DEFAULT_GAP_BUF_CAP: usize = 100;
pub const DEFAULT_CAPACITY: usize = 100;
pub struct TextBuffer {
pub lines: Vec<Option<GapBuffer>>,
pub lines_capacity: usize,
pub cursor_row: usize,
pub cursor_col: usize,
pub cursor_col_moved: bool,
pub last_line_loc: usize,
}
impl TextBuffer {
pub fn create(lines: usize, line_size: usize) -> Option<Self> {
if lines == 0 {
return None;
}
let mut text_buffer = Self {
lines: vec![None; lines],
lines_capacity: lines,
cursor_row: 0,
cursor_col: 0,
cursor_col_moved: false,
last_line_loc: 0,
};
text_buffer.lines[0] = Some(GapBuffer::create(line_size));
Some(text_buffer)
}
pub fn destroy(self) {}
pub fn move_cursor(&mut self, row: usize, col: usize) {
let row = row.min(self.last_line_loc);
self.cursor_row = row;
let line_len = self.lines[row].as_ref().map(|l| l.str_len).unwrap_or(0);
let col = col.min(line_len);
if self.cursor_col != col {
self.cursor_col_moved = true;
}
self.cursor_col = col;
}
pub fn insert(&mut self, ch: char) -> i32 {
if self.cursor_col_moved {
if let Some(line) = self.lines[self.cursor_row].as_mut() {
let err = line.move_gap(self.cursor_col);
if err != 0 {
return err;
}
}
self.cursor_col_moved = false;
}
if let Some(line) = self.lines[self.cursor_row].as_mut() {
let err = line.insert_char(ch);
if err != 0 {
return err;
}
self.cursor_col = line.gap_loc;
0
} else {
MEM_ERROR
}
}
pub fn backspace(&mut self) -> i32 {
if self.cursor_col_moved {
if let Some(line) = self.lines[self.cursor_row].as_mut() {
let err = line.move_gap(self.cursor_col);
if err != 0 {
return err;
}
}
self.cursor_col_moved = false;
}
if let Some(line) = self.lines[self.cursor_row].as_mut() {
line.backspace();
self.cursor_col = line.gap_loc;
0
} else {
MEM_ERROR
}
}
pub fn new_line(&mut self) -> i32 {
if self.cursor_col_moved {
if let Some(line) = self.lines[self.cursor_row].as_mut() {
let err = line.move_gap(self.cursor_col);
if err != 0 {
return err;
}
}
self.cursor_col_moved = false;
}
let newline = if let Some(line) = self.lines[self.cursor_row].as_mut() {
line.split()
} else {
return MEM_ERROR;
};
if self.last_line_loc == self.lines_capacity.saturating_sub(1) {
let new_capacity = if self.lines_capacity == 0 {
1
} else {
self.lines_capacity * 2
};
self.lines.resize(new_capacity, None);
self.lines_capacity = new_capacity;
}
let insert_index = self.cursor_row + 1;
let end_exclusive = self.last_line_loc + 1;
for i in (insert_index..end_exclusive).rev() {
self.lines[i + 1] = self.lines[i].take();
}
self.lines[insert_index] = Some(newline);
self.last_line_loc += 1;
self.cursor_row += 1;
self.cursor_col = self.lines[self.cursor_row]
.as_ref()
.map(|g| g.gap_loc)
.unwrap_or(0);
0
}
pub fn get_line(&self, row: usize) -> Option<String> {
if row > self.last_line_loc {
return None;
}
self.lines.get(row)?.as_ref().map(|g| g.get_string())
}
pub fn create_from_file(fp: &std::fs::File) -> Option<Self> {
let mut new_tbuffer = Self::create(DEFAULT_CAPACITY, DEFAULT_GAP_BUF_CAP)?;
new_tbuffer.lines[new_tbuffer.last_line_loc] = None;
let reader = BufReader::new(fp);
let mut any_line = false;
for line_result in reader.lines() {
let line = match line_result {
Ok(v) => v,
Err(_) => return None,
};
any_line = true;
if new_tbuffer.last_line_loc == new_tbuffer.lines_capacity.saturating_sub(1) {
let new_capacity = new_tbuffer.lines_capacity * 2;
new_tbuffer.lines.resize(new_capacity, None);
new_tbuffer.lines_capacity = new_capacity;
}
let read = line.chars().count() + 1;
let line_gap_size = if read * 2 < DEFAULT_GAP_BUF_CAP {
DEFAULT_GAP_BUF_CAP
} else {
read * 2
};
let idx = new_tbuffer.last_line_loc + 1;
new_tbuffer.lines[idx] = Some(GapBuffer::create_from_string(&line, line_gap_size));
new_tbuffer.last_line_loc += 1;
}
if !any_line {
new_tbuffer.last_line_loc = 0;
new_tbuffer.lines[0] = Some(GapBuffer::create(DEFAULT_GAP_BUF_CAP));
}
Some(new_tbuffer)
}
}
