
pub fn example_function() -> &'static str {
"Hello from my_regex"
}
#[derive(Debug, Clone, PartialEq)]
pub enum RegexToken {
Literal(char),
Wildcard,  
Star,      
Plus,      
Question,  
OpenParen, 
CloseParen,
Or,        
Start,     
End,       
CharClass(Vec<char>), 
}
impl Default for RegexToken {
fn default() -> Self {
RegexToken::Literal('\0')
}
}
pub struct RegexPattern {
pattern: String,
}
impl RegexPattern {
pub fn new(pattern: &str) -> Self {
RegexPattern {
pattern: pattern.to_string(),
}
}
pub fn pattern(&self) -> &str {
&self.pattern
}
}
pub fn regex_parse(pattern: &str) -> Vec<RegexToken> {
let mut tokens = Vec::new();
let mut chars = pattern.chars().peekable();
while let Some(c) = chars.next() {
let token = match c {
'.' => RegexToken::Wildcard,
'*' => RegexToken::Star,
'+' => RegexToken::Plus,
'?' => RegexToken::Question,
'(' => RegexToken::OpenParen,
')' => RegexToken::CloseParen,
'|' => RegexToken::Or,
'^' => RegexToken::Start,
'$' => RegexToken::End,
'[' => {
let mut class_chars = Vec::new();
while let Some(next_c) = chars.next() {
if next_c == ']' {
break;
}
class_chars.push(next_c);
}
RegexToken::CharClass(class_chars)
}
_ => RegexToken::Literal(c),
};
tokens.push(token);
}
tokens
}
pub fn print_regex_tokens(tokens: &[RegexToken]) {
for (i, token) in tokens.iter().enumerate() {
println!("Token {}: {:?}", i, token);
}
}
pub fn regex_match(pattern: &str, text: &str) -> bool {
let tokens = regex_parse(pattern);
match_regex_tokens(&tokens, text)
}
fn match_regex_tokens(tokens: &[RegexToken], text: &str) -> bool {
if tokens.is_empty() {
return text.is_empty();
}
match &tokens[0] {
RegexToken::Literal(c) => {
if let Some(first) = text.chars().next() {
if first == *c {
return match_regex_tokens(&tokens[1..], &text[1..]);
}
}
false
}
RegexToken::Wildcard => {
if !text.is_empty() {
return match_regex_tokens(&tokens[1..], &text[1..]);
}
false
}
_ => {
match_regex_tokens(&tokens[1..], text)
}
}
}
