/// The maximum size of each sparse_array_group.
pub const GROUP_SIZE: usize = 48;
/// The default size of the hash table. Used to init bucket_max.
pub const STARTING_SIZE: usize = 32;
/// The default 'should we resize' percentage, out of 100.
pub const RESIZE_PERCENT: usize = 80;
/// Number of bits per u32.
pub const BITCHUNK_SIZE: usize = std::mem::size_of::<u32>() * 8;
/// The minimum number of u32 entries required to hold GROUP_SIZE bits.
pub const BITMAP_SIZE: usize = (GROUP_SIZE - 1) / BITCHUNK_SIZE + 1;

/// Represents one stored key/value pair.
#[derive(Debug, Clone)]
pub struct SparseBucket {
/// The key as an owned String.
pub key: String,
/// The length of the key.
pub klen: usize,
/// The value as a vector of bytes.
pub val: Vec<u8>,
/// The length of the value.
pub vlen: usize,
/// The hash of the key.
pub hash: u64,
}

/// One group in a sparse array.
#[derive(Debug, Clone)]
pub struct SparseArrayGroup {
/// The number of items currently in this group.
pub count: u32,
/// The maximum size of each element.
pub elem_size: usize,
/// The storage for the elements.
pub group: Vec<u8>,
/// A bitmap tracking which slots in `group` are occupied.
pub bitmap: [u32; BITMAP_SIZE],
}

/// A sparse array consisting of one or more groups.
#[derive(Debug, Clone)]
pub struct SparseArray {
/// The maximum number of items that can be stored.
pub maximum: usize,
/// The groups that hold the elements.
pub groups: Vec<SparseArrayGroup>,
}

/// A sparse dictionary that maps keys to values.
#[derive(Debug)]
pub struct SparseDict {
/// The current maximum number of buckets in the dictionary.
pub bucket_max: usize,
/// The number of buckets that are currently occupied.
pub bucket_count: usize,
/// An array of sparse arrays (buckets).
pub buckets: Vec<SparseArray>,
}

fn full_elem_size(arr: &SparseArrayGroup) -> usize {
arr.elem_size + std::mem::size_of::<usize>()
}

fn max_arr_size(maximum: usize) -> usize {
((maximum - 1) / GROUP_SIZE) + 1
}

fn hash_fnv1a(key: &str, klen: usize) -> u64 {
let fnv_prime: u64 = 1099511628211;
let fnv_offset_bias: u64 = 14695981039346656037;
let mut hash = fnv_offset_bias;
let bytes = key.as_bytes();
let iterations = klen.min(bytes.len());
for &b in &bytes[..iterations] {
hash ^= b as u64;
hash = hash.wrapping_mul(fnv_prime);
}
hash
}

fn charbit(position: u32) -> u32 {
position >> 5
}

fn modbit(position: u32) -> u32 {
1u32 << (position & 31)
}

fn popcount_32(mut x: u32) -> u32 {
let m1: u32 = 0x5555_5555;
let m2: u32 = 0x3333_3333;
let m4: u32 = 0x0f0f_0f0f;
x -= (x >> 1) & m1;
x = (x & m2) + ((x >> 2) & m2);
x = (x + (x >> 4)) & m4;
x += x >> 8;
(x + (x >> 16)) & 0x3f
}

fn position_to_offset(bitmap: &[u32; BITMAP_SIZE], position: u32) -> u32 {
let mut retval = 0u32;
let mut pos = position;
let mut bitmap_iter = 0usize;

while pos >= BITCHUNK_SIZE as u32 {
retval += popcount_32(bitmap[bitmap_iter]);
bitmap_iter += 1;
pos -= BITCHUNK_SIZE as u32;
}

retval + popcount_32(bitmap[bitmap_iter] & (((1u32) << pos) - 1u32))
}

fn is_position_occupied(bitmap: &[u32; BITMAP_SIZE], position: u32) -> bool {
(bitmap[charbit(position) as usize] & modbit(position)) != 0
}

fn set_position(bitmap: &mut [u32; BITMAP_SIZE], position: u32) {
bitmap[charbit(position) as usize] |= modbit(position);
}

fn sparse_array_group_set(arr: &mut SparseArrayGroup, i: u32, val: &[u8], vlen: usize) -> i32 {
if vlen > arr.elem_size || vlen > val.len() {
return 0;
}

let offset = position_to_offset(&arr.bitmap, i) as usize;
let fes = full_elem_size(arr);

if !is_position_occupied(&arr.bitmap, i) {
let to_move_siz = (arr.count as usize).saturating_sub(offset) * fes;
let old_len = arr.group.len();
arr.group.resize(old_len + fes, 0);

if to_move_siz > 0 {
let src_start = offset * fes;
let src_end = src_start + to_move_siz;
let dst_start = (offset + 1) * fes;
arr.group.copy_within(src_start..src_end, dst_start);
}

arr.count += 1;
set_position(&mut arr.bitmap, i);
}

let start = offset * fes;
let size_bytes = vlen.to_ne_bytes();
let usize_len = std::mem::size_of::<usize>();
arr.group[start..start + usize_len].copy_from_slice(&size_bytes);
arr.group[start + usize_len..start + usize_len + vlen].copy_from_slice(&val[..vlen]);

1
}

fn sparse_array_group_get<'a>(
arr: &'a SparseArrayGroup,
i: u32,
outsize: Option<&mut usize>,
) -> Option<&'a [u8]> {
if !is_position_occupied(&arr.bitmap, i) {
return None;
}

let offset = position_to_offset(&arr.bitmap, i) as usize;
let fes = full_elem_size(arr);
let start = offset * fes;
let usize_len = std::mem::size_of::<usize>();

if start + usize_len > arr.group.len() {
return None;
}

let mut size_arr = [0u8; std::mem::size_of::<usize>()];
size_arr.copy_from_slice(&arr.group[start..start + usize_len]);
let stored_size = usize::from_ne_bytes(size_arr);

if stored_size == 0 {
return None;
}

if let Some(out) = outsize {
*out = stored_size;
}

let data_start = start + usize_len;
let data_end = data_start + stored_size;
if data_end > arr.group.len() {
return None;
}

Some(&arr.group[data_start..data_end])
}

fn sparse_array_group_free(arr: &mut SparseArrayGroup) -> i32 {
arr.group.clear();
arr.count = 0;
1
}

fn encode_bucket(bucket: &SparseBucket) -> Vec<u8> {
let mut out = Vec::with_capacity(
std::mem::size_of::<u64>()
+ std::mem::size_of::<usize>() * 2
+ bucket.key.len()
+ bucket.val.len(),
);
out.extend_from_slice(&bucket.hash.to_ne_bytes());
out.extend_from_slice(&bucket.klen.to_ne_bytes());
out.extend_from_slice(&bucket.vlen.to_ne_bytes());
out.extend_from_slice(bucket.key.as_bytes());
out.extend_from_slice(&bucket.val);
out
}

fn decode_bucket(bytes: &[u8]) -> Option<SparseBucket> {
let u64_len = std::mem::size_of::<u64>();
let usize_len = std::mem::size_of::<usize>();
let header_len = u64_len + usize_len + usize_len;

if bytes.len() < header_len {
return None;
}

let mut hash_arr = [0u8; 8];
hash_arr.copy_from_slice(&bytes[0..u64_len]);
let hash = u64::from_ne_bytes(hash_arr);

let mut klen_arr = [0u8; std::mem::size_of::<usize>()];
klen_arr.copy_from_slice(&bytes[u64_len..u64_len + usize_len]);
let klen = usize::from_ne_bytes(klen_arr);

let mut vlen_arr = [0u8; std::mem::size_of::<usize>()];
vlen_arr.copy_from_slice(&bytes[u64_len + usize_len..header_len]);
let vlen = usize::from_ne_bytes(vlen_arr);

if bytes.len() < header_len + klen + vlen {
return None;
}

let key_bytes = &bytes[header_len..header_len + klen];
let val_bytes = &bytes[header_len + klen..header_len + klen + vlen];

let key = match String::from_utf8(key_bytes.to_vec()) {
Ok(s) => s,
Err(_) => return None,
};

Some(SparseBucket {
key,
klen,
val: val_bytes.to_vec(),
vlen,
hash,
})
}

fn quadratic_probe(key_hash: u64, num_probes: usize, maximum: usize) -> usize {
((key_hash as usize).wrapping_add(num_probes.wrapping_mul(num_probes))) & (maximum - 1)
}

fn create_and_insert_new_bucket(
array: &mut SparseArray,
i: usize,
key: &str,
klen: usize,
value: &[u8],
vlen: usize,
key_hash: u64,
) -> i32 {
if vlen > value.len() || klen > key.len() {
return 0;
}

let key_prefix = &key.as_bytes()[..klen];
let key_string = match String::from_utf8(key_prefix.to_vec()) {
Ok(s) => s,
Err(_) => return 0,
};

let bucket = SparseBucket {
key: key_string,
klen,
val: value[..vlen].to_vec(),
vlen,
hash: key_hash,
};

let encoded = encode_bucket(&bucket);
sparse_array_set(array, i as u32, &encoded, encoded.len())
}

fn rehash_and_grow_table(dict: &mut SparseDict) -> i32 {
let new_bucket_max = dict.bucket_max * 2;
let mut new_buckets = match sparse_array_init(
std::mem::size_of::<SparseBucket>(),
new_bucket_max as u32,
) {
Some(b) => *b,
None => return 0,
};

let mut buckets_rehashed = 0usize;
let old_buckets = &dict.buckets[0];

for i in 0..dict.bucket_max {
let mut bucket_siz = 0usize;
let bucket_bytes = sparse_array_get(old_buckets, i as u32, Some(&mut bucket_siz));
if bucket_siz != 0 {
if let Some(bytes) = bucket_bytes {
let bucket = match decode_bucket(bytes) {
Some(b) => b,
None => return 0,
};

let key_hash = bucket.hash;
let mut num_probes = 0usize;
let probed_val = loop {
let p = quadratic_probe(key_hash, num_probes, new_bucket_max);
let mut current_value_siz = 0usize;
let current_value = sparse_array_get(&new_buckets, p as u32, Some(&mut current_value_siz));
if current_value_siz == 0 && current_value.is_none() {
break p;
}

if num_probes > dict.bucket_count {
return 0;
}
num_probes += 1;
};

let encoded = encode_bucket(&bucket);
if sparse_array_set(&mut new_buckets, probed_val as u32, &encoded, encoded.len()) == 0 {
return 0;
}
buckets_rehashed += 1;
}
}

if buckets_rehashed == dict.bucket_count {
break;
}
}

dict.buckets[0] = new_buckets;
dict.bucket_max = new_bucket_max;
1
}

/// Creates a new sparse array.
///
/// # Parameters
/// - `element_size`: Maximum size (in bytes) of each element.
/// - `maximum`: The maximum number of elements.
///
/// # Returns
/// An owned pointer (boxed) to a new `SparseArray` or `None` on failure.
pub fn sparse_array_init(element_size: usize, maximum: u32) -> Option<Box<SparseArray>> {
if maximum == 0 {
return None;
}

let group_count = max_arr_size(maximum as usize);
let mut groups = Vec::with_capacity(group_count);
for _ in 0..group_count {
groups.push(SparseArrayGroup {
count: 0,
elem_size: element_size,
group: Vec::new(),
bitmap: [0u32; BITMAP_SIZE],
});
}

Some(Box::new(SparseArray {
maximum: maximum as usize,
groups,
}))
}

/// Sets the element at index `i` to `val`.
///
/// # Parameters
/// - `arr`: The sparse array.
/// - `i`: The index at which to set the value.
/// - `val`: A slice of bytes holding the new element.
/// - `vlen`: The length of the value (in bytes).
///
/// # Returns
/// A nonzero integer on success and 0 on failure.
pub fn sparse_array_set(arr: &mut SparseArray, i: u32, val: &[u8], vlen: usize) -> i32 {
if i as usize > arr.maximum {
return 0;
}

let group_index = (i as usize) / GROUP_SIZE;
let position = (i as usize) % GROUP_SIZE;
if group_index >= arr.groups.len() {
return 0;
}

sparse_array_group_set(&mut arr.groups[group_index], position as u32, val, vlen)
}

/// Retrieves the element at index `i`.
///
/// # Parameters
/// - `arr`: The sparse array.
/// - `i`: The index to retrieve.
/// - `outsize`: An optional mutable reference that will be set to the size (in bytes)
///   of the retrieved element.
///
/// # Returns
/// An optional slice reference to the element; `None` if the index is invalid.
pub fn sparse_array_get<'a>(
arr: &'a SparseArray,
i: u32,
outsize: Option<&mut usize>,
) -> Option<&'a [u8]> {
if i as usize > arr.maximum {
return None;
}

let group_index = (i as usize) / GROUP_SIZE;
let position = (i as usize) % GROUP_SIZE;
if group_index >= arr.groups.len() {
return None;
}

sparse_array_group_get(&arr.groups[group_index], position as u32, outsize)
}

/// Frees the sparse array.
///
/// # Parameters
/// - `arr`: The sparse array to free.
///
/// # Returns
/// A nonzero integer on success and 0 on failure.
pub fn sparse_array_free(mut arr: Box<SparseArray>) -> i32 {
for group in &mut arr.groups {
let _ = sparse_array_group_free(group);
}
1
}

// ---------- Sparse Dictionary API ----------

/// Creates a new sparse dictionary.
///
/// # Returns
/// An owned pointer (boxed) to a new `SparseDict` or `None` on failure.
pub fn sparse_dict_init() -> Option<Box<SparseDict>> {
let buckets = match sparse_array_init(std::mem::size_of::<SparseBucket>(), STARTING_SIZE as u32) {
Some(b) => *b,
None => return None,
};

Some(Box::new(SparseDict {
bucket_max: STARTING_SIZE,
bucket_count: 0,
buckets: vec![buckets],
}))
}

/// Inserts a key/value pair into the dictionary.
///
/// # Parameters
/// - `dict`: The sparse dictionary.
/// - `key`: The key as a string slice.
/// - `klen`: The length of the key.
/// - `value`: A slice of bytes for the value.
/// - `vlen`: The length of the value.
///
/// # Returns
/// A nonzero integer on success and 0 on failure.
pub fn sparse_dict_set(
dict: &mut SparseDict,
key: &str,
klen: usize,
value: &[u8],
vlen: usize,
) -> i32 {
let key_hash = hash_fnv1a(key, klen);
let mut num_probes = 0usize;

loop {
let probed_val = quadratic_probe(key_hash, num_probes, dict.bucket_max);

let mut current_value_siz = 0usize;
let current_value = sparse_array_get(&dict.buckets[0], probed_val as u32, Some(&mut current_value_siz));

if current_value_siz == 0 && current_value.is_none() {
if create_and_insert_new_bucket(
&mut dict.buckets[0],
probed_val,
key,
klen,
value,
vlen,
key_hash,
) != 0
{
break;
} else {
return 0;
}
} else if let Some(bytes) = current_value {
let existing_bucket = match decode_bucket(bytes) {
Some(b) => b,
None => return 0,
};

if existing_bucket.hash == key_hash
&& existing_bucket.klen == klen
&& existing_bucket.key.as_bytes() == &key.as_bytes()[..klen.min(key.len())]
{
if create_and_insert_new_bucket(
&mut dict.buckets[0],
probed_val,
key,
klen,
value,
vlen,
key_hash,
) != 0
{
return 1;
} else {
return 0;
}
}
}

num_probes += 1;
if num_probes > dict.bucket_count {
return 0;
}
}

dict.bucket_count += 1;

if (dict.bucket_count as f32) / (dict.bucket_max as f32) >= (RESIZE_PERCENT as f32) / 100.0 {
return rehash_and_grow_table(dict);
}

1
}

/// Retrieves the value associated with a key.
///
/// # Parameters
/// - `dict`: The sparse dictionary.
/// - `key`: The key as a string slice.
/// - `klen`: The length of the key.
/// - `outsize`: An optional mutable reference that will be set to the length of the value.
///
/// # Returns
/// An optional slice reference to the value; `None` if the key is not found.
pub fn sparse_dict_get<'a>(
dict: &'a SparseDict,
key: &str,
klen: usize,
outsize: Option<&mut usize>,
) -> Option<&'a [u8]> {
let key_hash = hash_fnv1a(key, klen);
let mut num_probes = 0usize;

loop {
let probed_val = quadratic_probe(key_hash, num_probes, dict.bucket_max);
let mut current_value_siz = 0usize;
let current_value = sparse_array_get(&dict.buckets[0], probed_val as u32, Some(&mut current_value_siz));

if current_value_siz != 0 {
if let Some(bytes) = current_value {
let bucket = decode_bucket(bytes)?;
if bucket.hash == key_hash
&& bucket.klen == klen
&& bucket.key.as_bytes() == &key.as_bytes()[..klen.min(key.len())]
{
if let Some(out) = outsize {
*out = bucket.vlen;
}

let u64_len = std::mem::size_of::<u64>();
let usize_len = std::mem::size_of::<usize>();
let header_len = u64_len + usize_len + usize_len;
let value_start = header_len + bucket.klen;
let value_end = value_start + bucket.vlen;
if value_end <= bytes.len() {
return Some(&bytes[value_start..value_end]);
}
return None;
}
}
} else {
return None;
}

num_probes += 1;
if num_probes > dict.bucket_count {
return None;
}
}
}

/// Frees the sparse dictionary.
///
/// # Parameters
/// - `dict`: The sparse dictionary to free.
///
/// # Returns
/// A nonzero integer on success and 0 on failure.
pub fn sparse_dict_free(_dict: Box<SparseDict>) -> i32 {
1
}
