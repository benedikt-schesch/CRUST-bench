use crate::{parser, throw};
use crate::stack::Stack;

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
let program = sbin.as_mut().expect("Program is None");
let mut stack = Stack::new();
let mut pc: usize = 0;
let codes = &program.codes;

loop {
if pc >= codes.len() {
break;
}

let opcode = codes[pc];

match opcode {
0x00 => { // EXIT
if stack.is_empty() {
return 0;
} else {
return stack.pop().unwrap();
}
}
0x01 => { // PUSH
pc += 1;
if pc >= codes.len() {
break;
}
stack.push(codes[pc] as i32);
pc += 1;
}
0x02 => { // ADD
let b = stack.pop().unwrap();
let a = stack.pop().unwrap();
stack.push(a + b);
pc += 1;
}
0x03 => { // SUB
let b = stack.pop().unwrap();
let a = stack.pop().unwrap();
stack.push(a - b);
pc += 1;
}
0x04 => { // MULT
let b = stack.pop().unwrap();
let a = stack.pop().unwrap();
stack.push(a * b);
pc += 1;
}
0x05 => { // DIV
let b = stack.pop().unwrap();
let a = stack.pop().unwrap();
if b == 0 {
throw::math_err("division by zero");
}
if a == i32::MIN && b == -1 {
throw::math_err("division by zero");
}
stack.push(a / b);
pc += 1;
}
0x06 => { // COMP
let b = stack.pop().unwrap();
let a = stack.pop().unwrap();
pc += 1;
if pc >= codes.len() {
break;
}
let comp_code = codes[pc];
let res = match comp_code {
0x01 => a == b,
0x02 => a != b,
0x03 => a < b,
0x04 => a <= b,
0x05 => a > b,
0x06 => a >= b,
_ => {
throw::op_err("comparison", comp_code);
false
}
};
stack.push(res as i32);
pc += 1;
}
0x07 => { // INP
pc += 1;
if pc >= codes.len() {
break;
}
let type_code = codes[pc];
match type_code {
0x01 => { // INT
print!(">");
use std::io::Write;
std::io::stdout().flush().unwrap();
let mut input = String::new();
std::io::stdin().read_line(&mut input).unwrap();
let x: i32 = input.trim().parse().unwrap_or(0);
stack.push(x);
}
0x02 => { // CHR
let mut input = String::new();
std::io::stdin().read_line(&mut input).unwrap();
let x = input.chars().next().unwrap_or('\0') as i32;
stack.push(x);
}
_ => {
throw::op_err("input type", type_code);
}
}
pc += 1;
}
0x08 => { // OUT
pc += 1;
if pc >= codes.len() {
break;
}
let type_code = codes[pc];
match type_code {
0x01 => { // INT
let x = stack.pop().unwrap();
print!("{}", x);
}
0x02 => { // CHR
let x = stack.pop().unwrap();
print!("{}", x as u8 as char);
}
_ => {
throw::op_err("output type", type_code);
}
}
pc += 1;
}
0x09 => { // GOTO
pc += 1;
if pc >= codes.len() {
break;
}
if stack.pop().unwrap() == 1 {
pc = codes[pc] as usize;
} else {
pc += 1;
}
}
0x0A => { // DUP
let x = stack.pop().unwrap();
stack.push(x);
stack.push(x);
pc += 1;
}
_ => {
throw::op_err("operation", opcode);
}
}
}

0
}
