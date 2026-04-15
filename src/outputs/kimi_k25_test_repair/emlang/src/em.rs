use crate::data::Instruction;

#[derive(Debug)]
pub struct Program {
pub instructions: Vec<Instruction>,
}

impl Program {
pub fn new() -> Self {
Program {
instructions: Vec::new(),
}
}
}
