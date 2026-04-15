use std::fs;

/// Returns the string contents of the file at `path` and sets the file length in `len`.
/// Returns `None` on error.
pub fn file2strl(path: &str, len: &mut u32) -> Option<String> {
let bytes = match fs::read(path) {
Ok(data) => data,
Err(_) => return None,
};

let file_len_with_nul = bytes.len().checked_add(1)?;
*len = u32::try_from(file_len_with_nul).ok()?;

let mut contents = String::from_utf8_lossy(&bytes).into_owned();
contents.push('\0');
Some(contents)
}

/// Returns the string contents of the file at `path`, or `None` on error.
pub fn file2str(path: &str) -> Option<String> {
let bytes = match fs::read(path) {
Ok(data) => data,
Err(_) => return None,
};

let mut contents = String::from_utf8_lossy(&bytes).into_owned();
contents.push('\0');
Some(contents)
}
