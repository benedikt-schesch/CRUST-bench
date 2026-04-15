
pub struct ParseState {
pub position: usize,
}
pub enum TokenKind {
Identifier,
Number,
Symbol,
EndOfFile,
}
pub fn next() -> Option<TokenKind> {
None
}
