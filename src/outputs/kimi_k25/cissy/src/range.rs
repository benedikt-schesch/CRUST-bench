pub enum RangeType {
Single(i32),
Range(i32, i32),
GreaterEqual(i32),
}

pub struct RangeElement {
pub range_type: RangeType,
}

impl RangeElement {
pub fn add_single(n: i32, mut list: Vec<RangeElement>) -> Vec<RangeElement> {
list.push(RangeElement {
range_type: RangeType::Single(n),
});
list
}

pub fn add_start_end(start: i32, end: i32, mut list: Vec<RangeElement>) -> Vec<RangeElement> {
list.push(RangeElement {
range_type: RangeType::Range(start, end),
});
list
}

pub fn add_greater_equal(n: i32, mut list: Vec<RangeElement>) -> Vec<RangeElement> {
list.push(RangeElement {
range_type: RangeType::GreaterEqual(n),
});
list
}

pub fn contains_num(n: i32, list: &[RangeElement]) -> bool {
for elem in list {
match &elem.range_type {
RangeType::Single(s) => if *s == n { return true; },
RangeType::Range(start, end) => if n >= *start && n <= *end { return true; },
RangeType::GreaterEqual(start) => if n >= *start { return true; },
}
}
false
}

pub fn list_to_string(buf: &mut [u8], bufsize: usize, list: &[RangeElement]) {
let limit = std::cmp::min(buf.len(), bufsize);
if limit == 0 {
return;
}

let mut s = String::new();
for (i, elem) in list.iter().enumerate() {
if i > 0 {
s.push(',');
}
match &elem.range_type {
RangeType::Single(n) => s.push_str(&n.to_string()),
RangeType::Range(start, end) => {
s.push_str(&start.to_string());
s.push('-');
s.push_str(&end.to_string());
}
RangeType::GreaterEqual(n) => {
s.push_str(&n.to_string());
s.push('+');
}
}
}

let bytes = s.as_bytes();
let len = std::cmp::min(bytes.len(), limit.saturating_sub(1));
if len > 0 {
buf[..len].copy_from_slice(&bytes[..len]);
}

if len < buf.len() {
buf[len] = 0;
}
}

pub fn parse_int_ranges(s: &str) -> Vec<RangeElement> {
let mut list = Vec::new();
if s.is_empty() {
return list;
}

for part in s.split(',') {
if part.is_empty() {
continue;
}

if part.contains('-') {
let mut parts = part.split('-');
if let (Some(start), Some(end)) = (parts.next(), parts.next()) {
if let (Ok(start), Ok(end)) = (start.parse::<i32>(), end.parse::<i32>()) {
list.push(RangeElement {
range_type: RangeType::Range(start, end),
});
}
}
} else if part.ends_with('+') {
let num = &part[..part.len()-1];
if let Ok(n) = num.parse::<i32>() {
list.push(RangeElement {
range_type: RangeType::GreaterEqual(n),
});
}
} else {
if let Ok(n) = part.parse::<i32>() {
list.push(RangeElement {
range_type: RangeType::Single(n),
});
}
}
}
list
}
}
