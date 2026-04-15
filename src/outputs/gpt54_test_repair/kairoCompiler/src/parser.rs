use crate::compiler::{
CompileProcess, Pos, Token, PARSE_ALL_OK, TOKEN_TYPE_IDENTIFIER, TOKEN_TYPE_NUMBER,
TOKEN_TYPE_STRING,
};
use crate::node::{node_create, node_peek, node_set_vector};
use crate::vector::{vector_push, vector_set_peek_pointer};
fn parser_ignore_nl_or_comments(_process: &mut CompileProcess, _token_opt: &mut Option<Token>) {}
fn token_peek_no_increment(_process: &mut CompileProcess) -> Option<Token> {
None
}
fn token_next(process: &mut CompileProcess, parser_last_token: &mut Option<Token>) -> Option<Token> {
let tok = token_peek_no_increment(process);
*parser_last_token = tok.clone();
tok
}
fn next_token_placeholder(_vec: &mut crate::vector::Vector, _pos: &Pos) -> Option<Token> {
None
}
fn parse_single_token_to_node(process: &mut CompileProcess, parser_last_token: &mut Option<Token>) {
let token = match token_next(process, parser_last_token) {
Some(t) => t,
None => return,
};
match token.r#type {
TOKEN_TYPE_NUMBER => {
let _ = node_create(&crate::node::Node {
r#type: crate::compiler::NODE_TYPE_NUMBER,
llnum: token.llnum,
..Default::default()
});
}
TOKEN_TYPE_IDENTIFIER => {
let _ = node_create(&crate::node::Node {
r#type: crate::compiler::NODE_TYPE_IDENTIFIER,
sval: token.sval.clone(),
..Default::default()
});
}
TOKEN_TYPE_STRING => {
let _ = node_create(&crate::node::Node {
r#type: crate::compiler::NODE_TYPE_STRING,
sval: token.sval.clone(),
..Default::default()
});
}
_ => {}
}
}
fn parse_next(process: &mut CompileProcess, parser_last_token: &mut Option<Token>) -> i32 {
let token = token_peek_no_increment(process);
if token.is_none() {
return -1;
}
parse_single_token_to_node(process, parser_last_token);
0
}
pub fn parse(process: &mut CompileProcess) -> i32 {
let mut parser_last_token: Option<Token> = None;
if let (Some(node_vec), Some(node_tree_vec)) = (process.node_vec.clone(), process.node_tree_vec.clone()) {
node_set_vector(node_vec, node_tree_vec);
}
if let Some(ref mut token_vec) = process.token_vec {
vector_set_peek_pointer(token_vec, 0);
}
while parse_next(process, &mut parser_last_token) == 0 {
let node = node_peek();
if let Some(ref mut root) = process.node_tree_vec {
vector_push(root, &(0u64.to_le_bytes()));
let _ = node;
}
}
PARSE_ALL_OK
}
