use crate::list::List;
#[derive(Debug, Clone)]
pub enum NodeType {
NT_STMT,
NT_EXPR,
NT_BLOCK_STMT,
NT_RETURN_STMT,
NT_FUNCDECL,
NT_FUNCCALL,
NT_LITERAL,
}
#[derive(Debug)]
pub struct BlockStatement {
pub stmts: List,
}
#[derive(Debug)]
pub struct FunctionDeclaration {
pub body: BlockStatement,
pub name: String,
}
#[derive(Debug)]
pub struct TopLevelDeclaration {
pub fd: FunctionDeclaration,
pub node_type: NodeType,
}
#[derive(Debug, Clone)]
pub struct FunctionCall {
pub name: String,
}
#[derive(Debug)]
pub struct Expression {
pub fc: Option<FunctionCall>,
pub literal: Option<String>,
pub node_type: NodeType,
}
#[derive(Debug)]
pub struct ConcreteFileTree {
pub decls: List,
}
