use crate::token::{Token, TokenType};
use crate::ast::{Program, Node};

pub fn parse(tokens: &[Token]) -> Result<Program, String> {
let mut program = Program::new();
let mut pos = 0;

while pos < tokens.len() && tokens[pos].token_type != TokenType::TOKEN_EOF {
let (stmt, new_pos) = parse_statement(tokens, pos)?;
program.statements.push(stmt);
pos = new_pos;
}

Ok(program)
}

fn parse_statement(tokens: &[Token], pos: usize) -> Result<(Node, usize), String> {
if pos >= tokens.len() {
return Err("Unexpected end of input".to_string());
}

let token = &tokens[pos];
match token.token_type {
TokenType::TOKEN_LET => {
// let identifier = expression;
let ident_token = &tokens[pos + 1];
if ident_token.token_type != TokenType::TOKEN_IDENTIFIER {
return Err("Expected identifier after let".to_string());
}

// Skip =
let expr_pos = pos + 3;
let (expr, new_pos) = parse_expression(tokens, expr_pos)?;

// Create a let node with the identifier as value and expression as left child
let mut let_node = Node::new(TokenType::TOKEN_LET, ident_token.lexeme.clone());
let_node.left = Some(Box::new(expr));

// Skip semicolon
let final_pos = new_pos + 1;
Ok((let_node, final_pos))
}
TokenType::TOKEN_DIS => {
// dis expression;
let (expr, new_pos) = parse_expression(tokens, pos + 1)?;
let mut dis_node = Node::new(TokenType::TOKEN_DIS, "".to_string());
dis_node.left = Some(Box::new(expr));
// Skip semicolon
Ok((dis_node, new_pos + 1))
}
_ => {
let (expr, new_pos) = parse_expression(tokens, pos)?;
Ok((expr, new_pos))
}
}
}

fn parse_expression(tokens: &[Token], pos: usize) -> Result<(Node, usize), String> {
if pos >= tokens.len() {
return Err("Unexpected end of input".to_string());
}

parse_additive(tokens, pos)
}

fn parse_additive(tokens: &[Token], pos: usize) -> Result<(Node, usize), String> {
let (mut left, mut pos) = parse_multiplicative(tokens, pos)?;

while pos < tokens.len() {
match tokens[pos].token_type {
TokenType::TOKEN_PLUS | TokenType::TOKEN_MINUS => {
let op_type = tokens[pos].token_type.clone();
let (right, new_pos) = parse_multiplicative(tokens, pos + 1)?;
let mut node = Node::new(op_type, "".to_string());
node.left = Some(Box::new(left));
node.right = Some(Box::new(right));
left = node;
pos = new_pos;
}
_ => break,
}
}

Ok((left, pos))
}

fn parse_multiplicative(tokens: &[Token], pos: usize) -> Result<(Node, usize), String> {
let (mut left, mut pos) = parse_primary(tokens, pos)?;

while pos < tokens.len() {
match tokens[pos].token_type {
TokenType::Star | TokenType::Slash => {
let op_type = if tokens[pos].token_type == TokenType::Star {
TokenType::Star
} else {
TokenType::Slash
};
let (right, new_pos) = parse_primary(tokens, pos + 1)?;
let mut node = Node::new(op_type, "".to_string());
node.left = Some(Box::new(left));
node.right = Some(Box::new(right));
left = node;
pos = new_pos;
}
_ => break,
}
}

Ok((left, pos))
}

fn parse_primary(tokens: &[Token], pos: usize) -> Result<(Node, usize), String> {
if pos >= tokens.len() {
return Err("Unexpected end of input".to_string());
}

let token = &tokens[pos];
match token.token_type {
TokenType::TOKEN_INT => {
let mut node = Node::new(TokenType::TOKEN_INT, token.lexeme.clone());
Ok((node, pos + 1))
}
TokenType::TOKEN_IDENTIFIER => {
let mut node = Node::new(TokenType::TOKEN_IDENTIFIER, token.lexeme.clone());
Ok((node, pos + 1))
}
_ => Err(format!("Unexpected token: {:?}", token.token_type)),
}
}
