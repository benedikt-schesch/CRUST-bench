use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use frunk_core::{hlist, HList, poly_fn};
use std::fmt;
use std::sync::Mutex;

pub const CLOG_MAX_LOGGERS: usize = 16;
pub const CLOG_FORMAT_LENGTH: usize = 256;
pub const CLOG_DATETIME_LENGTH: usize = 256;
pub const CLOG_DEFAULT_FORMAT: &str = "%d %t %f(%n): %l: %m\n";
pub const CLOG_DEFAULT_DATE_FORMAT: &str = "%Y-%m-%d";
pub const CLOG_DEFAULT_TIME_FORMAT: &str = "%H:%M:%S";

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ClogLevel {
Debug,
Info,
Warn,
Error,
}

impl ClogLevel {
fn as_usize(&self) -> usize {
match self {
ClogLevel::Debug => 0,
ClogLevel::Info => 1,
ClogLevel::Warn => 2,
ClogLevel::Error => 3,
}
}
fn as_str(&self) -> &'static str {
match self {
ClogLevel::Debug => "DEBUG",
ClogLevel::Info => "INFO",
ClogLevel::Warn => "WARN",
ClogLevel::Error => "ERROR",
}
}
}

pub struct Clog {
level: ClogLevel,
file: Option<File>,
fmt: [char; CLOG_FORMAT_LENGTH],
date_fmt: [char; CLOG_FORMAT_LENGTH],
time_fmt: [char; CLOG_FORMAT_LENGTH],
opened: bool,
}

const INIT: Option<Clog> = None;
static LOGGERS: Mutex<[Option<Clog>; CLOG_MAX_LOGGERS]> = Mutex::new([INIT; CLOG_MAX_LOGGERS]);

fn str_to_chars(s: &str) -> [char; CLOG_FORMAT_LENGTH] {
let mut chars = ['\0'; CLOG_FORMAT_LENGTH];
for (i, c) in s.chars().take(CLOG_FORMAT_LENGTH - 1).enumerate() {
chars[i] = c;
}
chars
}

pub fn clog_init_path(id: usize, path: &str) -> io::Result<()> {
let file = OpenOptions::new().create(true).write(true).append(true).open(path)?;
clog_init_fd(id, file)?;
if let Ok(mut loggers) = LOGGERS.lock() {
if let Some(logger) = &mut loggers[id] {
logger.opened = true;
}
}
Ok(())
}

pub fn clog_init_fd(id: usize, file: File) -> io::Result<()> {
if id >= CLOG_MAX_LOGGERS {
return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid logger id"));
}
let mut loggers = LOGGERS.lock().unwrap();
if loggers[id].is_some() {
eprintln!("Logger {} already initialized.", id);
return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Logger already initialized"));
}

let logger = Clog {
level: ClogLevel::Debug,
file: Some(file),
fmt: str_to_chars(CLOG_DEFAULT_FORMAT),
date_fmt: str_to_chars(CLOG_DEFAULT_DATE_FORMAT),
time_fmt: str_to_chars(CLOG_DEFAULT_TIME_FORMAT),
opened: false,
};
loggers[id] = Some(logger);
Ok(())
}

pub fn clog_free(id: usize) {
if id < CLOG_MAX_LOGGERS {
if let Ok(mut loggers) = LOGGERS.lock() {
loggers[id] = None;
}
}
}

pub fn clog_set_level(id: usize, level: ClogLevel) -> Result<(), ()> {
if id >= CLOG_MAX_LOGGERS {
return Err(());
}
if let Ok(mut loggers) = LOGGERS.lock() {
if let Some(logger) = &mut loggers[id] {
logger.level = level;
return Ok(());
}
}
Err(())
}

pub fn clog_set_date_fmt(id: usize, fmt: &str) -> Result<(), ()> {
if id >= CLOG_MAX_LOGGERS {
eprintln!("clog_set_date_fmt: No such logger: {}", id);
return Err(());
}
if fmt.chars().count() >= CLOG_FORMAT_LENGTH {
eprintln!("clog_set_date_fmt: Format specifier too long.");
return Err(());
}
if let Ok(mut loggers) = LOGGERS.lock() {
if let Some(logger) = &mut loggers[id] {
logger.date_fmt = str_to_chars(fmt);
return Ok(());
}
}
eprintln!("clog_set_date_fmt: No such logger: {}", id);
Err(())
}

pub fn clog_set_fmt(id: usize, fmt: &str) -> Result<(), ()> {
if id >= CLOG_MAX_LOGGERS {
eprintln!("clog_set_fmt: No such logger: {}", id);
return Err(());
}
if fmt.chars().count() >= CLOG_FORMAT_LENGTH {
eprintln!("clog_set_fmt: Format specifier too long.");
return Err(());
}
if let Ok(mut loggers) = LOGGERS.lock() {
if let Some(logger) = &mut loggers[id] {
logger.fmt = str_to_chars(fmt);
return Ok(());
}
}
eprintln!("clog_set_fmt: No such logger: {}", id);
Err(())
}

fn _clog_basename(path: &str) -> &str {
path.split(|c| c == '/' || c == '\\').last().unwrap_or(path)
}

fn _clog_format(logger: &Clog, sfile: &str, sline: i32, level: ClogLevel, message: &str) -> String {
let mut result = String::new();
let mut state = 0; // 0 = NORMAL, 1 = SUBST

let sfile_base = _clog_basename(sfile);

let time_str = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
Ok(n) => n.as_secs().to_string(),
Err(_) => "0".to_string(),
};

for &c in &logger.fmt {
if c == '\0' {
break;
}
if state == 0 {
if c == '%' {
state = 1;
} else {
result.push(c);
}
} else {
match c {
'%' => result.push('%'),
't' => result.push_str(&time_str),
'd' => result.push_str(&time_str),
'l' => result.push_str(level.as_str()),
'n' => result.push_str(&sline.to_string()),
'f' => result.push_str(sfile_base),
'm' => result.push_str(message),
_ => {
result.push('%');
result.push(c);
}
}
state = 0;
}
}
result
}

fn _clog_log(sfile: &str, sline: i32, level: ClogLevel, id: usize, args: std::fmt::Arguments) {
if id >= CLOG_MAX_LOGGERS {
eprintln!("No such logger: {}", id);
return;
}

let mut loggers = LOGGERS.lock().unwrap();
let logger = match &mut loggers[id] {
Some(l) => l,
None => {
eprintln!("No such logger: {}", id);
return;
}
};

if level.as_usize() < logger.level.as_usize() {
return;
}

let message = format!("{}", args);
let formatted_message = _clog_format(logger, sfile, sline, level, &message);

if let Some(file) = &mut logger.file {
if let Err(e) = file.write_all(formatted_message.as_bytes()) {
eprintln!("Unable to write to log file: {}", e);
}
}
}

pub fn clog_debug(sfile: &str, sline: i32, id: usize, args: std::fmt::Arguments) {
_clog_log(sfile, sline, ClogLevel::Debug, id, args);
}

pub fn clog_info(sfile: &str, sline: i32, id: usize, args: std::fmt::Arguments) {
_clog_log(sfile, sline, ClogLevel::Info, id, args);
}

pub fn clog_warn(sfile: &str, sline: i32, id: usize, args: std::fmt::Arguments) {
_clog_log(sfile, sline, ClogLevel::Warn, id, args);
}

pub fn clog_error(sfile: &str, sline: i32, id: usize, args: std::fmt::Arguments) {
_clog_log(sfile, sline, ClogLevel::Error, id, args);
}
