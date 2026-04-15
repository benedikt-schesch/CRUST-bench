pub struct SubstringEnumerator<'a> {
data: &'a [u8],
min_length: usize,
max_length: usize,
}
impl<'a> SubstringEnumerator<'a> {
pub fn memory_usage(data_size: usize) -> usize {
std::mem::size_of::<usize>() * data_size + std::mem::size_of::<Self>()
}
pub fn new(data: &'a [u8], min_length: usize, max_length: usize) -> Self {
Self {
data,
min_length,
max_length,
}
}
pub fn for_each<F>(&self, pos: usize, mut callback: F)
where
F: FnMut(usize, usize),
{
if self.min_length != 2 {
return;
}
if pos == 0 || pos + 1 >= self.data.len() {
return;
}
for position in 0..pos {
let mut matched = 0usize;
while pos + matched < self.data.len()
&& position + matched < self.data.len()
&& matched < self.max_length
&& self.data[pos + matched] == self.data[position + matched]
{
matched += 1;
if matched >= self.min_length {
callback(position, matched);
}
}
}
}
}
