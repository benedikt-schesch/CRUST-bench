use std::collections::HashMap;
use std::fmt;
#[derive(Debug, Clone, PartialEq)]
pub enum CJson {
Null,
Bool(bool),
Number(f64),
String(String),
Array(Vec<CJson>),
Object(HashMap<String, CJson>),
}
#[derive(Debug, Clone)]
pub enum CJsonError {
UnexpectedEOF { pos: usize },
UnexpectedToken { ch: char, pos: usize },
InvalidLiteral { expected: &'static str, pos: usize },
InvalidNumber { pos: usize },
InvalidEscape { pos: usize },
InvalidUnicodeEscape { pos: usize },
ExpectedColon { pos: usize },
ExpectedCommaOrEnd { pos: usize },
}
impl fmt::Display for CJsonError {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
match self {
CJsonError::UnexpectedEOF { pos } => write!(f, "unexpected end of input at {}", pos),
CJsonError::UnexpectedToken { ch, pos } => {
write!(f, "unexpected token '{}' at {}", ch, pos)
}
CJsonError::InvalidLiteral { expected, pos } => {
write!(f, "invalid literal, expected '{}' at {}", expected, pos)
}
CJsonError::InvalidNumber { pos } => write!(f, "invalid number at {}", pos),
CJsonError::InvalidEscape { pos } => write!(f, "invalid escape at {}", pos),
CJsonError::InvalidUnicodeEscape { pos } => {
write!(f, "invalid unicode escape at {}", pos)
}
CJsonError::ExpectedColon { pos } => write!(f, "expected ':' at {}", pos),
CJsonError::ExpectedCommaOrEnd { pos } => {
write!(f, "expected ',' or end delimiter at {}", pos)
}
}
}
}
impl std::error::Error for CJsonError {}
struct Parser<'a> {
input: &'a str,
pos: usize,
}
impl<'a> Parser<'a> {
fn new(input: &'a str) -> Self {
Self { input, pos: 0 }
}
fn peek(&self) -> Option<char> {
self.input[self.pos..].chars().next()
}
fn next_char(&mut self) -> Option<char> {
let ch = self.peek()?;
self.pos += ch.len_utf8();
Some(ch)
}
fn take_while<F>(&mut self, mut predicate: F) -> &'a str
where
F: FnMut(char) -> bool,
{
let start = self.pos;
while let Some(ch) = self.peek() {
if !predicate(ch) {
break;
}
self.pos += ch.len_utf8();
}
&self.input[start..self.pos]
}
fn skip_whitespace(&mut self) {
self.take_while(|c| (c as u32) <= 32);
}
fn expect_char(&mut self, expected: char) -> Result<(), CJsonError> {
match self.next_char() {
Some(ch) if ch == expected => Ok(()),
Some(ch) => Err(CJsonError::UnexpectedToken { ch, pos: self.pos }),
None => Err(CJsonError::UnexpectedEOF { pos: self.pos }),
}
}
fn parse_value(&mut self) -> Result<CJson, CJsonError> {
self.skip_whitespace();
match self.peek() {
Some('n') => self.parse_null(),
Some('f') | Some('t') => self.parse_bool(),
Some('"') => self.parse_string().map(CJson::String),
Some('[') => self.parse_array(),
Some('{') => self.parse_object(),
Some('-') | Some('0'..='9') => self.parse_number(),
Some(ch) => Err(CJsonError::UnexpectedToken { ch, pos: self.pos }),
None => Err(CJsonError::UnexpectedEOF { pos: self.pos }),
}
}
fn parse_null(&mut self) -> Result<CJson, CJsonError> {
let start = self.pos;
if self.input[self.pos..].starts_with("null") {
self.pos += 4;
Ok(CJson::Null)
} else {
Err(CJsonError::InvalidLiteral {
expected: "null",
pos: start,
})
}
}
fn parse_bool(&mut self) -> Result<CJson, CJsonError> {
let start = self.pos;
if self.input[self.pos..].starts_with("true") {
self.pos += 4;
Ok(CJson::Bool(true))
} else if self.input[self.pos..].starts_with("false") {
self.pos += 5;
Ok(CJson::Bool(false))
} else {
Err(CJsonError::InvalidLiteral {
expected: "true or false",
pos: start,
})
}
}
fn parse_number(&mut self) -> Result<CJson, CJsonError> {
let start = self.pos;
if self.peek() == Some('-') {
self.next_char();
}
match self.peek() {
Some('0') => {
self.next_char();
}
Some('1'..='9') => {
self.next_char();
self.take_while(|c| c.is_ascii_digit());
}
_ => {
return Err(CJsonError::InvalidNumber { pos: start });
}
}
if self.peek() == Some('.') {
let save = self.pos;
self.next_char();
if !matches!(self.peek(), Some('0'..='9')) {
return Err(CJsonError::InvalidNumber { pos: save });
}
self.take_while(|c| c.is_ascii_digit());
}
if matches!(self.peek(), Some('e') | Some('E')) {
let save = self.pos;
self.next_char();
if matches!(self.peek(), Some('+') | Some('-')) {
self.next_char();
}
if !matches!(self.peek(), Some('0'..='9')) {
return Err(CJsonError::InvalidNumber { pos: save });
}
self.take_while(|c| c.is_ascii_digit());
}
let num_str = &self.input[start..self.pos];
match num_str.parse::<f64>() {
Ok(n) => Ok(CJson::Number(n)),
Err(_) => Err(CJsonError::InvalidNumber { pos: start }),
}
}
fn parse_string(&mut self) -> Result<String, CJsonError> {
self.expect_char('"')?;
let mut out = String::new();
loop {
let ch = match self.next_char() {
Some(c) => c,
None => return Err(CJsonError::UnexpectedEOF { pos: self.pos }),
};
match ch {
'"' => return Ok(out),
'\\' => {
let esc_pos = self.pos;
let esc = self
.next_char()
.ok_or(CJsonError::UnexpectedEOF { pos: self.pos })?;
match esc {
'"' => out.push('"'),
'\\' => out.push('\\'),
'/' => out.push('/'),
'b' => out.push('\u{0008}'),
'f' => out.push('\u{000C}'),
'n' => out.push('\n'),
'r' => out.push('\r'),
't' => out.push('\t'),
'u' => {
let uc = self.parse_hex4()?;
if (0xDC00..=0xDFFF).contains(&uc) || uc == 0 {
return Err(CJsonError::InvalidUnicodeEscape { pos: esc_pos });
}
let codepoint = if (0xD800..=0xDBFF).contains(&uc) {
if self.next_char() != Some('\\') || self.next_char() != Some('u') {
return Err(CJsonError::InvalidUnicodeEscape { pos: esc_pos });
}
let uc2 = self.parse_hex4()?;
if !(0xDC00..=0xDFFF).contains(&uc2) {
return Err(CJsonError::InvalidUnicodeEscape { pos: esc_pos });
}
0x10000 + (((uc & 0x3FF) << 10) | (uc2 & 0x3FF))
} else {
uc
};
if let Some(decoded) = char::from_u32(codepoint) {
out.push(decoded);
} else {
return Err(CJsonError::InvalidUnicodeEscape { pos: esc_pos });
}
}
_ => return Err(CJsonError::InvalidEscape { pos: esc_pos }),
}
}
c if (c as u32) < 32 => {
return Err(CJsonError::UnexpectedToken { ch: c, pos: self.pos });
}
_ => out.push(ch),
}
}
}
fn parse_hex4(&mut self) -> Result<u32, CJsonError> {
let start = self.pos;
let mut value = 0u32;
for _ in 0..4 {
let ch = self
.next_char()
.ok_or(CJsonError::UnexpectedEOF { pos: self.pos })?;
let digit = match ch {
'0'..='9' => ch as u32 - '0' as u32,
'A'..='F' => 10 + (ch as u32 - 'A' as u32),
'a'..='f' => 10 + (ch as u32 - 'a' as u32),
_ => return Err(CJsonError::InvalidUnicodeEscape { pos: start }),
};
value = (value << 4) | digit;
}
Ok(value)
}
fn parse_array(&mut self) -> Result<CJson, CJsonError> {
self.expect_char('[')?;
self.skip_whitespace();
let mut items = Vec::new();
if self.peek() == Some(']') {
self.next_char();
return Ok(CJson::Array(items));
}
loop {
self.skip_whitespace();
items.push(self.parse_value()?);
self.skip_whitespace();
match self.peek() {
Some(',') => {
self.next_char();
self.skip_whitespace();
}
Some(']') => {
self.next_char();
return Ok(CJson::Array(items));
}
Some(_) => return Err(CJsonError::ExpectedCommaOrEnd { pos: self.pos }),
None => return Err(CJsonError::UnexpectedEOF { pos: self.pos }),
}
}
}
fn parse_object(&mut self) -> Result<CJson, CJsonError> {
self.expect_char('{')?;
self.skip_whitespace();
let mut map = HashMap::new();
if self.peek() == Some('}') {
self.next_char();
return Ok(CJson::Object(map));
}
loop {
self.skip_whitespace();
let key = match self.peek() {
Some('"') => self.parse_string()?,
Some(ch) => return Err(CJsonError::UnexpectedToken { ch, pos: self.pos }),
None => return Err(CJsonError::UnexpectedEOF { pos: self.pos }),
};
self.skip_whitespace();
if self.peek() != Some(':') {
return Err(CJsonError::ExpectedColon { pos: self.pos });
}
self.next_char();
self.skip_whitespace();
let value = self.parse_value()?;
map.insert(key, value);
self.skip_whitespace();
match self.peek() {
Some(',') => {
self.next_char();
self.skip_whitespace();
}
Some('}') => {
self.next_char();
return Ok(CJson::Object(map));
}
Some(_) => return Err(CJsonError::ExpectedCommaOrEnd { pos: self.pos }),
None => return Err(CJsonError::UnexpectedEOF { pos: self.pos }),
}
}
}
}
pub fn parse(input: &str, require_end: bool) -> Result<CJson, CJsonError> {
let mut parser = Parser::new(input);
let value = parser.parse_value()?;
if require_end {
parser.skip_whitespace();
if let Some(ch) = parser.peek() {
return Err(CJsonError::UnexpectedToken {
ch,
pos: parser.pos,
});
}
}
Ok(value)
}
fn escape_string(s: &str) -> String {
let mut out = String::with_capacity(s.len() + 2);
out.push('"');
for ch in s.chars() {
match ch {
'"' => out.push_str("\\\""),
'\\' => out.push_str("\\\\"),
'\u{0008}' => out.push_str("\\b"),
'\u{000C}' => out.push_str("\\f"),
'\n' => out.push_str("\\n"),
'\r' => out.push_str("\\r"),
'\t' => out.push_str("\\t"),
c if (c as u32) < 32 => {
let _ = fmt::write(&mut out, format_args!("\\u{:04x}", c as u32));
}
c => out.push(c),
}
}
out.push('"');
out
}
fn write_json_compact(f: &mut impl fmt::Write, value: &CJson) -> fmt::Result {
match value {
CJson::Null => f.write_str("null"),
CJson::Bool(false) => f.write_str("false"),
CJson::Bool(true) => f.write_str("true"),
CJson::Number(n) => {
if *n == 0.0 {
f.write_str("0")
} else if n.fract() == 0.0 && n.abs() < 1.0e60 {
write!(f, "{:.0}", n)
} else if n.abs() < 1.0e-6 || n.abs() > 1.0e9 {
write!(f, "{:e}", n)
} else {
write!(f, "{}", n)
}
}
CJson::String(s) => f.write_str(&escape_string(s)),
CJson::Array(items) => {
f.write_str("[")?;
for (i, item) in items.iter().enumerate() {
if i > 0 {
f.write_str(",")?;
}
write_json_compact(f, item)?;
}
f.write_str("]")
}
CJson::Object(map) => {
f.write_str("{")?;
let mut entries: Vec<_> = map.iter().collect();
entries.sort_by(|a, b| a.0.cmp(b.0));
for (i, (k, v)) in entries.into_iter().enumerate() {
if i > 0 {
f.write_str(",")?;
}
f.write_str(&escape_string(k))?;
f.write_str(":")?;
write_json_compact(f, v)?;
}
f.write_str("}")
}
}
}
fn write_json_pretty(f: &mut impl fmt::Write, value: &CJson, indent: usize) -> fmt::Result {
match value {
CJson::Null | CJson::Bool(_) | CJson::Number(_) | CJson::String(_) => {
write_json_compact(f, value)
}
CJson::Array(items) => {
if items.is_empty() {
return f.write_str("[]");
}
f.write_str("[")?;
for (i, item) in items.iter().enumerate() {
if i > 0 {
f.write_str(", ")?;
}
write_json_pretty(f, item, indent + 1)?;
}
f.write_str("]")
}
CJson::Object(map) => {
if map.is_empty() {
f.write_str("{\n")?;
for _ in 0..indent.saturating_sub(1) {
f.write_str("\t")?;
}
return f.write_str("}");
}
f.write_str("{\n")?;
let mut entries: Vec<_> = map.iter().collect();
entries.sort_by(|a, b| a.0.cmp(b.0));
for (idx, (k, v)) in entries.into_iter().enumerate() {
for _ in 0..(indent + 1) {
f.write_str("\t")?;
}
f.write_str(&escape_string(k))?;
f.write_str(":\t")?;
write_json_pretty(f, v, indent + 1)?;
if idx + 1 != map.len() {
f.write_str(",")?;
}
f.write_str("\n")?;
}
for _ in 0..indent {
f.write_str("\t")?;
}
f.write_str("}")
}
}
}
impl CJson {
pub fn print_unformatted(&self) -> String {
let mut out = String::new();
let _ = write_json_compact(&mut out, self);
out
}
pub fn print_formatted(&self) -> String {
let mut out = String::new();
let _ = write_json_pretty(&mut out, self, 0);
out
}
pub fn get_array_size(&self) -> Option<usize> {
match self {
CJson::Array(items) => Some(items.len()),
_ => None,
}
}
pub fn get_array_item(&self, index: usize) -> Option<&CJson> {
match self {
CJson::Array(items) => items.get(index),
_ => None,
}
}
pub fn get_object_item(&self, key: &str) -> Option<&CJson> {
match self {
CJson::Object(map) => map.get(key),
_ => None,
}
}
pub fn create_null() -> Self {
CJson::Null
}
pub fn create_bool(b: bool) -> Self {
CJson::Bool(b)
}
pub fn create_number(n: f64) -> Self {
CJson::Number(n)
}
pub fn create_string<S: Into<String>>(s: S) -> Self {
CJson::String(s.into())
}
pub fn create_array() -> Self {
CJson::Array(Vec::new())
}
pub fn create_object() -> Self {
CJson::Object(HashMap::new())
}
pub fn add_item_to_array(&mut self, item: CJson) -> Result<(), &'static str> {
match self {
CJson::Array(items) => {
items.push(item);
Ok(())
}
_ => Err("not an array"),
}
}
pub fn add_item_to_object<S: Into<String>>(
&mut self,
key: S,
value: CJson,
) -> Result<(), &'static str> {
match self {
CJson::Object(map) => {
map.insert(key.into(), value);
Ok(())
}
_ => Err("not an object"),
}
}
}
impl fmt::Display for CJson {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
write_json_pretty(f, self, 0)
}
}
