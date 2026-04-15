use crate::token::TokenType;
use crate::list::List;

#[derive(Debug, Clone)]
pub struct CST {
nodes: List<Node>,
}

impl CST {
pub fn new() -> Self {
CST {
nodes: List::new(),
}
}

pub fn add_node(&mut self, node: Node) {
self.nodes.push(node);
}
}

#[derive(Debug, Clone)]
pub enum Node {
Token(TokenType),
Internal {
kind: String,
children: List<Node>,
},
}
