use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq)]
pub struct PgnMetadata {
pub event: Option<String>,
pub site: Option<String>,
pub date: Option<String>,
pub round: Option<String>,
pub white: Option<String>,
pub black: Option<String>,
pub result: Option<String>,
inner: HashMap<String, String>,
}
impl PgnMetadata {
pub fn new() -> Self {
Self {
event: None,
site: None,
date: None,
round: None,
white: None,
black: None,
result: None,
inner: HashMap::new(),
}
}
pub fn get(&self, key: &str) -> Option<&str> {
match key {
"Event" => self.event.as_deref(),
"Site" => self.site.as_deref(),
"Date" => self.date.as_deref(),
"Round" => self.round.as_deref(),
"White" => self.white.as_deref(),
"Black" => self.black.as_deref(),
"Result" => self.result.as_deref(),
_ => self.inner.get(key).map(|s| s.as_str()),
}
}
pub fn insert(&mut self, key: String, value: String) {
match key.as_str() {
"Event" => self.event = Some(value),
"Site" => self.site = Some(value),
"Date" => self.date = Some(value),
"Round" => self.round = Some(value),
"White" => self.white = Some(value),
"Black" => self.black = Some(value),
"Result" => self.result = Some(value),
_ => { self.inner.insert(key, value); }
}
}
}
