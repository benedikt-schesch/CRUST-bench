
pub fn xmalloc(size: usize) -> Vec<u8> {
let mut vec = Vec::with_capacity(size);
vec.resize(size, 0);
vec
}
pub fn xrealloc(mut data: Vec<u8>, new_size: usize) -> Vec<u8> {
data.resize(new_size, 0);
data
}
