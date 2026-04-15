use crate::token::TokenType;

#[derive(Debug)]
pub struct Program {
pub statements: Vec<Node>,
}

impl Program {
pub fn new() -> Self {
Program {
statements: Vec::new(),
}
}
}

#[derive(Debug)]
pub struct Node {
pub type_: TokenType,
pub left: Option<Box<Node>>,
pub right: Option<Box<Node>>,
pub value: String,
}

impl Node {
pub fn new(type_: TokenType, value: String) -> Self {
Node {
type_,
left: None,
right: None,
value,
}
}
}

// Keep old types for compatibility if needed, but tests use the Node structure
#[derive(Debug)]
pub enum Statement {
Expression(Expression),
Print(Expression),
VarDeclaration(String, Option<Expression>),
Block(Vec<Statement>),
}

#[derive(Debug)]
pub enum Expression {
Number(f64),
String(String),
Identifier(String),
Binary(Box<Expression>, BinaryOp, Box<Expression>),
Unary(UnaryOp, Box<Expression>),
}

#[derive(Debug)]
pub enum BinaryOp {
Add,
Subtract,
Multiply,
Divide,
Equal,
}

#[derive(Debug)]
pub enum UnaryOp {
Negate,
Not,
}
