use crate::slothvm::SlothProgram;
use crate::slothvm;
use std::fs::File;
use std::io::Read;

pub fn parse(filename: &str) -> Option<SlothProgram> {
let mut file = match File::open(filename) {
Ok(f) => f,
Err(_) => {
eprintln!("[ERROR] File could not be opened.");
return None;
}
};

let mut contents = String::new();
if file.read_to_string(&mut contents).is_err() {
eprintln!("[ERROR] File could not be read.");
return None;
}

let num_codes = prog_len_from_str(&contents);
let mut byte_code: Vec<slothvm::UByte> = Vec::with_capacity(num_codes);

for line in contents.lines() {
let code_part = match line.find('#') {
Some(idx) => &line[..idx],
None => line,
};

let bytes = code_part.as_bytes();
let len = bytes.len();
let mut count: usize = 0;

while count < len {
if count + 6 <= len && &bytes[count..count + 6] == b"slothy" {
byte_code.push(0x01);
count += 6;
} else if count + 5 <= len && &bytes[count..count + 5] == b"sloth" {
let mut value: u8 = 0;
while count + 5 <= len && &bytes[count..count + 5] == b"sloth" {
value = value.wrapping_add(1);
count += 5;
}
byte_code.push(value);
} else if count + 3 <= len && &bytes[count..count + 3] == b"nap" {
byte_code.push(0x00);
count += 3;
} else if count + 3 <= len && &bytes[count..count + 3] == b"and" {
count += 3;
} else {
count += 1;
}
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
let mut contents = String::new();
f.read_to_string(&mut contents).ok()?;
contents.lines().next().map(|s| s.to_string())
}

pub fn prog_len(file: &std::fs::File) -> usize {
let mut f = match file.try_clone() {
Ok(v) => v,
Err(_) => return 0,
};
let mut s = String::new();
if f.read_to_string(&mut s).is_err() {
return 0;
}
prog_len_from_str(&s)
}

fn prog_len_from_str(s: &str) -> usize {
let mut count = 0usize;
for line in s.lines() {
let code_part = match line.find('#') {
Some(idx) => &line[..idx],
None => line,
};

let bytes = code_part.as_bytes();
let mut i = 0usize;
while i < bytes.len() {
if i + 6 <= bytes.len() && &bytes[i..i + 6] == b"slothy" {
count += 1;
i += 6;
} else if i + 5 <= bytes.len() && &bytes[i..i + 5] == b"sloth" {
count += 1;
while i + 5 <= bytes.len() && &bytes[i..i + 5] == b"sloth" {
i += 5;
}
} else if i + 3 <= bytes.len() && &bytes[i..i + 3] == b"nap" {
count += 1;
i += 3;
} else if i + 3 <= bytes.len() && &bytes[i..i + 3] == b"and" {
i += 3;
} else {
i += 1;
}
}
}
count
}
