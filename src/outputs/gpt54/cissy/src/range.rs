// Generated Rust Code
pub mod range {
#[derive(Debug, Clone, PartialEq)]
pub enum RangeType {
Empty,
Single,
StartEnd,
GreaterEqual,
}

#[derive(Debug, Clone)]
pub struct RangeElement {
pub start: u32,
pub end: u32,
pub rangetype: RangeType,
pub next: Option<Box<RangeElement>>,
}

impl RangeElement {
pub fn new() -> Self {
Self {
start: u32::MAX,
end: u32::MAX,
rangetype: RangeType::Empty,
next: None,
}
}

pub fn add_single(
num: u32,
start_of_list: Option<Box<RangeElement>>,
) -> Option<Box<RangeElement>> {
assert!(num > 0);
match start_of_list {
None => Some(Box::new(RangeElement {
start: num,
end: u32::MAX,
rangetype: RangeType::Single,
next: None,
})),
Some(mut head) => {
let mut ptr = &mut head;
while ptr.next.is_some() {
ptr = ptr.next.as_mut().expect("next exists");
}
ptr.next = Some(Box::new(RangeElement {
start: num,
end: u32::MAX,
rangetype: RangeType::Single,
next: None,
}));
Some(head)
}
}
}

pub fn add_start_end(
start: u32,
end: u32,
start_of_list: Option<Box<RangeElement>>,
) -> Option<Box<RangeElement>> {
assert!(start > 0);
assert!(end >= start);

let rangetype = if start == end {
RangeType::Single
} else {
RangeType::StartEnd
};

match start_of_list {
None => Some(Box::new(RangeElement {
start,
end: if start == end { u32::MAX } else { end },
rangetype,
next: None,
})),
Some(mut head) => {
let mut ptr = &mut head;
while ptr.next.is_some() {
ptr = ptr.next.as_mut().expect("next exists");
}
ptr.next = Some(Box::new(RangeElement {
start,
end: if start == end { u32::MAX } else { end },
rangetype,
next: None,
}));
Some(head)
}
}
}

pub fn add_greater_equal(
num: u32,
start_of_list: Option<Box<RangeElement>>,
) -> Option<Box<RangeElement>> {
match start_of_list {
None => Some(Box::new(RangeElement {
start: num,
end: u32::MAX,
rangetype: RangeType::GreaterEqual,
next: None,
})),
Some(mut head) => {
let mut ptr = &mut head;
while ptr.next.is_some() {
ptr = ptr.next.as_mut().expect("next exists");
}
ptr.next = Some(Box::new(RangeElement {
start: num,
end: u32::MAX,
rangetype: RangeType::GreaterEqual,
next: None,
}));
Some(head)
}
}
}

pub fn contains_num(num: u32, start_of_list: &Option<Box<RangeElement>>) -> bool {
let mut ptr = start_of_list.as_ref();
while let Some(node) = ptr {
match node.rangetype {
RangeType::Single => {
if node.start == num {
return true;
}
}
RangeType::GreaterEqual => {
if num >= node.start {
return true;
}
}
RangeType::StartEnd => {
if num >= node.start && num <= node.end {
return true;
}
}
RangeType::Empty => {}
}
ptr = node.next.as_ref();
}
false
}

pub fn to_string<'a>(&self, buf: &'a mut String, bufsize: usize) -> &'a str {
let s = match self.rangetype {
RangeType::Single => format!("[{}]", self.start),
RangeType::GreaterEqual => format!("[{}-]", self.start),
RangeType::StartEnd => format!("[{}-{}]", self.start, self.end),
RangeType::Empty => "[]".to_string(),
};
buf.clear();
if bufsize == 0 {
return buf.as_str();
}
let take_len = s.len().min(bufsize);
buf.push_str(&s[..take_len]);
buf.as_str()
}

pub fn list_to_string<'a>(
buf: &'a mut String,
bufsize: usize,
start_of_list: &'a Option<Box<RangeElement>>,
) -> &'a str {
buf.clear();
let mut ptr = start_of_list.as_ref();

while let Some(node) = ptr {
let part = match node.rangetype {
RangeType::Single => format!("[{}]", node.start),
RangeType::GreaterEqual => format!("[{}-]", node.start),
RangeType::StartEnd => format!("[{}-{}]", node.start, node.end),
RangeType::Empty => "[]".to_string(),
};

if buf.len() >= bufsize {
break;
}
let remaining = bufsize - buf.len();
let take_len = part.len().min(remaining);
buf.push_str(&part[..take_len]);

ptr = node.next.as_ref();
}

buf.as_str()
}

pub fn parse_int_ranges(text: &str) -> Option<Box<RangeElement>> {
let mut state = 0i32;
let flag_dash = 0x01i32;
let mut numbuf = String::new();
let mut rv: Option<Box<RangeElement>> = None;
let mut start = 0u32;
let mut end;

for ch in text.chars() {
if ch.is_ascii_digit() {
numbuf.push(ch);
} else if ch == '-' {
if (state & flag_dash) != 0 {
panic!("error: too many dashes in range");
}
state |= flag_dash;
if numbuf.is_empty() {
panic!("error: range cannot start with '-'");
}
start = numbuf.parse::<u32>().unwrap_or(0);
if start == 0 {
panic!("error: start range <= 0");
}
numbuf.clear();
} else if ch == ',' {
if numbuf.is_empty() {
if (state & flag_dash) != 0 {
rv = RangeElement::add_greater_equal(start, rv);
}
} else if start > 0 {
end = numbuf.parse::<u32>().unwrap_or(0);
if end == 0 || start > end {
panic!("error: illegal start/end ranges");
}
rv = RangeElement::add_start_end(start, end, rv);
} else {
start = numbuf.parse::<u32>().unwrap_or(0);
rv = RangeElement::add_single(start, rv);
}
numbuf.clear();
start = 0;
state = 0;
}
}

if numbuf.is_empty() {
if (state & flag_dash) != 0 && start > 0 {
rv = RangeElement::add_greater_equal(start, rv);
}
} else if start > 0 {
end = numbuf.parse::<u32>().unwrap_or(0);
if end == 0 || start > end {
panic!("error: illegal start/end range {}", text);
}
rv = RangeElement::add_start_end(start, end, rv);
} else {
start = numbuf.parse::<u32>().unwrap_or(0);
rv = RangeElement::add_single(start, rv);
}

rv
}
}
}
