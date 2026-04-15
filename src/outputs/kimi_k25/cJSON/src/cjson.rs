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
CJsonError::UnexpectedEOF { pos } => write!(f, "Unexpected end of input at position {}", pos),
CJsonError::UnexpectedToken { ch, pos } => write!(f, "Unexpected token '{}' at position {}", ch, pos),
CJsonError::InvalidLiteral { expected, pos } => write!(f, "Expected {} at position {}", expected, pos),
CJsonError::InvalidNumber { pos } => write!(f, "Invalid number at position {}", pos),
CJsonError::InvalidEscape { pos } => write!(f, "Invalid escape sequence at position {}", pos),
CJsonError::InvalidUnicodeEscape { pos } => write!(f, "Invalid unicode escape sequence at position {}", pos),
CJsonError::ExpectedColon { pos } => write!(f, "Expected colon at position {}", pos),
CJsonError::ExpectedCommaOrEnd { pos } => write!(f, "Expected comma or end at position {}", pos),
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
Parser { input, pos: 0 }
}

fn peek(&self) -> Option<char> {
self.input.chars().nth(self.pos)
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
if predicate(ch) {
self.pos += ch.len_utf8();
} else {
break;
}
}
&self.input[start..self.pos]
}

fn skip_whitespace(&mut self) {
while let Some(ch) = self.peek() {
if ch.is_ascii_whitespace() {
self.pos += ch.len_utf8();
} else {
break;
}
}
}

fn expect_char(&mut self, expected: char) -> Result<(), CJsonError> {
match self.next_char() {
Some(ch) if ch == expected => Ok(()),
Some(ch) => Err(CJsonError::UnexpectedToken { ch, pos: self.pos - ch.len_utf8() }),
None => Err(CJsonError::UnexpectedEOF { pos: self.pos }),
}
}

fn parse_value(&mut self) -> Result<CJson, CJsonError> {
self.skip_whitespace();
match self.peek() {
Some('n') => self.parse_null(),
Some('t') | Some('f') => self.parse_bool(),
Some('"') => self.parse_string().map(CJson::String),
Some('[') => self.parse_array(),
Some('{') => self.parse_object(),
Some(ch) if ch == '-' || ch.is_ascii_digit() => self.parse_number(),
Some(ch) => Err(CJsonError::UnexpectedToken { ch, pos: self.pos }),
None => Err(CJsonError::UnexpectedEOF { pos: self.pos }),
}
}

fn parse_null(&mut self) -> Result<CJson, CJsonError> {
if self.input[self.pos..].starts_with("null") {
self.pos += 4;
Ok(CJson::Null)
} else {
Err(CJsonError::InvalidLiteral { expected: "null", pos: self.pos })
}
}

fn parse_bool(&mut self) -> Result<CJson, CJsonError> {
if self.input[self.pos..].starts_with("true") {
self.pos += 4;
Ok(CJson::Bool(true))
} else if self.input[self.pos..].starts_with("false") {
self.pos += 5;
Ok(CJson::Bool(false))
} else {
Err(CJsonError::InvalidLiteral { expected: "true or false", pos: self.pos })
}
}

fn parse_number(&mut self) -> Result<CJson, CJsonError> {
let start = self.pos;

// Optional minus sign
if self.peek() == Some('-') {
self.pos += 1;
}

// Integer part
if self.peek() == Some('0') {
self.pos += 1;
} else if let Some(ch) = self.peek() {
if ch.is_ascii_digit() {
while let Some(ch) = self.peek() {
if ch.is_ascii_digit() {
self.pos += ch.len_utf8();
} else {
break;
}
}
} else {
return Err(CJsonError::InvalidNumber { pos: start });
}
} else {
return Err(CJsonError::UnexpectedEOF { pos: self.pos });
}

// Fractional part
if self.peek() == Some('.') {
self.pos += 1;
if let Some(ch) = self.peek() {
if !ch.is_ascii_digit() {
return Err(CJsonError::InvalidNumber { pos: self.pos });
}
} else {
return Err(CJsonError::UnexpectedEOF { pos: self.pos });
}
while let Some(ch) = self.peek() {
if ch.is_ascii_digit() {
self.pos += ch.len_utf8();
} else {
break;
}
}
}

// Exponent part
if let Some(ch) = self.peek() {
if ch == 'e' || ch == 'E' {
self.pos += 1;
if let Some(ch) = self.peek() {
if ch == '+' || ch == '-' {
self.pos += 1;
}
}
if let Some(ch) = self.peek() {
if !ch.is_ascii_digit() {
return Err(CJsonError::InvalidNumber { pos: self.pos });
}
} else {
return Err(CJsonError::UnexpectedEOF { pos: self.pos });
}
while let Some(ch) = self.peek() {
if ch.is_ascii_digit() {
self.pos += ch.len_utf8();
} else {
break;
}
}
}
}

let num_str = &self.input[start..self.pos];
match num_str.parse::<f64>() {
Ok(n) => Ok(CJson::Number(n)),
Err(_) => Err(CJsonError::InvalidNumber { pos: start }),
}
}

fn parse_string(&mut self) -> Result<String, CJsonError> {
self.expect_char('"')?;
let mut result = String::new();

while let Some(ch) = self.peek() {
match ch {
'"' => {
self.pos += 1;
return Ok(result);
}
'\\' => {
self.pos += 1;
match self.next_char() {
Some('"') => result.push('"'),
Some('\\') => result.push('\\'),
Some('/') => result.push('/'),
Some('b') => result.push('\x08'),
Some('f') => result.push('\x0C'),
Some('n') => result.push('\n'),
Some('r') => result.push('\r'),
Some('t') => result.push('\t'),
Some('u') => {
// Parse 4 hex digits
let hex_start = self.pos;
let mut hex_val = 0u32;
for _ in 0..4 {
match self.next_char() {
Some(c) if c.is_ascii_hexdigit() => {
hex_val = hex_val * 16 + c.to_digit(16).unwrap();
}
_ => return Err(CJsonError::InvalidUnicodeEscape { pos: hex_start }),
}
}

// Check for surrogate pairs
if (0xD800..=0xDBFF).contains(&hex_val) {
// High surrogate, expect low surrogate
if self.peek() == Some('\\') {
self.pos += 1;
if self.next_char() == Some('u') {
let hex_start2 = self.pos;
let mut hex_val2 = 0u32;
for _ in 0..4 {
match self.next_char() {
Some(c) if c.is_ascii_hexdigit() => {
hex_val2 = hex_val2 * 16 + c.to_digit(16).unwrap();
}
_ => return Err(CJsonError::InvalidUnicodeEscape { pos: hex_start2 }),
}
}
if (0xDC00..=0xDFFF).contains(&hex_val2) {
// Combine surrogates
let codepoint = 0x10000 + (((hex_val & 0x3FF) << 10) | (hex_val2 & 0x3FF));
if let Some(c) = char::from_u32(codepoint) {
result.push(c);
} else {
return Err(CJsonError::InvalidUnicodeEscape { pos: hex_start });
}
} else {
return Err(CJsonError::InvalidUnicodeEscape { pos: hex_start2 });
}
} else {
return Err(CJsonError::InvalidUnicodeEscape { pos: self.pos - 1 });
}
} else {
// Lone high surrogate
return Err(CJsonError::InvalidUnicodeEscape { pos: hex_start });
}
} else if (0xDC00..=0xDFFF).contains(&hex_val) {
// Lone low surrogate
return Err(CJsonError::InvalidUnicodeEscape { pos: hex_start });
} else {
if let Some(c) = char::from_u32(hex_val) {
result.push(c);
} else {
return Err(CJsonError::InvalidUnicodeEscape { pos: hex_start });
}
}
}
Some(c) => {
result.push(c);
}
None => return Err(CJsonError::UnexpectedEOF { pos: self.pos }),
}
}
c if c.is_control() => {
return Err(CJsonError::UnexpectedToken { ch: c, pos: self.pos });
}
_ => {
self.pos += ch.len_utf8();
result.push(ch);
}
}
}

Err(CJsonError::UnexpectedEOF { pos: self.pos })
}

fn parse_array(&mut self) -> Result<CJson, CJsonError> {
self.expect_char('[')?;
self.skip_whitespace();

if self.peek() == Some(']') {
self.pos += 1;
return Ok(CJson::Array(Vec::new()));
}

let mut items = Vec::new();
loop {
self.skip_whitespace();
let value = self.parse_value()?;
items.push(value);

self.skip_whitespace();
match self.peek() {
Some(',') => {
self.pos += 1;
continue;
}
Some(']') => {
self.pos += 1;
break;
}
Some(ch) => return Err(CJsonError::ExpectedCommaOrEnd { pos: self.pos }),
None => return Err(CJsonError::UnexpectedEOF { pos: self.pos }),
}
}

Ok(CJson::Array(items))
}

fn parse_object(&mut self) -> Result<CJson, CJsonError> {
self.expect_char('{')?;
self.skip_whitespace();

if self.peek() == Some('}') {
self.pos += 1;
return Ok(CJson::Object(HashMap::new()));
}

let mut map = HashMap::new();
loop {
self.skip_whitespace();

// Parse key (must be string)
let key = self.parse_string()?;

self.skip_whitespace();
if self.peek() != Some(':') {
return Err(CJsonError::ExpectedColon { pos: self.pos });
}
self.pos += 1; // consume ':'

self.skip_whitespace();
let value = self.parse_value()?;
map.insert(key, value);

self.skip_whitespace();
match self.peek() {
Some(',') => {
self.pos += 1;
continue;
}
Some('}') => {
self.pos += 1;
break;
}
Some(ch) => return Err(CJsonError::ExpectedCommaOrEnd { pos: self.pos }),
None => return Err(CJsonError::UnexpectedEOF { pos: self.pos }),
}
}

Ok(CJson::Object(map))
}
}

pub fn parse(input: &str, require_end: bool) -> Result<CJson, CJsonError> {
let mut parser = Parser::new(input);
let result = parser.parse_value()?;
if require_end {
parser.skip_whitespace();
if let Some(ch) = parser.peek() {
return Err(CJsonError::UnexpectedToken { ch, pos: parser.pos });
}
}
Ok(result)
}

fn escape_string(s: &str) -> String {
let mut result = String::with_capacity(s.len());
for ch in s.chars() {
match ch {
'"' => result.push_str("\\\""),
'\\' => result.push_str("\\\\"),
'\x08' => result.push_str("\\b"),
'\x0C' => result.push_str("\\f"),
'\n' => result.push_str("\\n"),
'\r' => result.push_str("\\r"),
'\t' => result.push_str("\\t"),
c if c.is_ascii_control() => {
result.push_str(&format!("\\u{:04x}", c as u32));
}
c => result.push(c),
}
}
result
}

fn write_json_compact(f: &mut impl fmt::Write, value: &CJson) -> fmt::Result {
match value {
CJson::Null => write!(f, "null"),
CJson::Bool(b) => write!(f, "{}", b),
CJson::Number(n) => {
if *n == 0.0 {
write!(f, "0")
} else if n.is_nan() || n.is_infinite() {
write!(f, "null")
} else {
let int_val = *n as i64;
if (int_val as f64 - *n).abs() < f64::EPSILON && *n >= i64::MIN as f64 && *n <= i64::MAX as f64 {
write!(f, "{}", int_val)
} else {
let abs_n = n.abs();
if abs_n < 1e-6 || abs_n > 1e9 {
write!(f, "{:e}", n)
} else {
write!(f, "{}", n)
}
}
}
}
CJson::String(s) => {
write!(f, "\"{}\"", escape_string(s))
}
CJson::Array(arr) => {
write!(f, "[")?;
for (i, item) in arr.iter().enumerate() {
if i > 0 {
write!(f, ",")?;
}
write_json_compact(f, item)?;
}
write!(f, "]")
}
CJson::Object(obj) => {
write!(f, "{{")?;
let mut first = true;
for (key, val) in obj.iter() {
if !first {
write!(f, ",")?;
}
first = false;
write!(f, "\"{}\"", escape_string(key))?;
write!(f, ":")?;
write_json_compact(f, val)?;
}
write!(f, "}}")
}
}
}

fn write_json_pretty(f: &mut impl fmt::Write, value: &CJson, indent: usize) -> fmt::Result {
match value {
CJson::Null => write!(f, "null"),
CJson::Bool(b) => write!(f, "{}", b),
CJson::Number(n) => {
if *n == 0.0 {
write!(f, "0")
} else if n.is_nan() || n.is_infinite() {
write!(f, "null")
} else {
let int_val = *n as i64;
if (int_val as f64 - *n).abs() < f64::EPSILON && *n >= i64::MIN as f64 && *n <= i64::MAX as f64 {
write!(f, "{}", int_val)
} else {
let abs_n = n.abs();
if abs_n < 1e-6 || abs_n > 1e9 {
write!(f, "{:e}", n)
} else {
write!(f, "{}", n)
}
}
}
}
CJson::String(s) => {
write!(f, "\"{}\"", escape_string(s))
}
CJson::Array(arr) => {
if arr.is_empty() {
return write!(f, "[]");
}
write!(f, "[\n")?;
for (i, item) in arr.iter().enumerate() {
for _ in 0..indent + 1 {
write!(f, "\t")?;
}
write_json_pretty(f, item, indent + 1)?;
if i < arr.len() - 1 {
write!(f, ",\n")?;
} else {
write!(f, "\n")?;
}
}
for _ in 0..indent {
write!(f, "\t")?;
}
write!(f, "]")
}
CJson::Object(obj) => {
if obj.is_empty() {
return write!(f, "{{}}");
}
write!(f, "{{\n")?;
let mut first = true;
for (key, val) in obj.iter() {
if !first {
write!(f, ",\n")?;
}
first = false;
for _ in 0..indent + 1 {
write!(f, "\t")?;
}
write!(f, "\"{}\": ", escape_string(key))?;
write_json_pretty(f, val, indent + 1)?;
}
write!(f, "\n")?;
for _ in 0..indent {
write!(f, "\t")?;
}
write!(f, "}}")
}
}
}

impl CJson {
pub fn print_unformatted(&self) -> String {
let mut s = String::new();
write_json_compact(&mut s, self).unwrap();
s
}

pub fn print_formatted(&self) -> String {
let mut s = String::new();
write_json_pretty(&mut s, self, 0).unwrap();
s
}

pub fn get_array_size(&self) -> Option<usize> {
match self {
CJson::Array(arr) => Some(arr.len()),
_ => None,
}
}

pub fn get_array_item(&self, index: usize) -> Option<&CJson> {
match self {
CJson::Array(arr) => arr.get(index),
_ => None,
}
}

pub fn get_object_item(&self, key: &str) -> Option<&CJson> {
match self {
CJson::Object(obj) => obj.get(key),
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
CJson::Array(arr) => {
arr.push(item);
Ok(())
}
_ => Err("Not an array"),
}
}

pub fn add_item_to_object<S: Into<String>>(
&mut self,
key: S,
value: CJson,
) -> Result<(), &'static str> {
match self {
CJson::Object(obj) => {
obj.insert(key.into(), value);
Ok(())
}
_ => Err("Not an object"),
}
}
}

impl fmt::Display for CJson {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
write_json_compact(f, self)
}
}
