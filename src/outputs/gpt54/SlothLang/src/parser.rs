use crate::slothvm::SlothProgram;
use crate::{slothvm, throw};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub fn parse(filename: &str) -> Option<SlothProgram> {
let file = match File::open(filename) {
Ok(f) => f,
Err(_) => {
eprintln!("[ERROR] File could not be opened.");
return None;
}
};

let num_codes = prog_len(&file);
let mut byte_code: Vec<slothvm::UByte> = Vec::with_capacity(num_codes);

let mut reader = match file.try_clone() {
Ok(f) => f,
Err(_) => {
eprintln!("[ERROR] File could not be opened.");
return None;
}
};

loop {
let line_opt = readline(&reader);
let line = match line_opt {
Some(s) => s,
None => break,
};

if line.is_empty() {
continue;
}

let bytes = line.as_bytes();
let len = bytes.len();
let mut count: usize = 0;
let mut current_code: u8 = 0;
let mut had_token = false;

while count < len && bytes[count] != b'#' {
if count + 6 <= len && &bytes[count..count + 6] == b"slothy" {
byte_code.push(0x01);
count += 6;
had_token = true;
} else if count + 5 <= len && &bytes[count..count + 5] == b"sloth" {
current_code = current_code.wrapping_add(1);
count += 5;
had_token = true;
} else if count + 3 <= len && &bytes[count..count + 3] == b"and" {
byte_code.push(current_code);
current_code = 0;
count += 3;
had_token = true;
} else if count + 3 <= len && &bytes[count..count + 3] == b"nap" {
byte_code.push(0x00);
count += 3;
had_token = true;
} else {
count += 1;
}
}

if had_token {
byte_code.push(current_code);
}
}

Some(SlothProgram {
codes: byte_code,
pc: 0,
})
}

pub fn free_program(_program: Option<SlothProgram>) {}

pub fn readline(file: &std::fs::File) -> Option<String> {
let mut f = file.try_clone().ok()?;
let pos = f.stream_position().ok()?;

let mut all = Vec::new();
f.read_to_end(&mut all).ok()?;

if all.is_empty() {
return None;
}

let mut end = 0usize;
while end < all.len() && all[end] != b'\n' {
end += 1;
}

let consumed = if end < all.len() { end + 1 } else { end };

let mut original = file.try_clone().ok()?;
original.seek(SeekFrom::Start(pos + consumed as u64)).ok()?;

Some(String::from_utf8_lossy(&all[..end]).into_owned())
}

pub fn prog_len(file: &std::fs::File) -> usize {
let mut f = match file.try_clone() {
Ok(v) => v,
Err(_) => return 0,
};

if f.seek(SeekFrom::Start(0)).is_err() {
return 0;
}

let mut s = String::new();
if f.read_to_string(&mut s).is_err() {
return 0;
}

let count = s.bytes().filter(|b| *b == b'\n').count();
count * 3
}
