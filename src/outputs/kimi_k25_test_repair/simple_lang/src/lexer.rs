use crate::token::{Token, TokenType};
pub fn tokenize(input: &str) -> Vec<Token> {
let mut tokens = Vec::new();
let mut line = 1;
let chars: Vec<char> = input.chars().collect();
let mut i = 0;
while i < chars.len() {
let ch = chars[i];
match ch {
'\n' => {
line += 1;
i += 1;
}
' ' | '\t' | '\r' => {
i += 1;
}
'+' => {
tokens.push(Token::new(TokenType::TOKEN_PLUS, ch.to_string(), line));
i += 1;
}
'-' => {
tokens.push(Token::new(TokenType::TOKEN_MINUS, ch.to_string(), line));
i += 1;
}
'*' => {
tokens.push(Token::new(TokenType::Star, ch.to_string(), line));
i += 1;
}
'/' => {
tokens.push(Token::new(TokenType::Slash, ch.to_string(), line));
i += 1;
}
'(' => {
tokens.push(Token::new(TokenType::LeftParen, ch.to_string(), line));
i += 1;
}
')' => {
tokens.push(Token::new(TokenType::RightParen, ch.to_string(), line));
i += 1;
}
'{' => {
tokens.push(Token::new(TokenType::LeftBrace, ch.to_string(), line));
i += 1;
}
'}' => {
tokens.push(Token::new(TokenType::RightBrace, ch.to_string(), line));
i += 1;
}
';' => {
tokens.push(Token::new(TokenType::TOKEN_SEMICOLON, ch.to_string(), line));
i += 1;
}
'=' => {
tokens.push(Token::new(TokenType::TOKEN_ASSIGN, ch.to_string(), line));
i += 1;
}
c if c.is_alphabetic() => {
let mut ident = String::new();
let start = i;
while i < chars.len() && chars[i].is_alphabetic() {
ident.push(chars[i]);
i += 1;
}
let token_type = match ident.as_str() {
"let" => TokenType::TOKEN_LET,
"dis" => TokenType::TOKEN_DIS,
_ => TokenType::TOKEN_IDENTIFIER,
};
tokens.push(Token::new(token_type, ident, line));
}
c if c.is_numeric() => {
let mut num = String::new();
while i < chars.len() && chars[i].is_numeric() {
num.push(chars[i]);
i += 1;
}
tokens.push(Token::new(TokenType::TOKEN_INT, num, line));
}
_ => {
i += 1;
}
}
}
tokens.push(Token::new(TokenType::TOKEN_EOF, "".to_string(), line));
tokens
}
