use std::fs;

use crate::compiler::{CompileProcess, CompileProcessInputFile, Pos};
use crate::vector::vector_create;

pub fn compile_process_create(
filename: &str,
filename_out: &str,
flags: i32,
) -> Option<CompileProcess> {
let _content = fs::read_to_string(filename).ok()?;
let ofile = if filename_out.is_empty() {
None
} else {
crate::compiler::ClonableFile::new(filename_out).ok()
};

Some(CompileProcess {
flags,
pos: Pos::default(),
cfile: CompileProcessInputFile {
fp: crate::compiler::ClonableFile::new(filename).ok(),
abs_path: Some(filename.to_string()),
},
token_vec: None,
node_vec: Some(vector_create(8)),
node_tree_vec: Some(vector_create(8)),
ofile,
})
}

pub fn compile_process_next_char(lex_process: &mut crate::lex_process::LexProcess) -> char {
if let Some(compiler) = lex_process.compiler.as_mut() {
compiler.pos.col += 1;
}
'\0'
}

pub fn compile_process_peek_char(_lex_process: &mut crate::lex_process::LexProcess) -> char {
'\0'
}

pub fn compile_process_push_char(_lex_process: &mut crate::lex_process::LexProcess, _c: char) {}
