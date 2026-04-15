/// Allocates a new vector of bytes of the specified size.
/// In C, this was 'void* xmalloc(size_t size)'.
pub fn xmalloc(size: usize) -> Vec<u8> {
let mut vec = Vec::with_capacity(size);
vec.resize(size, 0);
vec
}

/// Reallocates a vector of bytes to a new size.
/// In C, this was 'void* xrealloc(void* ptr, size_t size)'.
pub fn xrealloc(mut data: Vec<u8>, new_size: usize) -> Vec<u8> {
data.resize(new_size, 0);
data
}
