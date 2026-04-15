use crate::utils::cursor::pgn_cursor_skip_whitespace;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PgnCommentPosition {
Unknown = 0,
BeforeMove,
BetweenMove,
AfterMove,
AfterAlternative,
}
const PGN_COMMENTS_INITIAL_SIZE: usize = 1;
const PGN_COMMENTS_GROW_SIZE: usize = 1;
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PgnComment {
pub position: PgnCommentPosition,
pub value: String,
}
impl PgnComment {
pub fn new() -> Self {
Self {
position: PgnCommentPosition::Unknown,
value: String::new(),
}
}
pub fn value(&self) -> &str {
&self.value
}
pub fn cleanup(&mut self) {
self.value.clear();
self.position = PgnCommentPosition::Unknown;
}
pub fn from_string(str: &str, consumed: &mut usize) -> Self {
let bytes = str.as_bytes();
let mut cursor = 0usize;
let mut comment = PgnComment::new();
assert_eq!(bytes.get(cursor).copied(), Some(b'{'));
cursor += 1;
let mut left_brace_count = 1u32;
let mut right_brace_count = 0u32;
loop {
let ch = *bytes.get(cursor).expect("unterminated comment");
if ch == b'{' {
left_brace_count += 1;
}
if ch == b'}' {
right_brace_count += 1;
}
if right_brace_count == left_brace_count {
break;
}
comment.value.push(ch as char);
cursor += 1;
}
assert_eq!(bytes.get(cursor).copied(), Some(b'}'));
cursor += 1;
*consumed += cursor;
comment
}
}
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PgnComments {
pub values: Vec<PgnComment>,
}
impl PgnComments {
pub fn new() -> Self {
let _ = PGN_COMMENTS_INITIAL_SIZE;
let _ = PGN_COMMENTS_GROW_SIZE;
Self { values: Vec::new() }
}
pub fn push(&mut self, comment: PgnComment) {
self.values.push(comment);
}
pub fn poll(&mut self, pos: PgnCommentPosition, str: &str) -> usize {
let bytes = str.as_bytes();
let mut cursor = 0usize;
if bytes.get(cursor).copied() == Some(b'{') {
while bytes.get(cursor).copied() == Some(b'{') {
let mut local_consumed = 0usize;
let mut comment = PgnComment::from_string(&str[cursor..], &mut local_consumed);
cursor += local_consumed;
comment.position = pos;
self.push(comment);
pgn_cursor_skip_whitespace(str, &mut cursor);
}
assert_ne!(bytes.get(cursor).copied(), Some(b'{'));
assert_ne!(bytes.get(cursor).copied(), Some(b'}'));
pgn_cursor_skip_whitespace(str, &mut cursor);
}
cursor
}
pub fn get_first_after_alternative_index(&self) -> Option<usize> {
self.values
.iter()
.position(|c| c.position == PgnCommentPosition::AfterAlternative)
}
pub fn cleanup(self) {}
}
