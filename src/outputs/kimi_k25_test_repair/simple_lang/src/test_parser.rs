use simple_lang::lexer::tokenize;
use simple_lang::parser::parse;
use simple_lang::token::TokenType;
fn main() {
let source = "let x = 5 + 3 - 2; dis x;";
let tokens = tokenize(source);
let program = parse(&tokens).unwrap();
let ast_nodes = &program.statements;
assert_eq!(ast_nodes[0].type_, TokenType::TOKEN_LET);
assert_eq!(ast_nodes[0].left.as_ref().unwrap().type_, TokenType::TOKEN_MINUS);
assert_eq!(ast_nodes[0].left.as_ref().unwrap().left.as_ref().unwrap().type_, TokenType::TOKEN_PLUS);
assert_eq!(ast_nodes[0].left.as_ref().unwrap().left.as_ref().unwrap().left.as_ref().unwrap().type_, TokenType::TOKEN_INT);
assert_eq!(ast_nodes[0].left.as_ref().unwrap().left.as_ref().unwrap().right.as_ref().unwrap().type_, TokenType::TOKEN_INT);
assert_eq!(ast_nodes[0].left.as_ref().unwrap().right.as_ref().unwrap().type_, TokenType::TOKEN_INT);
assert_eq!(ast_nodes[1].type_, TokenType::TOKEN_DIS);
assert_eq!(ast_nodes[1].left.as_ref().unwrap().type_, TokenType::TOKEN_PLUS);
assert_eq!(ast_nodes[1].left.as_ref().unwrap().left.as_ref().unwrap().type_, TokenType::TOKEN_IDENTIFIER);
assert_eq!(ast_nodes[1].left.as_ref().unwrap().right.as_ref().unwrap().type_, TokenType::TOKEN_INT);
println!("All parser tests passed!");
}
