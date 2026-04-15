pub struct BitWriter {
pub cache: [u8; 1024],
pub pos: usize,
pub byte: u8,
pub bit_count: u8,
}
impl BitWriter {
pub fn new() -> Self {
BitWriter {
cache: [0; 1024],
pos: 0,
byte: 0,
bit_count: 0,
}
}
pub fn write_bit(&mut self, bit: bool) {
if bit {
self.byte |= 1 << (7 - self.bit_count);
}
self.bit_count += 1;
if self.bit_count == 8 {
if self.pos < self.cache.len() {
self.cache[self.pos] = self.byte;
self.pos += 1;
}
self.byte = 0;
self.bit_count = 0;
}
}
pub fn flush(&mut self) {
if self.bit_count > 0 {
if self.pos < self.cache.len() {
self.cache[self.pos] = self.byte;
self.pos += 1;
}
}
}
}
pub struct BitReader<'a> {
pub data: &'a [u8],
pub len: usize,
pub v: u8,
pub n: u8,
}
impl<'a> BitReader<'a> {
pub fn new(data: &'a [u8]) -> Self {
BitReader {
data,
len: data.len(),
v: 0,
n: 0,
}
}
pub fn read_bit(&mut self) -> Option<bool> {
if self.n == 0 {
if self.data.is_empty() {
return None;
}
self.v = self.data[0];
self.data = &self.data[1..];
self.len -= 1;
self.n = 8;
}
self.n -= 1;
let bit = (self.v >> self.n) & 1 == 1;
Some(bit)
}
}
pub struct FloatEncoder {
pub w: BitWriter,
pub val: u64,
pub leading: u8,
pub trailing: u8,
pub finished: bool,
}
impl FloatEncoder {
pub fn new() -> Self {
FloatEncoder {
w: BitWriter::new(),
val: 0,
leading: 0,
trailing: 0,
finished: false,
}
}
pub fn float_encoder_init(&mut self) {
self.finished = false;
self.val = 0;
self.leading = 0;
self.trailing = 0;
self.w.pos = 0;
self.w.byte = 0;
self.w.bit_count = 0;
self.w.cache = [0; 1024];
}
pub fn float_encode_write(&mut self, value: f64) {
let bits = value.to_bits();
if self.val == 0 && self.leading == 0 && self.trailing == 0 && !self.finished {
for i in (0..64).rev() {
self.w.write_bit((bits >> i) & 1 == 1);
}
self.val = bits;
self.leading = bits.leading_zeros() as u8;
self.trailing = bits.trailing_zeros() as u8;
} else {
let xor = bits ^ self.val;
if xor == 0 {
self.w.write_bit(false);
} else {
self.w.write_bit(true);
for i in (0..64).rev() {
self.w.write_bit((xor >> i) & 1 == 1);
}
}
self.val = bits;
}
}
pub fn float_encode_flush(&mut self, buffer: &mut [u8], length: &mut usize) {
self.w.flush();
let count = self.w.pos.min(buffer.len());
buffer[..count].copy_from_slice(&self.w.cache[..count]);
*length = count;
self.finished = true;
}
}
pub struct FloatDecoder<'a> {
pub val: u64,
pub leading: u8,
pub trailing: u8,
pub br: BitReader<'a>,
pub b: [u8; 1024],
pub finished: bool,
pub err: i32,
}
impl<'a> FloatDecoder<'a> {
pub fn new(data: &'a [u8]) -> Self {
FloatDecoder {
val: 0,
leading: 0,
trailing: 0,
br: BitReader::new(data),
b: [0; 1024],
finished: false,
err: 0,
}
}
pub fn float_decode_block(&mut self, buffer: &'a [u8], output: &mut [f64], length: &mut usize) -> i32 {
self.br = BitReader::new(buffer);
self.finished = false;
self.err = 0;
let mut count = 0;
let max_count = output.len();
if max_count == 0 {
*length = 0;
return 0;
}
let mut bits: u64 = 0;
for _ in 0..64 {
match self.br.read_bit() {
Some(bit) => {
bits = (bits << 1) | (bit as u64);
}
None => {
self.err = -1;
*length = count;
return self.err;
}
}
}
output[count] = f64::from_bits(bits);
self.val = bits;
count += 1;
while count < max_count {
match self.br.read_bit() {
Some(false) => {
output[count] = f64::from_bits(self.val);
count += 1;
}
Some(true) => {
let mut xor: u64 = 0;
for _ in 0..64 {
match self.br.read_bit() {
Some(bit) => {
xor = (xor << 1) | (bit as u64);
}
None => {
self.err = -1;
*length = count;
return self.err;
}
}
}
let bits = self.val ^ xor;
output[count] = f64::from_bits(bits);
self.val = bits;
count += 1;
}
None => {
break;
}
}
}
*length = count;
self.finished = true;
0
}
}
