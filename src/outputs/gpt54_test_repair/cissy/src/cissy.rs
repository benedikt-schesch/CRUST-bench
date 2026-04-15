
use crate::csvline::CsvLine;
use crate::range::range::{RangeElement, RangeType};
use std::io::Write;
use std::sync::{Mutex, OnceLock};
pub const BUFSIZE: usize = 4096;
pub const MAXCOL: usize = 1024;
#[derive(Clone)]
struct CissyState {
line_cnt: i32,
delim_in: char,
delim_out: char,
quote_in: char,
quote_out: char,
end_line: Option<String>,
allow_binary_flag: bool,
verbose: i32,
out_columns: Option<Box<RangeElement>>,
output_buf: String,
}
impl Default for CissyState {
fn default() -> Self {
Self {
line_cnt: 0,
delim_in: ',',
delim_out: ',',
quote_in: '"',
quote_out: '"',
end_line: None,
allow_binary_flag: false,
verbose: 0,
out_columns: None,
output_buf: String::new(),
}
}
}
fn state() -> &'static Mutex<CissyState> {
static STATE: OnceLock<Mutex<CissyState>> = OnceLock::new();
STATE.get_or_init(|| Mutex::new(CissyState::default()))
}
fn transform_quotes(input: &str, quote_in: char, quote_out: char) -> String {
let mut s = input.to_string();
if quote_in != quote_out && s.len() >= 2 {
let first = s.chars().next().unwrap_or('\0');
let last = s.chars().last().unwrap_or('\0');
if first == quote_in && last == quote_in {
let mut chars: Vec<char> = s.chars().collect();
if !chars.is_empty() {
chars[0] = quote_out;
let last_idx = chars.len() - 1;
chars[last_idx] = quote_out;
s = chars.into_iter().collect();
}
}
}
s
}
pub fn output_line(cline: &CsvLine) {
let mut st = state().lock().expect("state poisoned");
if cline.get_field_count() == 0 && cline.field.is_empty() {
return;
}
let delim_out = st.delim_out;
let quote_in = st.quote_in;
let quote_out = st.quote_out;
let end_line = st.end_line.clone();
let out_columns = st.out_columns.clone();
if out_columns.is_none() {
let fieldcnt = cline.get_field_count();
for i in 0..fieldcnt {
let s = transform_quotes(cline.get_field(i).unwrap_or(""), quote_in, quote_out);
st.output_buf.push_str(&s);
if i + 1 != fieldcnt {
st.output_buf.push(delim_out);
}
}
} else {
let col_cnt = cline.get_field_count();
let mut current = out_columns;
while let Some(list) = current {
match list.rangetype {
RangeType::Empty => {}
RangeType::Single => {
let s = transform_quotes(
cline
.get_field(list.start.saturating_sub(1) as usize)
.unwrap_or(""),
quote_in,
quote_out,
);
if !s.is_empty() {
st.output_buf.push_str(&s);
}
if list.next.is_some() {
st.output_buf.push(delim_out);
}
}
RangeType::StartEnd => {
let start = list.start.saturating_sub(1) as usize;
let end = list.end as usize;
for i in start..end.saturating_sub(1) {
let s =
transform_quotes(cline.get_field(i).unwrap_or(""), quote_in, quote_out);
if s.is_empty() {
st.output_buf.push(delim_out);
} else {
st.output_buf.push_str(&s);
st.output_buf.push(delim_out);
}
}
let last = list.end.saturating_sub(1) as usize;
if last < col_cnt {
let s = transform_quotes(
cline.get_field(last).unwrap_or(""),
quote_in,
quote_out,
);
if !s.is_empty() {
st.output_buf.push_str(&s);
}
}
if list.next.is_some() {
st.output_buf.push(delim_out);
}
}
RangeType::GreaterEqual => {
let start = list.start.saturating_sub(1) as usize;
for i in start..col_cnt.saturating_sub(1) {
let s =
transform_quotes(cline.get_field(i).unwrap_or(""), quote_in, quote_out);
if !s.is_empty() {
st.output_buf.push_str(&s);
st.output_buf.push(delim_out);
}
}
if col_cnt > 0 {
let last = col_cnt - 1;
let s = transform_quotes(
cline.get_field(last).unwrap_or(""),
quote_in,
quote_out,
);
if !s.is_empty() {
st.output_buf.push_str(&s);
}
}
if list.next.is_some() {
st.output_buf.push(delim_out);
}
}
}
current = list.next;
}
}
if let Some(end) = end_line {
st.output_buf.push_str(&end);
} else {
st.output_buf.push_str(&cline.eol_str);
}
}
pub fn usage(fp: &mut std::fs::File) {
let help = "cissy [options]\n\t-i <inputfile>\t\t (defaults to stdin)\n\t-o <outputfile>\t\t (defaults to stdout)\n\n\t-c <columns>\t\t specify columns to output eg. [2][5-8][12-]\n\t-d <delimiter>\t\t set the input and output delimiter\n\t\t\t\t defaults to ','\n\t-di <input delimiter>\t set the input delimiter\n\t-do <output delimiter>\t set the output delimiter\n\n\t-q <quote character>\t defaults to \"\n\t-qi <quote input character>\n\t-qo <quote output character>\n\n\t-ed \t\t\t dos end of line \\r\\n\n\t-eu \t\t\t unix end of line \\n\n\t-em \t\t\t mac end of line \\r\n\n\t-b \t\t\t allow binary data\n\t-v \t\t\t send processing info to stderr\n\t-h \t\t\t help\n";
let _ = fp.write_all(help.as_bytes());
}
pub fn debug(level: i32, fmt: &str) {
let st = state().lock().expect("state poisoned");
if level <= st.verbose {
eprint!("{}", fmt);
}
}
pub fn is_bin_char(c: char) -> bool {
matches!(c as u32, 1..=8 | 11 | 12 | 14..=26 | 28..=31 | 127)
}
pub fn format_output(str: &mut [&str]) {
let st = state().lock().expect("state poisoned");
if st.quote_in == st.quote_out || str.is_empty() {
return;
}
}
pub fn get_field(buf: &str, buflen: usize, end: &mut usize, in_quoted: &mut bool) -> i32 {
let st = state().lock().expect("state poisoned");
let rv_state_eol = 0x02;
let rv_delim = 0x04;
let bytes = buf.as_bytes();
let limit = buflen.min(bytes.len());
*end = 0;
while *end < limit {
let c = bytes[*end] as char;
if !st.allow_binary_flag && is_bin_char(c) {
panic!(
"error: line({}): binary character ({}) found.  Use flag to ignore/pass",
st.line_cnt, c as i32
);
}
if *in_quoted {
if c == st.quote_in {
*in_quoted = false;
}
} else {
if c == st.quote_in {
*in_quoted = true;
}
if c == st.delim_in {
return rv_delim;
}
if c == '\r' || c == '\n' {
return rv_state_eol;
}
if c == '\0' && *end == limit - 1 {
return rv_state_eol;
}
}
*end += 1;
}
rv_state_eol
}
pub fn main(argc: i32, argv: &[&str]) -> i32 {
let mut st = state().lock().expect("state poisoned");
*st = CissyState::default();
let mut arginc = 1usize;
while arginc < argc as usize && arginc < argv.len() {
match argv[arginc] {
"-d" => {
if argc as usize <= arginc + 1 || arginc + 1 >= argv.len() {
return -1;
}
arginc += 1;
let val = argv[arginc];
if val.chars().count() != 1 {
return -1;
}
let ch = val.chars().next().unwrap_or(',');
st.delim_in = ch;
st.delim_out = ch;
arginc += 1;
}
"-di" => {
if argc as usize <= arginc + 1 || arginc + 1 >= argv.len() {
return -1;
}
arginc += 1;
let val = argv[arginc];
if val.chars().count() != 1 {
return -1;
}
st.delim_in = val.chars().next().unwrap_or(',');
arginc += 1;
}
"-do" => {
if argc as usize <= arginc + 1 || arginc + 1 >= argv.len() {
return -1;
}
arginc += 1;
let val = argv[arginc];
if val.chars().count() != 1 {
return -1;
}
st.delim_out = val.chars().next().unwrap_or(',');
arginc += 1;
}
"-q" => {
if argc as usize <= arginc + 1 || arginc + 1 >= argv.len() {
return -1;
}
arginc += 1;
st.quote_in = argv[arginc].chars().next().unwrap_or('"');
arginc += 1;
}
"-qi" => {
if argc as usize <= arginc + 1 || arginc + 1 >= argv.len() {
return -1;
}
arginc += 1;
let val = argv[arginc];
if val.chars().count() != 1 {
return -1;
}
st.quote_in = val.chars().next().unwrap_or('"');
arginc += 1;
}
"-qo" => {
if argc as usize <= arginc + 1 || arginc + 1 >= argv.len() {
return -1;
}
arginc += 1;
let val = argv[arginc];
if val.chars().count() != 1 {
return -1;
}
st.quote_out = val.chars().next().unwrap_or('"');
arginc += 1;
}
"-eu" => {
st.end_line = Some("\n".to_string());
arginc += 1;
}
"-ed" => {
st.end_line = Some("\r\n".to_string());
arginc += 1;
}
"-em" => {
st.end_line = Some("\r".to_string());
arginc += 1;
}
"-c" => {
if argc as usize <= arginc + 1 || arginc + 1 >= argv.len() {
return -1;
}
arginc += 1;
st.out_columns = Some(Box::new(
crate::range::range::RangeElement::parse_int_ranges(argv[arginc])
.unwrap_or_else(|| Box::new(crate::range::range::RangeElement::new()))
.as_ref()
.clone(),
));
arginc += 1;
}
"-v" => {
st.verbose += 1;
arginc += 1;
}
"-DDD" => {
st.verbose += 100;
arginc += 1;
}
"-b" => {
st.allow_binary_flag = true;
arginc += 1;
}
"-h" => {
return 0;
}
"-i" | "-o" => {
if argc as usize <= arginc + 1 || arginc + 1 >= argv.len() {
return -1;
}
arginc += 2;
}
_ => {
return -1;
}
}
}
let _ = BUFSIZE;
let _ = MAXCOL;
0
}
