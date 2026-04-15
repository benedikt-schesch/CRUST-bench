
pub struct TextBuffer {
content: String,
pub cursor_row: usize,
pub cursor_col: usize,
}
impl TextBuffer {
pub fn new() -> Self {
TextBuffer {
content: String::new(),
cursor_row: 0,
cursor_col: 0,
}
}
pub fn create(rows: usize, cols: usize) -> Result<Self, &'static str> {
if rows == 0 || cols == 0 {
return Err("Rows and columns must be greater than zero");
}
Ok(TextBuffer {
content: String::with_capacity(rows * cols),
cursor_row: 0,
cursor_col: 0,
})
}
fn get_linear_position(&self) -> Result<usize, &'static str> {
let lines: Vec<&str> = self.content.split('\n').collect();
if self.cursor_row > lines.len() {
return Err("Row out of bounds");
}
let mut pos = 0;
for i in 0..self.cursor_row {
pos += lines[i].len() + 1; 
}
if self.cursor_row < lines.len() && self.cursor_col > lines[self.cursor_row].len() {
return Err("Column out of bounds");
}
pos += self.cursor_col;
Ok(pos)
}
pub fn get_line(&self, row: usize) -> Option<String> {
let lines: Vec<&str> = self.content.split('\n').collect();
if row < lines.len() {
Some(lines[row].to_string())
} else if row == lines.len() && (self.content.is_empty() || self.content.ends_with('\n')) {
Some(String::new())
} else {
None
}
}
pub fn insert(&mut self, c: char) -> i32 {
match self.get_linear_position() {
Ok(pos) => {
self.content.insert(pos, c);
self.cursor_col += 1;
0
}
Err(_) => -1,
}
}
pub fn backspace(&mut self) {
if self.cursor_col > 0 {
if let Ok(pos) = self.get_linear_position() {
self.content.remove(pos - 1);
self.cursor_col -= 1;
}
} else if self.cursor_row > 0 {
let lines: Vec<&str> = self.content.split('\n').collect();
if self.cursor_row <= lines.len() {
let prev_line_len = lines[self.cursor_row - 1].len();
if let Ok(pos) = self.get_linear_position() {
self.content.remove(pos - 1);
self.cursor_row -= 1;
self.cursor_col = prev_line_len;
}
}
}
}
pub fn move_cursor(&mut self, row: usize, col: usize) {
self.cursor_row = row;
self.cursor_col = col;
}
pub fn new_line(&mut self) -> i32 {
match self.get_linear_position() {
Ok(pos) => {
self.content.insert(pos, '\n');
self.cursor_row += 1;
self.cursor_col = 0;
0
}
Err(_) => -1,
}
}
}
