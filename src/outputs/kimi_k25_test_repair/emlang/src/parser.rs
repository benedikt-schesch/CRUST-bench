use crate::em::Program;
use crate::env::RuntimeResult;
use crate::data::Instruction;

pub struct Parser {
source: String,
path: String,
}

impl Parser {
pub fn new() -> Self {
Parser {
source: String::new(),
path: String::new(),
}
}

pub fn load_file(&mut self, path: &str) -> RuntimeResult<()> {
self.path = path.to_string();
match std::fs::read_to_string(path) {
Ok(content) => {
self.source = content;
RuntimeResult {
prog: Ok(()),
em: Ok(()),
path: path.to_string(),
row: 0,
col: 0,
code: 0,
}
}
Err(e) => RuntimeResult {
prog: Err(format!("Failed to load file: {}", e)),
em: Err(format!("Failed to load file: {}", e)),
path: path.to_string(),
row: 0,
col: 0,
code: 1,
}
}
}

pub fn parse(&self) -> RuntimeResult<Program> {
let mut program = Program::new();

for line in self.source.lines() {
for token in line.split_whitespace() {
match token {
"+" => program.instructions.push(Instruction::Add),
"-" => program.instructions.push(Instruction::Sub),
"*" => program.instructions.push(Instruction::Mul),
"/" => program.instructions.push(Instruction::Div),
_ => {
if let Ok(n) = token.parse::<i64>() {
program.instructions.push(Instruction::Push(n));
}
}
}
}
}

RuntimeResult {
prog: Ok(program),
em: Ok(()),
path: self.path.clone(),
row: 0,
col: 0,
code: 0,
}
}
}
