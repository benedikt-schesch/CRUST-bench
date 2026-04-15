pub const VEC_VERSION: &str = "0.2.1";

pub fn vec_insert<T>(data: &mut Vec<T>, idx: usize, value: T) -> i32 {
if vec_expand(data) != 0 {
return -1;
}
data.insert(idx, value);
0
}

pub fn vec_splice<T>(data: &mut Vec<T>, start: usize, count: usize) {
if start >= data.len() {
return;
}
let end = (start + count).min(data.len());
data.drain(start..end);
}

pub fn vec_swapsplice<T>(data: &mut Vec<T>, start: usize, count: usize) {
let len = data.len();
if start >= len {
return;
}
let count = count.min(len - start);
if count == 0 {
return;
}
let src = len - count;
// Swap elements from the end to fill the gap, avoiding the need for Copy
for i in 0..count {
data.swap(start + i, src + i);
}
data.truncate(len - count);
}

pub fn vec_reserve_po2<T>(data: &mut Vec<T>, n: usize) -> i32 {
if n == 0 {
return 0;
}
let mut n2 = 1;
while n2 < n {
n2 <<= 1;
}
vec_reserve(data, n2)
}

pub fn vec_expand<T>(data: &mut Vec<T>) -> i32 {
if data.len() == data.capacity() {
let additional = if data.capacity() == 0 { 1 } else { data.capacity() };
match data.try_reserve(additional) {
Ok(()) => 0,
Err(_) => -1,
}
} else {
0
}
}

pub fn vec_reserve<T>(data: &mut Vec<T>, n: usize) -> i32 {
if n > data.capacity() {
let additional = n.saturating_sub(data.len());
match data.try_reserve(additional) {
Ok(()) => 0,
Err(_) => -1,
}
} else {
0
}
}

pub fn vec_compact<T>(data: &mut Vec<T>) -> i32 {
data.shrink_to_fit();
0
}

pub fn vec_swap<T>(data: &mut Vec<T>, idx1: usize, idx2: usize) {
if idx1 == idx2 {
return;
}
if idx1 < data.len() && idx2 < data.len() {
data.swap(idx1, idx2);
}
}
