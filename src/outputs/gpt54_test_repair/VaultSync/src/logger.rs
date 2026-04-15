
use std::time::{SystemTime, UNIX_EPOCH};
pub const KNORMAL: &str = "\x1B[0m";
pub const KRED: &str = "\x1B[31m";
pub const KGREEN: &str = "\x1B[32m";
pub const KYELLOW: &str = "\x1B[33m";
pub const KBLUE: &str = "\x1B[34m";
pub const KMAGENTA: &str = "\x1B[35m";
pub const KCYAN: &str = "\x1B[36m";
pub const KWHITE: &str = "\x1B[37m";
pub const COLOR_BOLD: &str = "\x1B[1m";
pub const COLOR_OFF: &str = "\x1B[m";
pub enum LOGGING_TAG {
INFO_TAG = 1,
ERROR_TAG = 2,
WARNING_TAG = 3,
}
pub fn logger(tag: LOGGING_TAG, message: &str) {
let now = SystemTime::now()
.duration_since(UNIX_EPOCH)
.map(|d| d.as_secs())
.unwrap_or(0);
match tag {
LOGGING_TAG::INFO_TAG => {
println!("{now} {COLOR_BOLD}[INFO]{COLOR_OFF}: {message}");
}
LOGGING_TAG::ERROR_TAG => {
eprintln!(
"{now} {KRED}{COLOR_BOLD}[ERROR]{COLOR_OFF}{KNORMAL}: {message}"
);
}
LOGGING_TAG::WARNING_TAG => {
println!("{now} {COLOR_BOLD}[WARNING]{COLOR_OFF}: {message}");
}
}
}
