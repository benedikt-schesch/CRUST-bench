// Import statements
use crate::{ulid};
use std::io::{self, BufRead, Write};

// Function Declarations
pub fn main(_argc: i32, argv: &[&str]) -> i32 {
let mut n: i64 = 1;
let mut tflag = false;
let mut i = 1;

// Parse command line arguments
while i < argv.len() {
let arg = argv[i];
if arg == "-t" {
tflag = true;
i += 1;
} else if arg == "-n" {
if i + 1 < argv.len() {
i += 1;
if let Ok(val) = argv[i].parse::<i64>() {
n = val;
}
}
i += 1;
} else if arg.starts_with("-n") && arg.len() > 2 {
// Handle -n5 style
if let Ok(val) = arg[2..].parse::<i64>() {
n = val;
}
i += 1;
} else {
// Unknown option or positional argument, skip
i += 1;
}
}

let mut ulid_arr = ['\0'; ulid::ULID_LENGTH];

if tflag {
// Timestamp mode: read lines from stdin and prefix with ULID
let stdin = io::stdin();
let stdout = io::stdout();
let mut reader = stdin.lock();
let mut handle = stdout.lock();
let mut line = String::new();

while reader.read_line(&mut line).unwrap_or(0) > 0 {
ulid::ulidgen_r(&mut ulid_arr);
let ulid_str: String = ulid_arr.iter().take(26).collect();
if write!(handle, "{} {}", ulid_str, line).is_err() {
return 1;
}
line.clear();
}
} else {
// Normal mode: generate n ULIDs
let stdout = io::stdout();
let mut handle = stdout.lock();
let count = if n > 0 { n as usize } else { 0 };

for _ in 0..count {
ulid::ulidgen_r(&mut ulid_arr);
let ulid_str: String = ulid_arr.iter().take(26).collect();
if writeln!(handle, "{}", ulid_str).is_err() {
return 1;
}
}
}

if io::stdout().flush().is_err() {
return 1;
}

0
}
