
pub struct CircularArray<T> {
data: Vec<T>,
capacity: usize,
start: usize,
len: usize,
}
impl<T> CircularArray<T> {
pub fn new(capacity: usize) -> Self {
Self {
data: Vec::with_capacity(capacity),
capacity,
start: 0,
len: 0,
}
}
pub fn capacity(&self) -> usize {
self.capacity
}
pub fn len(&self) -> usize {
self.len
}
pub fn is_empty(&self) -> bool {
self.len == 0
}
}
pub fn gca_roundup32(x: u32) -> u32 {
if x == 0 {
return 0;
}
let mut x = x - 1;
x |= x >> 1;
x |= x >> 2;
x |= x >> 4;
x |= x >> 8;
x |= x >> 16;
x + 1
}
pub fn gca_roundup64(x: u64) -> u64 {
if x == 0 {
return 0;
}
let mut x = x - 1;
x |= x >> 1;
x |= x >> 2;
x |= x >> 4;
x |= x >> 8;
x |= x >> 16;
x |= x >> 32;
x + 1
}
pub fn gca_calc_gcd(a: u64, b: u64) -> u64 {
if a == 0 && b == 0 {
return 0;
}
let mut a = a;
let mut b = b;
while b != 0 {
let temp = b;
b = a % b;
a = temp;
}
a
}
