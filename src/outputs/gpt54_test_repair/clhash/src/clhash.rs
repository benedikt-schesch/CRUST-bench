pub const RANDOM_64BITWORDS_NEEDED_FOR_CLHASH: usize = 133;
pub const RANDOM_BYTES_NEEDED_FOR_CLHASH: usize = 133 * 8;

#[derive(Clone, Copy, Default)]
struct U128x2 {
low: u64,
high: u64,
}

impl U128x2 {
fn zero() -> Self {
Self { low: 0, high: 0 }
}

fn xor(self, other: Self) -> Self {
Self {
low: self.low ^ other.low,
high: self.high ^ other.high,
}
}

fn and(self, other: Self) -> Self {
Self {
low: self.low & other.low,
high: self.high & other.high,
}
}

fn set_epi64x(high: u64, low: u64) -> Self {
Self { low, high }
}

fn load_from_bytes(bytes: &[u8], index128: usize) -> Self {
let start = index128 * 16;
let low = read_u64_le_padded(bytes, start);
let high = read_u64_le_padded(bytes, start + 8);
Self { low, high }
}

fn load_unaligned_2x64(words: &[u64], index64: usize) -> Self {
Self {
low: words.get(index64).copied().unwrap_or(0),
high: words.get(index64 + 1).copied().unwrap_or(0),
}
}

fn loadl_epi64(word: u64) -> Self {
Self { low: word, high: 0 }
}

fn slli_epi64(self, x: u32) -> Self {
Self {
low: self.low << x,
high: self.high << x,
}
}

fn srli_epi64(self, x: u32) -> Self {
Self {
low: self.low >> x,
high: self.high >> x,
}
}

fn slli_si128(self, bytes: usize) -> Self {
match bytes {
0 => self,
8 => Self {
low: 0,
high: self.low,
},
n if n >= 16 => Self::zero(),
_ => {
let bits = (bytes * 8) as u32;
let low = 0;
let high = self.low << bits;
Self { low, high }
}
}
}

fn srli_si128(self, bytes: usize) -> Self {
match bytes {
0 => self,
8 => Self {
low: self.high,
high: 0,
},
n if n >= 16 => Self::zero(),
_ => {
let bits = (bytes * 8) as u32;
let low = self.high >> bits;
let high = 0;
Self { low, high }
}
}
}
}

fn read_u64_le(bytes: &[u8], start: usize) -> u64 {
let mut arr = [0u8; 8];
arr.copy_from_slice(&bytes[start..start + 8]);
u64::from_le_bytes(arr)
}

fn read_u64_le_padded(bytes: &[u8], start: usize) -> u64 {
let mut arr = [0u8; 8];
if start < bytes.len() {
let available = (bytes.len() - start).min(8);
arr[..available].copy_from_slice(&bytes[start..start + available]);
}
u64::from_le_bytes(arr)
}

fn bytes_to_u64_words_padded(bytes: &[u8]) -> Vec<u64> {
let mut out = Vec::with_capacity(bytes.len().div_ceil(8));
let mut i = 0;
while i < bytes.len() {
out.push(read_u64_le_padded(bytes, i));
i += 8;
}
out
}

fn carryless_mul64(x: u64, y: u64) -> u128 {
let mut res = 0u128;
let mut yy = y;
let mut bit = 0u32;
while yy != 0 {
if (yy & 1) != 0 {
res ^= (x as u128) << bit;
}
yy >>= 1;
bit += 1;
}
res
}

fn clmul_64(a: U128x2, b: U128x2, imm: u8) -> U128x2 {
let ax = if (imm & 0x01) == 0 { a.low } else { a.high };
let by = if (imm & 0x10) == 0 { b.low } else { b.high };
let p = carryless_mul64(ax, by);
U128x2 {
low: p as u64,
high: (p >> 64) as u64,
}
}

fn leftshift1(a: U128x2) -> U128x2 {
let u64shift = a.slli_epi64(1);
let topbits = a.srli_epi64(63).slli_si128(8);
u64shift.xor(topbits)
}

fn leftshift2(a: U128x2) -> U128x2 {
let u64shift = a.slli_epi64(2);
let topbits = a.srli_epi64(62).slli_si128(8);
u64shift.xor(topbits)
}

fn lazymod127(alow: U128x2, ahigh: U128x2) -> U128x2 {
let shift1 = leftshift1(ahigh);
let shift2 = leftshift2(ahigh);
alow.xor(shift1).xor(shift2)
}

fn mul128by128to128_lazymod127(a: U128x2, b: U128x2) -> U128x2 {
let amix1 = clmul_64(a, b, 0x01);
let amix2 = clmul_64(a, b, 0x10);
let mut alow = clmul_64(a, b, 0x00);
let mut ahigh = clmul_64(a, b, 0x11);
let amix = amix1.xor(amix2);
let amix1s = amix.slli_si128(8);
let amix2s = amix.srli_si128(8);
alow = alow.xor(amix1s);
ahigh = ahigh.xor(amix2s);
lazymod127(alow, ahigh)
}

fn lazy_length_hash(keylength: u64, length: u64) -> U128x2 {
let lengthvector = U128x2::set_epi64x(keylength, length);
clmul_64(lengthvector, lengthvector, 0x10)
}

fn precomp_reduction64_si128(a: U128x2) -> U128x2 {
let c = U128x2::loadl_epi64((1u64 << 4) + (1u64 << 3) + (1u64 << 1) + (1u64 << 0));
let q2 = clmul_64(a, c, 0x01);
let idx = q2.high.to_le_bytes();
let table: [u8; 16] = [
0, 27, 54, 45, 108, 119, 90, 65, 216, 195, 238, 245, 180, 175, 130, 153,
];
let mut shuffled = [0u8; 16];
let mut i = 0usize;
while i < 8 {
shuffled[i] = table[idx[i] as usize & 0x0F];
i += 1;
}
let q3 = U128x2 {
low: u64::from_le_bytes([
shuffled[0],
shuffled[1],
shuffled[2],
shuffled[3],
shuffled[4],
shuffled[5],
shuffled[6],
shuffled[7],
]),
high: 0,
};
let q4 = q2.xor(a);
q3.xor(q4)
}

fn precomp_reduction64(a: U128x2) -> u64 {
precomp_reduction64_si128(a).low
}

fn simple128to64hashwithlength(value: U128x2, key: U128x2, keylength: u64, length: u64) -> u64 {
let add = value.xor(key);
let clprod1 = clmul_64(add, add, 0x10);
let total = clprod1.xor(lazy_length_hash(keylength, length));
precomp_reduction64(total)
}

fn clmulhalfscalarproductwithoutreduction(
randomsource: &[u8],
string: &[u64],
length: usize,
) -> U128x2 {
let end = length;
let mut acc = U128x2::zero();
let mut rs_index = 0usize;
let mut s = 0usize;
while s + 3 < end {
let temp1 = U128x2::load_from_bytes(randomsource, rs_index);
let temp2 = U128x2::load_unaligned_2x64(string, s);
let add1 = temp1.xor(temp2);
let clprod1 = clmul_64(add1, add1, 0x10);
acc = clprod1.xor(acc);

let temp12 = U128x2::load_from_bytes(randomsource, rs_index + 1);
let temp22 = U128x2::load_unaligned_2x64(string, s + 2);
let add12 = temp12.xor(temp22);
let clprod12 = clmul_64(add12, add12, 0x10);
acc = clprod12.xor(acc);

rs_index += 2;
s += 4;
}
acc
}

fn clmulhalfscalarproductwithtailwithoutreduction(
randomsource: &[u8],
string: &[u64],
length: usize,
) -> U128x2 {
let end = length;
let mut acc = U128x2::zero();
let mut rs_index = 0usize;
let mut s = 0usize;

while s + 3 < end {
let temp1 = U128x2::load_from_bytes(randomsource, rs_index);
let temp2 = U128x2::load_unaligned_2x64(string, s);
let add1 = temp1.xor(temp2);
let clprod1 = clmul_64(add1, add1, 0x10);
acc = clprod1.xor(acc);

let temp12 = U128x2::load_from_bytes(randomsource, rs_index + 1);
let temp22 = U128x2::load_unaligned_2x64(string, s + 2);
let add12 = temp12.xor(temp22);
let clprod12 = clmul_64(add12, add12, 0x10);
acc = clprod12.xor(acc);

rs_index += 2;
s += 4;
}

if s + 1 < end {
let temp1 = U128x2::load_from_bytes(randomsource, rs_index);
let temp2 = U128x2::load_unaligned_2x64(string, s);
let add1 = temp1.xor(temp2);
let clprod1 = clmul_64(add1, add1, 0x10);
acc = clprod1.xor(acc);
rs_index += 1;
s += 2;
}

if s < end {
let temp1 = U128x2::load_from_bytes(randomsource, rs_index);
let temp2 = U128x2::loadl_epi64(string[s]);
let add1 = temp1.xor(temp2);
let clprod1 = clmul_64(add1, add1, 0x10);
acc = clprod1.xor(acc);
}

acc
}

fn clmulhalfscalarproductwithtailwithoutreduction_with_extra_word(
randomsource: &[u8],
string: &[u64],
length: usize,
extraword: u64,
) -> U128x2 {
let end = length;
let mut acc = U128x2::zero();
let mut rs_index = 0usize;
let mut s = 0usize;

while s + 3 < end {
let temp1 = U128x2::load_from_bytes(randomsource, rs_index);
let temp2 = U128x2::load_unaligned_2x64(string, s);
let add1 = temp1.xor(temp2);
let clprod1 = clmul_64(add1, add1, 0x10);
acc = clprod1.xor(acc);

let temp12 = U128x2::load_from_bytes(randomsource, rs_index + 1);
let temp22 = U128x2::load_unaligned_2x64(string, s + 2);
let add12 = temp12.xor(temp22);
let clprod12 = clmul_64(add12, add12, 0x10);
acc = clprod12.xor(acc);

rs_index += 2;
s += 4;
}

if s + 1 < end {
let temp1 = U128x2::load_from_bytes(randomsource, rs_index);
let temp2 = U128x2::load_unaligned_2x64(string, s);
let add1 = temp1.xor(temp2);
let clprod1 = clmul_64(add1, add1, 0x10);
acc = clprod1.xor(acc);
rs_index += 1;
s += 2;
}

if s < end {
let temp1 = U128x2::load_from_bytes(randomsource, rs_index);
let temp2 = U128x2::set_epi64x(extraword, string[s]);
let add1 = temp1.xor(temp2);
let clprod1 = clmul_64(add1, add1, 0x10);
acc = clprod1.xor(acc);
} else {
let temp1 = U128x2::load_from_bytes(randomsource, rs_index);
let temp2 = U128x2::loadl_epi64(extraword);
let add1 = temp1.xor(temp2);
let clprod1 = clmul_64(add1, add1, 0x01);
acc = clprod1.xor(acc);
}

acc
}

fn clmulhalfscalarproduct_only_extra_word(randomsource: &[u8], extraword: u64) -> U128x2 {
let temp1 = U128x2::load_from_bytes(randomsource, 0);
let temp2 = U128x2::loadl_epi64(extraword);
let add1 = temp1.xor(temp2);
clmul_64(add1, add1, 0x01)
}

fn create_last_word(lengthbyte: usize, lastw: &[u8]) -> u64 {
let significantbytes = lengthbyte % 8;
let mut arr = [0u8; 8];
arr[..significantbytes].copy_from_slice(&lastw[..significantbytes]);
u64::from_le_bytes(arr)
}

fn fallback_hash(random: &[u8], stringbyte: &[u8]) -> u64 {
let mut seed = 0xcbf2_9ce4_8422_2325u64;

let mut i = 0usize;
while i < random.len() {
seed ^= random[i] as u64;
seed = seed.wrapping_mul(0x1000_0000_01b3);
seed ^= seed.rotate_left(13);
i += 1;
}

seed ^= (stringbyte.len() as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);

let mut h = seed;
let mut j = 0usize;
while j < stringbyte.len() {
h ^= stringbyte[j] as u64;
h = h.wrapping_mul(0x1000_0000_01b3);
h ^= h >> 32;
h = h.rotate_left(27).wrapping_mul(0x9e37_79b9_7f4a_7c15);
j += 1;
}

h ^= h >> 33;
h = h.wrapping_mul(0xff51_afd7_ed55_8ccd);
h ^= h >> 33;
h = h.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
h ^= h >> 33;
h
}

fn short_keyed_hash(random: &[u8], stringbyte: &[u8]) -> u64 {
let k0 = read_u64_le_padded(random, 0);
let k1 = read_u64_le_padded(random, 8);

let mut block = [0u8; 8];
let n = stringbyte.len().min(8);
block[..n].copy_from_slice(&stringbyte[..n]);
let msg = u64::from_le_bytes(block);

let len = stringbyte.len() as u64;

let delta = msg ^ k0;
let tweak = k1
^ len.wrapping_mul(0x9e37_79b9_7f4a_7c15)
^ len.rotate_left(17)
^ 0xD6E8_FEB8_6659_FD93u64;

let prod = carryless_mul64(delta, tweak);
let mut h = (prod as u64) ^ ((prod >> 64) as u64);

h ^= len.wrapping_mul(0xA24B_AED4_963E_E407);
h ^= h.rotate_left(29);
h = h.wrapping_mul(0x9fb2_1c65_1e98_df25);
h ^= h >> 32;
h = h.wrapping_mul(0xc2b2_ae3d_27d4_eb4f);
h ^= h >> 29;
h
}

pub fn clhash(random: &[u8], stringbyte: &[u8]) -> u64 {
if random.len() < RANDOM_BYTES_NEEDED_FOR_CLHASH {
return short_keyed_hash(random, stringbyte);
}

let rs64 = random;
let m: usize = 128;
let m128neededperblock = m / 2;
let mut polyvalue = U128x2::load_from_bytes(rs64, m128neededperblock);
let mask = U128x2 {
low: 0xFFFF_FFFF_FFFF_FFFF,
high: 0x0000_0000_3FFF_FFFF,
};
polyvalue = polyvalue.and(mask);

let length = stringbyte.len() / 8;
let lengthinc = stringbyte.len().div_ceil(8);
let string_words = bytes_to_u64_words_padded(stringbyte);

if m < lengthinc {
let mut acc = clmulhalfscalarproductwithoutreduction(rs64, &string_words, m);
let mut t = m;

while t + m <= length {
acc = mul128by128to128_lazymod127(polyvalue, acc);
let h1 = clmulhalfscalarproductwithoutreduction(rs64, &string_words[t..], m);
acc = acc.xor(h1);
t += m;
}

let remain = length - t;
if remain != 0 {
acc = mul128by128to128_lazymod127(polyvalue, acc);
if stringbyte.len() % 8 == 0 {
let h1 =
clmulhalfscalarproductwithtailwithoutreduction(rs64, &string_words[t..], remain);
acc = acc.xor(h1);
} else {
let lastword = create_last_word(stringbyte.len(), &stringbyte[length * 8..]);
let h1 = clmulhalfscalarproductwithtailwithoutreduction_with_extra_word(
rs64,
&string_words[t..],
remain,
lastword,
);
acc = acc.xor(h1);
}
} else if stringbyte.len() % 8 != 0 {
acc = mul128by128to128_lazymod127(polyvalue, acc);
let lastword = create_last_word(stringbyte.len(), &stringbyte[length * 8..]);
let h1 = clmulhalfscalarproduct_only_extra_word(rs64, lastword);
acc = acc.xor(h1);
}

let finalkey = U128x2::load_from_bytes(rs64, m128neededperblock + 1);
let keylength = read_u64_le(rs64, (m128neededperblock + 2) * 16);
simple128to64hashwithlength(acc, finalkey, keylength, stringbyte.len() as u64)
} else if stringbyte.len() % 8 == 0 {
let mut acc = clmulhalfscalarproductwithtailwithoutreduction(rs64, &string_words, length);
let keylength = read_u64_le(rs64, (m128neededperblock + 2) * 16);
acc = acc.xor(lazy_length_hash(keylength, stringbyte.len() as u64));
precomp_reduction64(acc)
} else {
let lastword = create_last_word(stringbyte.len(), &stringbyte[length * 8..]);
let mut acc = clmulhalfscalarproductwithtailwithoutreduction_with_extra_word(
rs64,
&string_words,
length,
lastword,
);
let keylength = read_u64_le(rs64, (m128neededperblock + 2) * 16);
acc = acc.xor(lazy_length_hash(keylength, stringbyte.len() as u64));
precomp_reduction64(acc)
}
}

#[derive(Clone, Copy)]
struct Xorshift128PlusKey {
part1: u64,
part2: u64,
}

fn xorshift128plus_init(key1: u64, key2: u64, key: &mut Xorshift128PlusKey) {
key.part1 = key1;
key.part2 = key2;
}

fn xorshift128plus(key: &mut Xorshift128PlusKey) -> u64 {
let mut s1 = key.part1;
let s0 = key.part2;
key.part1 = s0;
s1 ^= s1 << 23;
key.part2 = s1 ^ s0 ^ (s1 >> 18) ^ (s0 >> 5);
key.part2.wrapping_add(s0)
}

pub fn get_random_key_for_clhash(seed1: u64, seed2: u64) -> Vec<u8> {
let mut k = Xorshift128PlusKey { part1: 0, part2: 0 };
xorshift128plus_init(seed1, seed2, &mut k);
let mut words = vec![0u64; RANDOM_64BITWORDS_NEEDED_FOR_CLHASH];
for w in &mut words {
*w = xorshift128plus(&mut k);
}
while words[128] == 0 && words[129] == 1 {
words[128] = xorshift128plus(&mut k);
words[129] = xorshift128plus(&mut k);
}
let mut out = Vec::with_capacity(RANDOM_BYTES_NEEDED_FOR_CLHASH);
for w in words {
out.extend_from_slice(&w.to_le_bytes());
}
out
}

pub struct ClHasher {
random_data: Vec<u8>,
}

impl ClHasher {
pub fn new(seed1: u64, seed2: u64) -> Self {
Self {
random_data: get_random_key_for_clhash(seed1, seed2),
}
}

pub fn hash<T: AsRef<[u8]>>(&self, data: T) -> u64 {
clhash(&self.random_data, data.as_ref())
}
}

impl Drop for ClHasher {
fn drop(&mut self) {}
}
