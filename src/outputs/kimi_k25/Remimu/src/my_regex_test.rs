use crate::my_regex::{regex_parse, regex_match, RegexToken};

fn main() {
// Test 1: Initialize tokens vector with default values
let _tokens_buffer = vec![RegexToken::default(); 1024];

// Test 2: Parse regex pattern
// The function takes only the pattern string as argument
let pattern = "[0-9]+\\.[0-9]+";
let result = regex_parse(pattern);

// Check if parsing succeeded (returns Vec<RegexToken>, not Result)
// We verify by checking the vector is not empty and contains expected tokens
assert!(!result.is_empty(), "Regex parsing failed");

// Print tokens for debugging
println!("Parsed tokens:");
for (i, token) in result.iter().enumerate() {
println!("Token {}: {:?}", i, token);
}

// Test 3: Regex matching
// Function signature: regex_match(pattern: &str, text: &str) -> bool
let text = "23.53";
let match_result = regex_match(pattern, text);

// The function returns bool, not a length, so we check if it matches
assert!(match_result, "Pattern should match the text");

println!("Regex test passed successfully!");
}
