
use crate::ulidgen;
use std::fs::File;
use std::io::Read;
use std::process::abort;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
pub const ULID_LENGTH: usize = 27;
pub fn ulidgen_r(ulid: &mut [char; ULID_LENGTH]) {
let b32alphabet: [char; 32] = [
'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
'J', 'K', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X', 'Y', 'Z',
];
let mut same = true;
ulid[26] = '\0';
let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
let mut t = now.as_millis() as u64;
for i in (0..=9).rev() {
let c = b32alphabet[(t % 32) as usize];
if ulid[i] != c {
ulid[i] = c;
same = false;
}
t /= 32;
}
if same {
let mut i: i32 = 15;
while i >= 0 && ulid[10 + i as usize] == 'Z' {
ulid[10 + i as usize] = '0';
i -= 1;
}
if i < 0 {
sleep(Duration::new(0, 1234567));
ulidgen_r(ulid);
return;
}
let current_char = ulid[10 + i as usize];
if let Some(pos) = b32alphabet.iter().position(|&x| x == current_char) {
if pos + 1 < b32alphabet.len() {
ulid[10 + i as usize] = b32alphabet[pos + 1];
return;
}
}
}
let mut rnd = [0u8; 16];
if let Ok(mut file) = File::open("/dev/urandom") {
if file.read_exact(&mut rnd).is_err() {
abort();
}
} else {
abort();
}
for i in 0..16 {
ulid[10 + i] = b32alphabet[(rnd[i] % 32) as usize];
}
}
