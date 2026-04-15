use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use crate::common;
use crate::hash_table;
use crate::io;
use crate::reducer;

static mut N_COUNTER_DUMMY: usize = 0;

fn file_len(in_: &mut File) -> u64 {
let cur = in_.stream_position().unwrap_or(0);
let end = in_.seek(SeekFrom::End(0)).unwrap_or(cur);
let _ = in_.seek(SeekFrom::Start(cur));
end
}

pub fn parse_token(token: char) -> common::tokens_t {
if token == '(' {
common::tokens_t::L_PAREN
} else if token == ')' {
common::tokens_t::R_PAREN
} else if token == '@' {
common::tokens_t::LAMBDA
} else if token == '.' {
common::tokens_t::DOT
} else if is_variable(token) {
common::tokens_t::VARIABLE
} else if token == ' ' || token == '\t' {
common::tokens_t::WHITESPACE
} else if token == '\n' {
common::tokens_t::NEWLINE
} else if token == '=' {
common::tokens_t::EQ
} else if token == '"' {
common::tokens_t::QUOTE
} else if token == ':' {
common::tokens_t::COLON
} else {
common::tokens_t::ERROR
}
}

pub fn p_print_token(token: common::tokens_t) {
match token {
common::tokens_t::L_PAREN => print!("( "),
common::tokens_t::R_PAREN => print!(") "),
common::tokens_t::LAMBDA => print!("@ "),
common::tokens_t::DOT => print!(". "),
common::tokens_t::VARIABLE => print!("VARIABLE "),
common::tokens_t::WHITESPACE => print!("WHITESPACE "),
common::tokens_t::NEWLINE => print!("NEWLINE "),
common::tokens_t::EQ => print!("= "),
_ => print!("ERROR "),
}
}

pub fn p_print_astNode_type(n: &common::AstNode) {
match n.type_ {
common::AstNodeType::LAMBDA_EXPR => println!("AstNode Type: LAMBDA_EXPR"),
common::AstNodeType::APPLICATION => println!("AstNode Type: APPLICATION"),
common::AstNodeType::VAR => println!("AstNode Type: VAR"),
common::AstNodeType::DEFINITION => println!("AstNode Type: DEFINITION"),
}
}

pub fn print_ast(node: &common::AstNode) {
match &node.type_ {
common::AstNodeType::LAMBDA_EXPR => {
if let common::AstNodeUnion::LambdaExpr(lambda) = &node.node {
print!("(LAMBDA {} : {}", lambda.parameter, lambda.type_);
if let Some(body) = &lambda.body {
print_ast(body);
}
print!(") ");
}
}
common::AstNodeType::APPLICATION => {
if let common::AstNodeUnion::Application(app) = &node.node {
print!("(APP ");
if let Some(function) = &app.function {
print_ast(function);
}
if let Some(argument) = &app.argument {
print_ast(argument);
}
print!(") ");
}
}
common::AstNodeType::VAR => {
if let common::AstNodeUnion::Variable(var) = &node.node {
print!("(VAR {} ", var.name);
if !var.type_.is_empty() {
print!(": {}", var.type_);
}
print!(")");
}
}
common::AstNodeType::DEFINITION => {
if let common::AstNodeUnion::Variable(var) = &node.node {
print!("(DEFINITION {}) ", var.name);
}
}
}
}

pub fn is_variable(token: char) -> bool {
token == '_' || token.is_ascii_alphabetic()
}

pub fn peek(in_: &mut File) -> char {
let pos = in_.stream_position().unwrap_or(0);
let mut buf = [0u8; 1];
let c = match in_.read(&mut buf) {
Ok(0) | Err(_) => '\0',
Ok(_) => buf[0] as char,
};
let _ = in_.seek(SeekFrom::Start(pos));
c
}

pub fn peek_print(in_: &mut File, n: usize) {
let pos = in_.stream_position().unwrap_or(0);
let mut buffer = vec![0u8; n];
let read_n = in_.read(&mut buffer).unwrap_or(0);
let s = String::from_utf8_lossy(&buffer[..read_n]);
print!("{}", s);
let _ = in_.seek(SeekFrom::Start(pos));
}

pub fn consume(t: common::tokens_t, in_: &mut File, expected: &str) {
let c = io::next(in_).unwrap_or('\0');
let p = parse_token(c);
if p != t {
expect(expected, c);
}
}

pub fn create_variable(name: &str, type_: &str) -> common::AstNode {
common::AstNode {
type_: common::AstNodeType::VAR,
node: common::AstNodeUnion::Variable(common::Variable {
name: name.to_string(),
type_: type_.to_string(),
}),
}
}

pub fn create_application(function: &common::AstNode, argument: &common::AstNode) -> common::AstNode {
common::AstNode {
type_: common::AstNodeType::APPLICATION,
node: common::AstNodeUnion::Application(common::Application {
function: Some(Box::new(function.clone())),
argument: Some(Box::new(argument.clone())),
}),
}
}

pub fn create_lambda(variable: &str, body: &common::AstNode, type_: &str) -> common::AstNode {
common::AstNode {
type_: common::AstNodeType::LAMBDA_EXPR,
node: common::AstNodeUnion::LambdaExpr(common::LambdaExpression {
parameter: variable.to_string(),
body: Some(Box::new(body.clone())),
type_: type_.to_string(),
}),
}
}

pub fn alpha_convert(old: &str) -> String {
use std::sync::atomic::{AtomicUsize, Ordering};
static N: AtomicUsize = AtomicUsize::new(1);
let current = N.fetch_add(1, Ordering::SeqCst);
format!("{}_{}", old, current)
}

pub fn is_used(table: &hash_table::HashTable, variable: &str) -> bool {
table.table_exists(variable)
}

pub fn parse_space_chars(in_: &mut File) {
let mut c = peek(in_);
while c == ' ' || c == '\n' || c == '\t' {
let _ = io::next(in_);
c = peek(in_);
}
}

pub fn parse_lambda(table: &mut hash_table::HashTable, in_: &mut File) -> common::AstNode {
if parse_token(peek(in_)) != common::tokens_t::VARIABLE {
expect("A variable", peek(in_));
}
let var = parse_variable(in_);
let mut new_var: Option<String> = None;

if is_used(table, &var) {
if table.search(&var).is_some() {
let error_msg = common::format(
"",
format_args!(
"A definition with name {} already exists. Cannot use same name for lambda abstraction.\n",
var
),
);
common::error(&error_msg, file!(), line!() as i32, "parse_lambda");
}
let converted = alpha_convert(&var);
table.insert(&converted, common::AstNode::default());
new_var = Some(converted);
} else {
table.insert(&var, common::AstNode::default());
}

parse_space_chars(in_);
consume(common::tokens_t::COLON, in_, ":");
parse_space_chars(in_);
if parse_token(peek(in_)) != common::tokens_t::VARIABLE {
common::error(
"Lambda abstractions should be typed.",
file!(),
line!() as i32,
"parse_lambda",
);
}
let type_ = parse_type(table, in_);
consume(common::tokens_t::DOT, in_, ".");
let mut body = parse_expression(table, in_);
if let Some(new_name) = new_var {
reducer::replace(&mut body, &var, &new_name);
common::print_verbose(
"",
format_args!("Alpha converted {} to {}\n", var, new_name),
);
return create_lambda(&new_name, &body, &type_);
}
create_lambda(&var, &body, &type_)
}

pub fn parse_expression(table: &mut hash_table::HashTable, in_: &mut File) -> common::AstNode {
while {
let tk = parse_token(peek(in_));
tk == common::tokens_t::WHITESPACE || tk == common::tokens_t::NEWLINE
} {
let _ = io::next(in_);
}

let scanned = parse_token(peek(in_));
if scanned == common::tokens_t::ERROR {
println!("Error: {} is  a valid token", peek(in_));
std::process::exit(1);
}

if scanned == common::tokens_t::LAMBDA {
let _ = io::next(in_);
parse_lambda(table, in_)
} else if scanned == common::tokens_t::L_PAREN {
let _ = io::next(in_);
let expr = parse_expression(table, in_);
print_ast(&expr);
let next_token = parse_token(peek(in_));

if next_token == common::tokens_t::WHITESPACE {
let expr_2 = parse_expression(table, in_);
let application = create_application(&expr, &expr_2);
consume(common::tokens_t::R_PAREN, in_, ")");
return application;
}
consume(common::tokens_t::R_PAREN, in_, ")");
expr
} else if scanned == common::tokens_t::VARIABLE {
let var_name = parse_variable(in_);
if var_name == "def" {
parse_definition(table, in_);
if in_.stream_position().unwrap_or(0) < file_len(in_) {
return parse_expression(table, in_);
}
return common::AstNode::default();
} else if var_name == "import" {
parse_import(table, in_);
if in_.stream_position().unwrap_or(0) < file_len(in_) {
return parse_expression(table, in_);
}
return common::AstNode::default();
} else if var_name == "type" {
parse_type_definition(table, in_);
if in_.stream_position().unwrap_or(0) < file_len(in_) {
return parse_expression(table, in_);
}
return common::AstNode::default();
}

let mut type_ = String::new();
parse_space_chars(in_);

if !table.table_exists(&var_name) {
if parse_token(peek(in_)) != common::tokens_t::COLON {
let error_msg = common::format(
"",
format_args!(
"Constant Variable {} is not typed. Please provide a type.\n",
var_name
),
);
common::error(&error_msg, file!(), line!() as i32, "parse_expression");
}
consume(common::tokens_t::COLON, in_, ":");
parse_space_chars(in_);
type_ = parse_type(table, in_);
}
let mut variable = create_variable(&var_name, &type_);
if table.search(&var_name).is_some() {
variable.type_ = common::AstNodeType::DEFINITION;
}
variable
} else {
common::AstNode::default()
}
}

pub fn parse_import(table: &mut hash_table::HashTable, in_: &mut File) {
consume(common::tokens_t::WHITESPACE, in_, "a whitespace");
consume(common::tokens_t::QUOTE, in_, "\"");

let mut file_path = String::new();
let mut next_token = io::next(in_).unwrap_or('\0');
let mut n = parse_token(next_token);

while n != common::tokens_t::QUOTE {
if file_path.len() < 99 {
file_path.push(next_token);
} else {
common::error(
"File path is too long. Please make sure it is less than 100 characters.",
file!(),
line!() as i32,
"parse_import",
);
}
next_token = io::next(in_).unwrap_or('\0');
n = parse_token(next_token);
if next_token == '\0' {
break;
}
}

if n != common::tokens_t::QUOTE {
expect("a closing quote", next_token);
}

let mut imported_file = crate::io::get_file(&file_path, "r").unwrap_or_else(|_| {
common::error(
&format!("Could not open file {}", file_path),
file!(),
line!() as i32,
"parse_import",
);
unreachable!()
});

while imported_file.stream_position().unwrap_or(0) < file_len(&mut imported_file) {
let imported_tkn = peek(&mut imported_file);
let scanned = parse_token(imported_tkn);
parse_space_chars(&mut imported_file);
if scanned == common::tokens_t::VARIABLE {
let var_name = parse_variable(&mut imported_file);
if var_name == "def" {
parse_definition(table, &mut imported_file);
} else if var_name == "type" {
parse_type_definition(table, &mut imported_file);
} else {
let error_msg = common::format(
"",
format_args!(
"Expected a definition in the imported file, but got {}\n",
var_name
),
);
common::error(&error_msg, file!(), line!() as i32, "parse_import");
}
} else {
let _ = io::next(&mut imported_file);
}
}
}

pub fn parse_definition(table: &mut hash_table::HashTable, in_: &mut File) {
consume(common::tokens_t::WHITESPACE, in_, "a whitespace");
if parse_token(peek(in_)) != common::tokens_t::VARIABLE {
expect("a variable", peek(in_));
}
let def_name = parse_variable(in_);
consume(common::tokens_t::WHITESPACE, in_, "a whitespace");
consume(common::tokens_t::EQ, in_, "=");
consume(common::tokens_t::WHITESPACE, in_, "a whitespace");
let definition = parse_expression(table, in_);
table.insert(&def_name, definition);
}

pub fn is_uppercase(c: char) -> bool {
c.is_ascii_uppercase()
}

pub fn parse_type_definition(types_table: &mut hash_table::HashTable, in_: &mut File) {
let next_token = io::next(in_).unwrap_or('\0');
let n = parse_token(next_token);
if n != common::tokens_t::WHITESPACE {
expect(" ", next_token);
}

let next_token = peek(in_);
let n = parse_token(next_token);
if n != common::tokens_t::VARIABLE {
expect("a type definition", next_token);
}
if !is_uppercase(next_token) {
common::error(
"Type names must start with an uppercase letter",
file!(),
line!() as i32,
"parse_type_definition",
);
}
let type_name = parse_variable(in_);
if types_table.table_exists(&type_name) {
let error_msg = common::format("", format_args!("Type {} was already defined.\n", type_name));
common::error(&error_msg, file!(), line!() as i32, "parse_type_definition");
}
types_table.insert(&type_name, common::AstNode::default());
}

pub fn parse_type(types_table: &mut hash_table::HashTable, in_: &mut File) -> String {
let mut type_name = String::new();
let token = io::next(in_).unwrap_or('\0');
if !is_uppercase(token) {
common::error(
"Types should start with an uppercase letter.",
file!(),
line!() as i32,
"parse_type",
);
}
type_name.push(token);
while is_variable(peek(in_)) {
type_name.push(io::next(in_).unwrap_or('\0'));
}
if !types_table.table_exists(&type_name) {
let error_msg = common::format("", format_args!("Type {} was not defined.\n", type_name));
common::error(&error_msg, file!(), line!() as i32, "parse_type");
}
type_name
}

pub fn parse_variable(in_: &mut File) -> String {
let mut variable_name = String::new();
while is_variable(peek(in_)) {
variable_name.push(io::next(in_).unwrap_or('\0'));
}
variable_name
}

pub fn expect(expected: &str, received: char) {
println!("Syntax Error: Expected {} , received {} ", expected, received);
std::process::exit(1);
}

pub fn free_ast(_node: &mut common::AstNode) {}
