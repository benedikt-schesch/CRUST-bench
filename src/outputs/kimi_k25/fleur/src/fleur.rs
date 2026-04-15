use std::{fmt, fs::File};
use std::io::{Read, Write, Seek, SeekFrom};
use crate::fnv;

const M: u64 = 18446744073709551557;
const G: u64 = 18446744073709550147;
const FNV_PRIME: u64 = 1099511628211;
const FNV_OFFSET_BASIS: u64 = 14695981039346656037;

#[derive(Debug, Clone)]
pub struct Header {
pub version: u64,
pub n: u64,
pub p: f64,
pub k: u64,
pub m: u64,
pub n_value: u64,
}

impl fmt::Display for Header {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
write!(f, "Header details:\n version: {}\n n: {} \n p: {}\n k: {} \n m: {} \n N: {} \n",
self.version, self.n, self.p, self.k, self.m, self.n_value)
}
}

#[derive(Debug, Clone)]
pub struct BloomFilter {
pub version: u64,
pub datasize: u64,
pub h: Header,
pub m: u64,
pub v: Vec<u64>,
pub data: Vec<u8>,
pub modified: i32,
pub error: i32,
}

impl fmt::Display for BloomFilter {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
let data_str = String::from_utf8_lossy(&self.data);
write!(f, "Filter details:\n n: {} \n p: {}\n k: {} \n m: {} \n N: {} \n M: {}\n Data: {}.\n",
self.h.n, self.h.p, self.h.k, self.h.m, self.h.n_value, self.m, data_str)
}
}

impl BloomFilter {
pub fn fingerprint(&self, buf: &[u8]) -> Vec<u64> {
let mut result = vec![0u64; self.h.k as usize];
let h = fnv::fnv1(buf);
let mut hn = h % M;
for i in 0..self.h.k {
hn = (hn.wrapping_mul(G)) % M;
result[i as usize] = hn % self.h.m;
}
result
}

pub fn add(&mut self, buf: &[u8]) -> i32 {
if (self.h.n_value + 1) <= self.h.n {
let mut new_value = 0;
let fp = self.fingerprint(buf);
for &hash in &fp {
let k = (hash / 64) as usize;
let l = (hash % 64) as u64;
let v = 1u64 << l;
if (self.v[k] & v) == 0 {
new_value = 1;
}
self.v[k] |= v;
}
if new_value == 1 {
self.h.n_value += 1;
self.modified = 1;
1
} else {
0
}
} else {
-1
}
}

pub fn join(&mut self, bf: &BloomFilter) -> i32 {
if bf.h.n != self.h.n {
eprintln!("[ERROR] Filters characteristics mismatch - cannot join.");
return 0;
}
if bf.h.p != self.h.p {
eprintln!("[ERROR] Filters characteristics mismatch - cannot join.");
return 0;
}
if bf.h.k != self.h.k {
eprintln!("[ERROR] Filters characteristics mismatch - cannot join.");
return 0;
}
if bf.h.m != self.h.m {
eprintln!("[ERROR] Filters characteristics mismatch - cannot join.");
return 0;
}
if bf.m != self.m {
eprintln!("[ERROR] Filters characteristics mismatch - cannot join.");
return 0;
}
if (self.h.n_value + bf.h.n_value) > self.h.n {
eprintln!("[ERROR] Destination Filter is full.");
return -1;
}
for i in 0..self.m as usize {
self.v[i] |= bf.v[i];
}
self.h.n_value += bf.h.n_value;
1
}

pub fn check(&self, buf: &[u8]) -> i32 {
let fp = self.fingerprint(buf);
for &hash in &fp {
let k = (hash / 64) as usize;
let l = (hash % 64) as u64;
let v = 1u64 << l;
if (self.v[k] & v) == 0 {
return 0;
}
}
1
}

pub fn set_data(&mut self, buf: &[u8]) {
self.data = buf.to_vec();
self.datasize = buf.len() as u64;
}

pub fn to_file(&self, mut file: File) -> i32 {
let version: u64 = 1;

if file.write_all(&version.to_ne_bytes()).is_err() {
eprintln!("[ERROR] writing filter file.");
return 0;
}
if file.write_all(&self.h.n.to_ne_bytes()).is_err() {
eprintln!("[ERROR] writing filter file.");
return 0;
}
if file.write_all(&self.h.p.to_ne_bytes()).is_err() {
eprintln!("[ERROR] writing filter file.");
return 0;
}
if file.write_all(&self.h.k.to_ne_bytes()).is_err() {
eprintln!("[ERROR] writing filter file.");
return 0;
}
if file.write_all(&self.h.m.to_ne_bytes()).is_err() {
eprintln!("[ERROR] writing filter file.");
return 0;
}
if file.write_all(&self.h.n_value.to_ne_bytes()).is_err() {
eprintln!("[ERROR] writing filter file.");
return 0;
}

for &val in &self.v {
if file.write_all(&val.to_ne_bytes()).is_err() {
eprintln!("[ERROR] writing filter file.");
return 0;
}
}

if file.write_all(&self.data).is_err() {
eprintln!("[ERROR] writing filter file.");
return 0;
}

1
}

pub fn initialize(n: u64, p: f64, buf: &[u8]) -> BloomFilter {
let m = ((n as f64) * p.ln() / (2.0f64.ln().powi(2))).ceil().abs() as u64;
let k = ((2.0f64.ln() * m as f64) / n as f64).ceil() as u64;
let big_m = ((m as f64) / 64.0).ceil() as u64;

BloomFilter {
version: 1,
datasize: buf.len() as u64,
h: Header {
version: 1,
n,
p,
k,
m,
n_value: 0,
},
m: big_m,
v: vec![0u64; big_m as usize],
data: buf.to_vec(),
modified: 0,
error: 0,
}
}

pub fn from_file(mut file: File) -> BloomFilter {
let mut bf = BloomFilter {
version: 0,
datasize: 0,
h: Header {
version: 0,
n: 0,
p: 0.0,
k: 0,
m: 0,
n_value: 0,
},
m: 0,
v: Vec::new(),
data: Vec::new(),
modified: 0,
error: 0,
};

let mut header_buf = [0u8; 48];
if file.read_exact(&mut header_buf).is_err() {
eprintln!("[ERROR] reading filter file.");
bf.error = 1;
return bf;
}

bf.h.version = u64::from_ne_bytes(header_buf[0..8].try_into().unwrap());
bf.h.n = u64::from_ne_bytes(header_buf[8..16].try_into().unwrap());
bf.h.p = f64::from_ne_bytes(header_buf[16..24].try_into().unwrap());
bf.h.k = u64::from_ne_bytes(header_buf[24..32].try_into().unwrap());
bf.h.m = u64::from_ne_bytes(header_buf[32..40].try_into().unwrap());
bf.h.n_value = u64::from_ne_bytes(header_buf[40..48].try_into().unwrap());

if !bf.h.check() {
eprintln!("[ERROR] Incoherent header.");
bf.error = 1;
return bf;
}

bf.m = ((bf.h.m as f64) / 64.0).ceil() as u64;

let file_size = match file.seek(SeekFrom::End(0)) {
Ok(pos) => pos,
Err(_) => {
eprintln!("[ERROR] Cannot seek in binary file.");
bf.error = 1;
return bf;
}
};

if file.seek(SeekFrom::Start(48)).is_err() {
eprintln!("[ERROR] Cannot seek in binary file.");
bf.error = 1;
return bf;
}

if (bf.m as i64) <= (file_size as i64 - 48) {
bf.datasize = (file_size - (bf.m * 8) - 48) as u64;

bf.v = vec![0u64; bf.m as usize];
for i in 0..bf.m {
let mut val_buf = [0u8; 8];
if file.read_exact(&mut val_buf).is_err() {
eprintln!("[ERROR] Cannot load bitarray.");
bf.error = 1;
return bf;
}
bf.v[i as usize] = u64::from_ne_bytes(val_buf);
}

if bf.datasize > 0 {
bf.data = vec![0u8; (bf.datasize + 1) as usize];
if file.read_exact(&mut bf.data[..bf.datasize as usize]).is_err() {
eprintln!("[ERROR] Cannot load bloom filter metadata.");
bf.error = 1;
return bf;
}
bf.data[bf.datasize as usize] = 0;
} else {
bf.data = vec![0u8; 1];
bf.data[0] = 0;
}
}

bf.modified = 0;
bf.error = 0;
bf.version = bf.h.version;
bf
}
}

impl Header {
pub fn check(&self) -> bool {
if self.version != 1 {
eprintln!("[ERROR] Current filter version not supported.");
return false;
}

let maxint: u64 = 9223372036854775807;
if self.k >= maxint {
eprintln!("[ERROR] value of k (number of hash functions) is too high.");
return false;
}
if self.p <= f64::EPSILON {
eprintln!("[ERROR] p is too small.");
return false;
}
if self.p > 1.0 {
eprintln!("[ERROR] p is more than one.");
return false;
}
if self.n_value > self.n {
eprintln!("[ERROR] incoherent filter.");
return false;
}
let tmp_m = ((self.n as f64) * self.p.ln() / (2.0f64.ln().powi(2))).ceil().abs() as u64;
if tmp_m != self.m {
eprintln!("[ERROR] incoherent filter.");
return false;
}
let tmp_k = ((2.0f64.ln() * self.m as f64) / self.n as f64).ceil() as u64;
if tmp_k != self.k {
eprintln!("[ERROR] incoherent filter values.");
return false;
}
true
}
}
