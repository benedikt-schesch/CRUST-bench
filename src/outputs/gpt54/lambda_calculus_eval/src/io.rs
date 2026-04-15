use std::fs::{remove_file, File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};

pub fn create_file(path: &str) -> io::Result<File> {
File::create(path)
}

pub fn get_file(path: &str, mode: &str) -> io::Result<File> {
match mode {
"r" => File::open(path),
"w" => File::create(path),
"a" => OpenOptions::new().append(true).create(true).open(path),
"r+" => OpenOptions::new().read(true).write(true).open(path),
"w+" => OpenOptions::new()
.read(true)
.write(true)
.create(true)
.truncate(true)
.open(path),
_ => File::open(path),
}
}

pub fn write_to_file(file: &mut File, content: &str) -> io::Result<()> {
file.write_all(content.as_bytes())?;
file.flush()?;
file.seek(SeekFrom::Start(0))?;
Ok(())
}

pub fn delete_file(path: &str) -> io::Result<()> {
remove_file(path)
}

pub fn close_file(file: File) -> io::Result<()> {
file.sync_all()?;
drop(file);
Ok(())
}

pub fn next(file: &mut File) -> io::Result<char> {
let mut buf = [0u8; 1];
let n = file.read(&mut buf)?;
if n == 0 {
Ok('\0')
} else {
Ok(buf[0] as char)
}
}
