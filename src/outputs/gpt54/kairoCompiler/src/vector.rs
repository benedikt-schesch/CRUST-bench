pub const VECTOR_ELEMENT_INCREMENT: usize = 20;
pub const VECTOR_FLAG_PEEK_DECREMENT: i32 = 0b00000001;

#[derive(Debug, Default, Clone)]
pub struct Vector {
pub data: Vec<u8>,
pub pindex: i32,
pub rindex: i32,
pub mindex: i32,
pub count: i32,
pub flags: i32,
pub esize: usize,
pub saves: Vec<(i32, i32, i32, i32, i32, Vec<u8>)>,
}

fn in_bounds_for_at(vector: &Vector, index: i32) -> bool {
index >= 0 && index < vector.rindex
}

fn elem_range(vector: &Vector, index: i32) -> Option<std::ops::Range<usize>> {
if vector.esize == 0 || index < 0 {
return None;
}
let start = index as usize * vector.esize;
let end = start + vector.esize;
if end <= vector.data.len() {
Some(start..end)
} else {
None
}
}

fn ensure_capacity_elements(vector: &mut Vector, elements: i32) {
if elements <= vector.mindex {
return;
}
let new_mindex = elements + VECTOR_ELEMENT_INCREMENT as i32;
let need_bytes = new_mindex as usize * vector.esize;
if vector.data.len() < need_bytes {
vector.data.resize(need_bytes, 0);
}
vector.mindex = new_mindex;
}

pub fn vector_create(esize: usize) -> Vector {
let mindex = VECTOR_ELEMENT_INCREMENT as i32;
Vector {
data: vec![0; esize * VECTOR_ELEMENT_INCREMENT],
pindex: 0,
rindex: 0,
mindex,
count: 0,
flags: 0,
esize,
saves: Vec::new(),
}
}

pub fn vector_free(_vector: Vector) {}

pub fn vector_at(vector: &mut Vector, index: i32) -> Option<&mut [u8]> {
let range = elem_range(vector, index)?;
Some(&mut vector.data[range])
}

pub fn vector_peek_ptr_at(vector: &mut Vector, index: i32) -> Option<&mut [u8]> {
vector_at(vector, index)
}

pub fn vector_peek_no_increment(vector: &mut Vector) -> Option<&mut [u8]> {
if !in_bounds_for_at(vector, vector.pindex) {
return None;
}
vector_at(vector, vector.pindex)
}

pub fn vector_peek(vector: &mut Vector) -> Option<&mut [u8]> {
let idx = vector.pindex;
if !in_bounds_for_at(vector, idx) {
return None;
}
if vector.flags & VECTOR_FLAG_PEEK_DECREMENT != 0 {
vector.pindex -= 1;
} else {
vector.pindex += 1;
}
vector_at(vector, idx)
}

pub fn vector_peek_at(vector: &mut Vector, index: i32) -> Option<&mut [u8]> {
if !in_bounds_for_at(vector, index) {
return None;
}
vector_at(vector, index)
}

pub fn vector_set_flag(vector: &mut Vector, flag: i32) {
vector.flags |= flag;
}

pub fn vector_unset_flag(vector: &mut Vector, flag: i32) {
vector.flags &= !flag;
}

pub fn vector_pop_last_peek(vector: &mut Vector) {
if vector.pindex >= 1 {
vector_pop_at(vector, vector.pindex - 1);
}
}

pub fn vector_peek_ptr(vector: &mut Vector) -> Option<&mut [u8]> {
vector_peek(vector)
}

pub fn vector_set_peek_pointer(vector: &mut Vector, index: i32) {
vector.pindex = index;
}

pub fn vector_set_peek_pointer_end(vector: &mut Vector) {
vector.pindex = vector.rindex - 1;
}

pub fn vector_push(vector: &mut Vector, elem: &[u8]) {
let esize = vector.esize;
let rindex = vector.rindex;
ensure_capacity_elements(vector, rindex + 1);
if let Some(dst) = vector_at(vector, rindex) {
let n = esize.min(elem.len());
dst.fill(0);
dst[..n].copy_from_slice(&elem[..n]);
}
vector.rindex += 1;
vector.count += 1;
}

pub fn vector_push_at(vector: &mut Vector, index: i32, ptr: &[u8]) {
if index < 0 {
return;
}
ensure_capacity_elements(vector, vector.rindex + 1);
let idx = index.min(vector.rindex) as usize;
let start = idx * vector.esize;
let end = vector.rindex as usize * vector.esize;
let new_end = end + vector.esize;
if vector.data.len() < new_end {
vector.data.resize(new_end, 0);
}
vector.data.copy_within(start..end, start + vector.esize);
let dst = &mut vector.data[start..start + vector.esize];
dst.fill(0);
let n = vector.esize.min(ptr.len());
dst[..n].copy_from_slice(&ptr[..n]);
vector.rindex += 1;
vector.count += 1;
}

pub fn vector_pop(vector: &mut Vector) {
if vector.count > 0 {
vector.rindex -= 1;
vector.count -= 1;
}
}

pub fn vector_peek_pop(vector: &mut Vector) {
vector_pop_at(vector, vector.pindex);
}

pub fn vector_back(vector: &mut Vector) -> Option<&mut [u8]> {
if !in_bounds_for_at(vector, vector.rindex - 1) {
return None;
}
vector_at(vector, vector.rindex - 1)
}

pub fn vector_back_or_null(vector: &mut Vector) -> Option<&mut [u8]> {
vector_back(vector)
}

pub fn vector_back_ptr(vector: &mut Vector) -> Option<&mut [u8]> {
vector_back(vector)
}

pub fn vector_back_ptr_or_null(vector: &mut Vector) -> Option<&mut [u8]> {
vector_back(vector)
}

pub fn vector_string(vec: &Vector) -> Option<&str> {
std::str::from_utf8(&vec.data).ok()
}

pub fn vector_empty(vector: &Vector) -> bool {
vector.count == 0
}

pub fn vector_clear(vector: &mut Vector) {
vector.rindex = 0;
vector.count = 0;
vector.pindex = 0;
}

pub fn vector_count(vector: &Vector) -> i32 {
vector.count
}

pub fn vector_fread(_vector: &mut Vector, _amount: i32, _fp: std::fs::File) -> i32 {
0
}

pub fn vector_data_ptr(vector: &Vector) -> &[u8] {
let end = (vector.rindex.max(0) as usize)
.saturating_mul(vector.esize)
.min(vector.data.len());
&vector.data[..end]
}

pub fn vector_insert(vector_dst: &mut Vector, vector_src: &Vector, dst_index: i32) -> i32 {
if vector_dst.esize != vector_src.esize {
return -1;
}
let total = vector_src.count.max(0) as usize;
for i in 0..total {
let start = i * vector_src.esize;
let end = start + vector_src.esize;
vector_push_at(vector_dst, dst_index + i as i32, &vector_src.data[start..end]);
}
0
}

pub fn vector_pop_at_data_address(_vector: &mut Vector, _address: *const u8) -> i32 {
-1
}

pub fn vector_pop_value(vector: &mut Vector, val: &[u8]) -> i32 {
for i in 0..vector.count {
if let Some(slice) = vector_peek_ptr_at(vector, i) {
if slice == val {
vector_pop_at(vector, i);
return 0;
}
}
}
0
}

pub fn vector_pop_at(vector: &mut Vector, index: i32) {
if !in_bounds_for_at(vector, index) {
return;
}
let idx = index as usize;
let start = idx * vector.esize;
let end = vector.rindex as usize * vector.esize;
let next = start + vector.esize;
vector.data.copy_within(next..end, start);
vector.rindex -= 1;
vector.count -= 1;
}

pub fn vector_peek_back(vector: &mut Vector) {
vector.pindex -= 1;
}

pub fn vector_current_index(vector: &Vector) -> i32 {
vector.rindex
}

pub fn vector_save(vector: &mut Vector) {
vector.saves.push((
vector.pindex,
vector.rindex,
vector.mindex,
vector.count,
vector.flags,
vector.data.clone(),
));
}

pub fn vector_restore(vector: &mut Vector) {
if let Some((pindex, rindex, mindex, count, flags, data)) = vector.saves.pop() {
vector.pindex = pindex;
vector.rindex = rindex;
vector.mindex = mindex;
vector.count = count;
vector.flags = flags;
vector.data = data;
}
}

pub fn vector_save_purge(vector: &mut Vector) {
let _ = vector.saves.pop();
}

pub fn vector_element_size(vector: &Vector) -> usize {
vector.esize
}

pub fn vector_clone(vector: &Vector) -> Vector {
vector.clone()
}
