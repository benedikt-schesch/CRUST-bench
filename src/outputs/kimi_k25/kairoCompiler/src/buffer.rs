//! Buffer module for handling input/output operations
pub struct Buffer {
content: String,
}

impl Buffer {
pub fn new() -> Self {
Self {
content: String::new(),
}
}

pub fn from_string(s: String) -> Self {
Self { content: s }
}

pub fn content(&self) -> &str {
&self.content
}

pub fn append(&mut self, s: &str) {
self.content.push_str(s);
}
}
