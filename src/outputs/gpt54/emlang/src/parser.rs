// Generated Rust Code
use crate::em;
use core::fmt;

pub const PARSER_MAX_TOKEN_LENGTH: usize = 1024;
pub const PARSER_MAX_NESTS: usize = 256;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParserError {
UnexpectedEscape,
UnknownEscape,
UnterminatedQuotes,
UnexpectedEnd,
IllegalPrintNest,
ExpectedEnd,
}

impl fmt::Display for ParserError {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match self {
ParserError::UnexpectedEscape => write!(f, "Unexpected escape"),
ParserError::UnknownEscape => write!(f, "Unknown escape"),
ParserError::UnterminatedQuotes => write!(f, "Unterminated quotes"),
ParserError::UnexpectedEnd => write!(f, "Unexpected end"),
ParserError::IllegalPrintNest => write!(f, "Illegal print nesting"),
ParserError::ExpectedEnd => write!(f, "Expected matching end"),
}
}
}

#[derive(Debug)]
pub struct ParserResult {
pub path: String,
pub row: usize,
pub col: usize,
pub prog: Result<em::Program, ParserError>,
}

#[derive(Debug)]
pub struct Parser {
pub path: String,
pub row: usize,
pub col: usize,
pub from_file: bool,
pub input: String,
pub ch: i32,
pub pos: usize,
pub tok: String,
pub tok_len: usize,
pub prog: em::Program,
}

impl Parser {
pub fn new() -> Self {
Parser {
path: String::new(),
row: 1,
col: 0,
from_file: false,
input: String::new(),
ch: 0,
pos: 0,
tok: String::new(),
tok_len: 0,
prog: em::Program::new(em::DEFAULT_PROGRAM_CAP),
}
}

pub fn load_mem(&mut self, input: &str) {
self.input = input.to_string();
self.path.clear();
self.from_file = false;
self.pos = 0;
self.row = 1;
self.col = 0;
self.ch = 0;
}

pub fn load_file(&mut self, path: &str) -> i32 {
self.from_file = true;
self.path = path.to_string();
match std::fs::read_to_string(path) {
Ok(s) => {
self.input = s;
self.pos = 0;
self.row = 1;
self.col = 0;
self.ch = 0;
0
}
Err(_) => -1,
}
}

pub fn parse(&mut self) -> ParserResult {
self.advance();
if self.ch == 0 {
return ParserResult {
path: self.path.clone(),
row: self.row,
col: self.col,
prog: Ok(self.prog.clone()),
};
}

loop {
let result = self.parse_next();
if result.prog.is_err() {
return result;
}
if self.ch == 0 {
break;
}
}

let result = self.cross_ref();
if result.prog.is_err() {
return result;
}

ParserResult {
path: self.path.clone(),
row: self.row,
col: self.col,
prog: Ok(self.prog.clone()),
}
}

pub fn cross_ref(&mut self) -> ParserResult {
let mut expects: Vec<em::EmType> = Vec::with_capacity(PARSER_MAX_NESTS);
let mut begins: Vec<usize> = Vec::with_capacity(PARSER_MAX_NESTS);
let mut print = false;

for i in 0..self.prog.size {
let typ = self.prog.ems[i].em_type;
match typ {
em::EmType::PrintBegin => {
if print {
return ParserResult {
path: self.prog.ems[i].path.clone(),
row: self.prog.ems[i].row,
col: self.prog.ems[i].col,
prog: Err(ParserError::IllegalPrintNest),
};
}
print = true;
expects.push(em::EmType::PrintEnd);
begins.push(i);
}
em::EmType::IfBegin => {
expects.push(em::EmType::IfEnd);
begins.push(i);
}
em::EmType::LoopBegin => {
expects.push(em::EmType::LoopEnd);
begins.push(i);
}
em::EmType::PrintEnd => {
print = false;
if expects.is_empty() || *expects.last().unwrap() != em::EmType::PrintEnd {
return ParserResult {
path: self.prog.ems[i].path.clone(),
row: self.prog.ems[i].row,
col: self.prog.ems[i].col,
prog: Err(ParserError::UnexpectedEnd),
};
}
let begin = begins.pop().unwrap();
expects.pop();
self.prog.ems[begin].r#ref = i;
self.prog.ems[i].r#ref = begin;
}
em::EmType::IfEnd | em::EmType::LoopEnd => {
if expects.is_empty() || *expects.last().unwrap() != typ {
return ParserResult {
path: self.prog.ems[i].path.clone(),
row: self.prog.ems[i].row,
col: self.prog.ems[i].col,
prog: Err(ParserError::UnexpectedEnd),
};
}
let begin = begins.pop().unwrap();
expects.pop();
self.prog.ems[begin].r#ref = i;
self.prog.ems[i].r#ref = begin;
}
_ => {}
}
}

if !begins.is_empty() {
let idx = *begins.last().unwrap();
return ParserResult {
path: self.prog.ems[idx].path.clone(),
row: self.prog.ems[idx].row,
col: self.prog.ems[idx].col,
prog: Err(ParserError::ExpectedEnd),
};
}

ParserResult {
path: self.path.clone(),
row: self.row,
col: self.col,
prog: Ok(self.prog.clone()),
}
}

pub fn advance(&mut self) {
if self.ch == '\n' as i32 {
self.row += 1;
self.col = 0;
}

if self.pos >= self.input.chars().count() {
self.ch = 0;
return;
}

let c = self.input.chars().nth(self.pos).unwrap();
self.pos += 1;
self.ch = c as i32;

if self.ch != 0 {
self.col += 1;
}
}

pub fn parse_plain(&mut self) -> ParserResult {
self.tok.clear();
self.tok_len = 0;
let start_row = self.row;
let start_col = self.col;

if self.ch == '\\' as i32 {
self.advance();
if self.ch == 0 || (self.ch as u8 as char).is_whitespace() {
return ParserResult {
path: self.path.clone(),
row: start_row,
col: start_col,
prog: Err(ParserError::UnexpectedEscape),
};
} else if self.ch != '"' as i32 {
self.tok.push('\\');
self.tok_len += 1;
}
}

let mut is_int = true;
loop {
let c = char::from_u32(self.ch as u32).unwrap_or('\0');
if is_int && !(self.tok_len == 0 && c == '-') && !c.is_ascii_digit() {
is_int = false;
}
self.tok.push(c);
self.tok_len += 1;
self.advance();
if self.ch == 0 {
break;
}
let nc = char::from_u32(self.ch as u32).unwrap_or('\0');
if nc.is_whitespace() {
break;
}
}

if self.tok == "-" {
is_int = false;
}

let tok = self.tok.clone();
let mut emv: Option<em::Em> = None;

let keywords: Vec<(em::EmType, &str)> = vec![
(em::EmType::Pop, ":P"),
(em::EmType::Add, ";)"),
(em::EmType::Sub, ";("),
(em::EmType::Mul, "x)"),
(em::EmType::Div, "x("),
(em::EmType::Grt, ":>"),
(em::EmType::Less, ":<"),
(em::EmType::Equ, ":|"),
(em::EmType::Nequ, "x|"),
(em::EmType::PrintBegin, ":O"),
(em::EmType::IfBegin, ":/"),
(em::EmType::IfEnd, ":\\"),
(em::EmType::LoopBegin, ":@"),
(em::EmType::LoopEnd, "@:"),
(em::EmType::Exit, "X_X"),
(em::EmType::Dup, ":D"),
(em::EmType::Swap, ":S"),
#[cfg(debug_assertions)]
(em::EmType::Debug, "D:"),
];

for (t, kw) in keywords {
if tok == kw {
emv = Some(em::Em::new(t));
break;
}
}

if emv.is_none() {
if tok == ":x" {
while self.ch != 0 && char::from_u32(self.ch as u32).unwrap_or('\0') != '\n' {
self.advance();
}
return ParserResult {
path: self.path.clone(),
row: self.row,
col: self.col,
prog: Ok(self.prog.clone()),
};
} else if tok == ":)" {
emv = Some(em::Em::new_with_data(
em::EmType::PrintEnd,
crate::data::Data::new_int(em::DATA_STDOUT as i64),
));
} else if tok == ":(" {
emv = Some(em::Em::new_with_data(
em::EmType::PrintEnd,
crate::data::Data::new_int(em::DATA_STDERR as i64),
));
} else if tok == ":3" || tok == ";3" || tok == "<3" || tok == "x3" || tok == "><>" {
let text = match tok.chars().next().unwrap() {
':' => "meow",
';' => "nya",
'x' => "rawr",
'>' => "le fishe",
'<' => "i <3 emlang",
_ => "",
};
emv = Some(em::Em::new_with_data(
em::EmType::Push,
crate::data::Data::new_str(text.to_string()),
));
} else if is_int {
emv = Some(em::Em::new_with_data(
em::EmType::Push,
crate::data::Data::new_int(tok.parse::<i64>().unwrap_or(0)),
));
} else {
emv = Some(em::Em::new_with_data(
em::EmType::Push,
crate::data::Data::new_str(tok),
));
}
}

let mut final_em = emv.unwrap();
final_em.row = start_row;
final_em.col = start_col;
final_em.path = self.path.clone();
self.prog.push(final_em);

ParserResult {
path: self.path.clone(),
row: self.row,
col: self.col,
prog: Ok(self.prog.clone()),
}
}

pub fn parse_quotes(&mut self) -> ParserResult {
self.tok.clear();
self.tok_len = 0;
let start_row = self.row;
let start_col = self.col;
let mut escape = false;

loop {
self.advance();
if self.ch == 0 || self.ch == '\n' as i32 {
return ParserResult {
path: self.path.clone(),
row: start_row,
col: start_col,
prog: Err(ParserError::UnterminatedQuotes),
};
}

let c = char::from_u32(self.ch as u32).unwrap_or('\0');

if escape {
let mapped = match c {
'n' => '\n',
'r' => '\r',
't' => '\t',
'f' => '\u{000C}',
'v' => '\u{000B}',
'b' => '\u{0008}',
'a' => '\u{0007}',
'"' => '"',
'e' => '\u{001B}',
'\\' => '\\',
_ => {
return ParserResult {
path: self.path.clone(),
row: self.row,
col: self.col,
prog: Err(ParserError::UnknownEscape),
};
}
};
self.tok.push(mapped);
self.tok_len += 1;
escape = false;
} else if c == '\\' {
escape = true;
} else if c == '"' {
break;
} else {
self.tok.push(c);
self.tok_len += 1;
}
}

self.advance();

let mut emv = em::Em::new_with_data(
em::EmType::Push,
crate::data::Data::new_str(self.tok.clone()),
);
emv.row = start_row;
emv.col = start_col;
emv.path = self.path.clone();
self.prog.push(emv);

ParserResult {
path: self.path.clone(),
row: self.row,
col: self.col,
prog: Ok(self.prog.clone()),
}
}

pub fn parse_next(&mut self) -> ParserResult {
while self.ch != 0 && char::from_u32(self.ch as u32).unwrap_or('\0').is_whitespace() {
self.advance();
if self.ch == 0 {
return ParserResult {
path: self.path.clone(),
row: self.row,
col: self.col,
prog: Ok(self.prog.clone()),
};
}
}

if self.ch == '"' as i32 {
self.parse_quotes()
} else {
self.parse_plain()
}
}
}
