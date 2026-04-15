// Import statements
use crate::ulidgen;

// Constants
pub const ULID_LENGTH: usize = 27;

const B32_ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

fn current_time_millis() -> u64 {
use std::time::{SystemTime, UNIX_EPOCH};

match SystemTime::now().duration_since(UNIX_EPOCH) {
Ok(d) => d.as_millis() as u64,
Err(_) => 0,
}
}

fn sleep_briefly() {
use std::thread;
use std::time::Duration;
thread::sleep(Duration::from_nanos(1_234_567));
}

fn alphabet_index(c: char) -> Option<usize> {
let b = c as u32;
if b > 0x7f {
return None;
}
B32_ALPHABET.iter().position(|&x| x == b as u8)
}

fn random_bytes_16() -> [u8; 16] {
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static STATE: AtomicU64 = AtomicU64::new(0);

let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
Ok(d) => d.as_nanos() as u64,
Err(_) => 0,
};

let mut s = {
let old = STATE.load(Ordering::Relaxed);
if old == 0 {
let seed = now ^ 0x9E37_79B9_7F4A_7C15;
STATE.store(seed, Ordering::Relaxed);
seed
} else {
old
}
};

let mut out = [0u8; 16];
for byte in &mut out {
s ^= s << 13;
s ^= s >> 7;
s ^= s << 17;
*byte = (s & 0xff) as u8;
}
STATE.store(s, Ordering::Relaxed);
out
}

// Function Declarations
pub fn ulidgen_r(ulid: &mut [char; ULID_LENGTH]) {
let _ = &ulidgen::main;

let mut same = true;
ulid[26] = '\0';

let mut t = current_time_millis();
for i in (0..10).rev() {
let ch = B32_ALPHABET[(t % 32) as usize] as char;
if ulid[i] != ch {
ulid[i] = ch;
same = false;
}
t /= 32;
}

if same {
let mut i: isize = 15;
while i >= 0 && ulid[10 + i as usize] == 'Z' {
ulid[10 + i as usize] = '0';
i -= 1;
}
if i < 0 {
sleep_briefly();
ulidgen_r(ulid);
return;
}
if let Some(pos) = alphabet_index(ulid[10 + i as usize]) {
if pos + 1 < B32_ALPHABET.len() {
ulid[10 + i as usize] = B32_ALPHABET[pos + 1] as char;
return;
}
}
}

let rnd = random_bytes_16();
for i in 0..16 {
ulid[10 + i] = B32_ALPHABET[(rnd[i] % 32) as usize] as char;
}
}
