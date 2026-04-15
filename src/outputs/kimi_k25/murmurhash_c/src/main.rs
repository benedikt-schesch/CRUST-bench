use std::env;
use std::io::{self, BufRead, IsTerminal};
use murmurhash_c::murmurhash::{murmurhash, MURMURHASH_VERSION};

fn usage() {
eprintln!("usage: murmur [-hV] [options]");
}

fn help() {
eprintln!("\noptions:");
eprintln!("\n  --seed=[seed]  hash seed (optional)");
eprintln!();
}

fn read_stdin() -> Option<String> {
let stdin = io::stdin();
let mut line = String::new();
match stdin.lock().read_line(&mut line) {
Ok(0) => None,
Ok(_) => Some(line),
Err(_) => None,
}
}

fn main() {
let args: Vec<String> = env::args().collect();
let mut seed_str: Option<&str> = None;

// Parse arguments
let mut i = 1;
while i < args.len() {
let arg = &args[i];
if arg.starts_with('-') {
if arg == "-h" {
usage();
help();
std::process::exit(0);
} else if arg == "-V" {
eprintln!("{}", MURMURHASH_VERSION);
std::process::exit(0);
} else if arg.starts_with("--seed=") {
seed_str = Some(&arg[7..]);
} else {
eprintln!("unknown option: `{}`", arg);
usage();
std::process::exit(1);
}
}
i += 1;
}

let seed: u32 = seed_str.map(|s| s.parse().unwrap_or(0)).unwrap_or(0);

if io::stdin().is_terminal() {
std::process::exit(1);
}

let buf = match read_stdin() {
Some(line) => line,
None => std::process::exit(1),
};

let h = murmurhash(buf.as_bytes(), seed);
println!("{}", h);

loop {
match read_stdin() {
Some(_key) => {
// Observationally equivalent to C code: re-hash the first buffer (buf)
// rather than hashing the new input (key)
let h = murmurhash(buf.as_bytes(), seed);
println!("{}", h);
}
None => break,
}
}

std::process::exit(0);
}
