#[derive(Debug, Clone, PartialEq)]
pub enum OpCodeType {
LOAD_CONST,
BINARY_ADD,
BINARY_SUB,
STORE_NAME,
LOAD_NAME,
STK_DIS,
Constant(f64),
Add,
Subtract,
Multiply,
Divide,
Negate,
Return,
Print,
Pop,
DefineGlobal(String),
GetGlobal(String),
SetGlobal(String),
}

#[derive(Debug, Clone)]
pub struct OpCode {
pub opcode: OpCodeType,
pub operand: String,
}

impl OpCode {
pub fn new(opcode: OpCodeType, operand: String) -> Self {
OpCode { opcode, operand }
}
}
