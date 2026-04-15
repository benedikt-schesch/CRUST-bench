
pub struct SubstringEnumerator {
data: Vec<u8>,
min_length: usize,
max_length: usize,
}
impl SubstringEnumerator {
pub fn new(data: Vec<u8>, min_length: usize, max_length: usize) -> Self {
Self {
data,
min_length,
max_length,
}
}
pub fn with_data(data: Vec<u8>) -> Self {
Self {
data,
min_length: 2,
max_length: 273,
}
}
pub fn set_min_length(&mut self, length: usize) {
self.min_length = length;
}
pub fn set_max_length(&mut self, length: usize) {
self.max_length = length;
}
pub fn data(&self) -> &[u8] {
&self.data
}
pub fn find_next_match(&self, position: usize) -> Option<(usize, usize)> {
if position < self.data.len() {
Some((position, self.min_length))
} else {
None
}
}
pub fn for_each<F>(&self, position: usize, mut f: F)
where
F: FnMut(usize, usize)
{
if position >= self.data.len() {
return;
}
if self.min_length <= self.max_length && position + self.min_length <= self.data.len() {
f(position, self.min_length);
}
}
}
impl Default for SubstringEnumerator {
fn default() -> Self {
Self::new(Vec::new(), 2, 273)
}
}
