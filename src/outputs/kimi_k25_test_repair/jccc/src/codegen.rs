use crate::cst::CST;
use crate::token::TokenType;
use crate::hashmap::HashMap;
use crate::list::List;
pub struct CodeGenerator;
impl CodeGenerator {
pub fn new() -> Self {
CodeGenerator
}
pub fn generate(&mut self, _cst: &CST) -> String {
String::new()
}
}
