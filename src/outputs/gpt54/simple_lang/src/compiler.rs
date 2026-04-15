// Generated Rust Code
use crate::{ast, lexer, parser, token, vm};

pub const SIMPLE_LANG_COMPILER_H: bool = true;

/// Replicates: Instruction* compile_asts(ASTNode** asts, int* instr_count);
/// In Rust: returns a vector of Instructions, updates an integer by reference.
pub fn compile_asts(asts: &[Box<ast::ASTNode>], instr_count: &mut i32) -> Vec<vm::Instruction> {
*instr_count = 0;

let mut instr_of_all: Vec<Vec<vm::Instruction>> = Vec::new();
let mut instr_count_of_all: Vec<i32> = Vec::new();

for ast_node in asts {
let mut count = 0;
let instrs = compile_statement(ast_node, &mut count);
instr_of_all.push(instrs);
instr_count_of_all.push(count);
}

for count in &instr_count_of_all {
*instr_count += *count;
}

let mut all_instr = Vec::new();
for instrs in instr_of_all {
all_instr.extend(instrs);
}

all_instr
}

/// Replicates: Instruction* compile(const char* source, int* instr_count);
pub fn compile(source: &str, instr_count: &mut i32) -> Vec<vm::Instruction> {
let tokens = lexer::tokenize(source);
let ast_nodes = parser::parse(&tokens);
compile_asts(&ast_nodes, instr_count)
}

/// Replicates: void compile_expression(ASTNode* node, Instruction* instructions, int* count);
/// In Rust: modifies a mutable vector of Instructions and a count reference.
pub fn compile_expression(
node: &ast::ASTNode,
instructions: &mut Vec<vm::Instruction>,
count: &mut i32,
) {
if node.type_ == token::TokenType::TOKEN_INT {
emit(instructions, count, vm::OpCode::LOAD_CONST, &node.value);
} else if node.type_ == token::TokenType::TOKEN_IDENTIFIER {
emit(instructions, count, vm::OpCode::LOAD_NAME, &node.value);
} else if node.type_ == token::TokenType::TOKEN_PLUS {
if let Some(left) = &node.left {
compile_expression(left, instructions, count);
}
if let Some(right) = &node.right {
compile_expression(right, instructions, count);
}
emit(instructions, count, vm::OpCode::BINARY_ADD, "");
} else if node.type_ == token::TokenType::TOKEN_MINUS {
if let Some(left) = &node.left {
compile_expression(left, instructions, count);
}
if let Some(right) = &node.right {
compile_expression(right, instructions, count);
}
emit(instructions, count, vm::OpCode::BINARY_SUB, "");
}
}

/// Replicates: Instruction* compile_statement(ASTNode* ast, int* instr_count);
pub fn compile_statement(ast: &ast::ASTNode, instr_count: &mut i32) -> Vec<vm::Instruction> {
let mut instructions = Vec::new();
*instr_count = 0;

if ast.type_ == token::TokenType::TOKEN_LET || ast.type_ == token::TokenType::TOKEN_ASSIGN {
if let Some(left) = &ast.left {
compile_expression(left, &mut instructions, instr_count);
}
emit(
&mut instructions,
instr_count,
vm::OpCode::STORE_NAME,
&ast.value,
);
} else if ast.type_ == token::TokenType::TOKEN_DIS {
if let Some(left) = &ast.left {
compile_expression(left, &mut instructions, instr_count);
}
emit(&mut instructions, instr_count, vm::OpCode::STK_DIS, &ast.value);
}

instructions
}

/// Replicates: void emit(Instruction* instructions, int* count, OpCode opcode, const char* operand);
pub fn emit(
instructions: &mut Vec<vm::Instruction>,
count: &mut i32,
opcode: vm::OpCode,
operand: &str,
) {
instructions.push(vm::new_instruction(opcode, operand));
*count += 1;
}
