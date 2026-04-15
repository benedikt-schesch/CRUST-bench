use crate::ast::{Program, Node};
use crate::vm::{OpCode, OpCodeType};
use crate::token::TokenType;

pub fn compile(program: &Program) -> Result<Vec<OpCode>, String> {
let mut bytecode = Vec::new();

for stmt in &program.statements {
compile_node(stmt, &mut bytecode)?;
}

Ok(bytecode)
}

fn compile_node(node: &Node, bytecode: &mut Vec<OpCode>) -> Result<(), String> {
match node.type_ {
TokenType::TOKEN_LET => {
// Left child is the expression, value is the variable name
if let Some(ref expr) = node.left {
compile_node(expr, bytecode)?;
}
bytecode.push(OpCode::new(OpCodeType::STORE_NAME, node.value.clone()));
}
TokenType::TOKEN_DIS => {
if let Some(ref expr) = node.left {
compile_node(expr, bytecode)?;
}
bytecode.push(OpCode::new(OpCodeType::STK_DIS, "".to_string()));
}
TokenType::TOKEN_INT => {
bytecode.push(OpCode::new(OpCodeType::LOAD_CONST, node.value.clone()));
}
TokenType::TOKEN_IDENTIFIER => {
bytecode.push(OpCode::new(OpCodeType::LOAD_NAME, node.value.clone()));
}
TokenType::TOKEN_PLUS => {
if let Some(ref left) = node.left {
compile_node(left, bytecode)?;
}
if let Some(ref right) = node.right {
compile_node(right, bytecode)?;
}
bytecode.push(OpCode::new(OpCodeType::BINARY_ADD, "".to_string()));
}
TokenType::TOKEN_MINUS => {
if let Some(ref left) = node.left {
compile_node(left, bytecode)?;
}
if let Some(ref right) = node.right {
compile_node(right, bytecode)?;
}
bytecode.push(OpCode::new(OpCodeType::BINARY_SUB, "".to_string()));
}
_ => {}
}
Ok(())
}
