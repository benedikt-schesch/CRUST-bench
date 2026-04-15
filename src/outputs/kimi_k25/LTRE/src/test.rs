// Fixed import: use crate::ltre instead of LTRE::ltre
// Binaries in src/bin/ should use crate:: to access the library
use crate::ltre::*;

fn main() {
println!("{}", example_function());
}
