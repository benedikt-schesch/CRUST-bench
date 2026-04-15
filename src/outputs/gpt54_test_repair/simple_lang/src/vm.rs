
use crate::settings;
pub const SIMPLE_LANG_VM_H: bool = true;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
LOAD_CONST,
LOAD_NAME,
STORE_NAME,
BINARY_ADD,
BINARY_SUB,
STK_DIS,
}
#[derive(Debug, Clone)]
pub struct Instruction {
pub opcode: OpCode,
pub operand: String,
}
#[derive(Debug, Clone)]
pub struct Frame {
pub stack: [i32; settings::STAKE_LENGTH as usize],
pub sp: i32,
pub variables: [i32; 100],
pub var_names: [String; 100],
pub var_count: i32,
}
pub fn new_instruction(opcode: OpCode, operand: &str) -> Instruction {
Instruction {
opcode,
operand: operand.to_string(),
}
}
pub fn free_instruction(_instruction: &mut Instruction) {}
pub fn eval(frame: &mut Frame, instructions: &[Instruction]) {
for instr in instructions {
match instr.opcode {
OpCode::LOAD_CONST => {
frame.sp += 1;
frame.stack[frame.sp as usize] = instr.operand.parse::<i32>().unwrap_or(0);
}
OpCode::LOAD_NAME => {
for i in 0..frame.var_count as usize {
if frame.var_names[i] == instr.operand {
frame.sp += 1;
frame.stack[frame.sp as usize] = frame.variables[i];
break;
}
}
}
OpCode::STORE_NAME => {
let mut found_index: Option<usize> = None;
for i in 0..frame.var_count as usize {
if frame.var_names[i] == instr.operand {
found_index = Some(i);
break;
}
}
if let Some(i) = found_index {
frame.variables[i] = frame.stack[frame.sp as usize];
frame.sp -= 1;
} else {
let idx = frame.var_count as usize;
frame.var_names[idx] = instr.operand.clone();
frame.variables[idx] = frame.stack[frame.sp as usize];
frame.sp -= 1;
frame.var_count += 1;
}
}
OpCode::BINARY_ADD => {
let sp = frame.sp as usize;
frame.stack[sp - 1] = frame.stack[sp - 1] + frame.stack[sp];
frame.sp -= 1;
}
OpCode::BINARY_SUB => {
let sp = frame.sp as usize;
frame.stack[sp - 1] = frame.stack[sp - 1] - frame.stack[sp];
frame.sp -= 1;
}
OpCode::STK_DIS => {
println!("{}", frame.stack[frame.sp as usize]);
}
}
}
}
pub fn init_frame() -> Frame {
Frame {
stack: [0; settings::STAKE_LENGTH as usize],
sp: 0,
variables: [0; 100],
var_names: std::array::from_fn(|_| String::new()),
var_count: 0,
}
}
pub fn free_frame(_frame: &mut Frame) {}
