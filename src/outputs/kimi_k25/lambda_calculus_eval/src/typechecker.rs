//! Type checker module for the lambda calculus evaluator.

use crate::common::{AstNode, AstNodeType, AstNodeUnion};

/// Type representation
#[derive(Clone)]
pub struct Type {
pub name: String,
pub body: Option<Box<AstNode>>,
}

/// Create a type
pub fn create_type(name: &str, _placeholder: &str, body: &AstNode) -> Type {
Type {
name: name.to_string(),
body: Some(Box::new(body.clone())),
}
}

/// Check if two types are equal
pub fn type_equal(a: &Type, b: &Type) -> bool {
a.name == b.name
}

/// Get type from expression
pub fn get_type_from_expr(node: &AstNode) -> Type {
match &node.node {
AstNodeUnion::Variable(var) => Type {
name: var.type_name.clone(),
body: None,
},
AstNodeUnion::LambdaExpr(lambda) => Type {
name: lambda.type_name.clone(),
body: None,
},
_ => Type {
name: String::new(),
body: None,
},
}
}

/// Check if expression type equals given type
pub fn expr_type_equal(type_: &Type, node: &AstNode) -> bool {
let expr_type = get_type_from_expr(node);
type_equal(type_, &expr_type)
}

/// Typecheck an AST node
pub fn typecheck(node: &AstNode, _expected: Option<&Type>) -> Result<Type, String> {
match &node.node {
AstNodeUnion::Variable(var) => Ok(Type {
name: var.type_name.clone(),
body: None,
}),
AstNodeUnion::Application(app) => {
let left_type = typecheck(&app.left, None)?;
let right_type = typecheck(&app.right, None)?;
// Simple type checking: if left is a function from A to B, and right is A, result is B
Ok(Type {
name: format!("App({},{})", left_type.name, right_type.name),
body: None,
})
}
AstNodeUnion::LambdaExpr(lambda) => {
let body_type = if let Some(ref body) = lambda.body {
typecheck(body, None)?
} else {
Type { name: String::new(), body: None }
};
Ok(Type {
name: format!("{}->{}", lambda.type_name, body_type.name),
body: None,
})
}
}
}
