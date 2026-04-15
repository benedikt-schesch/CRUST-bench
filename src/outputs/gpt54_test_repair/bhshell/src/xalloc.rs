
pub fn xrealloc(mut data: Vec<u8>, new_size: usize) -> Vec<u8> {
if data.len() < new_size {
data.resize(new_size, 0);
} else {
data.truncate(new_size);
}
data
}
pub fn xmalloc(size: usize) -> Vec<u8> {
vec![0; size]
}
