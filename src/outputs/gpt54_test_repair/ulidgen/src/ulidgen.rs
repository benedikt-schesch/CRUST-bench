
use crate::ulid;
pub fn main(argc: i32, argv: &[&str]) -> i32 {
let _ = argc;
let mut ulid_buf = ['\0'; ulid::ULID_LENGTH];
let mut n: i64 = 1;
let mut tflag = false;
let mut i = 1usize;
while i < argv.len() {
match argv[i] {
"-n" => {
if i + 1 < argv.len() {
n = argv[i + 1].parse::<i64>().unwrap_or(0);
i += 1;
}
}
"-t" => {
tflag = true;
}
_ => {}
}
i += 1;
}
if tflag {
use std::io::{self, BufRead, Write};
let stdin = io::stdin();
let mut stdout = io::stdout();
for line_result in stdin.lock().lines() {
let line = match line_result {
Ok(s) => s,
Err(_) => return 1,
};
ulid::ulidgen_r(&mut ulid_buf);
let generated: String = ulid_buf[..26].iter().collect();
if writeln!(stdout, "{} {}", generated, line).is_err() {
return 1;
}
}
if stdout.flush().is_err() {
return 1;
}
0
} else {
use std::io::{self, Write};
let mut stdout = io::stdout();
let count = if n < 0 { 0 } else { n as usize };
for _ in 0..count {
ulid::ulidgen_r(&mut ulid_buf);
let generated: String = ulid_buf[..26].iter().collect();
if writeln!(stdout, "{}", generated).is_err() {
return 1;
}
}
if stdout.flush().is_err() {
return 1;
}
0
}
}
