use std::fs;
use std::os::fd::{self, AsFd, FromRawFd};
use std::io::{self, Read, Write, Seek, SeekFrom, Result};
use std::path::Path;
#[cfg(target_os = "windows")]
pub const FS_OPEN_READ: &str = "rb";
#[cfg(target_os = "windows")]
pub const FS_OPEN_WRITE: &str = "wb";
#[cfg(target_os = "windows")]
pub const FS_OPEN_READWRITE: &str = "rwb";
#[cfg(not(target_os = "windows"))]
pub const FS_OPEN_READ: &str = "r";
#[cfg(not(target_os = "windows"))]
pub const FS_OPEN_WRITE: &str = "w";
#[cfg(not(target_os = "windows"))]
pub const FS_OPEN_READWRITE: &str = "rw";
pub fn fs_error(s: &str) {
let err = io::Error::last_os_error();
eprintln!("fs: {}: error: {}", s, err);
}
pub fn fs_open(path: &str, flags: &str) -> Option<fd::OwnedFd> {
let mut options = fs::OpenOptions::new();
let is_read = flags.contains('r');
let is_write = flags.contains('w');
if is_read && is_write {
options.read(true).write(true).create(true);
} else if is_write {
options.write(true).create(true).truncate(true);
} else if is_read {
options.read(true);
} else {
return None;
}
match options.open(path) {
Ok(file) => Some(file.into()),
Err(_) => None,
}
}
pub fn fs_close(fd: fd::OwnedFd) -> Result<()> {
drop(fd);
Ok(())
}
pub fn fs_rename(from: &str, to: &str) -> Result<()> {
fs::rename(from, to)
}
pub fn fs_stat(path: &str) -> Result<fs::Metadata> {
fs::metadata(path)
}
pub fn fs_fstat(fd: &fd::OwnedFd) -> Result<fs::Metadata> {
let file = fs::File::from(fd.try_clone()?);
file.metadata()
}
pub fn fs_lstat(path: &str) -> Result<fs::Metadata> {
fs::symlink_metadata(path)
}
pub fn fs_ftruncate(fd: &fd::OwnedFd, length: u64) -> Result<()> {
let file = fs::File::from(fd.try_clone()?);
file.set_len(length)
}
pub fn fs_truncate(path: &str, length: u64) -> Result<()> {
let file = fs::OpenOptions::new().write(true).create(true).open(path)?;
file.set_len(length)
}
pub fn fs_chown(_path: &str, _uid: u32, _gid: u32) -> Result<()> {
Err(io::Error::new(io::ErrorKind::Unsupported, "chown not supported without unsafe FFI"))
}
pub fn fs_fchown(_fd: &fd::OwnedFd, _uid: u32, _gid: u32) -> Result<()> {
Err(io::Error::new(io::ErrorKind::Unsupported, "fchown not supported without unsafe FFI"))
}
pub fn fs_lchown(_path: &str, _uid: u32, _gid: u32) -> Result<()> {
Err(io::Error::new(io::ErrorKind::Unsupported, "lchown not supported without unsafe FFI"))
}
pub fn fs_size(path: &str) -> Result<u64> {
let mut file = fs::File::open(path)?;
let size = file.seek(SeekFrom::End(0))?;
Ok(size)
}
pub fn fs_fsize(fd: &fd::OwnedFd) -> Result<u64> {
let mut file = fs::File::from(fd.try_clone()?);
let current_pos = file.seek(SeekFrom::Current(0))?;
let size = file.seek(SeekFrom::End(0))?;
file.seek(SeekFrom::Start(current_pos))?;
Ok(size)
}
pub fn fs_read(path: &str) -> Result<Vec<u8>> {
fs::read(path)
}
pub fn fs_nread(path: &str, len: u64) -> Result<Vec<u8>> {
let file = fs::File::open(path)?;
let mut buffer = Vec::with_capacity(len as usize);
let mut handle = file.take(len);
handle.read_to_end(&mut buffer)?;
Ok(buffer)
}
pub fn fs_fread(fd: &fd::OwnedFd) -> Result<Vec<u8>> {
let mut file = fs::File::from(fd.try_clone()?);
let mut buffer = Vec::new();
file.read_to_end(&mut buffer)?;
Ok(buffer)
}
pub fn fs_fnread(fd: &fd::OwnedFd, len: u64) -> Result<Vec<u8>> {
let file = fs::File::from(fd.try_clone()?);
let mut buffer = Vec::with_capacity(len as usize);
let mut handle = file.take(len);
handle.read_to_end(&mut buffer)?;
Ok(buffer)
}
pub fn fs_write(path: &str, data: &[u8]) -> Result<u64> {
fs::write(path, data)?;
Ok(data.len() as u64)
}
pub fn fs_nwrite(path: &str, data: &[u8], len: u64) -> Result<u64> {
let mut file = fs::File::create(path)?;
let to_write = std::cmp::min(data.len(), len as usize);
let written = file.write(&data[..to_write])?;
Ok(written as u64)
}
pub fn fs_fwrite(fd: &fd::OwnedFd, data: &[u8]) -> Result<u64> {
let mut file = fs::File::from(fd.try_clone()?);
let written = file.write(data)?;
Ok(written as u64)
}
pub fn fs_fnwrite(fd: &fd::OwnedFd, data: &[u8], len: u64) -> Result<u64> {
let mut file = fs::File::from(fd.try_clone()?);
let to_write = std::cmp::min(data.len(), len as usize);
let written = file.write(&data[..to_write])?;
Ok(written as u64)
}
pub fn fs_mkdir(path: &str, mode: u32) -> Result<()> {
#[cfg(unix)]
{
use std::os::unix::fs::DirBuilderExt;
let mut builder = fs::DirBuilder::new();
builder.mode(mode);
builder.create(path)
}
#[cfg(not(unix))]
{
fs::create_dir(path)
}
}
pub fn fs_rmdir(path: &str) -> Result<()> {
fs::remove_dir(path)
}
pub fn fs_exists(path: &str) -> bool {
Path::new(path).exists()
}
