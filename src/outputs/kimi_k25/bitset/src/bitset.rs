use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Debug)]
pub struct Bitset {
pub flip: bool,
pub size: usize,
pub count: usize,
pub array: Vec<u8>,
}

impl Bitset {
fn bitset_size(nbits: usize) -> usize {
(nbits + 7) >> 3
}

fn bitset_byte(idx: usize) -> usize {
idx >> 3
}

fn bitset_bit_mask(idx: usize) -> u8 {
1 << (idx & 0x7)
}

pub fn new(nb_bits: usize) -> Self {
let array_size = Self::bitset_size(nb_bits);
Bitset {
flip: false,
size: nb_bits,
count: 0,
array: vec![0; array_size],
}
}

pub fn delete(self) {
// Ownership of self is taken and dropped automatically when this function returns,
// which frees the Vec<u8> and other fields
}

pub fn count(&self) -> usize {
if self.flip {
self.size - self.count
} else {
self.count
}
}

pub fn size(&self) -> usize {
self.size
}

pub fn test(&self, idx: usize) -> bool {
let byte_idx = Self::bitset_byte(idx);
let mask = Self::bitset_bit_mask(idx);
let bit_is_set = (self.array[byte_idx] & mask) != 0;
self.flip ^ bit_is_set
}

pub fn any(&self) -> bool {
if self.flip {
self.count != self.size
} else {
self.count != 0
}
}

pub fn none(&self) -> bool {
!self.any()
}

pub fn all(&self) -> bool {
if self.flip {
self.count == 0
} else {
self.count == self.size
}
}

pub fn set(&mut self, idx: usize, value: bool) {
let flip_value = if self.flip { !value } else { value };
let byte_idx = Self::bitset_byte(idx);
let mask = Self::bitset_bit_mask(idx);

let previous_count = self.array[byte_idx].count_ones() as usize;

if flip_value {
self.array[byte_idx] |= mask;
} else {
self.array[byte_idx] &= !mask;
}

let new_count = self.array[byte_idx].count_ones() as usize;
self.count += new_count - previous_count;
}

pub fn reset(&mut self) {
for byte in self.array.iter_mut() {
*byte = 0;
}
self.flip = false;
self.count = 0;
}

pub fn flip(&mut self) {
self.flip = !self.flip;
}

pub fn to_string(&self) -> String {
let mut res = String::with_capacity(self.size);
for i in 0..self.size {
if self.test(i) {
res.push('1');
} else {
res.push('0');
}
}
res
}

pub fn write(&self, path: &str) -> io::Result<()> {
let mut file = File::create(path)?;

file.write_all(&self.size.to_le_bytes())?;
file.write_all(&self.count.to_le_bytes())?;
file.write_all(&[self.flip as u8])?;
file.write_all(&self.array)?;

Ok(())
}

pub fn read(path: &str) -> io::Result<Self> {
let mut file = File::open(path)?;

let mut size_bytes = [0u8; std::mem::size_of::<usize>()];
file.read_exact(&mut size_bytes)?;
let size = usize::from_le_bytes(size_bytes);

let mut bs = Bitset::new(size);

let mut count_bytes = [0u8; std::mem::size_of::<usize>()];
file.read_exact(&mut count_bytes)?;
bs.count = usize::from_le_bytes(count_bytes);

let mut flip_byte = [0u8; 1];
file.read_exact(&mut flip_byte)?;
bs.flip = flip_byte[0] != 0;

file.read_exact(&mut bs.array)?;

Ok(bs)
}
}
