
use crate::common::{AstNode, AstNodeType, AstNodeUnion, Variable, LambdaExpr};
use crate::hash_table::HashTable;
pub fn expand_definitions(table: &mut HashTable, node: &mut AstNode) {
if let AstNodeUnion::Variable(var) = &node.node {
if let Some(def) = table.get(&var.name) {
*node = def.clone();
}
}
match &mut node.node {
AstNodeUnion::Application(app) => {
expand_definitions(table, &mut app.left);
expand_definitions(table, &mut app.right);
}
AstNodeUnion::LambdaExpr(lambda) => {
if let Some(ref mut body) = lambda.body {
expand_definitions(table, body);
}
}
_ => {}
}
}
pub fn replace(node: &mut AstNode, old_name: &str, new_name: &str) {
match &mut node.node {
AstNodeUnion::Variable(var) => {
if var.name == old_name {
var.name = new_name.to_string();
node.name = new_name.to_string();
}
}
AstNodeUnion::Application(app) => {
replace(&mut app.left, old_name, new_name);
replace(&mut app.right, old_name, new_name);
}
AstNodeUnion::LambdaExpr(lambda) => {
if lambda.param == old_name {
lambda.param = new_name.to_string();
node.name = new_name.to_string();
}
if let Some(ref mut body) = lambda.body {
replace(body, old_name, new_name);
}
}
}
}
pub fn reduce_ast(table: &mut HashTable, node: &AstNode) -> AstNode {
let mut result = node.clone();
expand_definitions(table, &mut result);
reduce(&mut result);
result
}
fn reduce(node: &mut AstNode) {
match &mut node.node {
AstNodeUnion::Application(app) => {
reduce(&mut app.left);
reduce(&mut app.right);
if let AstNodeUnion::LambdaExpr(lambda) = &app.left.node {
let param = lambda.param.clone();
let body = lambda.body.as_ref().unwrap().clone();
let arg = app.right.clone();
let mut new_body = (*body).clone();
substitute(&mut new_body, &param, &arg);
*node = new_body;
}
}
AstNodeUnion::LambdaExpr(lambda) => {
if let Some(ref mut body) = lambda.body {
reduce(body);
}
}
_ => {}
}
}
fn substitute(node: &mut AstNode, var: &str, replacement: &AstNode) {
match &mut node.node {
AstNodeUnion::Variable(v) => {
if v.name == var {
*node = replacement.clone();
}
}
AstNodeUnion::Application(app) => {
substitute(&mut app.left, var, replacement);
substitute(&mut app.right, var, replacement);
}
AstNodeUnion::LambdaExpr(lambda) => {
if lambda.param != var {
if let Some(ref mut body) = lambda.body {
substitute(body, var, replacement);
}
}
}
}
}
pub fn deepcopy(node: &AstNode) -> AstNode {
node.clone()
}
