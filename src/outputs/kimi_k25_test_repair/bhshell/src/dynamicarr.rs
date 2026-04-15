use crate::xalloc;
pub const DA_BUFFER_SIZE: usize = 16;
#[derive(Debug, Default)]
pub struct ArgList {
pub items: Vec<String>,
pub position: usize,
pub bufsize: usize,
}
#[derive(Debug, Default)]
pub struct Str {
pub items: String,
pub position: usize,
pub bufsize: usize,
}
pub fn get_args(l: &ArgList) -> Vec<String> {
l.items.clone()
}
pub fn get_string(s: &Str) -> String {
s.items.clone()
}
pub fn destroy_args(_args: Vec<String>) {
}
