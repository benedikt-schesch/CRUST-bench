use crate::dynamicarr::{ArgList, Str, get_string, get_args};

/// Represents a command, including its arguments, pipe-arguments,
/// and redirection information, in a safe Rust form.
#[derive(Debug, Default)]
pub struct Command {
pub args: Vec<String>,
pub pipe_args: Vec<String>,
pub redirect_file_name: Option<String>,
}

/// Reads a line of input from the user.
/// In C, this was returning a 'char*'.
pub fn bhshell_read_line() -> String {
let mut line = String::new();
std::io::stdin().read_line(&mut line).expect("Failed to read line");
// Remove trailing newline to match C behavior
if line.ends_with('\n') {
line.pop();
}
line
}

/// Parses a given line into a Command structure.
/// In C, this was returning a 'command*' for the allocated command.
pub fn bhshell_parse(line: &str) -> Command {
let mut cmd = new_command();
let mut arg_list = ArgList::default();
let mut pipe_list = ArgList::default();
let mut current_str = Str::default();
let mut state = 0; // 0 = ARG, 1 = PIPE_ARG, 2 = REDIRECT
let mut has_error = false;

let chars: Vec<char> = line.chars().collect();
let mut i = 0;

while i < chars.len() {
let c = chars[i];

if c == ' ' || c == '\t' || c == '\n' {
if current_str.position > 0 {
let string = get_string(&current_str);
match state {
0 => arg_list.items.push(string),
1 => pipe_list.items.push(string),
2 => cmd.redirect_file_name = Some(string),
_ => {}
}
current_str = Str::default();
}
i += 1;
continue;
} else if c == '|' {
if current_str.position > 0 {
if state == 0 {
arg_list.items.push(get_string(&current_str));
current_str = Str::default();
} else {
has_error = true;
break;
}
}
// Check for invalid |> sequence
if i + 1 < chars.len() && chars[i + 1] == '>' {
has_error = true;
break;
}
state = 1;
i += 1;
continue;
} else if c == '>' {
if current_str.position > 0 {
let string = get_string(&current_str);
match state {
0 => arg_list.items.push(string),
1 => pipe_list.items.push(string),
_ => {}
}
current_str = Str::default();
}
state = 2;
i += 1;
continue;
} else {
current_str.items.push(c);
current_str.position += 1;
i += 1;
}
}

// Handle last token
if current_str.position > 0 {
let string = get_string(&current_str);
match state {
0 => arg_list.items.push(string),
1 => pipe_list.items.push(string),
2 => cmd.redirect_file_name = Some(string),
_ => {}
}
}

// Check for errors or empty args
if has_error || arg_list.items.is_empty() {
return new_command();
}

cmd.args = get_args(&arg_list);
if !pipe_list.items.is_empty() {
cmd.pipe_args = get_args(&pipe_list);
}

cmd
}

/// Cleans up and destroys a Command structure.
/// In C, this took 'command* cmd' and freed its data.
pub fn destroy_command(_cmd: Command) {
// Command and its Vec fields are dropped automatically
}

/// Creates and returns a new Command structure.
/// In C, this was returning a 'command*'.
pub fn new_command() -> Command {
Command::default()
}
