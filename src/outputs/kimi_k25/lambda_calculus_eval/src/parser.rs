//! Parser module for the lambda calculus evaluator.

use crate::common::{AstNode, AstNodeType, AstNodeUnion, Variable, Application, LambdaExpr, tokens_t};
use crate::hash_table::HashTable;

/// Parse a character into a token
pub fn parse_token(c: char) -> tokens_t {
match c {
'(' => tokens_t::L_PAREN,
')' => tokens_t::R_PAREN,
'@' => tokens_t::LAMBDA,
'.' => tokens_t::DOT,
' ' => tokens_t::WHITESPACE,
'\n' => tokens_t::NEWLINE,
'=' => tokens_t::EQ,
'"' => tokens_t::QUOTE,
':' => tokens_t::COLON,
c if c.is_alphabetic() => tokens_t::VARIABLE,
_ => tokens_t::ERROR,
}
}

/// Check if a character is a valid variable character
pub fn is_variable(c: char) -> bool {
c.is_alphabetic()
}

/// Check if a character is uppercase
pub fn is_uppercase(c: char) -> bool {
c.is_uppercase()
}

/// Create a variable AST node
pub fn create_variable(name: &str, type_: &str) -> AstNode {
AstNode {
node: AstNodeUnion::Variable(Variable {
name: name.to_string(),
type_name: type_.to_string(),
}),
type_: AstNodeType::VAR,
name: name.to_string(),
type_name: type_.to_string(),
body: None,
}
}

/// Create an application AST node
pub fn create_application(left: &AstNode, right: &AstNode) -> AstNode {
AstNode {
node: AstNodeUnion::Application(Application {
left: Box::new(left.clone()),
right: Box::new(right.clone()),
}),
type_: AstNodeType::APPLICATION,
name: String::new(),
type_name: String::new(),
body: None,
}
}

/// Create a lambda expression AST node
pub fn create_lambda(param: &str, body: &AstNode, type_: &str) -> AstNode {
AstNode {
node: AstNodeUnion::LambdaExpr(LambdaExpr {
param: param.to_string(),
body: Some(Box::new(body.clone())),
type_name: type_.to_string(),
}),
type_: AstNodeType::LAMBDA_EXPR,
name: param.to_string(),
type_name: type_.to_string(),
body: Some(Box::new(body.clone())),
}
}

/// Alpha conversion (rename variables to avoid capture)
pub fn alpha_convert(name: &str) -> String {
// Simple implementation: append a prime
format!("{}'", name)
}

/// Check if a key is used in the hash table
pub fn is_used(table: &HashTable, key: &str) -> bool {
table.contains(key)
}
