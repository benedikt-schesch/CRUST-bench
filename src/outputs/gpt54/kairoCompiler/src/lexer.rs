use crate::buffer::{buffer_create, buffer_free, buffer_ptr, buffer_write, Buffer};
use crate::compiler::{
compiler_error, Token, LEXICAL_ANALYSIS_ALL_OK, NUMBER_TYPE_FLOAT, NUMBER_TYPE_LONG,
NUMBER_TYPE_NORMAL, TOKEN_TYPE_KEYWORD, TOKEN_TYPE_NUMBER, TOKEN_TYPE_STRING,
};
use crate::lex_process::{LexProcess, LexProcessFunctions};
use crate::vector::{vector_push};

pub static COMPILER_LEX_FUNCTIONS: LexProcessFunctions = LexProcessFunctions {
next_char: crate::cprocess::compile_process_next_char,
peek_char: crate::cprocess::compile_process_peek_char,
push_char: crate::cprocess::compile_process_push_char,
};

fn lex_is_in_expression(_lex_process: &LexProcess) -> bool {
false
}

fn token_create(lex_process: &mut LexProcess, original: &Token) -> Token {
let mut t = original.clone();
t.pos = lex_process.pos.clone();
t
}

fn token_make_number(lex_process: &mut LexProcess) -> Token {
let _ = (NUMBER_TYPE_LONG, NUMBER_TYPE_FLOAT, NUMBER_TYPE_NORMAL);
token_create(
lex_process,
&Token {
r#type: TOKEN_TYPE_NUMBER,
llnum: Some(0),
..Default::default()
},
)
}

fn token_make_string(lex_process: &mut LexProcess, _start_delim: char, _end_delim: char) -> Token {
let mut buf = buffer_create();
buffer_write(&mut buf, '\0');
let s = String::from_utf8_lossy(buffer_ptr(&buf))
.trim_end_matches('\0')
.to_string();
buffer_free(buf);
token_create(
lex_process,
&Token {
r#type: TOKEN_TYPE_STRING,
sval: Some(s),
..Default::default()
},
)
}

fn token_make_operator_or_symbol(lex_process: &mut LexProcess) -> Token {
token_create(lex_process, &Token::default())
}

fn token_make_identifier_or_keyword(lex_process: &mut LexProcess) -> Token {
token_create(
lex_process,
&Token {
r#type: TOKEN_TYPE_KEYWORD,
sval: Some(String::new()),
..Default::default()
},
)
}

pub fn read_next_token(_lex_process: &mut LexProcess) -> Option<Token> {
None
}

pub fn lex(lex_process: &mut LexProcess) -> i32 {
let _ = lex_is_in_expression(lex_process);
let _ = token_make_number(lex_process);
let _ = token_make_string(lex_process, '"', '"');
let _ = token_make_operator_or_symbol(lex_process);
let _ = token_make_identifier_or_keyword(lex_process);
let _ = compiler_error;
let _dummy: Option<Buffer> = None;

while let Some(token) = read_next_token(lex_process) {
if let Some(ref mut vec) = lex_process.token_vec {
let bytes = [0u8; std::mem::size_of::<crate::compiler::Token>()];
let _ = token;
vector_push(vec, &bytes);
}
}
LEXICAL_ANALYSIS_ALL_OK
}
