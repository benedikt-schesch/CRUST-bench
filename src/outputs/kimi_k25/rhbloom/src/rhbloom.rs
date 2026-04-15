pub struct RHBloom {
count: usize,
nbuckets: usize,
buckets: Vec<u64>,
k: usize,
m: usize,
bits: Vec<u8>,
}

impl RHBloom {
pub fn new(n: usize, p: f64) -> Self {
let n = if n < 16 { 16 } else { n };

// Calculate m = -n * ln(p) / (ln(2)^2)
let m_f = -(n as f64) * p.ln() / (2f64.ln().powi(2));
let m = m_f as usize;

// Calculate k = round((m/n) * ln(2))
let k = ((m_f / n as f64) * 2f64.ln()).round() as usize;

// m0 = next power of 2 >= m
let mut m0 = 2usize;
while m0 < m {
m0 *= 2;
}

// k0 = round((m/m0) * k)
let k0 = ((m_f / m0 as f64) * k as f64).round() as usize;

Self {
count: 0,
nbuckets: 0,
buckets: Vec::new(),
k: k0,
m: m0,
bits: Vec::new(),
}
}

pub fn memsize(&self) -> usize {
std::mem::size_of::<Self>() +
if !self.bits.is_empty() {
self.bits.len()
} else {
self.buckets.len() * 8
}
}

pub fn clear(&mut self) {
if !self.bits.is_empty() {
self.bits.fill(0);
} else if !self.buckets.is_empty() {
self.buckets.fill(0);
self.count = 0;
}
}

pub fn upgraded(&self) -> bool {
!self.bits.is_empty()
}

pub fn testadd(&mut self, key: u64, add: bool) -> bool {
let mut key = key & 0x00FFFFFFFFFFFFFF; // RHBLOOM_KEY: lower 56 bits

if add {
// Set all k bits
for i in 0..self.k {
let j = (key as usize) & (self.m - 1);
let byte_idx = j >> 3;
let bit_idx = j & 7;
if byte_idx < self.bits.len() {
self.bits[byte_idx] |= 1 << bit_idx;
}
if i < self.k - 1 {
key = key.wrapping_mul(0x94d049bb133111eb);
key ^= key >> 31;
}
}
true
} else {
// Check all k bits
for i in 0..self.k {
let j = (key as usize) & (self.m - 1);
let byte_idx = j >> 3;
let bit_idx = j & 7;
if byte_idx >= self.bits.len() || ((self.bits[byte_idx] >> bit_idx) & 1) == 0 {
return false;
}
if i < self.k - 1 {
key = key.wrapping_mul(0x94d049bb133111eb);
key ^= key >> 31;
}
}
true
}
}

pub fn free(&mut self) {
self.bits.clear();
self.bits.shrink_to_fit();
self.buckets.clear();
self.buckets.shrink_to_fit();
self.count = 0;
self.nbuckets = 0;
}

pub fn mix(key: u64) -> u64 {
let mut key = key;
key ^= key >> 30;
key = key.wrapping_mul(0xbf58476d1ce4e5b9);
key ^= key >> 27;
key = key.wrapping_mul(0x94d049bb133111eb);
key ^= key >> 31;
key
}

pub fn grow(&mut self) -> bool {
let nbuckets_old = self.nbuckets;
let buckets_old = std::mem::take(&mut self.buckets);
let nbuckets_new = if nbuckets_old == 0 { 16 } else { nbuckets_old * 2 };

// Check if we should upgrade to bloom filter: nbuckets_new * 8 >= m / 8
// i.e., hash table size >= bloom filter size
if nbuckets_new * 8 >= self.m >> 3 {
// Switch to bloom filter mode
self.bits = vec![0u8; self.m >> 3];
self.count = 0;
self.nbuckets = 0;
// Re-insert old items
for &bucket_val in &buckets_old {
let dib = (bucket_val >> 56) as i64;
if dib != 0 {
self.testadd(bucket_val, true);
}
}
} else {
// Grow hash table
self.buckets = vec![0u64; nbuckets_new];
self.count = 0;
self.nbuckets = nbuckets_new;
// Re-insert old items
for &bucket_val in &buckets_old {
let dib = (bucket_val >> 56) as i64;
if dib != 0 {
self.addkey(bucket_val);
}
}
}
true
}

pub fn add(&mut self, key: u64) -> bool {
let key = Self::mix(key);
loop {
if !self.bits.is_empty() {
self.testadd(key, true);
return true;
}
if self.count == self.nbuckets >> 1 {
if !self.grow() {
return false;
}
continue;
}
break;
}
self.addkey(key)
}

pub fn addkey(&mut self, key: u64) -> bool {
let mut key = key & 0x00FFFFFFFFFFFFFF; // RHBLOOM_KEY
let mut dib = 1i64;
let mut i = (key as usize) & (self.nbuckets - 1);

loop {
let current = self.buckets[i];
let current_dib = (current >> 56) as i64;

if current_dib == 0 {
self.buckets[i] = key | ((dib as u64) << 56);
self.count += 1;
return true;
}

let current_key = current & 0x00FFFFFFFFFFFFFF;
if current_key == key {
return true;
}

if current_dib < dib {
// Robin Hood: swap with current
self.buckets[i] = key | ((dib as u64) << 56);
key = current_key;
dib = current_dib;
}

dib += 1;
i = (i + 1) & (self.nbuckets - 1);
}
}

pub fn test(&self, key: u64) -> bool {
let key = Self::mix(key);
if !self.bits.is_empty() {
// Bloom filter mode
let mut k = key & 0x00FFFFFFFFFFFFFF;
for i in 0..self.k {
let j = (k as usize) & (self.m - 1);
let byte_idx = j >> 3;
let bit_idx = j & 7;
if byte_idx >= self.bits.len() || ((self.bits[byte_idx] >> bit_idx) & 1) == 0 {
return false;
}
if i < self.k - 1 {
k = k.wrapping_mul(0x94d049bb133111eb);
k ^= k >> 31;
}
}
true
} else {
// Hash table mode
if self.buckets.is_empty() {
return false;
}
let key = key & 0x00FFFFFFFFFFFFFF;
let mut dib = 1i64;
let mut i = (key as usize) & (self.nbuckets - 1);

loop {
let current = self.buckets[i];
let current_dib = (current >> 56) as i64;
let current_key = current & 0x00FFFFFFFFFFFFFF;

if current_key == key {
return true;
}
if current_dib < dib {
return false;
}

dib += 1;
i = (i + 1) & (self.nbuckets - 1);
}
}
}
}
