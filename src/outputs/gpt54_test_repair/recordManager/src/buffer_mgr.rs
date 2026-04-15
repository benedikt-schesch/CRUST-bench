use crate::dberror::RC;
use crate::storage_mgr::close_page_file;
use crate::storage_mgr::ensure_capacity;
use crate::storage_mgr::open_page_file;
use crate::storage_mgr::read_block;
use crate::storage_mgr::write_block;
use crate::storage_mgr::SM_FileHandle;
use crate::storage_mgr::SM_PageHandle;
use std::any::Any;
pub struct Bufferpool {
pub num_read: i32,
pub num_write: i32,
pub total_pages: i32,
pub updated_strategy: i32,
pub free_space: i32,
pub updated_order: Vec<i32>,
pub bitdirty: Vec<bool>,
pub fix_count: Vec<i32>,
pub access_time: Vec<i32>,
pub pagenum: Vec<i32>,
pub pagedata: String,
pub fhl: SM_FileHandle,
}
pub struct BM_PageHandle {
pub page_num: PageNumber,
pub data: String,
}
pub type PageNumber = i32;
pub const NO_PAGE: PageNumber = -1;
#[derive(Clone, Copy)]
pub enum ReplacementStrategy {
RsFifo = 0,
RsLru = 1,
RsClock = 2,
RsLfu = 3,
RsLruK = 4,
}
pub struct BM_BufferPool {
pub page_file: String,
pub num_pages: i32,
pub strategy: ReplacementStrategy,
pub mgmt_data: Option<Box<dyn Any>>,
}
fn strat_to_i32(strategy: ReplacementStrategy) -> i32 {
match strategy {
ReplacementStrategy::RsFifo => 0,
ReplacementStrategy::RsLru => 1,
ReplacementStrategy::RsClock => 2,
ReplacementStrategy::RsLfu => 3,
ReplacementStrategy::RsLruK => 4,
}
}
fn get_bp_mut(bm: &mut BM_BufferPool) -> Option<&mut Bufferpool> {
bm.mgmt_data
.as_mut()
.and_then(|b| b.downcast_mut::<Bufferpool>())
}
fn get_bp_ref(bm: &BM_BufferPool) -> Option<&Bufferpool> {
bm.mgmt_data
.as_ref()
.and_then(|b| b.downcast_ref::<Bufferpool>())
}
fn write_dirty_pages(bm: &mut BM_BufferPool) -> RC {
let Some(bpl) = get_bp_mut(bm) else {
return RC::Error;
};
for j in 0..bpl.total_pages as usize {
if bpl.bitdirty[j] {
let rc = ensure_capacity(bpl.pagenum[j] + 1, &mut bpl.fhl);
if rc != RC::Ok {
return rc;
}
let start = j * 4096;
let end = start + 4096;
let page_slice = if end <= bpl.pagedata.len() {
bpl.pagedata[start..end].to_string()
} else {
let mut s = String::new();
if start < bpl.pagedata.len() {
s.push_str(&bpl.pagedata[start..]);
}
while s.len() < 4096 {
s.push('\0');
}
s
};
let rc = write_block(bpl.pagenum[j], &mut bpl.fhl, &page_slice);
if rc != RC::Ok {
return RC::WriteFailed;
}
bpl.num_write += 1;
}
}
RC::Ok
}
pub fn init_buffer_pool(
bm: &mut BM_BufferPool,
page_file_name: &str,
num_pages: i32,
strategy: ReplacementStrategy,
_strat_data: Option<Box<dyn std::any::Any>>,
) -> RC {
let mut fh = SM_FileHandle {
file_name: String::new(),
total_num_pages: 0,
cur_page_pos: 0,
mgmt_info: None,
};
let rcode = open_page_file(page_file_name, &mut fh);
if rcode != RC::Ok {
return rcode;
}
let bp = Bufferpool {
num_read: 0,
num_write: 0,
total_pages: num_pages,
updated_strategy: strat_to_i32(strategy),
free_space: num_pages,
updated_order: vec![NO_PAGE; num_pages as usize],
bitdirty: vec![false; num_pages as usize],
fix_count: vec![0; num_pages as usize],
access_time: vec![0; num_pages as usize],
pagenum: vec![NO_PAGE; num_pages as usize],
pagedata: "\0".repeat((num_pages * 4096) as usize),
fhl: fh,
};
bm.page_file = page_file_name.to_string();
bm.num_pages = num_pages;
bm.strategy = strategy;
bm.mgmt_data = Some(Box::new(bp));
RC::Ok
}
pub fn shutdown_buffer_pool(bm: &mut BM_BufferPool) -> RC {
let Some(bpl) = get_bp_ref(bm) else {
return RC::Error;
};
for i in 0..bpl.total_pages as usize {
if bpl.fix_count[i] != 0 {
return RC::BufferpoolInUse;
}
}
let rc = write_dirty_pages(bm);
if rc != RC::Ok {
return rc;
}
let Some(bpl2) = get_bp_mut(bm) else {
return RC::Error;
};
if close_page_file(&mut bpl2.fhl) != RC::Ok {
return RC::CloseFailed;
}
bm.mgmt_data = None;
RC::Ok
}
pub fn force_flush_pool(bm: &mut BM_BufferPool) -> RC {
let Some(bpl) = get_bp_mut(bm) else {
return RC::Error;
};
for i in 0..bpl.total_pages as usize {
if bpl.fix_count[i] == 0 && bpl.bitdirty[i] {
let rc = ensure_capacity(bpl.pagenum[i] + 1, &mut bpl.fhl);
if rc != RC::Ok {
return RC::WriteFailed;
}
let start = i * 4096;
let end = start + 4096;
let page_data = if end <= bpl.pagedata.len() {
bpl.pagedata[start..end].to_string()
} else {
let mut s = String::new();
if start < bpl.pagedata.len() {
s.push_str(&bpl.pagedata[start..]);
}
while s.len() < 4096 {
s.push('\0');
}
s
};
if write_block(bpl.pagenum[i], &mut bpl.fhl, &page_data) != RC::Ok {
return RC::WriteFailed;
}
bpl.bitdirty[i] = false;
bpl.num_write += 1;
}
}
RC::Ok
}
pub fn mark_dirty(bm: &mut BM_BufferPool, page: &mut BM_PageHandle) -> RC {
let Some(bpl) = get_bp_mut(bm) else {
return RC::Error;
};
for i in 0..bpl.total_pages as usize {
if bpl.pagenum[i] == page.page_num {
if !bpl.bitdirty[i] {
bpl.bitdirty[i] = true;
}
break;
}
}
RC::Ok
}
pub fn unpin_page(bm: &mut BM_BufferPool, page: &mut BM_PageHandle) -> RC {
let Some(buffer_pool) = get_bp_mut(bm) else {
return RC::Error;
};
let mut idx = -1;
for i in 0..buffer_pool.total_pages as usize {
if buffer_pool.pagenum[i] == page.page_num {
idx = i as i32;
break;
}
}
if idx != -1 {
let u = idx as usize;
if buffer_pool.fix_count[u] > 0 {
buffer_pool.fix_count[u] -= 1;
}
}
RC::Ok
}
pub fn force_page(bm: &mut BM_BufferPool, page: &mut BM_PageHandle) -> RC {
let Some(bpl) = get_bp_mut(bm) else {
return RC::Error;
};
for i in 0..bpl.total_pages as usize {
if bpl.pagenum[i] == page.page_num {
bpl.bitdirty[i] = false;
bpl.num_write += 1;
return RC::Ok;
}
}
RC::WriteFailed
}
pub fn pin_page(bm: &mut BM_BufferPool, page: &mut BM_PageHandle, page_num: PageNumber) -> RC {
let Some(buffer_pool) = get_bp_mut(bm) else {
return RC::Error;
};
let void_page = buffer_pool.free_space == buffer_pool.total_pages;
if !void_page {
let total_pages = (buffer_pool.total_pages - buffer_pool.free_space) as usize;
for i in 0..total_pages {
if buffer_pool.pagenum[i] == page_num {
page.page_num = page_num;
buffer_pool.fix_count[i] += 1;
let start = i * 4096;
let end = start + 4096;
page.data = if end <= buffer_pool.pagedata.len() {
buffer_pool.pagedata[start..end].to_string()
} else {
String::new()
};
if buffer_pool.updated_strategy == 1 {
let last_position =
(buffer_pool.total_pages - buffer_pool.free_space - 1) as usize;
let mut swap_location: Option<usize> = None;
for j in 0..=last_position {
if buffer_pool.updated_order[j] == page_num {
swap_location = Some(j);
break;
}
}
if let Some(pos) = swap_location {
let val = buffer_pool.updated_order.remove(pos);
if last_position <= buffer_pool.updated_order.len() {
buffer_pool.updated_order.push(val);
buffer_pool.updated_order.truncate(buffer_pool.total_pages as usize);
}
}
}
return RC::Ok;
}
}
}
if buffer_pool.free_space > 0 {
let mut page_handle: SM_PageHandle = String::new();
let read_code = read_block(page_num, &mut buffer_pool.fhl, &mut page_handle);
if read_code != RC::Ok {
return read_code;
}
let memory_address = (buffer_pool.total_pages - buffer_pool.free_space) as usize;
let record_pointer = memory_address * 4096;
if buffer_pool.pagedata.len() < (buffer_pool.total_pages as usize * 4096) {
buffer_pool.pagedata.push_str(
&"\0".repeat(
(buffer_pool.total_pages as usize * 4096) - buffer_pool.pagedata.len(),
),
);
}
let mut bytes = buffer_pool.pagedata.clone().into_bytes();
let src = page_handle.into_bytes();
for i in 0..4096usize {
bytes[record_pointer + i] = if i < src.len() { src[i] } else { 0 };
}
buffer_pool.pagedata = String::from_utf8(bytes)
.unwrap_or_else(|e| String::from_utf8_lossy(&e.into_bytes()).to_string());
buffer_pool.free_space -= 1;
buffer_pool.updated_order[memory_address] = page_num;
buffer_pool.pagenum[memory_address] = page_num;
buffer_pool.num_read += 1;
buffer_pool.fix_count[memory_address] += 1;
buffer_pool.bitdirty[memory_address] = false;
page.page_num = page_num;
page.data = buffer_pool.pagedata[record_pointer..record_pointer + 4096].to_string();
return RC::Ok;
}
let mut updated_found = false;
let mut memory_address = 0usize;
let mut swap_location = 0usize;
let mut page_handle: SM_PageHandle = String::new();
let read_code = read_block(page_num, &mut buffer_pool.fhl, &mut page_handle);
if read_code != RC::Ok {
return read_code;
}
if buffer_pool.updated_strategy == 0 || buffer_pool.updated_strategy == 1 {
let mut j = 0usize;
while j < buffer_pool.total_pages as usize {
let swap_page = buffer_pool.updated_order[j];
let mut i = 0usize;
while i < buffer_pool.total_pages as usize {
if buffer_pool.pagenum[i] == swap_page && buffer_pool.fix_count[i] == 0 {
memory_address = i;
if buffer_pool.bitdirty[i] {
let _ = ensure_capacity(buffer_pool.pagenum[i] + 1, &mut buffer_pool.fhl);
let start = i * 4096;
let end = start + 4096;
let old_page = buffer_pool.pagedata[start..end].to_string();
let _ = write_block(buffer_pool.pagenum[i], &mut buffer_pool.fhl, &old_page);
buffer_pool.num_write += 1;
}
swap_location = j;
updated_found = true;
break;
}
i += 1;
}
if updated_found {
break;
}
j += 1;
}
}
if !updated_found {
return RC::BufferpoolFull;
}
let record_pointer = memory_address * 4096;
let mut bytes = buffer_pool.pagedata.clone().into_bytes();
let src = page_handle.into_bytes();
for i in 0..4096usize {
bytes[record_pointer + i] = if i < src.len() { src[i] } else { 0 };
}
buffer_pool.pagedata = String::from_utf8(bytes)
.unwrap_or_else(|e| String::from_utf8_lossy(&e.into_bytes()).to_string());
shift_updated_order(swap_location as i32, buffer_pool.total_pages - 1, bm, page_num);
update_bufferpool_stats(bm, memory_address as i32, page_num);
let Some(bp2) = get_bp_ref(bm) else {
return RC::Error;
};
page.page_num = page_num;
page.data = bp2.pagedata[record_pointer..record_pointer + 4096].to_string();
RC::Ok
}
fn shift_updated_order(start: i32, end: i32, bm: &mut BM_BufferPool, page_num: i32) {
let Some(bp) = get_bp_mut(bm) else {
return;
};
let s = start as usize;
let e = end as usize;
for i in s..e {
bp.updated_order[i] = bp.updated_order[i + 1];
}
bp.updated_order[e] = page_num;
}
fn update_bufferpool_stats(bm: &mut BM_BufferPool, address: i32, page_num: i32) {
let Some(bp) = get_bp_mut(bm) else {
return;
};
let a = address as usize;
bp.pagenum[a] = page_num;
bp.num_read += 1;
bp.fix_count[a] += 1;
bp.bitdirty[a] = false;
}
pub fn get_frame_contents(bm: &BM_BufferPool) -> Vec<PageNumber> {
if let Some(bpl) = get_bp_ref(bm) {
if bpl.free_space == bpl.total_pages {
Vec::new()
} else {
bpl.pagenum.clone()
}
} else {
Vec::new()
}
}
pub fn get_dirty_flags(bm: &BM_BufferPool) -> Vec<bool> {
get_bp_ref(bm)
.map(|bpl| bpl.bitdirty.clone())
.unwrap_or_default()
}
pub fn get_fix_counts(bm: &BM_BufferPool) -> Vec<i32> {
if let Some(bpl) = get_bp_ref(bm) {
if bpl.free_space == bpl.total_pages {
vec![0]
} else {
bpl.fix_count.clone()
}
} else {
Vec::new()
}
}
pub fn get_num_read_io(bm: &BM_BufferPool) -> i32 {
get_bp_ref(bm).map(|bpl| bpl.num_read).unwrap_or(0)
}
pub fn get_num_write_io(bm: &BM_BufferPool) -> i32 {
get_bp_ref(bm).map(|bpl| bpl.num_write).unwrap_or(0)
}
