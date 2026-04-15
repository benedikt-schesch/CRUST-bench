use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
pub fn file2strl(path: &str, len: &mut u32) -> Option<String> {
let mut file = match File::open(path) {
Ok(f) => f,
Err(_) => {
eprintln!("Unable to open file {}", path);
return None;
}
};
let file_len = match file.seek(SeekFrom::End(0)) {
Ok(pos) => pos,
Err(_) => {
eprintln!("Unable to seek file {}", path);
return None;
}
};
if let Err(_) = file.seek(SeekFrom::Start(0)) {
eprintln!("Unable to seek file {}", path);
return None;
}
if file_len > (u32::MAX as u64) - 1 {
return None;
}
let mut buffer = vec![0u8; file_len as usize];
if let Err(_) = file.read_exact(&mut buffer) {
eprintln!("Read error");
return None;
}
let contents = match String::from_utf8(buffer) {
Ok(s) => s,
Err(_) => return None,
};
*len = (file_len as u32) + 1;
Some(contents)
}
pub fn file2str(path: &str) -> Option<String> {
let mut dummy = 0;
file2strl(path, &mut dummy)
}
