//! Common types and utilities for the lambda calculus evaluator.

/// Token types
pub enum tokens_t {
L_PAREN,
R_PAREN,
LAMBDA,
DOT,
VARIABLE,
WHITESPACE,
NEWLINE,
EQ,
QUOTE,
COLON,
ERROR,
}

/// Types of AST nodes
#[derive(Clone)]
pub enum AstNodeType {
VAR,
APPLICATION,
LAMBDA_EXPR,
DEFINITION,
}

/// Variable structure
#[derive(Clone)]
pub struct Variable {
pub name: String,
pub type_name: String,
}

/// Application structure
#[derive(Clone)]
pub struct Application {
pub left: Box<AstNode>,
pub right: Box<AstNode>,
}

/// Lambda expression structure
#[derive(Clone)]
pub struct LambdaExpr {
pub param: String,
pub body: Option<Box<AstNode>>,
pub type_name: String,
}

/// Union type for AST node contents
#[derive(Clone)]
pub enum AstNodeUnion {
Variable(Variable),
Application(Application),
LambdaExpr(LambdaExpr),
}

/// Abstract Syntax Tree node
#[derive(Clone)]
pub struct AstNode {
pub node: AstNodeUnion,
pub type_: AstNodeType,
pub name: String,
pub type_name: String,
pub body: Option<Box<AstNode>>,
}

impl Default for AstNode {
fn default() -> Self {
AstNode {
node: AstNodeUnion::Variable(Variable {
name: String::new(),
type_name: String::new(),
}),
type_: AstNodeType::VAR,
name: String::new(),
type_name: String::new(),
body: None,
}
}
}
