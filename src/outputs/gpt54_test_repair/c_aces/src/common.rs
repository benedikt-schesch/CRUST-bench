use std::cell::RefCell;
use std::f64::consts::PI;
use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Xgcd {
pub gcd: u64,
pub a: i64,
pub b: i64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pair {
pub first: u64,
pub second: u64,
}
thread_local! {
static RNG_STATE: RefCell<u64> = RefCell::new(
SystemTime::now()
.duration_since(UNIX_EPOCH)
.map(|d| d.as_nanos() as u64)
.unwrap_or(0x9E3779B97F4A7C15)
);
}
fn next_u64() -> u64 {
RNG_STATE.with(|state| {
let mut x = *state.borrow();
if x == 0 {
x = 0x9E3779B97F4A7C15;
}
x ^= x << 13;
x ^= x >> 7;
x ^= x << 17;
*state.borrow_mut() = x;
x
})
}
fn rand_val() -> f64 {
(next_u64() as f64) / (u64::MAX as f64)
}
pub fn gcd(mut x: u64, mut y: u64) -> u64 {
if x == 0 {
return y;
}
if y == 0 {
return x;
}
while x != y {
if x > y {
x -= y;
} else {
y -= x;
}
}
x
}
pub fn xgcd(mut a: u64, mut b: u64) -> Xgcd {
let mut prev_a: i64 = 1;
let mut cur_a: i64 = 0;
let mut prev_b: i64 = 0;
let mut cur_b: i64 = 1;
while b != 0 {
let q = (a / b) as i64;
let temp = (a % b) as i64;
a = b;
b = temp as u64;
let t = cur_a;
cur_a = prev_a - q * cur_a;
prev_a = t;
let t2 = cur_b;
cur_b = prev_b - q * cur_b;
prev_b = t2;
}
Xgcd {
gcd: a,
a: prev_a,
b: prev_b,
}
}
pub fn are_coprime(x: u64, y: u64) -> bool {
!(gcd(x, y) > 1)
}
pub fn randinverse(value: u64) -> Pair {
let mut a = randrange(2, value - 1);
while !are_coprime(a, value) {
a = randrange(2, value - 1);
}
let result = xgcd(a, value);
Pair {
first: a,
second: if result.a > 0 {
result.a as u64
} else {
(result.a + value as i64) as u64
},
}
}
pub fn randrange(lower: u64, upper: u64) -> u64 {
if upper <= lower {
return lower;
}
(next_u64() % (upper - lower + 1)) + lower
}
pub fn normal_rand(mean: f64, stddev: f64) -> f64 {
let mut u = 0.0;
while u == 0.0 {
u = rand_val();
}
let r = (-2.0 * u.ln()).sqrt();
let mut theta = 0.0;
while theta == 0.0 {
theta = 2.0 * PI * rand_val();
}
let x = r * theta.cos();
(x * stddev) + mean
}
pub fn max(a: u64, b: u64) -> u64 {
if a > b { a } else { b }
}
pub fn min(a: u64, b: u64) -> u64 {
if a < b { a } else { b }
}
pub fn clamp(min_value: u64, max_value: u64, value: u64) -> u64 {
max(min_value, min(max_value, value))
}
