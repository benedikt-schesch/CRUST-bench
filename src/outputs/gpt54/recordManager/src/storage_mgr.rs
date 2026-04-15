use crate::dberror::RC;
use crate::tables::Schema;
use crate::tables::Value;
use std::fs::{remove_file, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

pub struct SM_FileHandle {
pub file_name: String,
pub total_num_pages: i32,
pub cur_page_pos: i32,
pub mgmt_info: Option<Box<dyn std::any::Any>>,
}

pub type SM_PageHandle = String;

pub fn init_storage_manager() {}

pub fn create_page_file(file_name: &str) -> RC {
let mut file = match OpenOptions::new()
.create(true)
.write(true)
.read(true)
.truncate(true)
.open(file_name)
{
Ok(f) => f,
Err(_) => return RC::FileNotFound,
};

let empty = vec![0u8; 4096];
if file.write_all(&empty).is_err() {
return RC::WriteFailed;
}
RC::Ok
}

pub fn open_page_file(file_name: &str, f_handle: &mut SM_FileHandle) -> RC {
let mut file = match OpenOptions::new().read(true).write(true).open(file_name) {
Ok(f) => f,
Err(_) => return RC::FileNotFound,
};

let mut page_data = vec![0u8; 4096];
if file.read_exact(&mut page_data).is_err() {
return RC::ReadFailed;
}

let header = String::from_utf8_lossy(&page_data)
.trim_matches(char::from(0))
.to_string();
f_handle.file_name = file_name.to_string();
f_handle.total_num_pages = header.parse::<i32>().unwrap_or(0);
f_handle.cur_page_pos = 0;
f_handle.mgmt_info = Some(Box::new(file));
RC::Ok
}

pub fn close_page_file(f_handle: &mut SM_FileHandle) -> RC {
let Some(anyf) = f_handle.mgmt_info.as_mut() else {
return RC::FileHandleNotInit;
};
let Some(file) = anyf.downcast_mut::<std::fs::File>() else {
return RC::FileHandleNotInit;
};

if file.seek(SeekFrom::Start(0)).is_err() {
return RC::SeekFailed;
}
let mut page = vec![0u8; 4096];
let text = f_handle.total_num_pages.to_string();
for (i, b) in text.as_bytes().iter().enumerate() {
if i < 4096 {
page[i] = *b;
}
}
if file.write_all(&page).is_err() {
return RC::WriteFailed;
}
RC::Ok
}

pub fn destroy_page_file(file_name: &str) -> RC {
match remove_file(file_name) {
Ok(_) => RC::Ok,
Err(_) => RC::DestroyFailed,
}
}

pub fn read_block(page_num: i32, f_handle: &mut SM_FileHandle, mem_page: &mut SM_PageHandle) -> RC {
if page_num < 0 || page_num >= f_handle.total_num_pages {
return RC::ReadNonExistingPage;
}
let Some(anyf) = f_handle.mgmt_info.as_mut() else {
return RC::FileHandleNotInit;
};
let Some(file) = anyf.downcast_mut::<std::fs::File>() else {
return RC::FileHandleNotInit;
};

let offset = ((page_num + 1) * 4096) as u64;
if file.seek(SeekFrom::Start(offset)).is_err() {
return RC::SeekFailed;
}
let mut buf = vec![0u8; 4096];
if file.read_exact(&mut buf).is_err() {
return RC::ReadFailed;
}
*mem_page = String::from_utf8_lossy(&buf).to_string();
f_handle.cur_page_pos = page_num;
RC::Ok
}

pub fn get_block_pos(f_handle: &SM_FileHandle) -> i32 {
f_handle.cur_page_pos
}

pub fn read_first_block(f_handle: &mut SM_FileHandle, mem_page: &mut SM_PageHandle) -> RC {
read_block(0, f_handle, mem_page)
}

pub fn read_previous_block(f_handle: &mut SM_FileHandle, mem_page: &mut SM_PageHandle) -> RC {
let page_num = f_handle.cur_page_pos - 1;
if page_num < 0 {
return RC::ReadNonExistingPage;
}
read_block(page_num, f_handle, mem_page)
}

pub fn read_current_block(f_handle: &mut SM_FileHandle, mem_page: &mut SM_PageHandle) -> RC {
if f_handle.mgmt_info.is_none() {
return RC::FileHandleNotInit;
}
let page_num = f_handle.cur_page_pos;
if page_num < 0 || page_num >= f_handle.total_num_pages {
return RC::ReadNonExistingPage;
}
read_block(page_num, f_handle, mem_page)
}

pub fn read_next_block(f_handle: &mut SM_FileHandle, mem_page: &mut SM_PageHandle) -> RC {
read_block(f_handle.cur_page_pos + 1, f_handle, mem_page)
}

pub fn read_last_block(f_handle: &mut SM_FileHandle, mem_page: &mut SM_PageHandle) -> RC {
let page_num = f_handle.total_num_pages - 1;
if page_num < 0 {
return RC::ReadNonExistingPage;
}
read_block(page_num, f_handle, mem_page)
}

pub fn write_block(page_num: i32, f_handle: &mut SM_FileHandle, mem_page: &SM_PageHandle) -> RC {
if page_num < 0 {
return RC::WriteFailed;
}
let Some(anyf) = f_handle.mgmt_info.as_mut() else {
return RC::FileNotFound;
};
let Some(file) = anyf.downcast_mut::<std::fs::File>() else {
return RC::FileNotFound;
};

let offset = ((page_num + 1) * 4096) as u64;
let file_size = match file.metadata() {
Ok(m) => m.len(),
Err(_) => return RC::WriteFailed,
};

if offset > file_size {
if page_num == f_handle.total_num_pages {
if file.seek(SeekFrom::End(0)).is_err() {
return RC::SeekFailed;
}
let pad = vec![0u8; (offset - file_size) as usize];
if file.write_all(&pad).is_err() {
return RC::WriteFailed;
}
f_handle.total_num_pages += 1;
} else {
return RC::WriteFailed;
}
}

if file.seek(SeekFrom::Start(offset)).is_err() {
return RC::SeekFailed;
}

let mut buf = mem_page.as_bytes().to_vec();
buf.resize(4096, 0);
if file.write_all(&buf[..4096]).is_err() {
return RC::WriteFailed;
}

f_handle.cur_page_pos = page_num;
RC::Ok
}

pub fn write_current_block(f_handle: &mut SM_FileHandle, mem_page: &SM_PageHandle) -> RC {
let page_num = f_handle.cur_page_pos;
if page_num < 0 || page_num >= f_handle.total_num_pages {
return RC::WriteFailed;
}
write_block(page_num, f_handle, mem_page)
}

pub fn append_empty_block(f_handle: &mut SM_FileHandle) -> RC {
let Some(anyf) = f_handle.mgmt_info.as_mut() else {
return RC::FileHandleNotInit;
};
let Some(file) = anyf.downcast_mut::<std::fs::File>() else {
return RC::FileHandleNotInit;
};

if file.seek(SeekFrom::End(0)).is_err() {
return RC::SeekFailed;
}
let page_data = vec![0u8; 4096];
if file.write_all(&page_data).is_err() {
return RC::WriteFailed;
}
f_handle.total_num_pages += 1;
f_handle.cur_page_pos = f_handle.total_num_pages - 1;
RC::Ok
}

pub fn ensure_capacity(number_of_pages: i32, f_handle: &mut SM_FileHandle) -> RC {
if f_handle.mgmt_info.is_none() {
return RC::FileHandleNotInit;
}
if number_of_pages <= f_handle.total_num_pages {
return RC::Ok;
}
let inc = number_of_pages - f_handle.total_num_pages;
for _ in 0..inc {
let rc = append_empty_block(f_handle);
if rc != RC::Ok {
return rc;
}
}
RC::Ok
}
