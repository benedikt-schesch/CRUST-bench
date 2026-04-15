pub mod buffer {
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PgnBuffer {
value: String,
}
impl PgnBuffer {
pub fn new() -> Self {
Self {
value: String::new(),
}
}
pub fn append(&mut self, ch: char) {
self.value.push(ch);
}
pub fn append_null_terminator(&mut self) {}
pub fn as_str(&self) -> &str {
&self.value
}
pub fn reset(&mut self) {
self.value.clear();
}
}
}
pub mod cursor {
pub fn pgn_cursor_skip_whitespace(s: &str, cursor: &mut usize) -> bool {
let bytes = s.as_bytes();
let start = *cursor;
while bytes
.get(*cursor)
.map(|b| b.is_ascii_whitespace())
.unwrap_or(false)
{
*cursor += 1;
}
*cursor > start
}
pub fn pgn_cursor_revisit_whitespace(s: &str, cursor: &mut usize) {
let bytes = s.as_bytes();
while *cursor > 0
&& bytes
.get(*cursor - 1)
.map(|b| b.is_ascii_whitespace())
.unwrap_or(false)
{
*cursor -= 1;
}
}
pub fn pgn_cursor_skip_newline(s: &str, cursor: &mut usize) -> bool {
let bytes = s.as_bytes();
let start = *cursor;
if bytes.get(*cursor).copied() == Some(b'\r') {
*cursor += 1;
}
if bytes.get(*cursor).copied() == Some(b'\n') {
*cursor += 1;
}
*cursor > start
}
}
