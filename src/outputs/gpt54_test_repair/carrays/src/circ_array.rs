
use std::mem;
pub struct CircBuf {
el: usize,
start: usize,
n: usize,
size: usize,
mask: usize,
b: std::ptr::NonNull<u8>,
storage: Vec<u8>,
}
impl CircBuf {
pub fn new(el: usize, size: usize) -> Self {
let size = roundup64(size as u64) as usize;
let size = size.max(1);
let len = el.saturating_mul(size);
let mut storage = vec![0u8; len];
let ptr =
std::ptr::NonNull::new(storage.as_mut_ptr()).unwrap_or(std::ptr::NonNull::dangling());
Self {
el,
start: 0,
n: 0,
size,
mask: size.saturating_sub(1),
b: ptr,
storage,
}
}
pub fn dealloc(&mut self) {
self.storage.clear();
self.storage.shrink_to_fit();
self.b = std::ptr::NonNull::dangling();
self.start = 0;
self.n = 0;
self.size = 0;
self.mask = 0;
}
pub fn resize(&mut self, size: usize) {
assert!(size > self.size && (size & (size - 1)) == 0);
let new_bytes = size.saturating_mul(self.el);
let mut old_vec = self.as_vec_clone_len(self.size * self.el);
old_vec.resize(new_bytes, 0);
if self.start + self.n > self.size {
let nend = self.size - self.start;
let nbeg = (self.start + self.n) & self.mask;
if nend < nbeg {
let src_start = (self.size - nend) * self.el;
let dst_start = (size - nend) * self.el;
let count = nend * self.el;
old_vec.copy_within(src_start..src_start + count, dst_start);
} else {
let src_start = 0;
let dst_start = self.size * self.el;
let count = nbeg * self.el;
old_vec.copy_within(src_start..src_start + count, dst_start);
}
}
self.storage = old_vec;
self.b = std::ptr::NonNull::new(self.storage.as_mut_ptr())
.unwrap_or(std::ptr::NonNull::dangling());
self.size = size;
self.mask = size - 1;
}
pub fn capacity(&mut self, size: usize) {
if size > self.size {
self.resize(roundup64(size as u64) as usize);
}
}
pub fn push(&mut self) -> &mut [u8] {
if self.n == self.size {
self.resize(self.size * 2);
}
self.start = if self.start != 0 {
self.start - 1
} else {
self.size - 1
};
self.n += 1;
let ptr = self.get(0);
ptr.fill(0);
ptr
}
pub fn pop(&mut self) -> &mut [u8] {
assert!(self.n > 0);
let old = self.start;
self.start = (self.start + 1) & self.mask;
self.n -= 1;
let start = old * self.el;
let end = start + self.el;
let bytes = self.as_mut_slice_total();
&mut bytes[start..end]
}
pub fn unshift(&mut self) -> &mut [u8] {
if self.n == self.size {
self.resize(self.size * 2);
}
let idx = self.n;
self.n += 1;
let ptr = self.get(idx);
ptr.fill(0);
ptr
}
pub fn shift(&mut self) -> &mut [u8] {
assert!(self.n > 0);
self.n -= 1;
let idx = self.n;
self.get(idx)
}
pub fn norm(&mut self) {
if self.start + self.n > self.size {
let newstart = (self.size - self.n) / 2;
let nleft = self.start + self.n - self.size;
let nright = self.size - self.start;
let el = self.el;
let start = self.start;
let size = self.size;
if nleft <= newstart {
let src1 = start * el;
let dst1 = newstart * el;
let cnt1 = nright * el;
let src2 = 0;
let dst2 = (newstart + nright) * el;
let cnt2 = nleft * el;
let total = self.as_mut_slice_total();
let first = total[src1..src1 + cnt1].to_vec();
total[dst1..dst1 + cnt1].copy_from_slice(&first);
let second = total[src2..src2 + cnt2].to_vec();
total[dst2..dst2 + cnt2].copy_from_slice(&second);
} else {
let shift = start - newstart;
let total = self.as_mut_slice_total();
gca_cycle_left(total, size, el, shift);
}
self.start = newstart;
}
}
fn get(&mut self, idx: usize) -> &mut [u8] {
let pos = (self.start + idx) & self.mask;
let start = pos * self.el;
let end = start + self.el;
let bytes = self.as_mut_slice_total();
&mut bytes[start..end]
}
fn as_mut_slice_total(&mut self) -> &mut [u8] {
self.b = std::ptr::NonNull::new(self.storage.as_mut_ptr())
.unwrap_or(std::ptr::NonNull::dangling());
self.storage.as_mut_slice()
}
fn as_vec_clone_len(&self, len: usize) -> Vec<u8> {
let mut out = self.storage.clone();
if out.len() < len {
out.resize(len, 0);
} else if out.len() > len {
out.truncate(len);
}
out
}
}
fn roundup64(mut x: u64) -> u64 {
if x <= 1 {
return 1;
}
x -= 1;
x |= x >> 1;
x |= x >> 2;
x |= x >> 4;
x |= x >> 8;
x |= x >> 16;
x |= x >> 32;
x + 1
}
fn gca_cycle_left(ptr: &mut [u8], n: usize, es: usize, shift: usize) {
if n <= 1 || shift == 0 || es == 0 {
return;
}
let shift = shift % n;
if shift == 0 {
return;
}
let gcd = gca_calc_gcd(n as u32, shift as u32) as usize;
let mut tmp = vec![0u8; es];
for i in 0..gcd {
let src = i * es;
tmp.copy_from_slice(&ptr[src..src + es]);
let mut j = i;
loop {
let mut k = j + shift;
if k >= n {
k -= n;
}
if k == i {
break;
}
let from = k * es;
let to = j * es;
let chunk = ptr[from..from + es].to_vec();
ptr[to..to + es].copy_from_slice(&chunk);
j = k;
}
let to = j * es;
ptr[to..to + es].copy_from_slice(&tmp);
}
}
fn gca_calc_gcd(mut a: u32, mut b: u32) -> u32 {
if a == 0 {
return b;
}
if b == 0 {
return a;
}
let mut shift = 0u32;
while ((a | b) & 1) == 0 {
a >>= 1;
b >>= 1;
shift += 1;
}
while (a & 1) == 0 {
a >>= 1;
}
loop {
while (b & 1) == 0 {
b >>= 1;
}
if a > b {
mem::swap(&mut a, &mut b);
}
b -= a;
if b == 0 {
break;
}
}
a << shift
}
