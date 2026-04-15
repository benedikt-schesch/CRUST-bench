use crate::{parser, slothvm, stack, throw};

fn main() {
let mut args = std::env::args();
let _program_name = args.next();
let file_name = match args.next() {
Some(v) if args.next().is_none() => v,
_ => {
eprintln!("Usage: sloth <source file>");
return;
}
};

let mut program = parser::parse(&file_name);
let x = slothvm::execute(&mut program);
println!("Returned: {}", x);
parser::free_program(program);
}
