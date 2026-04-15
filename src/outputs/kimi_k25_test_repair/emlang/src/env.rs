use std::collections::HashMap;
use crate::em::Program;
use crate::data::Instruction;

pub struct Env {
variables: HashMap<String, i64>,
stack: Vec<i64>,
popped: Vec<i64>,
}

impl Env {
pub fn new(stack_cap: usize, popped_cap: usize) -> Self {
Env {
variables: HashMap::new(),
stack: Vec::with_capacity(stack_cap),
popped: Vec::with_capacity(popped_cap),
}
}

pub fn run(&mut self, program: &Program) -> RuntimeResult<()> {
for instr in &program.instructions {
match instr {
Instruction::Push(n) => {
self.stack.push(*n);
}
Instruction::Add => {
if self.stack.len() < 2 {
return RuntimeResult {
prog: Ok(()),
em: Err("Stack underflow".to_string()),
path: String::new(),
row: 0,
col: 0,
code: 1,
};
}
let b = self.stack.pop().unwrap();
let a = self.stack.pop().unwrap();
self.stack.push(a + b);
}
Instruction::Sub => {
if self.stack.len() < 2 {
return RuntimeResult {
prog: Ok(()),
em: Err("Stack underflow".to_string()),
path: String::new(),
row: 0,
col: 0,
code: 1,
};
}
let b = self.stack.pop().unwrap();
let a = self.stack.pop().unwrap();
self.stack.push(a - b);
}
Instruction::Mul => {
if self.stack.len() < 2 {
return RuntimeResult {
prog: Ok(()),
em: Err("Stack underflow".to_string()),
path: String::new(),
row: 0,
col: 0,
code: 1,
};
}
let b = self.stack.pop().unwrap();
let a = self.stack.pop().unwrap();
self.stack.push(a * b);
}
Instruction::Div => {
if self.stack.len() < 2 {
return RuntimeResult {
prog: Ok(()),
em: Err("Stack underflow".to_string()),
path: String::new(),
row: 0,
col: 0,
code: 1,
};
}
let b = self.stack.pop().unwrap();
if b == 0 {
return RuntimeResult {
prog: Ok(()),
em: Err("Division by zero".to_string()),
path: String::new(),
row: 0,
col: 0,
code: 1,
};
}
let a = self.stack.pop().unwrap();
self.stack.push(a / b);
}
}
}

RuntimeResult {
prog: Ok(()),
em: Ok(()),
path: String::new(),
row: 0,
col: 0,
code: 0,
}
}
}

#[derive(Debug)]
pub struct RuntimeResult<T = ()> {
pub prog: Result<T, String>,
pub em: Result<(), String>,
pub path: String,
pub row: usize,
pub col: usize,
pub code: i32,
}

impl<T> RuntimeResult<T> {
pub fn expect(self, msg: &str) -> T {
match self.prog {
Ok(v) => v,
Err(_) => panic!("{}", msg),
}
}
}

impl<T> PartialEq<i32> for RuntimeResult<T> {
fn eq(&self, other: &i32) -> bool {
self.code == *other
}
}
