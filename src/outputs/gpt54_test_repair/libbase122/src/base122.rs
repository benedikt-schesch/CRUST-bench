
use std::error::Error;
use std::fmt;
#[derive(Debug, Clone)]
pub struct Base122Error {
pub message: String,
}
impl Base122Error {
fn new(message: &str) -> Self {
Self {
message: message.to_string(),
}
}
}
impl fmt::Display for Base122Error {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
self.message.fmt(f)
}
}
impl Error for Base122Error {}
pub struct BitReader<'a> {
pub input: &'a [u8],
pub byte_pos: usize,
pub bit_pos: usize,
}
impl<'a> BitReader<'a> {
pub fn new(input: &'a [u8]) -> Self {
Self {
input,
byte_pos: 0,
bit_pos: 0,
}
}
pub fn read(&mut self, nbits: u8) -> (u8, u8) {
assert!(nbits > 0);
assert!(nbits <= 8);
let cur_bit = self.byte_pos * 8 + self.bit_pos;
let bit_len = self.input.len() * 8;
let max_nbits = bit_len.saturating_sub(cur_bit);
let mut nbits_usize = nbits as usize;
if nbits_usize > max_nbits {
nbits_usize = max_nbits;
}
if nbits_usize == 0 {
return (0, 0);
}
let first_byte_index = cur_bit / 8;
let first_byte_cur_bit = cur_bit % 8;
let mut two_bytes: u16 = self.input[first_byte_index] as u16;
two_bytes <<= 8;
if first_byte_index + 1 < self.input.len() {
two_bytes |= self.input[first_byte_index + 1] as u16;
}
let shift = (8 - first_byte_cur_bit) + (8 - nbits_usize);
two_bytes >>= shift;
let mask = !((255u16) << nbits_usize);
let out = (two_bytes & mask) as u8;
let new_cur_bit = cur_bit + nbits_usize;
self.byte_pos = new_cur_bit / 8;
self.bit_pos = new_cur_bit % 8;
(nbits_usize as u8, out)
}
}
pub struct BitWriter<'a> {
pub output: Option<&'a mut [u8]>,
pub len: usize,
pub cur_bit: usize,
pub count_only: bool,
}
impl<'a> BitWriter<'a> {
pub fn new(output: Option<&'a mut [u8]>, len: usize) -> Self {
Self {
count_only: output.is_none(),
output,
len,
cur_bit: 0,
}
}
pub fn write(&mut self, nbits: u8, value: u8) -> Result<usize, Base122Error> {
assert!(nbits > 0);
assert!(nbits <= 8);
let nbits_usize = nbits as usize;
if self.count_only {
self.cur_bit += nbits_usize;
return Ok(self.cur_bit / 8);
}
let bit_len = self.len * 8;
if self.cur_bit + nbits_usize > bit_len {
return Err(Base122Error::new("Output does not have sufficient size"));
}
let first_byte_index = self.cur_bit / 8;
let first_byte_cur_bit = self.cur_bit % 8;
let out = self.output.as_deref_mut().ok_or_else(|| Base122Error::new("Output does not have sufficient size"))?;
let mask1: u8 = !(255u8 >> first_byte_cur_bit);
let mut two_bytes: u16 = (out[first_byte_index] & mask1) as u16;
two_bytes <<= 8;
let mask2: u8 = !((255u16 << nbits_usize) as u8);
two_bytes |= ((value & mask2) as u16) << (8 + (8 - first_byte_cur_bit) - nbits_usize);
out[first_byte_index] = (two_bytes >> 8) as u8;
if first_byte_cur_bit + nbits_usize > 8 {
out[first_byte_index + 1] = two_bytes as u8;
}
self.cur_bit += nbits_usize;
Ok(self.cur_bit / 8)
}
}
const ILLEGALS: [u8; 6] = [
0,  
10, 
13, 
34, 
38, 
92, 
];
fn is_illegal(val: u8) -> bool {
ILLEGALS.contains(&val)
}
fn get_illegal_index(val: u8) -> u8 {
for (i, &v) in ILLEGALS.iter().enumerate() {
if v == val {
return i as u8;
}
}
panic!("unreachable");
}
pub fn encode(input: &[u8]) -> Result<Vec<u8>, Base122Error> {
let mut out_written = 0usize;
encode_internal(input, None, &mut out_written)?;
let mut output = vec![0u8; out_written];
let mut written2 = 0usize;
encode_internal(input, Some(output.as_mut_slice()), &mut written2)?;
output.truncate(written2);
Ok(output)
}
fn encode_internal(input: &[u8], mut output: Option<&mut [u8]>, out_written: &mut usize) -> Result<(), Base122Error> {
let mut reader = BitReader::new(input);
let mut out_index = 0usize;
let count_only = output.is_none();
*out_written = 0;
loop {
let (nbits, mut bits) = reader.read(7);
if nbits == 0 {
break;
}
if nbits < 7 {
bits <<= 7 - nbits;
}
if is_illegal(bits) {
let illegal_index = get_illegal_index(bits);
let (next_nbits, mut next_bits) = reader.read(7);
let mut b1: u8 = 0xC2;
let mut b2: u8 = 0x80;
next_bits <<= 7 - next_nbits;
if next_nbits == 0 {
b1 |= 0x7 << 2;
next_bits = bits;
} else {
b1 |= illegal_index << 2;
}
let first_bit = (next_bits >> 6) & 1;
b1 |= first_bit;
b2 |= next_bits & 0x3F;
if count_only {
*out_written += 1;
} else {
let out = output
.as_deref_mut()
.ok_or_else(|| Base122Error::new("output does not have sufficient size"))?;
if out_index == out.len() {
return Err(Base122Error::new("output does not have sufficient size"));
}
out[out_index] = b1;
*out_written += 1;
out_index += 1;
}
if count_only {
*out_written += 1;
} else {
let out = output
.as_deref_mut()
.ok_or_else(|| Base122Error::new("output does not have sufficient size"))?;
if out_index == out.len() {
return Err(Base122Error::new("output does not have sufficient size"));
}
out[out_index] = b2;
*out_written += 1;
out_index += 1;
}
} else if count_only {
*out_written += 1;
} else {
let out = output
.as_deref_mut()
.ok_or_else(|| Base122Error::new("output does not have sufficient size"))?;
if out_index == out.len() {
return Err(Base122Error::new("output does not have sufficient size"));
}
out[out_index] = bits;
*out_written += 1;
out_index += 1;
}
}
Ok(())
}
fn write_last_7(writer: &mut BitWriter, byte_val: u8, error: &mut Base122Error) -> Result<(), Base122Error> {
let nbits = 8 - (writer.cur_bit % 8);
if nbits == 8 {
*error = Base122Error::new("Decoded data is not a byte multiple");
return Err(error.clone());
}
let mask = !((0xFFu16) << (7 - nbits)) as u8;
if (byte_val & mask) > 0 {
*error = Base122Error::new("Encoded data is malformed. Last byte has extra data.");
return Err(error.clone());
}
let byte_val = byte_val >> (7 - nbits);
if writer.write(nbits as u8, byte_val).is_err() {
*error = Base122Error::new("Output does not have sufficient size");
return Err(error.clone());
}
Ok(())
}
pub fn decode(input: &[u8]) -> Result<Vec<u8>, Base122Error> {
let mut count_writer = BitWriter::new(None, 0);
let mut out_written = 0usize;
decode_internal(input, &mut count_writer, &mut out_written)?;
let mut output = vec![0u8; out_written];
let out_len = output.len();
let mut writer = BitWriter::new(Some(output.as_mut_slice()), out_len);
let mut final_written = 0usize;
decode_internal(input, &mut writer, &mut final_written)?;
output.truncate(final_written);
Ok(output)
}
fn decode_internal(input: &[u8], writer: &mut BitWriter, out_written: &mut usize) -> Result<(), Base122Error> {
let mut cur_byte = 0usize;
while cur_byte < input.len() {
if input[cur_byte] >> 7 == 0 {
let cur_byte_val = input[cur_byte];
let mut err = Base122Error::new("");
if cur_byte + 1 == input.len() {
write_last_7(writer, cur_byte_val, &mut err)?;
} else if writer.write(7, cur_byte_val).is_err() {
return Err(Base122Error::new("Output does not have sufficient size"));
}
} else {
let cur_byte_val = input[cur_byte];
if (cur_byte_val & 0xE2) != 0xC2 {
return Err(Base122Error::new("First byte of two byte sequence malformed"));
}
if cur_byte + 1 == input.len() {
return Err(Base122Error::new("Two byte sequence is missing second byte"));
}
cur_byte += 1;
let next_byte_val = input[cur_byte];
if (next_byte_val & 0xC0) != 0x80 {
return Err(Base122Error::new("Second byte of two byte sequence malformed"));
}
let illegal_index = (cur_byte_val & 0x1C) >> 2;
if illegal_index == 0x7 {
if cur_byte + 1 != input.len() {
return Err(Base122Error::new(
"Got unexpected extra data after shortened two byte sequence",
));
}
let mut last_byte_val = cur_byte_val;
last_byte_val <<= 6;
last_byte_val |= next_byte_val & 0x3F;
let mut err = Base122Error::new("");
write_last_7(writer, last_byte_val, &mut err)?;
} else if (illegal_index as usize) < ILLEGALS.len() {
if writer.write(7, ILLEGALS[illegal_index as usize]).is_err() {
return Err(Base122Error::new("Output does not have sufficient size"));
}
let mut second_byte_val = cur_byte_val;
second_byte_val <<= 6;
second_byte_val |= next_byte_val & 0x3F;
let mut err = Base122Error::new("");
if cur_byte + 1 == input.len() {
write_last_7(writer, second_byte_val, &mut err)?;
} else if writer.write(7, second_byte_val).is_err() {
return Err(Base122Error::new("Output does not have sufficient size"));
}
} else {
return Err(Base122Error::new("Got unrecognized illegal index"));
}
}
cur_byte += 1;
}
*out_written = writer.cur_bit / 8;
Ok(())
}
