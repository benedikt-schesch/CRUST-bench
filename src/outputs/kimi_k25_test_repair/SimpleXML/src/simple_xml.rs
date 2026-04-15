use crate::simple_vector::Vector;
const BEGIN_TAG_TOKEN: char = '<';
const END_TAG_TOKEN: char = '>';
const SPLASH_TOKEN: char = '/';
pub struct XMLElement {
pub tag_name: String,
pub value: String,
pub parent: (),
pub children: Vector<XMLElement>,
}
#[derive(Debug, Clone, PartialEq)]
pub enum XMLTokenType {
BeginOpenTag,
BeginCloseTag,
EndTag,
Text,
}
pub struct XMLToken {
pub token_type: XMLTokenType,
pub data: Option<String>,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParseState {
State1,
State2,
State3,
State4,
State5,
State6,
State7,
State8,
StateError,
}
pub struct StackElement {
element: XMLElement,
depth: usize,
}
impl StackElement {
pub fn new(element: XMLElement, depth: usize) -> Self {
StackElement { element, depth }
}
pub fn release(&mut self) {
}
}
impl XMLElement {
pub fn new(tag_name: String, value: String) -> XMLElement {
XMLElement {
tag_name,
value,
parent: (),
children: Vector::new(8),
}
}
}
pub struct XMLParser {
input: String,
position: usize,
depth: usize,
state: ParseState,
tag_stack: Vector<String>,
value_stack: Vector<String>,
element_stack: Vector<StackElement>,
}
impl XMLParser {
pub fn new() -> Self {
XMLParser {
input: String::new(),
position: 0,
depth: 0,
state: ParseState::State1,
tag_stack: Vector::new(8),
value_stack: Vector::new(8),
element_stack: Vector::new(8),
}
}
pub fn parse(&mut self, text: &str) -> Result<XMLElement, String> {
self.input = text.to_string();
self.position = 0;
self.state = ParseState::State1;
self.depth = 0;
loop {
let token = match self.get_next_token() {
Some(t) => t,
None => break,
};
if token.token_type == XMLTokenType::Text && token.data.is_none() {
continue;
}
let new_state = Self::translate(self.state, &token.token_type);
if new_state == ParseState::StateError {
return Err("Parse error: invalid state transition".to_string());
}
match self.state {
ParseState::State1 => {}
ParseState::State2 => {
if let XMLTokenType::Text = token.token_type {
if let Some(data) = token.data {
self.tag_stack.push_back(data);
self.depth += 1;
}
}
}
ParseState::State3 => {}
ParseState::State4 => {
if let Some(data) = token.data {
self.value_stack.push_back(data);
}
}
ParseState::State5 => {}
ParseState::State6 => {
if let XMLTokenType::Text = token.token_type {
if let Some(data) = token.data {
let top_tag = self.tag_stack.top_back()
.ok_or("Tag stack empty in state 6")?;
if *top_tag != data {
return Err(format!("Mismatched tags: expected {}, found {}", top_tag, data));
}
}
}
}
ParseState::State7 => {
if let XMLTokenType::EndTag = token.token_type {
let current_tag = self.tag_stack.top_back()
.ok_or("Tag stack empty")?.clone();
let current_value = self.value_stack.pop_back()
.unwrap_or_default();
self.depth -= 1;
let current_depth = self.depth;
let mut current_elem = XMLElement::new(current_tag, current_value);
loop {
match self.element_stack.top_back() {
Some(elem) if elem.depth > current_depth => {
let elem = self.element_stack.pop_back().unwrap();
current_elem.children.push_front(elem.element);
}
_ => break,
}
}
self.element_stack.push_back(StackElement::new(current_elem, current_depth));
self.tag_stack.pop_back();
}
}
ParseState::State8 => {}
_ => {}
}
self.state = new_state;
}
if self.state != ParseState::State8 && self.state != ParseState::State1 {
return Err("Unexpected end of input".to_string());
}
if let Some(stack_elem) = self.element_stack.pop_back() {
Ok(stack_elem.element)
} else {
Err("No root element found".to_string())
}
}
fn get_next_token(&mut self) -> Option<XMLToken> {
let length = self.input.len();
if self.position >= length {
return None;
}
let begin_pos = self.position;
while self.position < length {
let ch = self.input.as_bytes()[self.position] as char;
self.position += 1;
match ch {
BEGIN_TAG_TOKEN => {
if self.position > begin_pos + 1 {
self.position -= 1;
return Some(self.get_text_token(begin_pos, self.position - 1));
} else {
if self.position < length && self.input.as_bytes()[self.position] as char == SPLASH_TOKEN {
self.position += 1;
return Some(XMLToken {
token_type: XMLTokenType::BeginCloseTag,
data: None,
});
} else {
return Some(XMLToken {
token_type: XMLTokenType::BeginOpenTag,
data: None,
});
}
}
}
END_TAG_TOKEN => {
if self.position > begin_pos + 1 {
self.position -= 1;
return Some(self.get_text_token(begin_pos, self.position - 1));
} else {
return Some(XMLToken {
token_type: XMLTokenType::EndTag,
data: None,
});
}
}
_ => {}
}
}
Some(self.get_text_token(begin_pos, self.position - 1))
}
fn get_text_token(&self, mut from: usize, mut to: usize) -> XMLToken {
let bytes = self.input.as_bytes();
while from <= to && from < bytes.len() && bytes[from] == b' ' {
from += 1;
}
while from <= to && to < bytes.len() && bytes[to] == b' ' {
if to == 0 { break; }
to -= 1;
}
if from <= to && from < bytes.len() && to < bytes.len() {
let text = self.input[from..=to].to_string();
XMLToken {
token_type: XMLTokenType::Text,
data: Some(text),
}
} else {
XMLToken {
token_type: XMLTokenType::Text,
data: None,
}
}
}
fn translate(state: ParseState, token: &XMLTokenType) -> ParseState {
match (state, token) {
(ParseState::State1, XMLTokenType::BeginOpenTag) => ParseState::State2,
(ParseState::State2, XMLTokenType::Text) => ParseState::State3,
(ParseState::State3, XMLTokenType::EndTag) => ParseState::State4,
(ParseState::State4, XMLTokenType::BeginOpenTag) => ParseState::State2,
(ParseState::State4, XMLTokenType::Text) => ParseState::State5,
(ParseState::State5, XMLTokenType::BeginCloseTag) => ParseState::State6,
(ParseState::State6, XMLTokenType::Text) => ParseState::State7,
(ParseState::State7, XMLTokenType::EndTag) => ParseState::State8,
(ParseState::State8, XMLTokenType::BeginOpenTag) => ParseState::State2,
(ParseState::State8, XMLTokenType::BeginCloseTag) => ParseState::State6,
_ => ParseState::StateError,
}
}
fn release(&mut self) {
self.tag_stack.release();
self.value_stack.release();
self.element_stack.release();
}
}
pub fn parse_xml_from_text(text: &str) -> Result<XMLElement, String> {
let mut parser = XMLParser::new();
parser.parse(text)
}
