
use crate::ulid;
use std::io::{self, BufRead, Write};
pub fn main(argc: i32, argv: &[&str]) -> i32 {
let _ = argc;
let mut ulid_arr: [char; ulid::ULID_LENGTH] = ['\0'; ulid::ULID_LENGTH];
let mut n: i64 = 1;
let mut tflag = false;
let mut optind = 1;
while optind < argv.len() {
let arg = argv[optind];
if !arg.starts_with('-') || arg == "-" {
break;
}
if arg == "--" {
optind += 1;
break;
}
let mut chars = arg.chars().skip(1).peekable();
while let Some(c) = chars.next() {
match c {
't' => tflag = true,
'n' => {
if chars.peek().is_some() {
let rest: String = chars.collect();
n = rest.parse().unwrap_or(0);
break;
} else {
optind += 1;
if optind < argv.len() {
n = argv[optind].parse().unwrap_or(0);
}
break;
}
}
_ => {}
}
}
optind += 1;
}
let mut stdout = io::stdout();
let mut error_occurred = false;
if tflag {
let stdin = io::stdin();
let mut handle = stdin.lock();
let mut line = String::new();
while let Ok(bytes) = handle.read_line(&mut line) {
if bytes == 0 {
break;
}
ulid::ulidgen_r(&mut ulid_arr);
let ulid_str: String = ulid_arr[..26].iter().collect();
if write!(stdout, "{} {}", ulid_str, line).is_err() {
error_occurred = true;
}
if stdout.flush().is_err() {
error_occurred = true;
}
line.clear();
}
} else {
for _ in 0..n {
ulid::ulidgen_r(&mut ulid_arr);
let ulid_str: String = ulid_arr[..26].iter().collect();
if writeln!(stdout, "{}", ulid_str).is_err() {
error_occurred = true;
}
}
}
if stdout.flush().is_err() {
error_occurred = true;
}
if error_occurred {
1
} else {
0
}
}
