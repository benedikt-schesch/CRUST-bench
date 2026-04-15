pub fn rpl_vsnprintf(
s: &mut String,
n: usize,
format: &str,
args: &[&str],
) -> i32 {
let mut out = String::new();
let mut chars = format.chars().peekable();
let mut argi = 0usize;
while let Some(ch) = chars.next() {
if ch != '%' {
out.push(ch);
continue;
}
if matches!(chars.peek(), Some('%')) {
chars.next();
out.push('%');
continue;
}
let mut left = false;
let mut zero = false;
loop {
match chars.peek().copied() {
Some('-') => {
left = true;
chars.next();
}
Some('0') => {
zero = true;
chars.next();
}
Some('+') | Some(' ') | Some('#') | Some('\'') => {
chars.next();
}
_ => break,
}
}
let mut width = 0usize;
while let Some(c) = chars.peek().copied() {
if c.is_ascii_digit() {
width = width.saturating_mul(10).saturating_add(c as usize - '0' as usize);
chars.next();
} else {
break;
}
}
let mut precision: Option<usize> = None;
if matches!(chars.peek(), Some('.')) {
chars.next();
let mut p = 0usize;
let mut any = false;
while let Some(c) = chars.peek().copied() {
if c.is_ascii_digit() {
p = p.saturating_mul(10).saturating_add(c as usize - '0' as usize);
chars.next();
any = true;
} else {
break;
}
}
precision = Some(if any { p } else { 0 });
}
while let Some(c) = chars.peek().copied() {
if matches!(c, 'h' | 'l' | 'L' | 'j' | 't' | 'z') {
chars.next();
if (c == 'h' || c == 'l') && matches!(chars.peek(), Some(x) if *x == c) {
chars.next();
}
} else {
break;
}
}
let conv = chars.next().unwrap_or('\0');
let val = if argi < args.len() { args[argi] } else { "" };
if conv != '\0' && conv != '%' {
argi += 1;
}
let mut piece = String::new();
match conv {
's' => {
let mut v = val.to_string();
if let Some(p) = precision {
v = v.chars().take(p).collect();
}
if width > v.chars().count() {
let pad = width - v.chars().count();
if left {
piece.push_str(&v);
piece.push_str(&" ".repeat(pad));
} else {
piece.push_str(&" ".repeat(pad));
piece.push_str(&v);
}
} else {
piece.push_str(&v);
}
}
'd' | 'i' => {
let num = val.parse::<i64>().unwrap_or(0);
let mut v = num.to_string();
if let Some(p) = precision {
let neg = v.starts_with('-');
let digits = if neg { &v[1..] } else { &v[..] };
let padded = if digits.len() < p {
format!("{}{}", "0".repeat(p - digits.len()), digits)
} else {
digits.to_string()
};
v = if neg { format!("-{}", padded) } else { padded };
}
if width > v.len() {
let pad = width - v.len();
let fill = if zero && precision.is_none() && !left { '0' } else { ' ' };
if left {
piece.push_str(&v);
piece.push_str(&" ".repeat(pad));
} else if fill == '0' && v.starts_with('-') {
piece.push('-');
piece.push_str(&"0".repeat(pad));
piece.push_str(&v[1..]);
} else {
piece.push_str(&fill.to_string().repeat(pad));
piece.push_str(&v);
}
} else {
piece.push_str(&v);
}
}
'u' | 'o' | 'x' | 'X' => {
let num = val.parse::<u64>().unwrap_or(0);
let mut v = match conv {
'u' => format!("{num}"),
'o' => format!("{num:o}"),
'x' => format!("{num:x}"),
_ => format!("{num:X}"),
};
if let Some(p) = precision {
if v.len() < p {
v = format!("{}{}", "0".repeat(p - v.len()), v);
}
}
if width > v.len() {
let pad = width - v.len();
let fill = if zero && precision.is_none() && !left { '0' } else { ' ' };
if left {
piece.push_str(&v);
piece.push_str(&" ".repeat(pad));
} else {
piece.push_str(&fill.to_string().repeat(pad));
piece.push_str(&v);
}
} else {
piece.push_str(&v);
}
}
'f' | 'F' | 'e' | 'E' | 'g' | 'G' => {
let num = val.parse::<f64>().unwrap_or(0.0);
let p = precision.unwrap_or(6);
let raw = match conv {
'f' | 'F' => format!("{num:.p$}"),
'e' => format!("{num:.p$e}"),
'E' => format!("{num:.p$E}"),
'g' => format!("{num:.p$}"),
'G' => format!("{num:.p$}"),
_ => String::new(),
};
if width > raw.len() {
let pad = width - raw.len();
if left {
piece.push_str(&raw);
piece.push_str(&" ".repeat(pad));
} else {
let fill = if zero { '0' } else { ' ' };
piece.push_str(&fill.to_string().repeat(pad));
piece.push_str(&raw);
}
} else {
piece.push_str(&raw);
}
}
'c' => {
let chv = val.chars().next().unwrap_or('\0');
piece.push(chv);
}
'p' => {
piece.push_str(val);
}
'%' => piece.push('%'),
_ => {}
}
out.push_str(&piece);
}
s.clear();
if n == 0 {
return out.len() as i32;
}
let truncated: String = out.chars().take(n.saturating_sub(1)).collect();
s.push_str(&truncated);
out.len() as i32
}
pub fn fmtstr(
s: &mut String,
size: usize,
value: &str,
width: usize,
precision: usize,
flags: i32,
) {
let left = (flags & 1) != 0;
let mut v: String = value.chars().take(precision).collect();
let vlen = v.chars().count();
if width > vlen {
let pad = width - vlen;
if left {
v.push_str(&" ".repeat(pad));
} else {
v = format!("{}{}", " ".repeat(pad), v);
}
}
if size > 0 {
let lim: String = v.chars().take(size.saturating_sub(1)).collect();
s.push_str(&lim);
}
}
pub fn fmtint(
s: &mut String,
size: usize,
value: i32,
width: usize,
precision: usize,
flags: i32,
) {
let left = (flags & 1) != 0;
let zero = (flags & 16) != 0;
let mut v = value.to_string();
let neg = v.starts_with('-');
let digits = if neg { &v[1..] } else { &v[..] };
let digits = if digits.len() < precision {
format!("{}{}", "0".repeat(precision - digits.len()), digits)
} else {
digits.to_string()
};
v = if neg { format!("-{}", digits) } else { digits };
let res = if width > v.len() {
let pad = width - v.len();
if left {
format!("{}{}", v, " ".repeat(pad))
} else if zero && v.starts_with('-') {
format!("-{}{}", "0".repeat(pad), &v[1..])
} else {
format!(
"{}{}",
if zero { "0".repeat(pad) } else { " ".repeat(pad) },
v
)
}
} else {
v
};
if size > 0 {
let lim: String = res.chars().take(size.saturating_sub(1)).collect();
s.push_str(&lim);
}
}
pub fn fmtflt(
s: &mut String,
size: usize,
value: f64,
width: usize,
precision: usize,
flags: i32,
) {
let left = (flags & 1) != 0;
let zero = (flags & 16) != 0;
let raw = format!("{value:.precision$}");
let res = if width > raw.len() {
let pad = width - raw.len();
if left {
format!("{}{}", raw, " ".repeat(pad))
} else {
format!(
"{}{}",
if zero { "0".repeat(pad) } else { " ".repeat(pad) },
raw
)
}
} else {
raw
};
if size > 0 {
let lim: String = res.chars().take(size.saturating_sub(1)).collect();
s.push_str(&lim);
}
}
pub fn printsep(s: &mut String, size: usize) {
if s.len() < size.saturating_sub(1) || size == 0 {
s.push(',');
}
}
pub fn getnumsep(digits: i32) -> i32 {
(digits - if digits % 3 == 0 { 1 } else { 0 }) / 3
}
pub fn getexponent(value: f64) -> i32 {
let mut tmp = if value >= 0.0 { value } else { -value };
let mut exponent = 0;
while tmp < 1.0 && tmp > 0.0 && exponent > -99 {
exponent -= 1;
tmp *= 10.0;
}
while tmp >= 10.0 && exponent < 99 {
exponent += 1;
tmp /= 10.0;
}
exponent
}
pub fn convert(
mut value: usize,
buf: &mut String,
base: usize,
caps: usize,
) {
let digits = if caps != 0 {
"0123456789ABCDEF"
} else {
"0123456789abcdef"
}
.chars()
.collect::<Vec<_>>();
buf.clear();
loop {
let ch = digits[value % base];
buf.push(ch);
value /= base;
if value == 0 {
break;
}
}
}
pub fn cast(value: f64) -> i32 {
if value.is_nan() {
0
} else if value >= i32::MAX as f64 {
i32::MAX
} else if value <= i32::MIN as f64 {
i32::MIN
} else {
value.floor() as i32
}
}
pub fn mypow10(exponent: i32) -> f64 {
let mut result = 1.0f64;
let mut e = exponent;
while e > 0 {
result *= 10.0;
e -= 1;
}
while e < 0 {
result /= 10.0;
e += 1;
}
result
}
pub fn rpl_vasprintf(
s: Vec<String>,
format: &str,
args: &[&str],
) -> i32 {
let mut out = String::new();
let len = rpl_vsnprintf(&mut out, usize::MAX, format, args);
let _ = s;
len
}
pub fn rpl_asprintf(
s: &mut String,
format: &str,
args: &[&str],
) -> i32 {
rpl_vsnprintf(s, usize::MAX, format, args)
}
pub fn main() {}
