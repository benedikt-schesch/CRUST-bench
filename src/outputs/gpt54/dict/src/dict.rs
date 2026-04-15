// Generated Rust Code
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash as StdHash, Hasher};
use std::sync::Mutex;

/// A collection of constants matching the original C #defines.
pub const HASH_MOD: u64 = 1000000007;
pub const HASH_BASE: u64 = 256;
pub const DEFAULT_STEP: usize = 2;
pub const DEFAULT_MOD: usize = 8;

/// Enum corresponding to the C enum `dict_type_t`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DictType {
Char,
WChar,
I32,
U32,
F32,
I64,
U64,
F64,
Ptr,
Str,
Struct,
}

/// A safe function pointer type for deep-copying values.
pub type DictDeepCopy = fn(dest: &mut [u8], src: &[u8]);

/// A safe function pointer type for destructors.
pub type DictDestructor = fn(data: &mut [u8]);

/// A safe function pointer type for comparisons.
pub type DictCmpr = fn(a: &[u8], b: &[u8]) -> i32;

/// A safe function pointer type for hashing.
pub type DictHash = fn(data: &[u8]) -> u64;

/// A safe function pointer type for memory allocation (unused in safe Rust).
pub type DictMalloc = fn(size: usize) -> Vec<u8>;

/// A safe function pointer type for freeing allocated memory (unused in safe Rust).
pub type DictFree = fn(_: Vec<u8>);

/// Corresponds to `dict_alloc_t` in C.
#[derive(Clone)]
pub struct DictAlloc {
pub malloc: Option<DictMalloc>,
pub free: Option<DictFree>,
}

/// Corresponds to `dict_key_attr_t` in C.
#[derive(Clone)]
pub struct DictKeyAttr {
pub type_: DictType,
pub size: usize,
pub copy: Option<DictDeepCopy>,
pub free: Option<DictDestructor>,
pub hash: Option<DictHash>,
pub cmpr: Option<DictCmpr>,
}

/// Corresponds to `dict_val_attr_t` in C.
#[derive(Clone)]
pub struct DictValAttr {
pub size: usize,
pub free: Option<DictDestructor>,
}

/// Corresponds to `dict_args_t` in C.
#[derive(Clone)]
pub struct DictArgs {
pub key: DictKeyAttr,
pub val: DictValAttr,
pub alloc: DictAlloc,
}

/// Each element in the dictionary. Mirrors the struct dict_elem in C.
/// We store the key bytes in `key` and the value bytes in `val`.
#[derive(Clone)]
pub struct DictElem {
pub code: u64,
pub key: Vec<u8>,
pub val: Vec<u8>,
}

/// A bucket (list) in the dictionary. Mirrors dict_list_t in C, but
/// instead of a linked list, we store a Vec for safe iteration/removal.
#[derive(Clone)]
pub struct DictBucket {
pub elements: Vec<DictElem>,
}

/// Corresponds to `struct dict` in C.
pub struct Dict {
pub key: DictKeyAttr,
pub val: DictValAttr,
pub alloc: DictAlloc,
pub mod_: usize,
pub buckets: Vec<DictBucket>,
pub key_temp: Vec<u8>,
pub keys_dump: Vec<u8>,
pub count: usize,
}

fn aligned_size(size: usize) -> usize {
let a = std::mem::size_of::<usize>();
(size + (a - 1)) & !(a - 1)
}

fn fixed_key_size(t: DictType, key_size: usize) -> usize {
match t {
DictType::Char => 1,
DictType::WChar => 4,
DictType::I32 => 4,
DictType::U32 => 4,
DictType::F32 => 4,
DictType::I64 => 8,
DictType::U64 => 8,
DictType::F64 => 8,
DictType::Ptr => std::mem::size_of::<usize>(),
DictType::Str => std::mem::size_of::<usize>(),
DictType::Struct => aligned_size(key_size),
}
}

fn read_u32_le(data: &[u8]) -> u32 {
let mut buf = [0u8; 4];
let n = data.len().min(4);
buf[..n].copy_from_slice(&data[..n]);
u32::from_le_bytes(buf)
}

fn read_u64_le(data: &[u8]) -> u64 {
let mut buf = [0u8; 8];
let n = data.len().min(8);
buf[..n].copy_from_slice(&data[..n]);
u64::from_le_bytes(buf)
}

fn key_from_input(dict: &Dict, key_data: &[u8]) -> Vec<u8> {
if let Some(copy_fn) = dict.key.copy {
let mut out = vec![0u8; dict.key.size];
copy_fn(&mut out, key_data);
return out;
}

match dict.key.type_ {
DictType::Str => {
let mut v = key_data.to_vec();
if v.last().copied() == Some(0) {
v.pop();
}
v
}
_ => {
let mut out = vec![0u8; dict.key.size];
let n = key_data.len().min(dict.key.size);
out[..n].copy_from_slice(&key_data[..n]);
out
}
}
}

fn key_equals(dict: &Dict, a: &[u8], b: &[u8]) -> bool {
if let Some(cmpr) = dict.key.cmpr {
return cmpr(a, b) == 0;
}
match dict.key.type_ {
DictType::Str => a == b,
_ => a == b,
}
}

fn key_equals_attr(key_attr: &DictKeyAttr, a: &[u8], b: &[u8]) -> bool {
if let Some(cmpr) = key_attr.cmpr {
return cmpr(a, b) == 0;
}
a == b
}

fn hash_bytes_default(bytes: &[u8]) -> u64 {
let mut code = 0u64;
for &ch in bytes {
code = (code * HASH_BASE + ch as u64) % HASH_MOD;
}
code
}

fn dict_get_hash_from_attr(key_attr: &DictKeyAttr, key: &[u8]) -> u64 {
if let Some(hash_fn) = key_attr.hash {
return hash_fn(key);
}

match key_attr.type_ {
DictType::Char => key.first().copied().unwrap_or(0) as i8 as i64 as u64,
DictType::WChar => read_u32_le(key) as u64,
DictType::I32 => read_u32_le(key) as i32 as i64 as u64,
DictType::U32 => read_u32_le(key) as u64,
DictType::F32 => read_u32_le(key) as u64,
DictType::I64 => read_u64_le(key),
DictType::U64 => read_u64_le(key),
DictType::F64 => read_u64_le(key),
DictType::Ptr => read_u64_le(key),
DictType::Str => hash_bytes_default(key),
DictType::Struct => hash_bytes_default(&key[..key_attr.size.min(key.len())]),
}
}

/// Create a dictionary using detailed arguments. Matches C's dict_create().
pub fn dict_create(args: DictArgs) -> Dict {
let mut key = args.key.clone();
key.size = fixed_key_size(args.key.type_, args.key.size);

let mut val = args.val.clone();
val.size = aligned_size(args.val.size);

Dict {
key: key.clone(),
val,
alloc: args.alloc.clone(),
mod_: DEFAULT_MOD,
buckets: vec![
DictBucket {
elements: Vec::new(),
};
DEFAULT_MOD
],
key_temp: vec![0u8; key.size],
keys_dump: Vec::new(),
count: 0,
}
}

/// Create a dictionary with derived arguments. Matches C's dict_new().
pub fn dict_new(key_type: DictType, key_size: usize, val_size: usize) -> Dict {
dict_create(DictArgs {
key: DictKeyAttr {
type_: key_type,
size: key_size,
copy: None,
free: None,
hash: None,
cmpr: None,
},
val: DictValAttr {
size: val_size,
free: None,
},
alloc: DictAlloc {
malloc: None,
free: None,
},
})
}

/// Destroy a dictionary. Matches C's dict_destroy().
/// In Rust, memory is freed automatically, but we emulate calling destructors if provided.
pub fn dict_destroy(dict: &mut Dict) {
let key_free = dict.key.free;
let val_free = dict.val.free;
let val_size = dict.val.size;

for bucket in &mut dict.buckets {
for elem in &mut bucket.elements {
if let Some(free_fn) = key_free {
free_fn(&mut elem.key);
}
if val_size != 0 {
if let Some(free_fn) = val_free {
free_fn(&mut elem.val);
}
}
}
bucket.elements.clear();
}
dict.count = 0;
dict.key_temp.clear();
dict.keys_dump.clear();
dict.buckets.clear();
dict.mod_ = 0;
}

/// Retrieve or create a value from the dictionary. In C, this used varargs. Here, we
/// accept a slice of bytes as the key. Returns a mutable slice of the value,
/// or None if something went wrong. Matches C's dict_get(dict_t*, ...).
pub fn dict_get<'dict>(dict: &'dict mut Dict, key_data: &[u8]) -> Option<&'dict mut [u8]> {
let key = key_from_input(dict, key_data);
let key_attr = dict.key.clone();
let code = dict_get_hash_from_attr(&key_attr, &key);

if dict.mod_ == 0 {
return None;
}

let index = code as usize % dict.mod_;

{
let bucket = &dict.buckets[index];
if bucket
.elements
.iter()
.any(|elem| elem.code == code && key_equals_attr(&key_attr, &elem.key, &key))
{
let bucket_mut = &mut dict.buckets[index];
let pos = bucket_mut
.elements
.iter()
.position(|elem| elem.code == code && key_equals_attr(&key_attr, &elem.key, &key))?;
return Some(bucket_mut.elements[pos].val.as_mut_slice());
}
}

let elem = DictElem {
code,
key: key.clone(),
val: vec![0u8; dict.val.size],
};

let need_reshape = {
let bucket = &mut dict.buckets[index];
bucket.elements.push(elem);
bucket.elements.len() > dict.mod_
};
dict.count += 1;

if need_reshape && !dict_reshape(dict, 1) {
return None;
}

let new_code = dict_get_hash_from_attr(&key_attr, &key);
let new_index = new_code as usize % dict.mod_;
let bucket = &mut dict.buckets[new_index];
let pos = bucket
.elements
.iter()
.position(|elem| elem.code == new_code && key_equals_attr(&key_attr, &elem.key, &key))?;
Some(bucket.elements[pos].val.as_mut_slice())
}

/// Remove a value from the dictionary. Matches C's dict_remove(dict_t*, ...).
/// Returns true if the element was found and removed.
pub fn dict_remove(dict: &mut Dict, key_data: &[u8]) -> bool {
let key = key_from_input(dict, key_data);
let code = dict_get_hash(dict, &key);
let index = code as usize % dict.mod_;

let pos = {
let bucket = &dict.buckets[index];
bucket
.elements
.iter()
.position(|elem| elem.code == code && key_equals(dict, &elem.key, &key))
};

if let Some(i) = pos {
let mut elem = dict.buckets[index].elements.remove(i);
dict_free_key(dict, &mut elem.key);
dict_free_val(dict, &mut elem.val);
dict.count -= 1;
true
} else {
false
}
}

/// Check if a key exists in the dictionary. Matches C's dict_has(const dict_t*, ...).
pub fn dict_has(dict: &Dict, key_data: &[u8]) -> bool {
let key = key_from_input(dict, key_data);
let code = dict_get_hash(dict, &key);
let index = code as usize % dict.mod_;

dict.buckets[index]
.elements
.iter()
.any(|elem| elem.code == code && key_equals(dict, &elem.key, &key))
}

/// Return the number of elements in the dictionary. Matches C's dict_len().
pub fn dict_len(dict: &Dict) -> usize {
dict.count
}

/// Return a snapshot of all keys. In C, it returns a newly allocated array of all keys
/// (size = key.size * dict_len). This is not thread-safe in the original C usage. In
/// safe Rust, we simulate returning a static buffer by leaking the allocation. This
/// avoids unsafe code, but does leak memory for each call. Matches C's dict_key().
pub fn dict_key(dict: &Dict, size: &mut usize) -> Option<&'static [u8]> {
*size = dict_len(dict);
if *size == 0 {
return None;
}

let mut out = Vec::with_capacity(dict.key.size * (*size));
for bucket in &dict.buckets {
for elem in &bucket.elements {
if dict.key.type_ == DictType::Str {
let mut k = elem.key.clone();
k.resize(dict.key.size, 0);
out.extend_from_slice(&k[..dict.key.size]);
} else {
out.extend_from_slice(&elem.key[..dict.key.size]);
}
}
}

Some(Box::leak(out.into_boxed_slice()))
}

/// Serialize a dictionary into a contiguous Vec<u8>. Matches C's dict_serialize().
pub fn dict_serialize(dict: &Dict, bytes: &mut usize) -> Option<Vec<u8>> {
let size = dict_len(dict) as u32;
let key_size = dict.key.size as u32;
let val_size = dict.val.size as u32;
let elem_size = if dict.key.type_ == DictType::Str {
4 + dict.val.size
} else {
dict.key.size + dict.val.size
};

let mut total = 12usize + size as usize * elem_size;

let mut strlen_table = Vec::new();
if dict.key.type_ == DictType::Str {
for bucket in &dict.buckets {
for elem in &bucket.elements {
let len = elem.key.len() as u32;
strlen_table.push(len);
total += len as usize;
}
}
}

let mut data = Vec::with_capacity(total);
data.extend_from_slice(&key_size.to_le_bytes());
data.extend_from_slice(&val_size.to_le_bytes());
data.extend_from_slice(&size.to_le_bytes());

if dict.key.type_ == DictType::Str {
let mut strings = Vec::new();
let mut idx = 0usize;
for bucket in &dict.buckets {
for elem in &bucket.elements {
data.extend_from_slice(&strlen_table[idx].to_le_bytes());
data.extend_from_slice(&elem.val);
strings.extend_from_slice(&elem.key);
idx += 1;
}
}
data.extend_from_slice(&strings);
} else {
for bucket in &dict.buckets {
for elem in &bucket.elements {
data.extend_from_slice(&elem.key);
data.extend_from_slice(&elem.val);
}
}
}

*bytes = data.len();
Some(data)
}

/// Deserialize a dictionary from a slice. Matches C's dict_deserialize().
pub fn dict_deserialize(args: DictArgs, data: &[u8]) -> Dict {
if data.len() < 12 {
return dict_create(args);
}

let mut ptr = 0usize;
let key_size_in = read_u32_le(&data[ptr..ptr + 4]) as usize;
ptr += 4;
let val_size_in = read_u32_le(&data[ptr..ptr + 4]) as usize;
ptr += 4;
let count = read_u32_le(&data[ptr..ptr + 4]) as usize;
ptr += 4;

let expected_key_size = fixed_key_size(args.key.type_, args.key.size);
let expected_val_size = aligned_size(args.val.size);

if key_size_in != expected_key_size || val_size_in != expected_val_size {
return dict_create(args);
}

let mut dict = dict_create(args.clone());
let elem_size = if dict.key.type_ == DictType::Str {
4 + dict.val.size
} else {
dict.key.size + dict.val.size
};

if dict.key.type_ == DictType::Str {
let strings_base = ptr + count * elem_size;
let mut meta_ptr = ptr;
let mut str_ptr = strings_base;

for _ in 0..count {
if meta_ptr + 4 > data.len() {
break;
}
let slen = read_u32_le(&data[meta_ptr..meta_ptr + 4]) as usize;
meta_ptr += 4;

if meta_ptr + dict.val.size > data.len() || str_ptr + slen > data.len() {
break;
}

let val = data[meta_ptr..meta_ptr + dict.val.size].to_vec();
meta_ptr += dict.val.size;

let key = data[str_ptr..str_ptr + slen].to_vec();
str_ptr += slen;

let code = dict_get_hash(&dict, &key);
let index = code as usize % dict.mod_;
dict.buckets[index].elements.push(DictElem { code, key, val });
dict.count += 1;
}
} else {
let mut p = ptr;
for _ in 0..count {
if p + dict.key.size + dict.val.size > data.len() {
break;
}

let key = data[p..p + dict.key.size].to_vec();
p += dict.key.size;
let val = data[p..p + dict.val.size].to_vec();
p += dict.val.size;

let code = dict_get_hash(&dict, &key);
let index = code as usize % dict.mod_;
dict.buckets[index].elements.push(DictElem { code, key, val });
dict.count += 1;
}
}

let mut max_bucket = 0usize;
for bucket in &dict.buckets {
if bucket.elements.len() > max_bucket {
max_bucket = bucket.elements.len();
}
}

if max_bucket > DEFAULT_MOD {
let step = max_bucket / DEFAULT_MOD;
let _ = dict_reshape(&mut dict, step);
}

dict
}

/// Convenience function to create a dictionary using inline arguments, mirroring the
/// C macro dict_create_args(...).
pub fn dict_create_args(args: DictArgs) -> Dict {
dict_create(args)
}

/// The original dict_key_equals. Kept for signature consistency but not used internally
/// to avoid borrow conflicts.
pub fn dict_key_equals(dict: &Dict, a: &[u8], b: &[u8]) -> bool {
key_equals(dict, a, b)
}

/// Not used in this design, but signature is kept.
pub fn dict_delete_node(_list: &mut DictBucket, _curr: &mut DictElem) {
// no-op in this safe design
}

/// The original dict_free_val. Kept for signature consistency.
pub fn dict_free_val(dict: &Dict, val: &mut [u8]) {
if let Some(free_fn) = dict.val.free {
free_fn(val);
}
}

/// Not used in pure Rust version, matching signature only.
pub fn dict_get_key(_dict: &Dict) -> Option<&mut [u8]> {
None
}

/// Internal function to reshape the dictionary. Matches C's dict_reshape().
/// We re-allocate and re-hash all elements with new capacity = old * step * DEFAULT_STEP.
pub fn dict_reshape(dict: &mut Dict, step: usize) -> bool {
let old_mod = dict.mod_;
let new_mod = old_mod.saturating_mul(step).saturating_mul(DEFAULT_STEP);
if new_mod == 0 {
return false;
}

let mut new_buckets = vec![
DictBucket {
elements: Vec::new(),
};
new_mod
];

for bucket in &mut dict.buckets {
let old_elements = std::mem::take(&mut bucket.elements);
for elem in old_elements {
let index = elem.code as usize % new_mod;
new_buckets[index].elements.push(elem);
}
}

dict.mod_ = new_mod;
dict.buckets = new_buckets;
true
}

/// Internal function to free a node. Matches C's dict_free_node().
pub fn dict_free_node(_dict: &Dict, _node: &mut DictElem) {
// no-op in safe Rust
}

/// Internal function to free a dictionary key. Kept for signature consistency.
pub fn dict_free_key(dict: &Dict, key: &mut [u8]) {
if let Some(free_fn) = dict.key.free {
free_fn(key);
}
}

/// The original dict_get_hash. Kept for signature consistency but not used internally
/// to avoid borrow conflicts.
pub fn dict_get_hash(dict: &Dict, key: &[u8]) -> u64 {
dict_get_hash_from_attr(&dict.key, key)
}
