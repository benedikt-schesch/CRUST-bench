// Generated Rust Code
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Debug)]
pub struct BigInt {
negative: bool,
digits: Vec<u8>,
}

impl BigInt {
pub fn zero() -> Self {
BigInt {
negative: false,
digits: vec![0],
}
}

pub fn is_64_bit(&self) -> bool {
if self.digits.len() > 10 {
return false;
}
if self.digits.len() < 10 {
return true;
}
false
}

pub fn remove_leading_zeros(&mut self) {
let mut leading_zeros = 0usize;
while leading_zeros < self.digits.len() && self.digits[leading_zeros] == 0 {
leading_zeros += 1;
}

if leading_zeros > 0 {
if leading_zeros >= self.digits.len() {
self.digits = vec![0];
self.negative = false;
return;
}
self.digits.drain(0..leading_zeros);
}
}

pub fn from_int(n: i64) -> Self {
let mut result = BigInt {
negative: n < 0,
digits: Vec::new(),
};

let mut m: i128 = if n < 0 { -(n as i128) } else { n as i128 };

result.digits.push(0);
while m > 0 {
result.digits.push(0);
m /= 10;
}

m = if n < 0 { -(n as i128) } else { n as i128 };
let size = result.digits.len();
for i in 0..size {
result.digits[size - i - 1] = (m % 10) as u8;
m /= 10;
}

result.remove_leading_zeros();
result
}

pub fn to_int(&self) -> i64 {
let mut result: i64 = 0;
for &d in &self.digits {
result = result * 10 + d as i64;
}
if self.negative { -result } else { result }
}

pub fn from_str(s: &str) -> Self {
let mut negative = false;
let mut rest = s;
if let Some(stripped) = s.strip_prefix('-') {
negative = true;
rest = stripped;
}

let digits: Vec<u8> = rest.bytes().map(|b| b - b'0').collect();
let mut result = BigInt {
negative,
digits: if digits.is_empty() { vec![0] } else { digits },
};
result.remove_leading_zeros();
result
}

pub fn copy(&self) -> BigInt {
self.clone()
}

pub fn print(&self) {
print!("{}", self);
}

pub fn is_zero(&self) -> bool {
let mut n = self.clone();
n.remove_leading_zeros();
n.digits.len() == 1 && n.digits[0] == 0
}

pub fn lt_zero(&self) -> bool {
let mut n = self.clone();
n.remove_leading_zeros();
if n.digits.len() == 1 && n.digits[0] == 0 {
return false;
}
n.negative
}

pub fn gt_zero(&self) -> bool {
let mut n = self.clone();
n.remove_leading_zeros();
if n.digits.len() == 1 && n.digits[0] == 0 {
return false;
}
!n.negative
}

pub fn lezero(&self) -> bool {
let mut n = self.clone();
n.remove_leading_zeros();
if n.digits.len() == 1 && n.digits[0] == 0 {
return true;
}
n.negative
}

pub fn gezero(&self) -> bool {
let mut n = self.clone();
n.remove_leading_zeros();
if n.digits.len() == 1 && n.digits[0] == 0 {
return true;
}
!n.negative
}

pub fn abs(&self) -> Self {
let mut n = self.clone();
n.negative = false;
n
}

pub fn add(&self, other: &Self) -> Self {
let mut a = self.clone();
let mut b = other.clone();
let mut result;

if a.negative && b.negative {
result = BigInt {
negative: true,
digits: Vec::new(),
};
} else if a.negative {
a.negative = false;
return b.sub(&a);
} else if b.negative {
b.negative = false;
return a.sub(&b);
} else {
result = BigInt {
negative: false,
digits: Vec::new(),
};
}

result.digits = vec![0; a.digits.len().max(b.digits.len())];

let mut carry: i64 = 0;
let result_len = result.digits.len();
for i in 0..result_len {
let mut sum = carry;
if i < a.digits.len() {
sum += a.digits[a.digits.len() - i - 1] as i64;
}
if i < b.digits.len() {
sum += b.digits[b.digits.len() - i - 1] as i64;
}
let idx = result_len - i - 1;
result.digits[idx] = (sum % 10) as u8;
carry = sum / 10;
}

if carry > 0 {
let mut new_digits = Vec::with_capacity(result.digits.len() + 1);
new_digits.push(carry as u8);
new_digits.extend(result.digits);
result.digits = new_digits;
}

result.remove_leading_zeros();
result
}

pub fn sub(&self, other: &Self) -> Self {
let mut a = self.clone();
let mut b = other.clone();
let mut result;

if a.negative && b.negative {
a.negative = false;
b.negative = false;
result = b.add(&a);
result.negative = true;
return result;
}

if a.negative {
b.negative = true;
return a.add(&b);
}
if b.negative {
b.negative = false;
return a.add(&b);
}

result = BigInt {
negative: false,
digits: vec![0; a.digits.len().max(b.digits.len())],
};

let is_negative = lt(&a, &b);

let mut carry: i64 = 0;
let larger = if gt(&a, &b) { a.clone() } else { b.clone() };
let smaller = if gt(&a, &b) { b.clone() } else { a.clone() };

let result_len = result.digits.len();
for i in 0..result_len {
let mut diff = carry;
if i < larger.digits.len() {
diff += larger.digits[larger.digits.len() - i - 1] as i64;
}
if i < smaller.digits.len() {
diff -= smaller.digits[smaller.digits.len() - i - 1] as i64;
}
if diff < 0 {
diff += 10;
carry = -1;
} else {
carry = 0;
}
let idx = result_len - i - 1;
result.digits[idx] = diff as u8;
}

result.remove_leading_zeros();
result.negative = is_negative && !result.is_zero();
result
}

pub fn inc(&mut self) {
let one = BigInt::from_str("1");
*self = self.add(&one);
}

pub fn dec(&mut self) {
let one = BigInt::from_str("1");
*self = self.sub(&one);
}

pub fn mul(&self, other: &Self) -> Self {
if self.is_64_bit() && other.is_64_bit() {
return BigInt::from_int(self.to_int() * other.to_int());
}

let mut result = BigInt {
negative: self.negative != other.negative,
digits: vec![0; self.digits.len() + other.digits.len()],
};

for i in 0..self.digits.len() {
for j in 0..other.digits.len() {
let idx = result.digits.len() - i - j - 1;
let val = result.digits[idx] as i64
+ (self.digits[self.digits.len() - i - 1] as i64)
* (other.digits[other.digits.len() - j - 1] as i64);
result.digits[idx] = val as u8;
}
}

let mut carry: i64 = 0;
for i in 0..result.digits.len() {
let idx = result.digits.len() - i - 1;
let sum = result.digits[idx] as i64 + carry;
result.digits[idx] = (sum % 10) as u8;
carry = sum / 10;
}

if carry > 0 {
let mut prefix = Vec::new();
let mut c = carry;
let mut temp = Vec::new();
while c > 0 {
temp.push((c % 10) as u8);
c /= 10;
}
temp.reverse();
prefix.extend(temp);
prefix.extend(result.digits);
result.digits = prefix;
}

result.remove_leading_zeros();
if result.is_zero() {
result.negative = false;
}
result
}

pub fn divmod(&self, other: &Self) -> (Self, Self) {
if self.is_64_bit() && other.is_64_bit() {
let quotient = self.to_int() / other.to_int();
let rem = self.to_int() % other.to_int();
return (BigInt::from_int(quotient), BigInt::from_int(rem));
}

let mut quotient = BigInt::from_str("0");

let negative = self.negative != other.negative;
let mut numerator = self.clone();
let mut denominator = other.clone();
numerator.negative = false;
denominator.negative = false;

numerator = numerator.copy();

while numerator.gt_zero() {
numerator = numerator.sub(&denominator);
if numerator.lt_zero() {
break;
}
quotient.inc();
}

if numerator.lt_zero() {
numerator = numerator.add(&denominator);
}

let mut remainder = numerator;
quotient.remove_leading_zeros();
if negative {
quotient.negative = true;
remainder.negative = true;
}

if quotient.is_zero() {
quotient.negative = false;
}
if remainder.is_zero() {
remainder.negative = false;
}

(quotient, remainder)
}

pub fn div(&self, other: &Self) -> Self {
if self.is_64_bit() && other.is_64_bit() {
return BigInt::from_int(self.to_int() / other.to_int());
}
let (q, _) = self.divmod(other);
q
}

pub fn r#mod(&self, other: &Self) -> Self {
if self.is_64_bit() && other.is_64_bit() {
return BigInt::from_int(self.to_int() % other.to_int());
}
let (_, r) = self.divmod(other);
r
}

pub fn pow(&self, exponent: &Self) -> Self {
if exponent.negative {
return BigInt::from_str("0");
}
let mut result = BigInt::from_str("1");
if exponent.is_zero() {
return result;
}
let mut b = exponent.copy();
while !b.is_zero() {
result = result.mul(self);
b.dec();
}
result
}

pub fn fast_pow(&self, exponent: &Self, modulo: &Self) -> Self {
if self.is_64_bit() && exponent.is_64_bit() && modulo.is_64_bit() {
let mut result: i64 = 1;
let mut base = self.to_int();
let mut exp = exponent.to_int();
let modu = modulo.to_int();
base %= modu;
while exp > 0 {
if exp % 2 == 1 {
result = (result * base) % modu;
}
exp >>= 1;
base = (base * base) % modu;
}
return BigInt::from_int(result);
}

if exponent.negative {
return BigInt::from_str("0");
}
if exponent.is_zero() {
return BigInt::from_str("1");
}

let mut b = exponent.copy();
let a = self.r#mod(modulo);
let mut result = a.copy();
let b_save_initial = b.copy();
b.dec();
let mut pow = BigInt::from_str("1");

loop {
let two = BigInt::from_str("2");
let tmp2 = b.div(&two);
if !tmp2.gt_zero() {
break;
}

result = result.mul(&result).r#mod(modulo);
b = b.div(&two);
pow = pow.add(&pow);
}

let mut b_save = b_save_initial.sub(&pow);
if b_save.is_odd() {
result = result.mul(&a).r#mod(modulo);
b_save.dec();
pow.inc();
}

if lt(&b_save, &BigInt::from_int(5)) {
while !b_save.is_zero() {
result = result.mul(&a).r#mod(modulo);
b_save.dec();
pow.inc();
}
return result;
}

let subpart = self.fast_pow(&b_save, modulo);
result = result.mul(&subpart);
result = result.r#mod(modulo);
result
}

pub fn modinv(&self, modulo: &Self) -> Self {
let m0 = modulo.copy();
let mut y = BigInt::from_str("0");
let mut x = BigInt::from_str("1");
let mut q = BigInt::from_str("0");
let mut t = BigInt::from_str("0");
let _temp = BigInt::from_str("0");
let one = BigInt::from_str("1");
let mut a = self.copy();
let mut m = modulo.copy();

while !a.is_zero() {
q = m.div(&a);
t = m.sub(&q.mul(&a));
m = a.copy();
a = t.copy();
t = y.sub(&q.mul(&x));
y = x.copy();
x = t.copy();
}

if m.lt_zero() {
m = m.add(&m0);
}

if m == one {
let mut result = y.r#mod(&m0);
if result.lt_zero() {
result = result.add(&m0);
}
return result;
}

BigInt::from_int(0)
}

pub fn sqrt(&self) -> Self {
let one = BigInt::from_str("1");
let two = BigInt::from_str("2");
let mut low = BigInt::from_str("0");
let mut high = self.copy();
let mut mid = BigInt::from_str("0");

while lt(&low, &high) {
mid = low.add(&high).div(&two);
let sq = mid.mul(&mid);
if lt(&sq, self) {
low = mid.add(&one);
} else {
high = mid.copy();
}
}

low
}

pub fn is_even(&self) -> bool {
self.digits[self.digits.len() - 1] % 2 == 0
}

pub fn is_odd(&self) -> bool {
self.digits[self.digits.len() - 1] % 2 == 1
}

pub fn is_prime(&self) -> bool {
if self.is_even() {
return false;
}

if self.digits.len() > 1 {
let last = self.digits[self.digits.len() - 1];
if last == 5 || last == 0 {
return false;
}
}

let mut sum = BigInt::from_str("0");
for &d in &self.digits {
sum = sum.add(&BigInt::from_int(d as i64));
}

let three = BigInt::from_str("3");
let rem = sum.r#mod(&three);
if rem.is_zero() {
return false;
}

let sqrt_n = self.sqrt();
let mut i = BigInt::from_str("2");
while le(&i, &sqrt_n) {
let rem2 = self.r#mod(&i);
if rem2.is_zero() {
return false;
}
i.inc();
}

true
}
}

pub fn gt(a: &BigInt, b: &BigInt) -> bool {
let mut a = a.clone();
let mut b = b.clone();
a.remove_leading_zeros();
b.remove_leading_zeros();

if a.negative != b.negative {
return b.negative;
}

if a.size_hint() > b.size_hint() {
return !a.negative;
}
if a.size_hint() < b.size_hint() {
return a.negative;
}
for i in 0..a.digits.len() {
if a.digits[i] > b.digits[i] {
return !a.negative;
}
if a.digits[i] < b.digits[i] {
return a.negative;
}
}
false
}

pub fn lt(a: &BigInt, b: &BigInt) -> bool {
!ge(a, b)
}

pub fn ge(a: &BigInt, b: &BigInt) -> bool {
gt(a, b) || a == b
}

pub fn le(a: &BigInt, b: &BigInt) -> bool {
!gt(a, b)
}

pub fn delete(a: &mut BigInt) {
a.digits.clear();
a.digits.push(0);
a.negative = false;
}

impl BigInt {
fn size_hint(&self) -> usize {
self.digits.len()
}
}

impl std::ops::Neg for BigInt {
type Output = BigInt;

fn neg(mut self) -> BigInt {
if !self.is_zero() {
self.negative = !self.negative;
}
self
}
}

impl PartialEq for BigInt {
fn eq(&self, other: &Self) -> bool {
let mut a = self.clone();
let mut b = other.clone();
a.remove_leading_zeros();
b.remove_leading_zeros();

if a.digits.len() != b.digits.len() {
return false;
}
if a.negative != b.negative {
return false;
}
for i in 0..a.digits.len() {
if a.digits[i] != b.digits[i] {
return false;
}
}
true
}
}

impl Eq for BigInt {}

impl PartialOrd for BigInt {
fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
if self == other {
Some(Ordering::Equal)
} else if gt(self, other) {
Some(Ordering::Greater)
} else {
Some(Ordering::Less)
}
}
}

impl fmt::Display for BigInt {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
if self.negative && !self.is_zero() {
write!(f, "-")?;
}
for d in &self.digits {
write!(f, "{}", d)?;
}
Ok(())
}
}
