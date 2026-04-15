use crate::csvfield::CsvField;
pub struct CsvLine {
pub field: Vec<CsvField>,
}
impl CsvLine {
pub fn new() -> Self {
CsvLine {
field: Vec::new(),
}
}
pub fn add_field(&mut self, text: &str, start: usize, end: usize) {
let mut new_field = CsvField::new();
new_field.set(text, start, end);
self.field.push(new_field);
}
pub fn get_field_count(&self) -> usize {
self.field.len()
}
pub fn get_field(&self, index: usize) -> Option<&str> {
self.field.get(index).map(|f| f.data.as_str())
}
pub fn reset(&mut self) {
self.field.clear();
}
}
