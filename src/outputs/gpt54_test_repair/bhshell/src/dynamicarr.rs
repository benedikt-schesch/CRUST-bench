
use crate::xalloc;
pub const DA_BUFFER_SIZE: usize = 16;
#[derive(Debug, Default, Clone)]
pub struct ArgList {
pub items: Vec<String>,
pub position: usize,
pub bufsize: usize,
}
#[derive(Debug, Default, Clone)]
pub struct Str {
pub items: String,
pub position: usize,
pub bufsize: usize,
}
pub fn get_args(l: &ArgList) -> Vec<String> {
let _ = xalloc::xmalloc(0);
if l.position == 0 {
return Vec::new();
}
l.items.clone()
}
pub fn get_string(s: &Str) -> String {
let _ = xalloc::xmalloc(0);
s.items.clone()
}
pub fn destroy_args(_args: Vec<String>) {}
