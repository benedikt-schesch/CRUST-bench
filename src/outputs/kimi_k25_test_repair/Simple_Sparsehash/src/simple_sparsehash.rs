use std::collections::HashMap;

pub const GROUP_SIZE: usize = 64;

pub struct SparseArray {
item_size: usize,
capacity: u32,
data: HashMap<u32, Vec<u8>>,
}

pub struct SparseDict {
data: HashMap<Vec<u8>, Vec<u8>>,
pub bucket_count: usize,
}

pub fn sparse_array_init(item_size: usize, capacity: u32) -> Option<SparseArray> {
Some(SparseArray {
item_size,
capacity,
data: HashMap::new(),
})
}

pub fn sparse_array_get<'a>(arr: &'a SparseArray, index: u32, size_out: Option<&mut usize>) -> Option<&'a [u8]> {
if index >= arr.capacity {
return None;
}
arr.data.get(&index).map(|v| {
if let Some(s) = size_out {
*s = v.len();
}
v.as_slice()
})
}

pub fn sparse_array_set(arr: &mut SparseArray, index: u32, data: &[u8], size: usize) -> i32 {
if index >= arr.capacity {
return 0;
}
if size > arr.item_size {
return 0;
}
if data.len() < size {
return 0;
}
let mut vec = Vec::with_capacity(size);
vec.extend_from_slice(&data[..size]);
arr.data.insert(index, vec);
1
}

pub fn sparse_array_free(_arr: SparseArray) -> i32 {
1
}

pub fn sparse_dict_init() -> Option<SparseDict> {
Some(SparseDict {
data: HashMap::new(),
bucket_count: 0,
})
}

pub fn sparse_dict_set(dict: &mut SparseDict, key: &str, key_len: usize, value: &[u8], value_len: usize) -> i32 {
if key.len() < key_len || value.len() < value_len {
return 0;
}
let k = key.as_bytes()[..key_len].to_vec();
let v = value[..value_len].to_vec();
dict.data.insert(k, v);
dict.bucket_count = dict.data.len();
1
}

pub fn sparse_dict_get<'a>(dict: &'a SparseDict, key: &str, key_len: usize, size_out: Option<&mut usize>) -> Option<&'a [u8]> {
if key.len() < key_len {
return None;
}
let k = &key.as_bytes()[..key_len];
dict.data.get(k).map(|v| {
if let Some(s) = size_out {
*s = v.len();
}
v.as_slice()
})
}

pub fn sparse_dict_free(_dict: SparseDict) -> i32 {
1
}

pub struct SimpleSparseHash;

impl SimpleSparseHash {
pub fn new() -> Self {
SimpleSparseHash
}
}

pub fn create_hash() -> SimpleSparseHash {
SimpleSparseHash::new()
}
