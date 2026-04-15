// Generated Rust Code
use crate::data;
use core::fmt;

pub const DEFAULT_PROGRAM_CAP: usize = 256;
pub const DATA_STDOUT: i32 = 1;
pub const DATA_STDERR: i32 = 2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EmType {
Push,
Pop,
Add,
Sub,
Mul,
Div,
Grt,
Less,
Equ,
Nequ,
PrintBegin,
PrintEnd,
IfBegin,
IfEnd,
LoopBegin,
LoopEnd,
Exit,
Dup,
Swap,
#[cfg(debug_assertions)]
Debug,
}

impl fmt::Display for EmType {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
match self {
EmType::Push => write!(f, "push"),
EmType::Pop => write!(f, "pop"),
EmType::Add => write!(f, "add"),
EmType::Sub => write!(f, "sub"),
EmType::Mul => write!(f, "mul"),
EmType::Div => write!(f, "div"),
EmType::Grt => write!(f, "grt"),
EmType::Less => write!(f, "less"),
EmType::Equ => write!(f, "equ"),
EmType::Nequ => write!(f, "nequ"),
EmType::PrintBegin => write!(f, "print_begin"),
EmType::PrintEnd => write!(f, "print_end"),
EmType::IfBegin => write!(f, "if_begin"),
EmType::IfEnd => write!(f, "if_end"),
EmType::LoopBegin => write!(f, "loop_begin"),
EmType::LoopEnd => write!(f, "loop_end"),
EmType::Exit => write!(f, "exit"),
EmType::Dup => write!(f, "dup"),
EmType::Swap => write!(f, "swap"),
#[cfg(debug_assertions)]
EmType::Debug => write!(f, "debug"),
}
}
}

#[derive(Debug, Clone)]
pub struct Em {
pub data: data::Data,
pub em_type: EmType,
pub path: String,
pub row: usize,
pub col: usize,
pub r#ref: usize,
pub ran: bool,
}

impl fmt::Display for Em {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
write!(f, "<{}", self.em_type)?;
match self.em_type {
EmType::Push => {
write!(f, " {}", self.data)?;
}
EmType::PrintEnd => {
let target = match &self.data.value {
data::DataValue::Int(v) if *v == DATA_STDOUT as i64 => "stdout",
_ => "stderr",
};
write!(f, " {}", target)?;
}
EmType::PrintBegin | EmType::IfBegin => {
write!(f, " ref: {}", self.r#ref)?;
}
_ => {}
}
write!(f, " {}:{}:{}>\n", self.path, self.row, self.col)
}
}

impl Em {
pub fn new(typ: EmType) -> Self {
Em {
data: data::Data::new(data::DataType::Int),
em_type: typ,
path: String::new(),
row: 0,
col: 0,
r#ref: 0,
ran: false,
}
}

pub fn new_with_data(typ: EmType, data: data::Data) -> Self {
Em {
data,
em_type: typ,
path: String::new(),
row: 0,
col: 0,
r#ref: 0,
ran: false,
}
}
}

#[derive(Debug, Clone)]
pub struct Program {
pub ems: Vec<Em>,
pub cap: usize,
pub size: usize,
}

impl Program {
pub fn new(cap: usize) -> Self {
assert!(cap > 0);
Program {
ems: Vec::with_capacity(cap),
cap,
size: 0,
}
}

pub fn push(&mut self, em: Em) {
if self.size >= self.cap {
self.cap *= 2;
let additional = self.cap.saturating_sub(self.ems.capacity());
if additional > 0 {
self.ems.reserve(additional);
}
}
self.ems.push(em);
self.size += 1;
}
}
