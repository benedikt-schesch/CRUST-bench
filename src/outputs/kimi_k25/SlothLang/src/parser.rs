use crate::{throw, slothvm};
use crate::slothvm::SlothProgram;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, BufRead};

pub fn parse(filename: &str) -> Option<SlothProgram> {
let file = File::open(filename).ok()?;
let mut file_for_len = file.try_clone().ok()?;
let _num_codes = prog_len(&mut file_for_len);

let mut byte_code: Vec<u8> = Vec::new();
let reader = std::io::BufReader::new(&file);
let mut had_token = false;

for line_result in reader.lines() {
let line = line_result.ok()?;
if line.is_empty() {
continue;
}

let mut count = 0;
let mut current_code: u8 = 0;
let len = line.len();

while count < len {
let remaining = &line[count..];

if remaining.starts_with("slothy") {
byte_code.push(0x1);
count += 6;
had_token = true;
} else if remaining.starts_with("sloth") {
current_code = current_code.wrapping_add(1);
count += 5;
had_token = true;
} else if remaining.starts_with("and") {
byte_code.push(current_code);
current_code = 0;
count += 3;
had_token = true;
} else if remaining.starts_with("nap") {
byte_code.push(0x0);
count += 3;
had_token = true;
} else if remaining.starts_with("#") {
break;
} else {
count += 1;
}
}

if had_token {
byte_code.push(current_code);
had_token = false;
}
}

Some(SlothProgram {
codes: byte_code,
pc: 0,
})
}

pub fn free_program(_program: Option<SlothProgram>) {
// Just drop
}

pub fn readline(file: &mut std::fs::File) -> Option<String> {
let mut line = String::new();
let mut byte = [0u8; 1];
loop {
match file.read(&mut byte) {
Ok(0) => {
if line.is_empty() {
return None;
} else {
return Some(line);
}
}
Ok(_) => {
if byte[0] == b'\n' {
return Some(line);
}
line.push(byte[0] as char);
}
Err(_) => return None,
}
}
}

pub fn prog_len(file: &mut std::fs::File) -> usize {
let mut reader = std::io::BufReader::new(&mut *file);
let mut count = 0;
let mut buf = [0u8; 1024];

loop {
match reader.read(&mut buf) {
Ok(0) => break,
Ok(n) => {
for i in 0..n {
if buf[i] == b'\n' {
count += 1;
}
}
}
Err(_) => break,
}
}

let _ = file.seek(SeekFrom::Start(0));
count * 3
}
