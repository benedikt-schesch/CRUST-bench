#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
TOKEN_LET,
TOKEN_IDENTIFIER,
TOKEN_ASSIGN,
TOKEN_INT,
TOKEN_PLUS,
TOKEN_MINUS,
TOKEN_SEMICOLON,
TOKEN_DIS,
TOKEN_EOF,
// Keep existing variants for compatibility
Identifier,
Number,
String,
Equals,
LeftParen,
RightParen,
LeftBrace,
RightBrace,
Slash,
Star,
Pop,
DefineGlobal(String),
GetGlobal(String),
SetGlobal(String),
}

#[derive(Debug, Clone)]
pub struct Token {
pub token_type: TokenType,
pub lexeme: String,
pub line: usize,
}

impl Token {
pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
Token {
token_type,
lexeme,
line,
}
}
}
