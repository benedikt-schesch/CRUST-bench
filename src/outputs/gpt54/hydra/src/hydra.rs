use std::fs;
use std::io::{self, Read, Write};

pub const RED: &str = "\x1b[0;31m";
pub const WHITE: &str = "\x1b[0;37m";
pub const COLOR_OFF: &str = "\x1b[0m";
pub const PURPLE: &str = "\x1b[0;35m";
pub const GREEN: &str = "\x1b[0;32m";
pub const YELLOW: &str = "\x1b[0;33m";
pub const BLUE: &str = "\x1b[0;34m";
pub const RIGHT_MARGIN: usize = 5;
pub const CYAN: &str = "\x1b[0;36m";
pub const DEFAULT_NAME: &str = "unnamed";

#[derive(Clone)]
pub struct Command {
pub key: char,
pub name: String,
pub command: String,
pub children: Option<Box<Command>>,
pub next: Option<Box<Command>>,
}

impl Command {
pub fn new(key: char, name: String, command: String) -> Self {
Command {
key,
name,
command,
children: None,
next: None,
}
}
}

pub fn read_field(file: &mut &[u8], field: &str) -> String {
let mut i = 0;
while i < file.len() && file[i] != b',' && file[i] != b'\n' && file[i] != 0 {
i += 1;
}

if i >= file.len() || file[i] != b',' {
let found = if i < file.len() { file[i] as char } else { '\0' };
eprintln!("Found incorrect end after {}, found: {}", field, found);
std::process::exit(1);
}

let result = String::from_utf8_lossy(&file[..i]).into_owned();
*file = &file[i + 1..];
result
}

pub fn load_file(c: &mut Command, file: &str) {
let content = read_file(file);
let mut slice = content.as_bytes();
while !slice.is_empty() && slice[0] != 0 {
read_line(c, &mut slice);
}
}

pub fn start(c: &Command) {
let mut current = c.clone();

while current.children.is_some() {
let last_printed_lines = print_command(&current);
let ch = getch();

let next = find_command(&current, ch).cloned();
clear_lines(last_printed_lines);

match next {
Some(cmd) => {
if command_run(&cmd) > 0 {
return;
}
current = cmd;
}
None => return,
}
}
}

pub fn tree_add_command(tree: &mut Command, keys: &str, name: &str, command: &str) {
let mut chars = keys.chars();
let first = match chars.next() {
Some(c) => c,
None => return,
};

if chars.clone().next().is_none() {
let mut cursor = tree.children.as_mut();
while let Some(child) = cursor {
if child.key == first {
child.name = name.to_string();
child.command = command.to_string();
return;
}
cursor = child.next.as_mut();
}

command_add_child(
tree,
Command::new(first, name.to_string(), command.to_string()),
);
return;
}

let mut exists = false;
{
let mut cursor = tree.children.as_mut();
while let Some(child) = cursor {
if child.key == first {
exists = true;
break;
}
cursor = child.next.as_mut();
}
}

if !exists {
command_add_child(
tree,
Command::new(first, DEFAULT_NAME.to_string(), String::new()),
);
}

let rest = &keys[first.len_utf8()..];
let mut cursor = tree.children.as_mut();
while let Some(child) = cursor {
if child.key == first {
tree_add_command(child, rest, name, command);
return;
}
cursor = child.next.as_mut();
}
}

pub fn read_line(c: &mut Command, file: &mut &[u8]) {
let key = read_field(file, "key");
let name = read_field(file, "name");
let command = read_until_eol(file);
tree_add_command(c, &key, &name, &command);
}

pub fn command_add_child(c: &mut Command, child: Command) {
fn insert_sorted(link: &mut Option<Box<Command>>, child: Command) {
match link {
None => {
*link = Some(Box::new(child));
}
Some(node) if node.key > child.key => {
let old = link.take();
let mut new_child = Box::new(child);
new_child.next = old;
*link = Some(new_child);
}
Some(node) => {
insert_sorted(&mut node.next, child);
}
}
}

insert_sorted(&mut c.children, child);
}

pub fn getch() -> char {
let mut buf = [0u8; 1];
match io::stdin().read_exact(&mut buf) {
Ok(_) => buf[0] as char,
Err(_) => '\0',
}
}

pub fn print_command(c: &Command) -> i32 {
let width: usize = 80;
let mut lines = 0;
let mut stderr = io::stderr();

if !c.name.is_empty() {
let _ = writeln!(stderr, "{}{}:{}", BLUE, c.name, COLOR_OFF);
lines += 1;
}

let mut max_line_width = 0usize;
let mut child = c.children.as_deref();
while let Some(node) = child {
let line_width = node.name.len();
if line_width > max_line_width {
max_line_width = line_width;
}
child = node.next.as_deref();
}

max_line_width += RIGHT_MARGIN;
if max_line_width > width {
max_line_width = width;
}

let denom = max_line_width + 5;
let items_per_row = if denom == 0 {
1
} else {
let v = width / denom;
if v == 0 { 1 } else { v }
};

child = c.children.as_deref();
let mut current_item = 0usize;
while let Some(node) = child {
current_item += 1;
if node.children.is_some() {
let _ = write!(
stderr,
"{}{}{} {}➔{} {}{:<width$}{}",
YELLOW,
node.key,
COLOR_OFF,
PURPLE,
COLOR_OFF,
BLUE,
node.name,
COLOR_OFF,
width = max_line_width
);
} else {
let _ = write!(
stderr,
"{}{}{} {}➔{}  {:<width$}",
YELLOW,
node.key,
COLOR_OFF,
PURPLE,
COLOR_OFF,
node.name,
width = max_line_width
);
}

if current_item % items_per_row == 0 {
let _ = writeln!(stderr);
lines += 1;
}

child = node.next.as_deref();
}

let _ = writeln!(stderr);
lines += 1;
lines
}

pub fn clear_lines(count: i32) {
let mut stderr = io::stderr();
let mut stdout = io::stdout();
let _ = stdout.flush();

for _ in 0..count {
let _ = write!(stderr, "\x1b[A\r\x1b[2K");
}
let _ = stderr.flush();
}

pub fn find_command(c: &Command, key: char) -> Option<&Command> {
let mut child = c.children.as_deref();
while let Some(node) = child {
if node.key == key {
return Some(node);
}
child = node.next.as_deref();
}
None
}

pub fn read_until_eol(file: &mut &[u8]) -> String {
let mut i = 0;
while i < file.len() && file[i] != b'\n' && file[i] != 0 {
i += 1;
}

let result = String::from_utf8_lossy(&file[..i]).into_owned();
if i < file.len() && file[i] == b'\n' {
*file = &file[i + 1..];
} else {
*file = &file[i..];
}
result
}

pub fn read_file(file: &str) -> String {
match fs::read_to_string(file) {
Ok(content) => content,
Err(e) => {
eprintln!("Failed to open file: {}", e);
std::process::exit(1);
}
}
}

pub fn command_run(c: &Command) -> i32 {
if !c.command.is_empty() {
print!("{}", c.command);
let _ = io::stdout().flush();
c.command.len() as i32
} else {
0
}
}
