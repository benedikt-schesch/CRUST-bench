use std::collections::HashMap;
use crate::em::Program;

/// Runtime environment holding variables and state
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

pub fn run(&mut self, _program: &Program) -> RuntimeResult<()> {
// Placeholder implementation for running the program
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

/// Result type for runtime operations with default type parameter
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
