use simple_lang::lexer::tokenize;
use simple_lang::parser::parse;
use simple_lang::compiler::compile;
use simple_lang::vm::{OpCode, OpCodeType};

fn main() {
let source = "let x = 5 + 3 - 2; dis x;";
let tokens = tokenize(source);
let program = parse(&tokens).unwrap();
let instructions = compile(&program).unwrap();

assert_eq!(instructions[0].opcode, OpCodeType::LOAD_CONST);
assert_eq!(instructions[0].operand, "5");

assert_eq!(instructions[1].opcode, OpCodeType::LOAD_CONST);
assert_eq!(instructions[1].operand, "3");

assert_eq!(instructions[2].opcode, OpCodeType::BINARY_ADD);

assert_eq!(instructions[3].opcode, OpCodeType::LOAD_CONST);
assert_eq!(instructions[3].operand, "2");

assert_eq!(instructions[4].opcode, OpCodeType::BINARY_SUB);

assert_eq!(instructions[5].opcode, OpCodeType::STORE_NAME);
assert_eq!(instructions[5].operand, "x");

assert_eq!(instructions[6].opcode, OpCodeType::LOAD_NAME);
assert_eq!(instructions[6].operand, "x");

assert_eq!(instructions[7].opcode, OpCodeType::STK_DIS);

println!("All tests passed!");
}
