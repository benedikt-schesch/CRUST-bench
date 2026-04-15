use std::sync::{Mutex, OnceLock};

use crate::common;
use crate::config;
use crate::hash_table;
use crate::parser;

pub const SIZE: usize = 122;

fn reduction_order_cell() -> &'static Mutex<config::reduction_order_t> {
static CELL: OnceLock<Mutex<config::reduction_order_t>> = OnceLock::new();
CELL.get_or_init(|| Mutex::new(config::reduction_order_t::APPLICATIVE))
}

pub fn set_reduction_order(t: config::reduction_order_t) {
let mut guard = reduction_order_cell().lock().unwrap();
*guard = t;
}

pub fn print_reduction_order(t: config::reduction_order_t) {
match t {
config::reduction_order_t::APPLICATIVE => print!("Applicative"),
config::reduction_order_t::NORMAL => print!("Normal"),
}
println!();
}

pub fn reduce(table: &mut hash_table::HashTable, n: &common::AstNode) -> common::AstNode {
common::print_verbose("Order of reduction is: ", format_args!(""));
let current = reduction_order_cell().lock().unwrap().clone();
print_reduction_order(current.clone());
common::print_verbose("-------------------------------------------\n", format_args!(""));

let mut expanded = deepcopy(n);
expand_definitions(table, &mut expanded);
common::print_verbose("Expanded expression:\n", format_args!(""));
common::print_ast_verbose(&expanded);

let reduced = reduce_ast(table, &expanded);
common::print_verbose("Final reduced expression:\n", format_args!(""));
common::print_ast_verbose(&reduced);
common::print_verbose("-------------------------------------------\n", format_args!(""));
reduced
}

pub fn expand_definitions(table: &mut hash_table::HashTable, n: &mut common::AstNode) {
match &mut n.node {
common::AstNodeUnion::LambdaExpr(lambda) if n.type_ == common::AstNodeType::LAMBDA_EXPR => {
if let Some(body) = &mut lambda.body {
expand_definitions(table, body);
}
}
common::AstNodeUnion::Application(app) if n.type_ == common::AstNodeType::APPLICATION => {
if let Some(function) = &mut app.function {
expand_definitions(table, function);
}
if let Some(argument) = &mut app.argument {
expand_definitions(table, argument);
}
}
common::AstNodeUnion::Variable(var) if n.type_ == common::AstNodeType::DEFINITION => {
let def_name = var.name.clone();
let expanded_def = table.search(&def_name).cloned().unwrap_or_else(|| {
common::error("Null pointer encountered", file!(), line!() as i32, "expand_definitions");
unreachable!()
});
common::print_verbose(
"",
format_args!("Expanding definition of: {} . Term expanded to:\n", def_name),
);
common::print_ast_verbose(&expanded_def);
*n = expanded_def;
}
_ => {}
}
}

pub fn replace(n: &mut common::AstNode, old: &str, new_name: &str) {
match &mut n.node {
common::AstNodeUnion::LambdaExpr(lambda) if n.type_ == common::AstNodeType::LAMBDA_EXPR => {
if lambda.parameter == old {
lambda.parameter = new_name.to_string();
}
if let Some(body) = &mut lambda.body {
replace(body, old, new_name);
}
}
common::AstNodeUnion::Application(app) if n.type_ == common::AstNodeType::APPLICATION => {
if let Some(function) = &mut app.function {
replace(function, old, new_name);
}
if let Some(argument) = &mut app.argument {
replace(argument, old, new_name);
}
}
common::AstNodeUnion::Variable(var) if n.type_ == common::AstNodeType::VAR => {
if var.name == old {
var.name = new_name.to_string();
}
}
_ => {}
}
}

pub fn reduce_ast(table: &mut hash_table::HashTable, n: &common::AstNode) -> common::AstNode {
match &n.node {
common::AstNodeUnion::LambdaExpr(lambda) if n.type_ == common::AstNodeType::LAMBDA_EXPR => {
let order = reduction_order_cell().lock().unwrap().clone();
if let Some(body) = &lambda.body {
if matches!(order, config::reduction_order_t::APPLICATIVE) {
return deepcopy_lambda_expr(
&lambda.parameter,
&reduce_ast(table, body),
&lambda.type_,
);
}
}
deepcopy(n)
}
common::AstNodeUnion::Application(app) if n.type_ == common::AstNodeType::APPLICATION => {
let function = app.function.as_ref().map(|f| reduce_ast(table, f));
let order = reduction_order_cell().lock().unwrap().clone();
let argument = if matches!(order, config::reduction_order_t::APPLICATIVE) {
app.argument.as_ref().map(|a| reduce_ast(table, a))
} else {
app.argument.as_ref().map(|a| (**a).clone())
};

let rebuilt = common::AstNode {
type_: common::AstNodeType::APPLICATION,
node: common::AstNodeUnion::Application(common::Application {
function: function.clone().map(Box::new),
argument: argument.clone().map(Box::new),
}),
};

if let Some(func) = &function {
if func.type_ == common::AstNodeType::LAMBDA_EXPR {
if let common::AstNodeUnion::LambdaExpr(lambda) = &func.node {
let reduced = substitute(
lambda.body.as_ref().unwrap(),
&lambda.parameter,
argument.as_ref().unwrap(),
);
common::print_verbose(
"",
format_args!(
"Applied substitution to lambda expr of parameter <{}> and resulted in:\n",
lambda.parameter
),
);
common::print_ast_verbose(&reduced);
if matches!(order, config::reduction_order_t::APPLICATIVE) {
return reduced;
}
return reduce_ast(table, &reduced);
}
}
}
rebuilt
}
_ => deepcopy(n),
}
}

pub fn substitute(
expression: &common::AstNode,
variable: &str,
replacement: &common::AstNode,
) -> common::AstNode {
match &expression.node {
common::AstNodeUnion::Variable(var) if expression.type_ == common::AstNodeType::VAR => {
if var.name == variable {
return deepcopy(replacement);
}
deepcopy(expression)
}
common::AstNodeUnion::LambdaExpr(lambda) if expression.type_ == common::AstNodeType::LAMBDA_EXPR => {
let substituted_body = substitute(lambda.body.as_ref().unwrap(), variable, replacement);
if lambda.parameter != variable {
return deepcopy_lambda_expr(&lambda.parameter, &substituted_body, &lambda.type_);
}
substituted_body
}
common::AstNodeUnion::Application(app) if expression.type_ == common::AstNodeType::APPLICATION => {
let new_function = substitute(app.function.as_ref().unwrap(), variable, replacement);
let new_argument = substitute(app.argument.as_ref().unwrap(), variable, replacement);
deepcopy_application(&new_function, &new_argument)
}
_ => deepcopy(expression),
}
}

pub fn deepcopy(n: &common::AstNode) -> common::AstNode {
match &n.node {
common::AstNodeUnion::Variable(var) if n.type_ == common::AstNodeType::VAR || n.type_ == common::AstNodeType::DEFINITION => {
let mut v = deepcopy_var(&var.name, &var.type_);
v.type_ = n.type_.clone();
v
}
common::AstNodeUnion::LambdaExpr(lambda) if n.type_ == common::AstNodeType::LAMBDA_EXPR => {
deepcopy_lambda_expr(&lambda.parameter, lambda.body.as_ref().unwrap(), &lambda.type_)
}
common::AstNodeUnion::Application(app) if n.type_ == common::AstNodeType::APPLICATION => {
deepcopy_application(app.function.as_ref().unwrap(), app.argument.as_ref().unwrap())
}
_ => common::AstNode::default(),
}
}

pub fn deepcopy_application(function: &common::AstNode, argument: &common::AstNode) -> common::AstNode {
common::AstNode {
type_: common::AstNodeType::APPLICATION,
node: common::AstNodeUnion::Application(common::Application {
function: Some(Box::new(deepcopy(function))),
argument: Some(Box::new(deepcopy(argument))),
}),
}
}

pub fn deepcopy_lambda_expr(parameter: &str, body: &common::AstNode, type_: &str) -> common::AstNode {
common::AstNode {
type_: common::AstNodeType::LAMBDA_EXPR,
node: common::AstNodeUnion::LambdaExpr(common::LambdaExpression {
parameter: parameter.to_string(),
type_: type_.to_string(),
body: Some(Box::new(deepcopy(body))),
}),
}
}

pub fn deepcopy_var(name: &str, type_: &str) -> common::AstNode {
common::AstNode {
type_: common::AstNodeType::VAR,
node: common::AstNodeUnion::Variable(common::Variable {
name: name.to_string(),
type_: type_.to_string(),
}),
}
}
