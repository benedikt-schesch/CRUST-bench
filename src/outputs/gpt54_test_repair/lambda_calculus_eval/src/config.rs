use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::common;
use crate::io;
pub const CONFIG_PATH: &str = "config";
#[derive(Debug, Clone)]
pub enum reduction_order_t {
APPLICATIVE,
NORMAL,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum option_type_t {
FILENAME,
STEP_REDUCTION,
REDUCTION_ORDER,
CONFIG_ERROR,
}
pub struct Options {
pub file: File,
pub step_by_step_reduction: bool,
pub reduction_order: reduction_order_t,
}
pub fn trim(str_: &mut String) {
*str_ = str_.trim().to_string();
}
pub fn get_config_type(key: &str) -> option_type_t {
if key == "file" {
option_type_t::FILENAME
} else if key == "step_by_step_reduction" {
option_type_t::STEP_REDUCTION
} else if key == "reduction_order" {
option_type_t::REDUCTION_ORDER
} else {
let error_msg = common::format("", format_args!("ERROR: Invalid key '{}' at config file.", key));
common::error(&error_msg, file!(), line!() as i32, "get_config_type");
option_type_t::CONFIG_ERROR
}
}
pub fn parse_config(line: &str, key: &mut String, value: &mut String) {
let Some(eq_pos) = line.find('=') else {
let error_msg = common::format(
"",
format_args!("Malformed config file at line: {} . Expected = sign.\n", line),
);
common::error(&error_msg, file!(), line!() as i32, "parse_config");
return;
};
*key = line[..eq_pos].to_string();
trim(key);
*value = line[eq_pos + 1..].to_string();
trim(value);
}
pub fn get_config_from_file() -> Options {
let config_file = io::get_file(CONFIG_PATH, "r").unwrap_or_else(|_| {
common::error(
&format!("Could not open config file {}", CONFIG_PATH),
file!(),
line!() as i32,
"get_config_from_file",
);
unreachable!()
});
let reader = BufReader::new(config_file);
let mut reduction_order = reduction_order_t::APPLICATIVE;
let mut step_by_step_reduction = false;
let mut file_opt: Option<File> = None;
for line_result in reader.lines() {
let line = line_result.unwrap_or_default();
if line.trim().is_empty() {
continue;
}
let mut key = String::new();
let mut value = String::new();
parse_config(&line, &mut key, &mut value);
let cfg = get_config_type(&key);
match cfg {
option_type_t::FILENAME => {
file_opt = Some(io::get_file(&value, "r").unwrap_or_else(|_| {
common::error(
&format!("Could not open file {}", value),
file!(),
line!() as i32,
"get_config_from_file",
);
unreachable!()
}));
}
option_type_t::STEP_REDUCTION => {
step_by_step_reduction = value == "true";
}
option_type_t::REDUCTION_ORDER => {
if value == "applicative" {
reduction_order = reduction_order_t::APPLICATIVE;
} else if value == "normal" {
reduction_order = reduction_order_t::NORMAL;
} else {
common::error(
"ERROR: reduction order in cfg file should be 'normal' or 'applicative'.",
file!(),
line!() as i32,
"get_config_from_file",
);
}
}
option_type_t::CONFIG_ERROR => {
let error_msg = common::format("", format_args!("Unrecognized key: {}", key));
common::error(&error_msg, file!(), line!() as i32, "get_config_from_file");
}
}
}
let file = file_opt.unwrap_or_else(|| {
common::error(
"ERROR: File cannot be null in cfg file.\n",
file!(),
line!() as i32,
"get_config_from_file",
);
unreachable!()
});
Options {
file,
step_by_step_reduction,
reduction_order,
}
}
