use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const SYM_INIT_SIZE: usize = 1024;
const TOKEN_SIZE: usize = 1024;

type Token = String;

pub fn fnv32(token: &str) -> usize {
const FNV_PRIME_32: u32 = 16777619;
const FNV_OFFSET_32: u32 = 2166136261u32;
let mut hsh = FNV_OFFSET_32;
for b in token.as_bytes() {
hsh ^= *b as u32;
hsh = hsh.wrapping_mul(FNV_PRIME_32);
}
hsh as usize
}

pub struct SymTable {
pub n_items: usize,
pub n_max: usize,
pub sym: Vec<Option<String>>,
pub rev: HashMap<String, usize>,
}

impl SymTable {
pub fn new() -> Self {
Self {
n_items: 0,
n_max: SYM_INIT_SIZE,
sym: Vec::new(),
rev: HashMap::new(),
}
}

pub fn remove(self) {}

pub fn reverse(&self) -> &HashMap<String, usize> {
&self.rev
}

pub fn read(&mut self, fin: &mut dyn BufRead) -> io::Result<()> {
let mut line_no = 1usize;
let mut line = String::new();
loop {
line.clear();
let n = fin.read_line(&mut line)?;
if n == 0 {
break;
}
let parts: Vec<&str> = line.trim_end().split('\t').collect();
if parts.len() == 2 {
let token = parts[0];
let token_id = parts[1].parse::<i32>().unwrap();
self.add(token_id, token);
} else {
return Err(io::Error::new(
io::ErrorKind::InvalidData,
format!("Invalid input line {}: {}", line_no, line),
));
}
line_no += 1;
}
Ok(())
}

pub fn fread(&mut self, filename: &str) -> io::Result<()> {
let file = File::open(filename)?;
let mut reader = BufReader::new(file);
self.read(&mut reader)
}

pub fn print(&self) {
for (i, s) in self.sym.iter().enumerate() {
if let Some(v) = s {
println!("{}\t{}", v, i);
}
}
}

pub fn add(&mut self, id: i32, token: &str) -> Option<String> {
if id < 0 {
return None;
}
let idu = id as usize;
while self.n_max <= idu {
self.n_max *= 2;
}
if self.sym.len() <= idu {
self.sym.resize(idu + 1, None);
}

let t = token.to_string();
if self.sym[idu].is_none() {
self.n_items += 1;
}
self.sym[idu] = Some(t.clone());
self.rev.insert(t.clone(), idu);
Some(t)
}

pub fn getr(&self, token: &str) -> Option<i32> {
Some(self.rev.get(token).map(|v| *v as i32).unwrap_or(-1))
}

pub fn get(&self, id: i32) -> Option<&str> {
if id < 0 {
return None;
}
let idx = id as usize;
if idx >= self.sym.len() {
return None;
}
self.sym[idx].as_deref()
}

pub fn compile(&mut self, token: &str) {
for line in token.lines() {
let parts: Vec<&str> = line.split('\t').collect();
if parts.len() == 2 {
let tok = parts[0];
let id = parts[1].parse::<i32>().unwrap();
self.add(id, tok);
} else {
panic!("Invalid input line: {}", line);
}
}
}
}
