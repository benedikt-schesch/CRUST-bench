use crate::lex::Lexer;
use crate::cst::CST;
use crate::token::TokenType;

pub struct Parser;

impl Parser {
pub fn new() -> Self {
Parser
}

pub fn parse(&mut self, _lexer: &mut Lexer) -> Result<CST, String> {
Ok(CST::new())
}
}
