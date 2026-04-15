use crate::common;
use crate::reducer;
#[derive(Clone)]
pub struct Type {
pub expr: common::AstNode,
pub type_: String,
pub return_type: String,
}
#[derive(Clone)]
pub struct TypeEnv {
pub type_: Type,
pub next: Option<Box<TypeEnv>>,
}
pub fn assert_(expr: bool, error_msg: &str) {
if expr {
return;
}
common::error(error_msg, file!(), line!() as i32, "assert_");
}
pub fn typecheck(expr: &common::AstNode, env: Option<&TypeEnv>) -> Type {
let mut local_env: Option<Box<TypeEnv>> = env.cloned().map(Box::new);
if expr.type_ == common::AstNodeType::VAR {
let type_ = get_type_from_expr(expr);
let t = create_type(&type_, "", expr);
add_to_env(&mut local_env, t.clone());
t
} else if expr.type_ == common::AstNodeType::APPLICATION {
if let common::AstNodeUnion::Application(app) = &expr.node {
let func_type = typecheck(app.function.as_ref().unwrap(), local_env.as_deref());
let arg_type = typecheck(app.argument.as_ref().unwrap(), local_env.as_deref());
assert_(type_equal(&func_type, &arg_type), "Type mismatch.");
func_type
} else {
create_type("", "", expr)
}
} else if expr.type_ == common::AstNodeType::LAMBDA_EXPR {
let type_ = get_type_from_expr(expr);
let t = create_type(&type_, "", expr);
add_to_env(&mut local_env, t.clone());
t
} else {
create_type("", "", expr)
}
}
pub fn type_equal(a: &Type, b: &Type) -> bool {
let type_eql = a.type_ == b.type_;
let return_eql = a.return_type == b.return_type;
type_eql && return_eql
}
pub fn get_type_from_expr(expr: &common::AstNode) -> String {
match &expr.node {
common::AstNodeUnion::Variable(var) if expr.type_ == common::AstNodeType::VAR || expr.type_ == common::AstNodeType::DEFINITION => {
var.type_.clone()
}
common::AstNodeUnion::LambdaExpr(lambda) if expr.type_ == common::AstNodeType::LAMBDA_EXPR => {
lambda.type_.clone()
}
_ => String::new(),
}
}
pub fn p_print_type(t: &Type) {
if !t.type_.is_empty() {
println!("Type: {}", t.type_);
}
if !t.return_type.is_empty() {
println!("Return type: {}", t.return_type);
}
}
pub fn create_type(type_: &str, return_type: &str, expr: &common::AstNode) -> Type {
Type {
expr: reducer::deepcopy(expr),
type_: type_.to_string(),
return_type: return_type.to_string(),
}
}
pub fn parse_function_type(type_: &str) -> Type {
Type {
expr: common::AstNode::default(),
type_: type_.to_string(),
return_type: String::new(),
}
}
pub fn expr_type_equal(t: &Type, expr: &common::AstNode) -> bool {
if common::ast_to_string(&t.expr) != common::ast_to_string(expr) {
return false;
}
let type_ = get_type_from_expr(expr);
if type_.is_empty() {
return false;
}
let parsed_type = parse_function_type(&type_);
if t.type_ != parsed_type.type_ {
return false;
}
if parsed_type.return_type.is_empty() {
return t.return_type.is_empty();
}
t.return_type == parsed_type.return_type
}
pub fn add_to_env(env: &mut Option<Box<TypeEnv>>, type_: Type) {
let new_env = TypeEnv {
type_,
next: env.take(),
};
*env = Some(Box::new(new_env));
}
pub fn lookup_type(env: &TypeEnv, expr: &common::AstNode) -> Type {
let mut current: Option<&TypeEnv> = Some(env);
while let Some(node) = current {
if expr_type_equal(&node.type_, expr) {
return node.type_.clone();
}
current = node.next.as_deref();
}
create_type("", "", expr)
}
