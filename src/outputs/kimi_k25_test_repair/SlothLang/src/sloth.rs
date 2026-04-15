use crate::{parser, throw, slothvm, stack};
fn main() {
let args: Vec<String> = std::env::args().collect();
if args.len() != 2 {
eprintln!("Usage: sloth <source file>");
std::process::exit(1);
}
let filename = &args[1];
let program = parser::parse(filename);
let mut program_opt = program;
let result = slothvm::execute(&mut program_opt);
println!("Returned: {}", result);
parser::free_program(program_opt);
}
