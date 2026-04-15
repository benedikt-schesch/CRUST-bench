use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

/// Returns the string contents of the file at `path` and sets the file length in `len`.
/// Returns `None` on error.
pub fn file2strl(path: &str, len: &mut u32) -> Option<String> {
// Open the file
let mut file = match File::open(path) {
Ok(f) => f,
Err(_) => {
eprintln!("Unable to open file {}", path);
return None;
}
};

// Seek to end to get file length
let file_len = match file.seek(SeekFrom::End(0)) {
Ok(pos) => pos,
Err(_) => {
eprintln!("Unable to seek file {}", path);
return None;
}
};

// Seek back to start
if let Err(_) = file.seek(SeekFrom::Start(0)) {
eprintln!("Unable to seek file {}", path);
return None;
}

// Check if file_len + 1 would overflow u32
if file_len > (u32::MAX as u64) - 1 {
return None;
}

// Allocate buffer and read contents
let mut buffer = vec![0u8; file_len as usize];
if let Err(_) = file.read_exact(&mut buffer) {
eprintln!("Read error");
return None;
}

// Convert to String (requires valid UTF-8)
let contents = match String::from_utf8(buffer) {
Ok(s) => s,
Err(_) => return None,
};

// Set length output (file_len + 1 to match C's null terminator inclusion)
*len = (file_len as u32) + 1;

Some(contents)
}

/// Returns the string contents of the file at `path`, or `None` on error.
pub fn file2str(path: &str) -> Option<String> {
let mut dummy = 0;
file2strl(path, &mut dummy)
}
