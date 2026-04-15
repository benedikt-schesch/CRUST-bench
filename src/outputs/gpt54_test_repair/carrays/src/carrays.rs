use std::cmp::Ordering;
use std::mem;

pub fn gca_roundup32(mut x: u32) -> u32 {
if x == 0 {
return 0;
}
if x == 1 {
return 1;
}
x -= 1;
x |= x >> 1;
x |= x >> 2;
x |= x >> 4;
x |= x >> 8;
x |= x >> 16;
x + 1
}

pub fn gca_roundup64(mut x: u64) -> u64 {
if x == 0 {
return 0;
}
if x == 1 {
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

pub fn gca_calc_gcd(mut a: u32, mut b: u32) -> u32 {
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

pub fn gca_capacity<'a>(
ptr: &'a mut Vec<u8>,
size: &'a mut usize,
es: usize,
new_size: usize,
) -> Option<&'a mut Vec<u8>> {
if new_size > *size {
let rounded = gca_roundup64(new_size as u64) as usize;
let new_len = rounded.checked_mul(es)?;
if ptr.len() < new_len {
ptr.resize(new_len, 0);
}
*size = rounded;
}
Some(ptr)
}

pub fn gca_swapm(a: &mut [u8], b: &mut [u8]) {
let len = a.len().min(b.len());
for i in 0..len {
mem::swap(&mut a[i], &mut b[i]);
}
}

pub fn gca_cycle_left(ptr: &mut [u8], n: usize, es: usize, shift: usize) {
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

pub fn gca_cycle_right(ptr: &mut [u8], n: usize, es: usize, shift: usize) {
if n == 0 || shift == 0 {
return;
}
let shift = shift % n;
if shift == 0 {
return;
}
gca_cycle_left(ptr, n, es, n - shift);
}

pub fn gca_reverse(ptr: &mut [u8], n: usize, es: usize) {
if n <= 1 || es == 0 {
return;
}
for i in 0..(n / 2) {
let j = n - 1 - i;
let a_start = i * es;
let b_start = j * es;
let left = ptr[a_start..a_start + es].to_vec();
let right = ptr[b_start..b_start + es].to_vec();
ptr[a_start..a_start + es].copy_from_slice(&right);
ptr[b_start..b_start + es].copy_from_slice(&left);
}
}

pub fn gca_is_sorted<T, F>(base: &[T], compar: F) -> bool
where
F: Fn(&T, &T) -> Ordering,
{
if base.len() < 2 {
return true;
}
for i in 0..base.len() - 1 {
if compar(&base[i], &base[i + 1]) == Ordering::Greater {
return false;
}
}
true
}

pub fn gca_is_rsorted<T, F>(base: &[T], compar: F) -> bool
where
F: Fn(&T, &T) -> Ordering,
{
if base.len() < 2 {
return true;
}
for i in 0..base.len() - 1 {
if compar(&base[i], &base[i + 1]) == Ordering::Less {
return false;
}
}
true
}

pub fn gca_max<T, F>(base: &[T], compar: F) -> Option<&T>
where
F: Fn(&T, &T) -> Ordering,
{
if base.is_empty() {
return None;
}
let mut max = &base[0];
for item in &base[1..] {
if compar(max, item) == Ordering::Less {
max = item;
}
}
Some(max)
}

pub fn gca_min<T, F>(base: &[T], compar: F) -> Option<&T>
where
F: Fn(&T, &T) -> Ordering,
{
if base.is_empty() {
return None;
}
let mut min = &base[0];
for item in &base[1..] {
if compar(min, item) == Ordering::Greater {
min = item;
}
}
Some(min)
}
