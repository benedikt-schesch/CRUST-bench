type word_t = u32;
const WORD_SIZE: usize = std::mem::size_of::<word_t>() * 8;
#[inline]
fn bindex(b: usize) -> usize {
b / WORD_SIZE
}
#[inline]
fn boffset(b: usize) -> usize {
b % WORD_SIZE
}
#[derive(Clone)]
pub struct BitSet {
words: Vec<word_t>,
}
impl BitSet {
pub fn new(nbits: usize) -> Self {
let n_words = nbits / WORD_SIZE + 1;
let mut bs = Self {
words: vec![0; n_words],
};
bs.clear_all();
bs
}
pub fn remove(&mut self) {
self.words.clear();
}
pub fn set(&mut self, b: usize) {
let idx = bindex(b);
self.words[idx] |= 1u32 << boffset(b);
}
pub fn clear(&mut self, b: usize) {
let idx = bindex(b);
self.words[idx] &= !(1u32 << boffset(b));
}
pub fn get(&self, b: usize) -> bool {
let idx = bindex(b);
(self.words[idx] & (1u32 << boffset(b))) != 0
}
pub fn clear_all(&mut self) {
for w in &mut self.words {
*w = 0;
}
}
pub fn set_all(&mut self) {
for w in &mut self.words {
*w = !0u32;
}
}
pub fn union(&mut self, other: &BitSet) {
assert_eq!(self.words.len(), other.words.len());
for i in 0..self.words.len() {
self.words[i] |= other.words[i];
}
}
pub fn intersect(&mut self, other: &BitSet) {
assert_eq!(self.words.len(), other.words.len());
for i in 0..self.words.len() {
self.words[i] &= other.words[i];
}
}
pub fn toggle_all(&self) -> Self {
let mut out = self.clone();
for w in &mut out.words {
*w ^= !0u32;
}
out
}
}
