
use std::env;
use std::fs::File;
use jccc::lex::{lex, ttype_name, Lexer};
use jccc::parse::parse;
use jccc::token::{Token, TokenType};
pub fn lexer_dump(filename: &str) -> i32 {
let fp = match File::open(filename) {
Ok(f) => f,
Err(_) => {
eprintln!("Error: jccc: File {} not found", filename);
return 1;
}
};
let mut lexer = Lexer::default();
lexer.fp = Some(fp);
lexer.unlexed_count = 0;
lexer.column = 1;
lexer.line = 1;
lexer.current_file = filename.to_string();
let mut t = Token {
token_type: TokenType::TT_NO_TOKEN,
contents: String::new(),
length: 0,
source_file: String::new(),
line: 0,
column: 0,
};
loop {
if lex(&mut lexer, &mut t) != 0 {
return 1;
}
println!(
"Contents: {:>20}, type: {:>20}, position: {}/{}",
t.contents,
ttype_name(t.token_type.clone()),
t.line,
t.column
);
if t.token_type == TokenType::TT_EOF {
break;
}
}
0
}
pub fn main() {
let args: Vec<String> = env::args().skip(1).collect();
if args.is_empty() {
eprintln!("jccc: Usage: --token-dump <filename> to see all tokens");
return;
}
if args.len() == 1 {
eprintln!(
"jccc: default compilation not supported yet -- try 'jccc --token-dump {}' instead.",
args[0]
);
return;
}
if args.len() > 2 {
eprintln!("jccc: expected only two arguments!");
return;
}
if args[0] == "--token-dump" {
let _ = lexer_dump(&args[1]);
} else if args[0] == "--test-parse" {
let _ = parse(&args[1]);
} else {
eprintln!("Error: jccc: option {} not recognized.", args[0]);
}
}
