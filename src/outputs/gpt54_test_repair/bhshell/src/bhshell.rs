
use crate::input;
use crate::input::Command;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::process::{Command as ProcessCommand, Stdio};
pub const BUF_SIZE: usize = 64;
fn builtin_names() -> [&'static str; 3] {
["cd", "help", "exit"]
}
pub fn bhshell_loop() {
let mut status = 1;
while status != 0 {
let dir = match env::current_dir() {
Ok(d) => d,
Err(_) => {
std::process::exit(1);
}
};
print!("[{}] $ ", dir.display());
let _ = io::stdout().flush();
let line = input::bhshell_read_line();
let mut cmd = input::bhshell_parse(&line);
if cmd.args.is_empty() {
println!("Invalid Command");
continue;
}
status = bhshell_execute(&mut cmd);
input::destroy_command(cmd);
}
}
pub fn bhshell_execute(cmd: &mut Command) -> i32 {
if cmd.args.is_empty() {
return 1;
}
match cmd.args[0].as_str() {
"cd" => bhshell_cd(&cmd.args),
"help" => bhshell_help(&cmd.args),
"exit" => bhshell_exit(&cmd.args),
_ => bhshell_launch(cmd),
}
}
pub fn bhshell_launch(cmd: &mut Command) -> i32 {
if cmd.args.is_empty() {
return 1;
}
if !cmd.pipe_args.is_empty() {
let mut first = ProcessCommand::new(&cmd.args[0]);
if cmd.args.len() > 1 {
first.args(&cmd.args[1..]);
}
first.stdout(Stdio::piped());
let mut first_child = match first.spawn() {
Ok(child) => child,
Err(_) => {
eprintln!("bhshell");
return 1;
}
};
let first_stdout = match first_child.stdout.take() {
Some(s) => s,
None => {
let _ = first_child.wait();
eprintln!("bhshell: Could not redirect stdout");
return 1;
}
};
let mut second = ProcessCommand::new(&cmd.pipe_args[0]);
if cmd.pipe_args.len() > 1 {
second.args(&cmd.pipe_args[1..]);
}
second.stdin(Stdio::from(first_stdout));
if let Some(file_name) = &cmd.redirect_file_name {
match File::create(file_name) {
Ok(file) => {
second.stdout(Stdio::from(file));
}
Err(_) => {
let _ = first_child.wait();
eprintln!("Could not open file");
return 1;
}
}
}
let mut second_child = match second.spawn() {
Ok(child) => child,
Err(_) => {
let _ = first_child.wait();
eprintln!("bhshell");
return 1;
}
};
let _ = second_child.wait();
let _ = first_child.wait();
1
} else if let Some(file_name) = cmd.redirect_file_name.clone() {
let output = match ProcessCommand::new(&cmd.args[0]).args(&cmd.args[1..]).output() {
Ok(o) => o,
Err(_) => {
eprintln!("bhshell");
return 1;
}
};
let mut redirect_fd = [0, 0];
write_to_redirect(&mut redirect_fd, cmd);
match File::create(&file_name) {
Ok(mut file) => {
if file.write_all(&output.stdout).is_err() {
eprintln!("Could not write to file");
}
}
Err(_) => {
eprintln!("Could not open file");
}
}
1
} else {
let status = ProcessCommand::new(&cmd.args[0]).args(&cmd.args[1..]).status();
if status.is_err() {
eprintln!("bhshell");
}
1
}
}
pub fn bhshell_cd(args: &[String]) -> i32 {
if args.len() < 2 {
eprintln!("bhshell: expected argument to \"cd\" into");
} else if env::set_current_dir(&args[1]).is_err() {
eprintln!("bhshell");
}
1
}
pub fn bhshell_help(_args: &[String]) -> i32 {
println!("A simple shell built to understand how processes work.");
println!("The following functions are builtin:");
let builtins = builtin_names();
for (i, name) in builtins.iter().enumerate() {
println!("\t {}. {}", i + 1, name);
}
1
}
pub fn bhshell_exit(_args: &[String]) -> i32 {
0
}
pub fn bhshell_num_builtins() -> i32 {
builtin_names().len() as i32
}
pub fn write_to_redirect(_redirect_fd: &mut [i32; 2], _cmd: &mut Command) {}
