#[derive(Debug, Clone)]
pub struct BitSet {
bits: Vec<u64>,
size: usize,
}
impl BitSet {
pub fn new(capacity: usize) -> Self {
let num_u64s = (capacity + 63) / 64;
BitSet {
bits: vec![0; num_u64s],
size: capacity,
}
}
pub fn with_capacity(capacity: usize) -> Self {
Self::new(capacity)
}
pub fn len(&self) -> usize {
self.size
}
pub fn is_empty(&self) -> bool {
self.size == 0
}
pub fn set(&mut self, bit: usize) {
if bit < self.size {
let idx = bit / 64;
let offset = bit % 64;
self.bits[idx] |= 1u64 << offset;
}
}
pub fn clear(&mut self, bit: usize) {
if bit < self.size {
let idx = bit / 64;
let offset = bit % 64;
self.bits[idx] &= !(1u64 << offset);
}
}
pub fn get(&self, bit: usize) -> bool {
if bit < self.size {
let idx = bit / 64;
let offset = bit % 64;
(self.bits[idx] >> offset) & 1u64 == 1
} else {
false
}
}
pub fn set_all(&mut self) {
for word in &mut self.bits {
*word = !0u64;
}
let remainder = self.size % 64;
if remainder != 0 && !self.bits.is_empty() {
let mask = (1u64 << remainder) - 1;
if let Some(last) = self.bits.last_mut() {
*last &= mask;
}
}
}
pub fn clear_all(&mut self) {
for word in &mut self.bits {
*word = 0;
}
}
}
