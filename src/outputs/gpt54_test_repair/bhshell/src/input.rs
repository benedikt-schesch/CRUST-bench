
use crate::dynamicarr;
use crate::dynamicarr::{ArgList, Str};
#[derive(Debug, Default, Clone)]
pub struct Command {
pub args: Vec<String>,
pub pipe_args: Vec<String>,
pub redirect_file_name: Option<String>,
}
pub fn bhshell_read_line() -> String {
let mut buffer = String::new();
match std::io::stdin().read_line(&mut buffer) {
Ok(_) => {
while buffer.ends_with('\n') || buffer.ends_with('\r') {
buffer.pop();
}
let s = Str {
items: buffer,
position: 0,
bufsize: 0,
};
dynamicarr::get_string(&s)
}
Err(_) => String::new(),
}
}
pub fn bhshell_parse(line: &str) -> Command {
#[derive(PartialEq, Eq)]
enum ArgType {
Arg,
PipeArg,
Redirect,
}
let chars: Vec<char> = line.chars().collect();
let mut args = ArgList::default();
let mut pipe_args = ArgList::default();
let mut s = String::new();
let mut redirect: Option<String> = None;
let mut current = ArgType::Arg;
let mut cmd = new_command();
let mut i = 0usize;
while i < chars.len() {
let ch = chars[i];
if ch == '\n' || ch == '\t' || ch == ' ' {
if !s.is_empty() {
let token = s.clone();
s.clear();
if current == ArgType::Arg {
args.items.push(token);
args.position += 1;
} else if current == ArgType::PipeArg {
pipe_args.items.push(token);
pipe_args.position += 1;
} else {
redirect = Some(token);
}
}
i += 1;
continue;
} else if ch == '|' {
if !s.is_empty() {
let token = s.clone();
s.clear();
if current == ArgType::Arg {
args.items.push(token);
args.position += 1;
} else {
return Command::default();
}
}
current = ArgType::PipeArg;
} else if ch == '>' {
if !s.is_empty() {
let token = s.clone();
s.clear();
if current == ArgType::Arg {
args.items.push(token);
args.position += 1;
} else if current == ArgType::PipeArg {
pipe_args.items.push(token);
pipe_args.position += 1;
}
}
current = ArgType::Redirect;
} else {
s.push(ch);
}
i += 1;
}
if s.is_empty() {
return Command::default();
}
let token = s;
if current == ArgType::Arg {
args.items.push(token);
args.position += 1;
} else if current == ArgType::PipeArg {
pipe_args.items.push(token);
pipe_args.position += 1;
} else {
redirect = Some(token);
}
if args.position == 0 {
return Command::default();
}
cmd.args = dynamicarr::get_args(&args);
if pipe_args.position > 0 {
cmd.pipe_args = dynamicarr::get_args(&pipe_args);
}
cmd.redirect_file_name = redirect;
cmd
}
pub fn destroy_command(cmd: Command) {
dynamicarr::destroy_args(cmd.args);
dynamicarr::destroy_args(cmd.pipe_args);
drop(cmd.redirect_file_name);
}
pub fn new_command() -> Command {
Command {
args: Vec::new(),
pipe_args: Vec::new(),
redirect_file_name: None,
}
}
