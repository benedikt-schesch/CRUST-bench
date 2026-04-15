use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};

static VERBOSE_MODE: AtomicBool = AtomicBool::new(false);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum tokens_t {
L_PAREN,
R_PAREN,
LAMBDA,
DOT,
VARIABLE,
ERROR,
WHITESPACE,
NEWLINE,
EQ,
QUOTE,
COLON,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AstNodeType {
LAMBDA_EXPR,
APPLICATION,
VAR,
DEFINITION,
}

#[derive(Debug, Clone)]
pub struct LambdaExpression {
pub parameter: String,
pub type_: String,
pub body: Option<Box<AstNode>>,
}

#[derive(Debug, Clone)]
pub struct Application {
pub function: Option<Box<AstNode>>,
pub argument: Option<Box<AstNode>>,
}

#[derive(Debug, Clone)]
pub struct Variable {
pub name: String,
pub type_: String,
}

#[derive(Debug, Clone)]
pub enum AstNodeUnion {
LambdaExpr(LambdaExpression),
Application(Application),
Variable(Variable),
}

#[derive(Debug, Clone)]
pub struct AstNode {
pub type_: AstNodeType,
pub node: AstNodeUnion,
}

impl Default for AstNode {
fn default() -> Self {
AstNode {
type_: AstNodeType::VAR,
node: AstNodeUnion::Variable(Variable {
name: String::new(),
type_: String::new(),
}),
}
}
}

pub fn set_verbose(verbose: bool) {
VERBOSE_MODE.store(verbose, Ordering::SeqCst);
}

pub fn print_ast_verbose(n: &AstNode) {
if !VERBOSE_MODE.load(Ordering::SeqCst) {
return;
}
let lambda_ast = ast_to_string(n);
println!("{}", lambda_ast);
}

pub fn print_verbose(format: &str, args: fmt::Arguments) {
if !VERBOSE_MODE.load(Ordering::SeqCst) {
return;
}
println!();
print!("{}", format_args!("{}", format));
print!("{}", args);
}

pub fn error(msg: &str, file: &str, line: i32, func: &str) {
eprintln!("ERROR: {} at {}:{} in {}()", msg, file, line, func);
std::process::exit(1);
}

pub fn format(_fmt: &str, args: fmt::Arguments) -> String {
fmt::format(args)
}

pub fn append_to_buffer(buffer: &mut String, str_: &str) {
buffer.push_str(str_);
}

pub fn append_ast_to_buffer(buffer: &mut String, node: &AstNode) {
match &node.type_ {
AstNodeType::LAMBDA_EXPR => {
append_to_buffer(buffer, "(@");
if let AstNodeUnion::LambdaExpr(lambda) = &node.node {
append_to_buffer(buffer, &lambda.parameter);
append_to_buffer(buffer, " : ");
append_to_buffer(buffer, &lambda.type_);
append_to_buffer(buffer, " .");
if let Some(body) = &lambda.body {
append_ast_to_buffer(buffer, body);
}
append_to_buffer(buffer, ") ");
}
}
AstNodeType::APPLICATION => {
append_to_buffer(buffer, "(");
if let AstNodeUnion::Application(app) = &node.node {
if let Some(function) = &app.function {
append_ast_to_buffer(buffer, function);
}
if let Some(argument) = &app.argument {
append_ast_to_buffer(buffer, argument);
}
append_to_buffer(buffer, ") ");
}
}
AstNodeType::VAR => {
append_to_buffer(buffer, "(");
if let AstNodeUnion::Variable(var) = &node.node {
append_to_buffer(buffer, &var.name);
if !var.type_.is_empty() {
append_to_buffer(buffer, " : ");
append_to_buffer(buffer, &var.type_);
}
append_to_buffer(buffer, ") ");
}
}
AstNodeType::DEFINITION => {
append_to_buffer(buffer, "(");
if let AstNodeUnion::Variable(var) = &node.node {
append_to_buffer(buffer, &var.name);
append_to_buffer(buffer, ") ");
}
}
}
}

pub fn ast_to_string(node: &AstNode) -> String {
let mut buffer = String::new();
append_ast_to_buffer(&mut buffer, node);
buffer
}
