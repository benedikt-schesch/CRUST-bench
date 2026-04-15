
use crate::{ulidgen};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::thread::sleep;
pub const ULID_LENGTH: usize = 27;
struct SimpleRng {
state: u64,
}
impl SimpleRng {
fn new(seed: u64) -> Self {
SimpleRng {
state: if seed == 0 { 0x123456789ABCDEF } else { seed }
}
}
fn next_u64(&mut self) -> u64 {
self.state ^= self.state >> 12;
self.state ^= self.state << 25;
self.state ^= self.state >> 27;
self.state.wrapping_mul(0x2545F4914F6CDD1D)
}
}
pub fn ulidgen_r(ulid: &mut [char; ULID_LENGTH]) {
const B32_ALPHABET: &[char] = &[
'0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K',
'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X', 'Y', 'Z'
];
ulid[26] = '\0';
let now = SystemTime::now()
.duration_since(UNIX_EPOCH)
.expect("System time before Unix epoch");
let mut t = now.as_millis() as u64;
let mut same = true;
for i in (0..10).rev() {
let idx = (t % 32) as usize;
let c = B32_ALPHABET[idx];
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
sleep(Duration::from_nanos(1234567));
ulidgen_r(ulid);
return;
}
let current = ulid[10 + i as usize];
if let Some(pos) = B32_ALPHABET.iter().position(|&c| c == current) {
if pos + 1 < B32_ALPHABET.len() {
ulid[10 + i as usize] = B32_ALPHABET[pos + 1];
return;
}
}
} else {
let seed = now.as_nanos() as u64;
let mut rng = SimpleRng::new(seed);
for i in 0..16 {
let r = (rng.next_u64() % 32) as usize;
ulid[10 + i] = B32_ALPHABET[r];
}
}
}
