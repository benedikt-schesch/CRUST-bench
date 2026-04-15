
use crate::buffer::TextBuffer;
use crate::defs::panic;
#[derive(Clone, Default)]
pub struct Cursor {
x: usize,
y: usize,
}
#[derive(Clone, Default)]
pub struct VirtualScreen {
buffer: Vec<char>,
buf_pos: usize,
len: usize,
cursor: Cursor,
width: usize,
height: usize,
render_start_line: usize,
}
impl VirtualScreen {
pub fn screen_append(&mut self, str_: &str, size: usize) {
let chars: Vec<char> = str_.chars().collect();
let write_size = size.min(chars.len());
if self.buffer.is_empty() {
self.buffer.extend(chars.iter().take(write_size).copied());
self.buf_pos = write_size;
self.len = self.buffer.len();
return;
}
if (self.len.saturating_sub(self.buf_pos)) > write_size {
for i in 0..write_size {
if self.buf_pos + i < self.buffer.len() {
self.buffer[self.buf_pos + i] = chars[i];
} else {
self.buffer.push(chars[i]);
}
}
self.buf_pos += write_size;
self.len = self.buffer.len();
} else {
for ch in chars.iter().take(write_size) {
self.buffer.push(*ch);
}
self.buf_pos = self.buffer.len();
self.len = self.buffer.len();
}
}
pub fn required_screen_rows(line_length: usize, screen_width: usize) -> i32 {
if line_length == 0 {
1
} else if screen_width == 0 {
0
} else {
((line_length / screen_width) + usize::from((line_length % screen_width) > 0)) as i32
}
}
pub fn move_cursor_in_view(buffer: &TextBuffer, screen: &mut VirtualScreen) {
let buffer_cursor_x = buffer.cursor_row;
let mut cumul_req_rows: usize = 0;
let mut cur_line = screen.render_start_line;
if buffer_cursor_x < screen.render_start_line {
screen.render_start_line = buffer_cursor_x;
} else {
while cur_line <= buffer.last_line_loc {
let cur_line_required_rows = Self::required_screen_rows(
buffer.lines[cur_line].as_ref().map(|g| g.str_len).unwrap_or(0),
screen.width,
) as usize;
if (cur_line_required_rows + cumul_req_rows) > screen.height.saturating_sub(1) {
cur_line = cur_line.saturating_sub(1);
break;
}
cumul_req_rows += cur_line_required_rows;
cur_line += 1;
}
if buffer_cursor_x > cur_line {
let mut rows_required = 0usize;
while cur_line <= buffer_cursor_x {
rows_required += Self::required_screen_rows(
buffer.lines[cur_line].as_ref().map(|g| g.str_len).unwrap_or(0),
screen.width,
) as usize;
cur_line += 1;
}
while rows_required > 0 {
rows_required = rows_required.saturating_sub(
Self::required_screen_rows(
buffer.lines[screen.render_start_line]
.as_ref()
.map(|g| g.str_len)
.unwrap_or(0),
screen.width,
) as usize,
);
screen.render_start_line += 1;
}
}
}
}
pub fn draw_editor_window(buffer: &TextBuffer, screen: &mut VirtualScreen) {
let mut cur_line = screen.render_start_line;
let mut lines_written = 0usize;
while cur_line <= buffer.last_line_loc && lines_written < screen.height.saturating_sub(1) {
let screen_cols = screen.width;
let line = match buffer.get_line(cur_line) {
Some(v) => v,
None => panic("draw editor cant get text of current line in buffer"),
};
if line.chars().count() > screen_cols {
let chars: Vec<char> = line.chars().collect();
let mut i = 0usize;
loop {
let remaining = chars.len().saturating_sub(i);
let len_to_write = screen_cols.min(remaining);
let segment: String = chars[i..i + len_to_write].iter().collect();
screen.screen_append(&segment, len_to_write);
screen.screen_append("\r\n", 2);
screen.screen_append("\x1b[K", 3);
i += len_to_write;
lines_written += 1;
if lines_written == screen.height.saturating_sub(2) {
break;
}
if i >= chars.len().saturating_sub(1) {
break;
}
}
} else {
screen.screen_append(&line, line.chars().count());
screen.screen_append("\r\n", 2);
lines_written += 1;
}
cur_line += 1;
}
while lines_written < screen.height.saturating_sub(2) {
screen.screen_append("\r\n", 2);
lines_written += 1;
}
}
pub fn set_virtual_cursor_position(buffer: &TextBuffer, screen: &mut VirtualScreen) {
let mut current_line = screen.render_start_line;
let mut virtual_cursor_row = 1usize;
while current_line != buffer.cursor_row {
let required_rows = Self::required_screen_rows(
buffer.lines[current_line].as_ref().map(|g| g.str_len).unwrap_or(0),
screen.width,
) as usize;
virtual_cursor_row += required_rows;
current_line += 1;
}
if screen.width != 0 {
virtual_cursor_row += buffer.cursor_col / screen.width;
screen.cursor.x = virtual_cursor_row;
screen.cursor.y = (buffer.cursor_col % screen.width) + 1;
} else {
screen.cursor.x = virtual_cursor_row;
screen.cursor.y = 1;
}
}
}
