use jccc::codegen::{self, CodeGenerator};
use jccc::lex::{ttype_many_chars, ttype_one_char, ttype_name, ttype_from_str};
use jccc::token::TokenType;
use jccc::list::List;
use jccc::hashmap::HashMap;
use std::any::Any;
fn main() {
let _tt_many = ttype_many_chars();
let _tt_one = ttype_one_char();
let _tt_name = ttype_name();
let _tt_from = ttype_from_str("example");
let _token_type = TokenType::Identifier;
let _list: List<i32> = List::new();
let _map: HashMap<String, i32> = HashMap::new();
let _codegen = CodeGenerator::new();
let r: Box<dyn Any> = Box::new(42i32);
if let Some(val) = r.downcast_ref::<i32>() {
println!("Successfully downcasted to i32: {}", val);
} else {
println!("Failed to downcast");
}
let value = 100i32;
let r_ref: &dyn Any = &value;
if let Some(val) = r_ref.downcast_ref::<i32>() {
println!("Reference downcast successful: {}", val);
}
}
