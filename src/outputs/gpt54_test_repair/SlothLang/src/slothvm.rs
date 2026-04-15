use crate::stack::Stack;
use crate::throw;

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
let mut stack = Stack::new();
let program = match sbin.as_mut() {
Some(p) => p,
None => return 0,
};

let p = &program.codes;
let mut pc: usize = program.pc;

loop {
if pc >= p.len() {
program.pc = pc;
return 0;
}

match p[pc] {
0x00 => {
program.pc = pc;
if stack.is_empty() {
return 0;
}
return stack.pop().unwrap_or(0);
}
0x01 => {
pc += 1;
let val = p.get(pc).copied().unwrap_or(0);
stack.push(val as i32);
pc += 1;
}
0x02 => {
let b = stack.pop().unwrap_or(0);
let a = stack.pop().unwrap_or(0);
stack.push(a + b);
pc += 1;
}
0x03 => {
let b = stack.pop().unwrap_or(0);
let a = stack.pop().unwrap_or(0);
stack.push(a - b);
pc += 1;
}
0x04 => {
let b = stack.pop().unwrap_or(0);
let a = stack.pop().unwrap_or(0);
stack.push(a * b);
pc += 1;
}
0x05 => {
let b = stack.pop().unwrap_or(0);
let a = stack.pop().unwrap_or(0);
if b == 0 {
throw::math_err("division by zero");
}
if a == i32::MIN && b == -1 {
throw::math_err("integer overflow");
}
stack.push(a / b);
pc += 1;
}
0x06 => {
let b = stack.pop().unwrap_or(0);
let a = stack.pop().unwrap_or(0);
pc += 1;
let res = match p.get(pc).copied().unwrap_or(0) {
0x01 => a == b,
0x02 => a != b,
0x03 => a < b,
0x04 => a <= b,
0x05 => a > b,
0x06 => a >= b,
code => {
throw::op_err("comparison", code);
}
};
stack.push(if res { 1 } else { 0 });
pc += 1;
}
0x07 => {
pc += 1;
match p.get(pc).copied().unwrap_or(0) {
0x01 => {
use std::io;
let mut input = String::new();
print!(">");
let _ = io::stdin().read_line(&mut input);
let x = input.trim().parse::<i32>().unwrap_or(0);
stack.push(x);
}
0x02 => {
use std::io;
let mut input = String::new();
let _ = io::stdin().read_line(&mut input);
let ch = input
.strip_prefix('>')
.unwrap_or(&input)
.chars()
.next()
.unwrap_or('\0');
stack.push(ch as i32);
}
code => {
throw::op_err("input type", code);
}
}
pc += 1;
}
0x08 => {
pc += 1;
match p.get(pc).copied().unwrap_or(0) {
0x01 => {
let x = stack.pop().unwrap_or(0);
print!("{}", x);
}
0x02 => {
let x = stack.pop().unwrap_or(0) as u8 as char;
print!("{}", x);
}
code => {
throw::op_err("output type", code);
}
}
pc += 1;
}
0x09 => {
pc += 1;
if stack.pop().unwrap_or(0) == 1 {
pc = p.get(pc).copied().unwrap_or(0) as usize;
} else {
pc += 1;
}
}
0x0A => {
let x = stack.pop().unwrap_or(0);
stack.push(x);
stack.push(x);
pc += 1;
}
code => {
throw::op_err("operation", code);
}
}

program.pc = pc;
}
}
