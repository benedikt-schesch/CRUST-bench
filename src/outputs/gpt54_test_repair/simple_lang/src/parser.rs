
use crate::{ast, token};
use std::cell::RefCell;
pub const SIMPLE_LANG_PARSER_H: bool = true;
thread_local! {
static PARSER_STATE: RefCell<(Vec<token::Token>, usize)> = const { RefCell::new((Vec::new(), 0)) };
}
pub fn consume() -> token::Token {
PARSER_STATE.with(|state| {
let mut state = state.borrow_mut();
let tok = state.0[state.1].clone();
state.1 += 1;
tok
})
}
pub fn parse_statement() -> Box<ast::ASTNode> {
let token = consume();
if token.token_type == token::TokenType::TOKEN_LET {
let identifier = consume();
if identifier.token_type != token::TokenType::TOKEN_IDENTIFIER {
panic!("Expected identifier after 'let'");
}
if consume().token_type != token::TokenType::TOKEN_ASSIGN {
panic!("Expected '=' after identifier");
}
let expr = parse_expression();
if consume().token_type != token::TokenType::TOKEN_SEMICOLON {
panic!("Expected ';' at end of statement");
}
let mut node = ast::new_ast_node(token::TokenType::TOKEN_LET, &identifier.value);
node.left = Some(expr);
node
} else if token.token_type == token::TokenType::TOKEN_IDENTIFIER {
let identifier = token;
if consume().token_type != token::TokenType::TOKEN_ASSIGN {
panic!("Expected '=' after identifier");
}
let expr = parse_expression();
if consume().token_type != token::TokenType::TOKEN_SEMICOLON {
panic!("Expected ';' at end of statement");
}
let mut node = ast::new_ast_node(token::TokenType::TOKEN_ASSIGN, &identifier.value);
node.left = Some(expr);
node
} else if token.token_type == token::TokenType::TOKEN_DIS {
let expr = parse_expression();
let mut node = ast::new_ast_node(token::TokenType::TOKEN_DIS, "");
if consume().token_type != token::TokenType::TOKEN_SEMICOLON {
panic!("Expected ';' at end of statement");
}
node.left = Some(expr);
node
} else {
panic!("Unexpected token: {}", token.value);
}
}
pub fn parse_primary() -> Box<ast::ASTNode> {
let token = consume();
if token.token_type == token::TokenType::TOKEN_INT {
ast::new_ast_node(token::TokenType::TOKEN_INT, &token.value)
} else if token.token_type == token::TokenType::TOKEN_IDENTIFIER {
ast::new_ast_node(token::TokenType::TOKEN_IDENTIFIER, &token.value)
} else {
panic!("Unexpected token in primary expression: {}", token.value);
}
}
pub fn parse_expression() -> Box<ast::ASTNode> {
let mut left = parse_primary();
let mut tok = lookahead();
while tok.token_type == token::TokenType::TOKEN_PLUS
|| tok.token_type == token::TokenType::TOKEN_MINUS
{
let op = consume();
let right = parse_primary();
let mut node = ast::new_ast_node(op.token_type.clone(), &op.value);
node.left = Some(left);
node.right = Some(right);
left = node;
tok = lookahead();
}
left
}
pub fn parse(tokens_array: &[token::Token]) -> Vec<Box<ast::ASTNode>> {
PARSER_STATE.with(|state| {
*state.borrow_mut() = (tokens_array.to_vec(), 0);
});
let mut ast_nodes = Vec::new();
loop {
let tok = lookahead();
if tok.token_type == token::TokenType::TOKEN_EOF {
break;
}
ast_nodes.push(parse_statement());
}
ast_nodes
}
pub fn lookahead() -> token::Token {
PARSER_STATE.with(|state| {
let state = state.borrow();
state.0[state.1].clone()
})
}
