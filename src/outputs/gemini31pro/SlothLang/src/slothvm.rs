use crate::{parser, throw, stack::Stack};
use std::io::{self, Read, Write};

pub type UByte = u8;

pub enum Opcodes {
Exit = 0x00,
Push = 0x01,
Add = 0x02,
Sub = 0x03,
Mult = 0x04,
Div = 0x05,
Comp = 0x06,
Inp = 0x07,
Out = 0x08,
Goto = 0x09,
Dup = 0x0A,
}

pub enum CompCodes {
Eq = 0x01,
Neq = 0x02,
Lt = 0x03,
Le = 0x04,
Gt = 0x05,
Ge = 0x06,
}

pub enum TypeCode {
Int = 0x01,
Chr = 0x02,
}

pub struct SlothProgram {
pub codes: Vec<UByte>,
pub pc: usize,
}

pub fn execute(sbin: &mut Option<SlothProgram>) -> i32 {
let program = sbin.as_mut().unwrap();
let mut s = Stack::new();
let mut pc = 0;
let p = &program.codes;
let mut run = true;

while run {
if pc >= p.len() {
break;
}
match p[pc] {
0x00 => { // EXIT
if s.is_empty() {
return 0;
}
return s.pop().unwrap();
}
0x02 => { // ADD
let b = s.pop().unwrap();
let a = s.pop().unwrap();
s.push(a + b);
pc += 1;
}
0x03 => { // SUB
let b = s.pop().unwrap();
let a = s.pop().unwrap();
s.push(a - b);
pc += 1;
}
0x04 => { // MULT
let b = s.pop().unwrap();
let a = s.pop().unwrap();
s.push(a * b);
pc += 1;
}
0x05 => { // DIV
let b = s.pop().unwrap();
let a = s.pop().unwrap();
if b == 0 {
throw::math_err("division by zero");
}
if a == std::i32::MIN && b == -1 {
throw::math_err("division by zero");
}
s.push(a / b);
pc += 1;
}
0x06 => { // COMP
let b = s.pop().unwrap();
let a = s.pop().unwrap();
pc += 1;
let res = match p[pc] {
0x01 => a == b,
0x02 => a != b,
0x03 => a < b,
0x04 => a <= b,
0x05 => a > b,
0x06 => a >= b,
_ => {
throw::op_err("comparison", p[pc]);
false
}
};
s.push(if res { 1 } else { 0 });
pc += 1;
}
0x07 => { // INP
pc += 1;
match p[pc] {
0x01 => { // INT
print!(">");
io::stdout().flush().unwrap();
let mut input = String::new();
if io::stdin().read_line(&mut input).is_ok() {
let x: i32 = input.trim().parse().unwrap_or(0);
s.push(x);
} else {
s.push(0);
}
}
0x02 => { // CHR
print!(">");
io::stdout().flush().unwrap();
let mut buf = [0; 1];
if io::stdin().read_exact(&mut buf).is_ok() {
s.push(buf[0] as i32);
} else {
s.push(0);
}
}
_ => {
throw::op_err("input type", p[pc]);
}
}
pc += 1;
}
0x08 => { // OUT
pc += 1;
match p[pc] {
0x01 => { // INT
let x = s.pop().unwrap();
print!("{}", x);
}
0x02 => { // CHR
let x = s.pop().unwrap();
print!("{}", x as u8 as char);
}
_ => {
throw::op_err("output type", p[pc]);
}
}
pc += 1;
}
0x09 => { // GOTO
pc += 1;
if s.pop().unwrap() == 1 {
pc = p[pc] as usize;
} else {
pc += 1;
}
}
0x01 => { // PUSH
pc += 1;
s.push(p[pc] as i32);
pc += 1;
}
0x0A => { // DUP
let x = s.pop().unwrap();
s.push(x);
s.push(x);
pc += 1;
}
_ => {
throw::op_err("operation", p[pc]);
}
}
}
0
}
