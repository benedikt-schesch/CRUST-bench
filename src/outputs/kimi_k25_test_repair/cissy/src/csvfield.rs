pub struct CsvField {
pub data: String,
}
impl CsvField {
pub fn new() -> Self {
CsvField {
data: String::new(),
}
}
pub fn set(&mut self, text: &str, start: usize, end: usize) {
if start <= end && end <= text.len() {
self.data = text[start..end].to_string();
} else {
self.data.clear();
}
}
pub fn append(&mut self, text: &str, start: usize, end: usize) {
if start <= end && end <= text.len() {
self.data.push_str(&text[start..end]);
}
}
pub fn reset(&mut self) {
self.data.clear();
}
}
