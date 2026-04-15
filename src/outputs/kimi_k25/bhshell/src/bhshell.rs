use crate::input::Command;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::process::{Command as ProcessCommand, Stdio};

pub const BUF_SIZE: usize = 64;

/// Runs the main bhshell loop.
pub fn bhshell_loop() {
loop {
let dir = env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("?"));
print!("[{}] $ ", dir.display());
io::stdout().flush().unwrap();

let line = crate::input::bhshell_read_line();
let mut cmd = crate::input::bhshell_parse(&line);

// Check for invalid command (empty args)
if cmd.args.is_empty() {
println!("Invalid Command");
continue;
}

let status = bhshell_execute(&mut cmd);
crate::input::destroy_command(cmd);

if status == 0 {
break;
}
}
}

/// Executes the given command.
/// Returns an integer status code (C equivalent of int).
pub fn bhshell_execute(cmd: &mut Command) -> i32 {
if cmd.args.is_empty() {
return 1;
}

let builtin_str = ["cd", "help", "exit"];

if let Some(first_arg) = cmd.args.get(0) {
for (i, builtin) in builtin_str.iter().enumerate() {
if first_arg == *builtin {
return match i {
0 => bhshell_cd(&cmd.args),
1 => bhshell_help(&cmd.args),
2 => bhshell_exit(&cmd.args),
_ => 1,
};
}
}
}

bhshell_launch(cmd)
}

/// Launches the given command.
/// Returns an integer status code (C equivalent of int).
pub fn bhshell_launch(cmd: &mut Command) -> i32 {
// Handle piped command
if !cmd.pipe_args.is_empty() {
let mut first_process = ProcessCommand::new(&cmd.args[0]);
first_process.args(&cmd.args[1..]).stdout(Stdio::piped());

let mut first_child = match first_process.spawn() {
Ok(child) => child,
Err(e) => {
eprintln!("bhshell: {}", e);
return 1;
}
};

let stdout = first_child.stdout.take().expect("Failed to capture stdout");

let mut second_process = ProcessCommand::new(&cmd.pipe_args[0]);
second_process.args(&cmd.pipe_args[1..]).stdin(stdout);

// Handle redirection for the second command
if let Some(ref filename) = cmd.redirect_file_name {
match File::create(filename) {
Ok(file) => {
second_process.stdout(file);
}
Err(e) => {
eprintln!("Could not open file: {}", e);
return 1;
}
}
}

let mut second_child = match second_process.spawn() {
Ok(child) => child,
Err(e) => {
eprintln!("bhshell: {}", e);
return 1;
}
};

let _ = first_child.wait();
let _ = second_child.wait();

} else if let Some(ref filename) = cmd.redirect_file_name {
// Handle redirection only
let file = match File::create(filename) {
Ok(f) => f,
Err(e) => {
eprintln!("Could not open file: {}", e);
return 1;
}
};

let mut process = ProcessCommand::new(&cmd.args[0]);
process.args(&cmd.args[1..]).stdout(file);

match process.spawn() {
Ok(mut child) => {
let _ = child.wait();
}
Err(e) => {
eprintln!("bhshell: {}", e);
}
}

} else {
// Simple command execution
let mut process = ProcessCommand::new(&cmd.args[0]);
process.args(&cmd.args[1..]);

match process.spawn() {
Ok(mut child) => {
let _ = child.wait();
}
Err(e) => {
eprintln!("bhshell: {}", e);
}
}
}

1
}

/// Changes the current directory.
/// Returns an integer status code (C equivalent of int).
pub fn bhshell_cd(args: &[String]) -> i32 {
if args.len() < 2 {
eprintln!("bhshell: expected argument to \"cd\" into");
} else {
if let Err(e) = env::set_current_dir(&args[1]) {
eprintln!("bhshell: {}", e);
}
}
1
}

/// Prints help information.
/// Returns an integer status code (C equivalent of int).
pub fn bhshell_help(_args: &[String]) -> i32 {
println!("A simple shell built to understand how processes work.");
println!("The following functions are builtin:");
let builtin_str = ["cd", "help", "exit"];
for (i, builtin) in builtin_str.iter().enumerate() {
println!("\t {}. {}", i + 1, builtin);
}
1
}

/// Handles exit request.
/// Returns an integer status code (C equivalent of int).
pub fn bhshell_exit(_args: &[String]) -> i32 {
0
}

/// Returns the number of built-in commands.
pub fn bhshell_num_builtins() -> i32 {
3
}

/// Writes to a redirected file descriptor array.
/// In C, this took an array 'int redirect_fd[2]' and a pointer to 'command'.
/// In this safe Rust implementation, redirection is handled directly in bhshell_launch
/// using std::process::Command and std::fs::File, so this manual write is not needed.
pub fn write_to_redirect(_redirect_fd: &mut [i32; 2], _cmd: &mut Command) {
// This function is kept for interface compatibility.
// The functionality is implemented via std::process::Command stdout redirection.
}
