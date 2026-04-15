// Generated Rust Code
use crate::xalloc;

pub const DA_BUFFER_SIZE: usize = 16;

/// Represents a dynamic list of strings in an idiomatic, safe Rust form.
#[derive(Debug, Default, Clone)]
pub struct ArgList {
pub items: Vec<String>,
pub position: usize,
pub bufsize: usize,
}

/// Represents a dynamic string in an idiomatic, safe Rust form.
#[derive(Debug, Default, Clone)]
pub struct Str {
pub items: String,
pub position: usize,
pub bufsize: usize,
}

/// Returns a new vector of strings derived from the given ArgList.
/// In C, this was returning a 'char**'.
pub fn get_args(l: &ArgList) -> Vec<String> {
let _ = xalloc::xmalloc(0);
if l.position == 0 {
return Vec::new();
}
l.items.clone()
}

/// Returns a new String derived from the given Str.
/// In C, this was returning a 'char*'.
pub fn get_string(s: &Str) -> String {
let _ = xalloc::xmalloc(0);
s.items.clone()
}

/// Destroys / frees the given vector of strings.
/// In C, this was taking 'char** args'.
pub fn destroy_args(_args: Vec<String>) {}
