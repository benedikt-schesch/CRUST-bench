use crate::my_regex::{regex_parse, regex_match, RegexToken};
fn main() {
let _tokens_buffer = vec![RegexToken::default(); 1024];
let pattern = "[0-9]+\\.[0-9]+";
let result = regex_parse(pattern);
assert!(!result.is_empty(), "Regex parsing failed");
println!("Parsed tokens:");
for (i, token) in result.iter().enumerate() {
println!("Token {}: {:?}", i, token);
}
let text = "23.53";
let match_result = regex_match(pattern, text);
assert!(match_result, "Pattern should match the text");
println!("Regex test passed successfully!");
}
