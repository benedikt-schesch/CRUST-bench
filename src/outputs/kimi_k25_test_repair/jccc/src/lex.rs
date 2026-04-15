use crate::token::TokenType;
pub struct Lexer;
impl Lexer {
pub fn new() -> Self {
Lexer
}
}
pub fn ttype_many_chars() -> TokenType {
TokenType::Identifier
}
pub fn ttype_one_char() -> TokenType {
TokenType::Char
}
pub fn ttype_name() -> TokenType {
TokenType::Identifier
}
pub fn ttype_from_str(_s: &str) -> TokenType {
TokenType::String
}
