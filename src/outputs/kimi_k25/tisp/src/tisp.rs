use std::io::Write;
use crate::core;
use crate::math;
use crate::string;

pub struct Tsp {
// Environment state for the TISP interpreter
memory_size: usize,
pub file: String,
pub filec: usize,
}

pub struct Value {
// Represents a TISP value
data: String,
}

impl Tsp {
pub fn new(size: usize) -> Self {
Tsp {
memory_size: size,
file: String::new(),
filec: 0,
}
}
}

pub fn tisp_env_init(size: usize) -> Tsp {
Tsp::new(size)
}

pub fn tisp_read(_st: &mut Tsp) -> Option<Value> {
// Read and parse input into a Value
None
}

pub fn tisp_eval(_st: &mut Tsp, val: Value) -> Option<Value> {
// Evaluate the given value in the environment
Some(val)
}

pub fn tisp_print<W: Write>(_writer: &mut W, _val: &Value) {
// Print the value to the given writer
}

pub fn tib_env_core(st: &mut Tsp) {
core::init(st);
}

pub fn tib_env_math(st: &mut Tsp) {
math::init(st);
}

pub fn tib_env_string(st: &mut Tsp) {
string::init(st);
}

pub fn tisp_env_lib(_st: &mut Tsp, _path: &str) {
// Load external library from path
}
