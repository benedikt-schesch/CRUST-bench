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
pub fn release(&mut self) {}
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
self.depth = 0;
self.state = ParseState::State1;
loop {
let token_opt = self.get_next_token();
let token = match token_opt {
Some(t) => t,
None => break,
};
if token.token_type == XMLTokenType::Text && token.data.is_none() {
continue;
}
let next_state = XMLParser::translate(self.state, &token.token_type);
if next_state != ParseState::StateError {
match self.state {
ParseState::State1 => {}
ParseState::State2 => {
if token.token_type == XMLTokenType::Text {
if let Some(data) = token.data.clone() {
self.tag_stack.push_back(data);
self.depth += 1;
}
}
}
ParseState::State3 => {}
ParseState::State4 => {
if let Some(data) = token.data.clone() {
self.value_stack.push_back(data);
}
}
ParseState::State5 => {}
ParseState::State6 => {
if token.token_type == XMLTokenType::Text {
if let Some(data) = token.data.as_ref() {
let top = self.tag_stack.top_back().cloned().unwrap_or_default();
assert!(data == &top);
}
}
}
ParseState::State7 => {
if token.token_type == XMLTokenType::EndTag {
let current_tag =
self.tag_stack.top_back().cloned().unwrap_or_default();
let current_value =
self.value_stack.top_back().cloned().unwrap_or_default();
let length = self.element_stack.size();
self.depth -= 1;
let mut current = XMLElement::new(current_tag, current_value);
let se_depth = self.depth;
for _ in 0..length {
let should_break = match self.element_stack.top_back() {
Some(elem) => elem.depth <= se_depth,
None => true,
};
if should_break {
break;
}
let child_se = self.element_stack.pop_back().unwrap();
current.children.push_front(child_se.element);
}
let se = StackElement::new(current, se_depth);
self.element_stack.push_back(se);
self.tag_stack.pop_back();
self.value_stack.pop_back();
}
}
ParseState::State8 => {}
ParseState::StateError => {}
}
}
if next_state == ParseState::StateError {
return Err("error while parsing".to_string());
}
self.state = next_state;
}
let stack_elem = match self.element_stack.pop_back() {
Some(se) => se,
None => {
self.release();
return Err("no root element parsed".to_string());
}
};
let xml_elem = stack_elem.element;
self.release();
Ok(xml_elem)
}
fn get_next_token(&mut self) -> Option<XMLToken> {
let chars: Vec<char> = self.input.chars().collect();
let length = chars.len();
let begin_pos = self.position;
if begin_pos >= length {
return None;
}
while self.position < length {
let ch = chars[self.position];
self.position += 1;
match ch {
BEGIN_TAG_TOKEN => {
if self.position > begin_pos + 1 {
self.position -= 1;
return Some(self.get_text_token(begin_pos, self.position - 1));
} else {
let next_char = chars.get(self.position).copied().unwrap_or('\0');
if next_char == SPLASH_TOKEN {
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
let chars: Vec<char> = self.input.chars().collect();
while from < chars.len() && chars[from] == ' ' {
from += 1;
}
while to < chars.len() && to >= from && chars[to] == ' ' {
if to == 0 {
break;
}
to -= 1;
}
if to >= from && from < chars.len() && to < chars.len() {
let data: String = chars[from..=to].iter().collect();
XMLToken {
token_type: XMLTokenType::Text,
data: Some(data),
}
} else {
XMLToken {
token_type: XMLTokenType::Text,
data: None,
}
}
}
fn translate(state: ParseState, token: &XMLTokenType) -> ParseState {
match state {
ParseState::State1 => match token {
XMLTokenType::BeginOpenTag => ParseState::State2,
_ => ParseState::StateError,
},
ParseState::State2 => match token {
XMLTokenType::Text => ParseState::State3,
_ => ParseState::StateError,
},
ParseState::State3 => match token {
XMLTokenType::EndTag => ParseState::State4,
_ => ParseState::StateError,
},
ParseState::State4 => match token {
XMLTokenType::BeginOpenTag => ParseState::State2,
XMLTokenType::Text => ParseState::State5,
_ => ParseState::StateError,
},
ParseState::State5 => match token {
XMLTokenType::BeginCloseTag => ParseState::State6,
_ => ParseState::StateError,
},
ParseState::State6 => match token {
XMLTokenType::Text => ParseState::State7,
_ => ParseState::StateError,
},
ParseState::State7 => match token {
XMLTokenType::EndTag => ParseState::State8,
_ => ParseState::StateError,
},
ParseState::State8 => match token {
XMLTokenType::BeginOpenTag => ParseState::State2,
XMLTokenType::BeginCloseTag => ParseState::State6,
_ => ParseState::StateError,
},
ParseState::StateError => ParseState::StateError,
}
}
fn release(&mut self) {
self.element_stack.release();
self.value_stack.release();
self.tag_stack.release();
self.input.clear();
self.position = 0;
self.depth = 0;
self.state = ParseState::State1;
}
}
pub fn parse_xml_from_text(text: &str) -> Result<XMLElement, String> {
let mut parser = XMLParser::new();
parser.parse(text)
}
