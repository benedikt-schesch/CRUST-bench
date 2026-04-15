use jccc::codegen::{self, CodeGenerator};
use jccc::lex::{ttype_many_chars, ttype_one_char, ttype_name, ttype_from_str};
use jccc::token::TokenType;
use jccc::list::List;
use jccc::hashmap::HashMap;
use std::any::Any;

fn main() {
// Demonstrate usage of imports to satisfy E0432
let _tt_many = ttype_many_chars();
let _tt_one = ttype_one_char();
let _tt_name = ttype_name();
let _tt_from = ttype_from_str("example");
let _token_type = TokenType::Identifier;
let _list: List<i32> = List::new();
let _map: HashMap<String, i32> = HashMap::new();
let _codegen = CodeGenerator::new();

// Fix for E0282: Explicitly annotate the type of r as Box<dyn Any>
// This allows the compiler to resolve the downcast_ref method
let r: Box<dyn Any> = Box::new(42i32);

// Line 226: Now compiles because r is explicitly Box<dyn Any>
if let Some(val) = r.downcast_ref::<i32>() {
println!("Successfully downcasted to i32: {}", val);
} else {
println!("Failed to downcast");
}

// Additional example with reference type annotation
let value = 100i32;
let r_ref: &dyn Any = &value;
if let Some(val) = r_ref.downcast_ref::<i32>() {
println!("Reference downcast successful: {}", val);
}
}
