use crate::{throw, slothvm};
use crate::slothvm::SlothProgram;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
pub fn parse(filename: &str) -> Option<SlothProgram> {
let file = match File::open(filename) {
Ok(f) => f,
Err(_) => {
eprintln!("[ERROR] File could not be opened.");
std::process::exit(1);
}
};
let num_codes = prog_len(&file);
let mut byte_code = vec![0; num_codes];
let mut code_num = 0;
loop {
let line = match readline(&file) {
Some(l) => l,
None => break,
};
let len = line.len();
if len == 0 {
continue;
}
let bytes = line.as_bytes();
let mut count = 0;
let mut current_code = 0;
let mut had_token = false;
while count < len {
let c = bytes[count];
if c == 0 || c == 255 || c == b'#' {
break;
}
if bytes[count..].starts_with(b"slothy") {
byte_code[code_num] = 0x1;
code_num += 1;
count += 6;
had_token = true;
} else if bytes[count..].starts_with(b"sloth") {
current_code += 1;
count += 5;
had_token = true;
} else if bytes[count..].starts_with(b"and") {
byte_code[code_num] = current_code;
code_num += 1;
current_code = 0;
count += 3;
had_token = true;
} else if bytes[count..].starts_with(b"nap") {
byte_code[code_num] = 0x0;
code_num += 1;
count += 3;
had_token = true;
} else {
count += 1;
}
}
if had_token {
byte_code[code_num] = current_code;
code_num += 1;
}
}
byte_code.truncate(code_num);
Some(SlothProgram {
codes: byte_code,
pc: 0,
})
}
pub fn free_program(_program: Option<SlothProgram>) {
}
pub fn readline(file: &std::fs::File) -> Option<String> {
let mut line = Vec::new();
let mut buf = [0; 1];
let mut f = file;
loop {
match f.read(&mut buf) {
Ok(1) => {
let c = buf[0];
if c == b'\n' {
break;
}
line.push(c);
}
_ => {
if line.is_empty() {
return None;
}
break;
}
}
}
Some(String::from_utf8_lossy(&line).into_owned())
}
pub fn prog_len(file: &std::fs::File) -> usize {
let mut count = 0;
let mut buf = [0; 1];
let mut f = file;
while let Ok(1) = f.read(&mut buf) {
if buf[0] == b'\n' {
count += 1;
}
}
f.seek(SeekFrom::Start(0)).unwrap();
count * 3
}
