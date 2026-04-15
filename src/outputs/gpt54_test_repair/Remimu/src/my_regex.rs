pub const REMIMU_FLAG_DOT_NO_NEWLINES: i32 = 1;
pub const REMIMU_KIND_NORMAL: u8 = 0;
pub const REMIMU_KIND_OPEN: u8 = 1;
pub const REMIMU_KIND_NCOPEN: u8 = 2;
pub const REMIMU_KIND_CLOSE: u8 = 3;
pub const REMIMU_KIND_OR: u8 = 4;
pub const REMIMU_KIND_CARET: u8 = 5;
pub const REMIMU_KIND_DOLLAR: u8 = 6;
pub const REMIMU_KIND_BOUND: u8 = 7;
pub const REMIMU_KIND_NBOUND: u8 = 8;
pub const REMIMU_KIND_END: u8 = 9;
pub const REMIMU_MODE_POSSESSIVE: u8 = 1;
pub const REMIMU_MODE_LAZY: u8 = 2;
pub const REMIMU_MODE_INVERTED: u8 = 128;
#[derive(Debug, Clone, Copy)]
pub struct RegexToken {
pub kind: u8,
pub mode: u8,
pub count_lo: u16,
pub count_hi: u16,
pub mask: [u16; 16],
pub pair_offset: i16,
}
impl RegexToken {
pub fn new(kind: u8, mode: u8) -> Self {
Self {
kind,
mode,
count_lo: 1,
count_hi: 2,
mask: [0; 16],
pair_offset: 0,
}
}
pub fn set_mask(&mut self, byte: u8) {
self.mask[(byte >> 4) as usize] |= 1u16 << (byte & 0xF);
}
pub fn invert_mask(&mut self) {
for m in &mut self.mask {
*m = !*m;
}
self.mode &= !REMIMU_MODE_INVERTED;
}
pub fn check_mask(&self, byte: u8) -> bool {
(self.mask[(byte >> 4) as usize] & (1u16 << (byte & 0xF))) != 0
}
pub fn push_to_vec(&mut self, tokens: &mut Vec<RegexToken>, max_len: usize) -> Result<(), i32> {
if tokens.is_empty()
|| tokens[tokens.len() - 1].kind != self.kind
|| (self.kind != REMIMU_KIND_BOUND && self.kind != REMIMU_KIND_NBOUND)
{
if (self.mode & REMIMU_MODE_INVERTED) != 0 {
self.invert_mask();
}
if tokens.len() >= max_len {
println!("buffer overflow");
return Err(-2);
}
tokens.push(*self);
*self = RegexToken::default();
}
Ok(())
}
}
impl Default for RegexToken {
fn default() -> Self {
Self::new(REMIMU_KIND_NORMAL, 0)
}
}
#[derive(Debug, Clone, Copy)]
pub struct RegexMatcherState {
pub k: u32,
pub group_state: u32,
pub prev: u32,
pub i: u64,
pub range_min: u64,
pub range_max: u64,
}
impl RegexMatcherState {
pub fn new(k: u32, i: u64) -> Self {
Self {
k,
group_state: 0,
prev: 0,
i,
range_min: 0,
range_max: 0,
}
}
}
fn parse_hex_byte(bytes: &[u8], i: &mut usize) -> Result<u8, i32> {
if *i + 2 >= bytes.len() {
return Err(-1);
}
let mut n0 = bytes[*i + 1];
let mut n1 = bytes[*i + 1];
if !(b'0'..=b'f').contains(&n0)
|| !(b'0'..=b'f').contains(&n1)
|| (n0 > b'9' && n0 < b'A')
|| (n1 > b'9' && n1 < b'A')
{
return Err(-1);
}
if n0 > b'F' {
n0 -= 0x20;
}
if n1 > b'F' {
n1 -= 0x20;
}
if n0 >= b'A' {
n0 -= b'A' - 10;
}
if n1 >= b'A' {
n1 -= b'A' - 10;
}
n0 -= b'0';
n1 -= b'0';
*i += 2;
Ok((n1 << 4) | n0)
}
fn fill_shorthand_mask(c: u8) -> [u16; 16] {
let mut m = [0u16; 16];
if c == b'd' || c == b'w' {
m[3] |= 0x03FF;
}
if c == b's' {
m[0] |= 0x3E00;
m[2] |= 1;
}
if c == b'w' {
m[4] |= 0xFFFE;
m[5] |= 0x87FF;
m[6] |= 0xFFFE;
m[7] |= 0x07FF;
}
m
}
pub fn regex_parse(
pattern: &str,
tokens: &mut Vec<RegexToken>,
token_count: &mut i16,
flags: i32,
) -> Result<(), i32> {
let tokens_len = *token_count as usize;
if *token_count == 0 {
return Err(-2);
}
tokens.clear();
let bytes = pattern.as_bytes();
let pattern_len = bytes.len();
let mut esc_state = 0i32;
let mut state = State::Normal;
let mut char_class_mem: i32 = -1;
let mut token = RegexToken::default();
let mut k: i16 = 0;
token.kind = REMIMU_KIND_OPEN;
token.count_lo = 0;
token.count_hi = 0;
let mut paren_count = 0i32;
let mut i = 0usize;
while i < pattern_len {
let c = bytes[i];
if matches!(state, State::Quant) {
state = State::Mode;
if c == b'?' {
token.count_lo = 0;
token.count_hi = 2;
i += 1;
continue;
} else if c == b'+' {
token.count_lo = 1;
token.count_hi = 0;
i += 1;
continue;
} else if c == b'*' {
token.count_lo = 0;
token.count_hi = 0;
i += 1;
continue;
} else if c == b'{' {
if i + 1 >= pattern_len || !(bytes[i + 1].is_ascii_digit()) {
state = State::Normal;
} else {
i += 1;
let mut val: u32 = 0;
while i < pattern_len && bytes[i].is_ascii_digit() {
val = val
.saturating_mul(10)
.saturating_add((bytes[i] - b'0') as u32);
if val > 0xFFFF {
println!("quantifier range too long");
return Err(-1);
}
i += 1;
}
token.count_lo = val as u16;
token.count_hi = val as u16 + 1;
if i < pattern_len && bytes[i] == b',' {
token.count_hi = 0;
i += 1;
if i < pattern_len && bytes[i].is_ascii_digit() {
let mut val2: u32 = 0;
while i < pattern_len && bytes[i].is_ascii_digit() {
val2 = val2
.saturating_mul(10)
.saturating_add((bytes[i] - b'0') as u32);
if val2 > 0xFFFF {
println!("quantifier range too long");
return Err(-1);
}
i += 1;
}
if val2 < val {
println!("quantifier range is backwards");
return Err(-1);
}
token.count_hi = val2 as u16 + 1;
}
}
if i < pattern_len && bytes[i] == b'}' {
i += 1;
continue;
} else {
println!("quantifier range syntax broken (no terminator)");
return Err(-1);
}
}
}
}
if matches!(state, State::Mode) {
state = State::Normal;
if c == b'?' {
token.mode |= REMIMU_MODE_LAZY;
i += 1;
continue;
} else if c == b'+' {
token.mode |= REMIMU_MODE_POSSESSIVE;
i += 1;
continue;
}
}
match state {
State::Normal => {
if esc_state == 1 {
esc_state = 0;
if c == b'n' {
token.set_mask(b'\n');
} else if c == b'r' {
token.set_mask(b'\r');
} else if c == b't' {
token.set_mask(b'\t');
} else if c == 0x0b {
token.set_mask(0x0b);
} else if c == b'f' {
token.set_mask(0x0c);
} else if c == b'x' {
let b = parse_hex_byte(bytes, &mut i)?;
token.set_mask(b);
} else if matches!(
c,
b'{' | b'}'
| b'['
| b']'
| b'-'
| b'('
| b')'
| b'|'
| b'^'
| b'$'
| b'*'
| b'+'
| b'?'
| b':'
| b'.'
| b'/'
| b'\\'
) {
token.set_mask(c);
state = State::Quant;
i += 1;
continue;
} else if matches!(c, b'd' | b's' | b'w' | b'D' | b'S' | b'W') {
let is_upper = c.is_ascii_uppercase();
let mut c2 = c;
if is_upper {
c2 += 0x20;
}
let m = fill_shorthand_mask(c2);
for (j, mj) in m.iter().enumerate() {
token.mask[j] |= if is_upper { !*mj } else { *mj };
}
token.kind = REMIMU_KIND_NORMAL;
state = State::Quant;
i += 1;
continue;
} else if c == b'b' {
token.kind = REMIMU_KIND_BOUND;
state = State::Normal;
i += 1;
continue;
} else if c == b'B' {
token.kind = REMIMU_KIND_NBOUND;
state = State::Normal;
i += 1;
continue;
} else {
println!("unsupported escape sequence");
return Err(-1);
}
i += 1;
continue;
} else {
token.push_to_vec(tokens, tokens_len)?;
k = tokens.len() as i16;
if c == b'\\' {
esc_state = 1;
} else if c == b'[' {
state = State::CharClassInit;
char_class_mem = -1;
token.kind = REMIMU_KIND_NORMAL;
if i + 1 < pattern_len && bytes[i + 1] == b'^' {
token.mode |= REMIMU_MODE_INVERTED;
i += 1;
}
} else if c == b'(' {
paren_count += 1;
state = State::Normal;
token.kind = REMIMU_KIND_OPEN;
token.count_lo = 0;
token.count_hi = 1;
if i + 2 < pattern_len && bytes[i + 1] == b'?' && bytes[i + 2] == b':' {
token.kind = REMIMU_KIND_NCOPEN;
i += 2;
} else if i + 2 < pattern_len
&& bytes[i + 1] == b'?'
&& bytes[i + 2] == b'>'
{
token.kind = REMIMU_KIND_NCOPEN;
token.push_to_vec(tokens, tokens_len)?;
k = tokens.len() as i16;
state = State::Normal;
token.kind = REMIMU_KIND_NCOPEN;
token.mode = REMIMU_MODE_POSSESSIVE;
token.count_lo = 1;
token.count_hi = 2;
i += 2;
}
} else if c == b')' {
paren_count -= 1;
if paren_count < 0 || k == 0 {
return Err(-1);
}
token.kind = REMIMU_KIND_CLOSE;
state = State::Quant;
let mut balance = 0i32;
let mut found: isize = -1;
let mut l = k as isize - 1;
while l >= 0 {
let kind = tokens[l as usize].kind;
if kind == REMIMU_KIND_NCOPEN || kind == REMIMU_KIND_OPEN {
if balance == 0 {
found = l;
break;
} else {
balance -= 1;
}
} else if kind == REMIMU_KIND_CLOSE {
balance += 1;
}
l -= 1;
}
if found == -1 {
return Err(-1);
}
let diff = k as isize - found;
if diff > 32767 {
return Err(-1);
}
token.pair_offset = -(diff as i16);
tokens[found as usize].pair_offset = diff as i16;
if tokens[found as usize].mode == REMIMU_MODE_POSSESSIVE {
token.push_to_vec(tokens, tokens_len)?;
k = tokens.len() as i16;
token.kind = REMIMU_KIND_CLOSE;
token.mode = REMIMU_MODE_POSSESSIVE;
token.pair_offset = -(diff as i16) - 2;
if found >= 1 {
tokens[found as usize - 1].pair_offset = diff as i16 + 2;
} else {
return Err(-1);
}
}
} else if matches!(c, b'?' | b'+' | b'*' | b'{') {
println!("quantifier in non-quantifier context");
return Err(-1);
} else if c == b'.' {
for m in &mut token.mask {
*m = 0xFFFF;
}
if (flags & REMIMU_FLAG_DOT_NO_NEWLINES) != 0 {
token.mask[1] ^= 0x04;
token.mask[1] ^= 0x20;
}
state = State::Quant;
} else if c == b'^' {
token.kind = REMIMU_KIND_CARET;
state = State::Normal;
} else if c == b'$' {
token.kind = REMIMU_KIND_DOLLAR;
state = State::Normal;
} else if c == b'|' {
token.kind = REMIMU_KIND_OR;
state = State::Normal;
} else {
token.set_mask(c);
state = State::Quant;
}
i += 1;
continue;
}
}
State::CharClassInit | State::CharClassNormal | State::CharClassRange => {
if c == b'\\' && esc_state == 0 {
esc_state = 1;
i += 1;
continue;
}
let mut esc_c = 0u8;
if esc_state == 1 {
esc_state = 0;
if c == b'n' {
esc_c = b'\n';
} else if c == b'r' {
esc_c = b'\r';
} else if c == b't' {
esc_c = b'\t';
} else if c == b'v' {
esc_c = 0x0b;
} else if c == b'f' {
esc_c = 0x0c;
} else if c == b'x' {
esc_c = parse_hex_byte(bytes, &mut i)?;
} else if matches!(
c,
b'{' | b'}'
| b'['
| b']'
| b'-'
| b'('
| b')'
| b'|'
| b'^'
| b'$'
| b'*'
| b'+'
| b'?'
| b':'
| b'.'
| b'/'
| b'\\'
) {
esc_c = c;
} else if matches!(c, b'd' | b's' | b'w' | b'D' | b'S' | b'W') {
if matches!(state, State::CharClassRange) {
println!("tried to use a shorthand as part of a range");
return Err(-1);
}
let is_upper = c.is_ascii_uppercase();
let mut c2 = c;
if is_upper {
c2 += 0x20;
}
let m = fill_shorthand_mask(c2);
for (j, mj) in m.iter().enumerate() {
token.mask[j] |= if is_upper { !*mj } else { *mj };
}
char_class_mem = -1;
i += 1;
continue;
} else {
println!(
"unknown/unsupported escape sequence in character class (\\{})",
c as char
);
return Err(-1);
}
}
match state {
State::CharClassInit => {
char_class_mem = c as i32;
token.set_mask(c);
state = State::CharClassNormal;
}
State::CharClassNormal => {
if c == b']' && esc_c == 0 {
char_class_mem = -1;
state = State::Quant;
i += 1;
continue;
} else if c == b'-' && esc_c == 0 && char_class_mem >= 0 {
state = State::CharClassRange;
i += 1;
continue;
} else {
char_class_mem = c as i32;
token.set_mask(c);
state = State::CharClassNormal;
}
}
State::CharClassRange => {
if c == b']' && esc_c == 0 {
char_class_mem = -1;
token.set_mask(b'-');
state = State::Quant;
i += 1;
continue;
} else {
if char_class_mem == -1 {
println!("character class range is broken");
return Err(-1);
}
if c < char_class_mem as u8 {
println!("character class range is misordered");
return Err(-1);
}
let mut j = c;
while j > char_class_mem as u8 {
token.set_mask(j);
j -= 1;
}
state = State::CharClassNormal;
char_class_mem = -1;
}
}
_ => {}
}
i += 1;
continue;
}
State::Quant | State::Mode => {}
}
}
if paren_count > 0 {
println!("(paren_count > 0)");
return Err(-1);
}
if esc_state != 0 {
println!("(esc_state != 0)");
return Err(-1);
}
if matches!(
state,
State::CharClassInit | State::CharClassNormal | State::CharClassRange
) {
println!("(state >= STATE_CC_INIT)");
return Err(-1);
}
token.push_to_vec(tokens, tokens_len)?;
token.kind = REMIMU_KIND_CLOSE;
token.count_lo = 1;
token.count_hi = 2;
token.push_to_vec(tokens, tokens_len)?;
token.kind = REMIMU_KIND_END;
token.push_to_vec(tokens, tokens_len)?;
k = tokens.len() as i16;
if k < 2 {
return Err(-1);
}
tokens[0].pair_offset = k - 2;
tokens[k as usize - 2].pair_offset = -(k - 2);
*token_count = k;
let mut n: u64 = 0;
for k2 in 0..k as usize {
if tokens[k2].kind == REMIMU_KIND_CLOSE {
tokens[k2].mask[0] = n as u16;
n += 1;
let k3 = (k2 as isize + tokens[k2].pair_offset as isize) as usize;
tokens[k3].count_lo = tokens[k2].count_lo;
tokens[k3].count_hi = tokens[k2].count_hi;
tokens[k3].mask[0] = n as u16;
n += 1;
tokens[k3].mode = tokens[k2].mode;
if n > 1024 {
return Err(-1);
}
} else if tokens[k2].kind == REMIMU_KIND_OR
|| tokens[k2].kind == REMIMU_KIND_OPEN
|| tokens[k2].kind == REMIMU_KIND_NCOPEN
{
let mut balance = 0i32;
let mut found: isize = -1;
let mut l = k2 + 1;
while l < tokens_len {
if tokens[l].kind == REMIMU_KIND_OR && balance == 0 {
found = l as isize;
break;
} else if tokens[l].kind == REMIMU_KIND_CLOSE {
if balance == 0 {
found = l as isize;
break;
} else {
balance -= 1;
}
} else if tokens[l].kind == REMIMU_KIND_NCOPEN || tokens[l].kind == REMIMU_KIND_OPEN
{
balance += 1;
}
l += 1;
}
if found == -1 {
println!("unbalanced parens...");
return Err(-1);
}
let diff = found - k2 as isize;
if diff > 32767 {
println!("too long...");
return Err(-1);
}
if tokens[k2].kind == REMIMU_KIND_OR {
tokens[k2].pair_offset = diff as i16;
} else {
tokens[k2].mask[15] = diff as u16;
}
}
}
Ok(())
}
#[derive(Debug, Clone, Copy)]
enum State {
Normal,
Quant,
Mode,
CharClassInit,
CharClassNormal,
CharClassRange,
}
fn check_is_w(byte: u8) -> bool {
let mut w_mask = [0u64; 16];
w_mask[3] = 0x03FF;
w_mask[4] = 0xFFFE;
w_mask[5] = 0x87FF;
w_mask[6] = 0xFFFE;
w_mask[7] = 0x07FF;
(w_mask[(byte >> 4) as usize] & (1u64 << (byte & 0xF))) != 0
}
pub fn regex_match(
tokens: &[RegexToken],
text: &str,
start_i: usize,
cap_slots: u16,
cap_pos: &mut [i64],
cap_span: &mut [i64],
) -> Option<usize> {
let stack_size_max: usize = 1024;
let aux_stats_size: usize = 1024;
let mut cap_slots = cap_slots as usize;
if cap_slots > aux_stats_size {
cap_slots = aux_stats_size;
}
let mut q_group_accepts_zero = vec![0u8; aux_stats_size];
let mut q_group_state = vec![0u32; aux_stats_size];
let mut q_group_stack = vec![0u32; aux_stats_size];
let mut q_group_cap_index = vec![u16::MAX; aux_stats_size];
let mut k = 0usize;
let mut caps = 0usize;
while k < tokens.len() && tokens[k].kind != REMIMU_KIND_END {
if tokens[k].kind == REMIMU_KIND_OPEN && caps < cap_slots {
let open_id = tokens[k].mask[0] as usize;
let close_idx = (k as isize + tokens[k].pair_offset as isize) as usize;
let close_id = tokens[close_idx].mask[0] as usize;
q_group_cap_index[open_id] = caps as u16;
q_group_cap_index[close_id] = caps as u16;
if caps < cap_pos.len() {
cap_pos[caps] = -1;
}
if caps < cap_span.len() {
cap_span[caps] = -1;
}
caps += 1;
}
k += 1;
if k < tokens.len()
&& (tokens[k].kind == REMIMU_KIND_CLOSE
|| tokens[k].kind == REMIMU_KIND_OPEN
|| tokens[k].kind == REMIMU_KIND_NCOPEN)
{
if tokens[k].mask[0] as usize >= aux_stats_size {
println!("too many qualified groups. returning");
return None;
}
q_group_state[tokens[k].mask[0] as usize] = 0;
q_group_stack[tokens[k].mask[0] as usize] = 0;
q_group_accepts_zero[tokens[k].mask[0] as usize] = 0;
}
}
let tokens_len = k;
let textb = text.as_bytes();
let mut rewind_stack: Vec<RegexMatcherState> = Vec::with_capacity(stack_size_max);
let mut i = start_i as u64;
let mut range_min = 0u64;
let mut range_max = 0u64;
let mut just_rewinded = false;
let text_at = |idx: u64| -> u8 {
if idx as usize >= textb.len() {
0
} else {
textb[idx as usize]
}
};
fn rewind_save(
rewind_stack: &mut Vec<RegexMatcherState>,
stack_size_max: usize,
tokens: &[RegexToken],
q_group_state: &[u32],
q_group_stack: &mut [u32],
k: usize,
i: u64,
range_min: u64,
range_max: u64,
is_dummy: bool,
) -> Result<(), ()> {
if rewind_stack.len() >= stack_size_max {
println!("out of backtracking room. returning");
return Err(());
}
let mut s = RegexMatcherState::new(k as u32, i);
s.range_min = range_min;
s.range_max = range_max;
s.prev = 0;
if is_dummy {
s.prev = 0xFAC7;
} else if tokens[s.k as usize].kind == REMIMU_KIND_CLOSE {
let id = tokens[s.k as usize].mask[0] as usize;
s.group_state = q_group_state[id];
s.prev = q_group_stack[id];
q_group_stack[id] = rewind_stack.len() as u32;
}
rewind_stack.push(s);
Ok(())
}
let mut limit = 10000usize;
k = 0;
while k < tokens_len {
if limit == 0 {
println!("iteration limit exceeded. returning");
return None;
}
limit -= 1;
let tk = tokens[k];
if tk.kind == REMIMU_KIND_CARET {
if i != 0 {
if rewind_stack.is_empty() {
return None;
}
rewind_stack.pop();
while !rewind_stack.is_empty()
&& rewind_stack[rewind_stack.len() - 1].prev == 0xFAC7
{
rewind_stack.pop();
}
if rewind_stack.is_empty() {
return None;
}
let s = rewind_stack[rewind_stack.len() - 1];
just_rewinded = true;
range_min = s.range_min;
range_max = s.range_max;
i = s.i;
k = s.k as usize;
if tokens[k].kind == REMIMU_KIND_CLOSE {
let id = tokens[k].mask[0] as usize;
q_group_state[id] = s.group_state;
q_group_stack[id] = s.prev;
}
if k == 0 {
return None;
}
k -= 1;
}
k += 1;
continue;
} else if tk.kind == REMIMU_KIND_DOLLAR {
if text_at(i) != 0 {
if rewind_stack.is_empty() {
return None;
}
rewind_stack.pop();
while !rewind_stack.is_empty()
&& rewind_stack[rewind_stack.len() - 1].prev == 0xFAC7
{
rewind_stack.pop();
}
if rewind_stack.is_empty() {
return None;
}
let s = rewind_stack[rewind_stack.len() - 1];
just_rewinded = true;
range_min = s.range_min;
range_max = s.range_max;
i = s.i;
k = s.k as usize;
if tokens[k].kind == REMIMU_KIND_CLOSE {
let id = tokens[k].mask[0] as usize;
q_group_state[id] = s.group_state;
q_group_stack[id] = s.prev;
}
if k == 0 {
return None;
}
k -= 1;
}
k += 1;
continue;
} else if tk.kind == REMIMU_KIND_BOUND {
let fail = if i == 0 && !check_is_w(text_at(i)) {
true
} else if i != 0 && text_at(i) == 0 && !check_is_w(text_at(i - 1)) {
true
} else {
i != 0 && text_at(i) != 0 && (check_is_w(text_at(i - 1)) == check_is_w(text_at(i)))
};
if fail {
if rewind_stack.is_empty() {
return None;
}
rewind_stack.pop();
while !rewind_stack.is_empty()
&& rewind_stack[rewind_stack.len() - 1].prev == 0xFAC7
{
rewind_stack.pop();
}
if rewind_stack.is_empty() {
return None;
}
let s = rewind_stack[rewind_stack.len() - 1];
just_rewinded = true;
range_min = s.range_min;
range_max = s.range_max;
i = s.i;
k = s.k as usize;
if tokens[k].kind == REMIMU_KIND_CLOSE {
let id = tokens[k].mask[0] as usize;
q_group_state[id] = s.group_state;
q_group_stack[id] = s.prev;
}
if k == 0 {
return None;
}
k -= 1;
}
k += 1;
continue;
} else if tk.kind == REMIMU_KIND_NBOUND {
let fail = if i == 0 && check_is_w(text_at(i)) {
true
} else if i != 0 && text_at(i) == 0 && check_is_w(text_at(i - 1)) {
true
} else {
i != 0 && text_at(i) != 0 && (check_is_w(text_at(i - 1)) != check_is_w(text_at(i)))
};
if fail {
if rewind_stack.is_empty() {
return None;
}
rewind_stack.pop();
while !rewind_stack.is_empty()
&& rewind_stack[rewind_stack.len() - 1].prev == 0xFAC7
{
rewind_stack.pop();
}
if rewind_stack.is_empty() {
return None;
}
let s = rewind_stack[rewind_stack.len() - 1];
just_rewinded = true;
range_min = s.range_min;
range_max = s.range_max;
i = s.i;
k = s.k as usize;
if tokens[k].kind == REMIMU_KIND_CLOSE {
let id = tokens[k].mask[0] as usize;
q_group_state[id] = s.group_state;
q_group_stack[id] = s.prev;
}
if k == 0 {
return None;
}
k -= 1;
}
k += 1;
continue;
}
if tk.count_hi == 1 {
if tk.kind == REMIMU_KIND_OPEN || tk.kind == REMIMU_KIND_NCOPEN {
k = (k as isize + tk.pair_offset as isize) as usize;
} else {
k += 1;
}
continue;
}
if tk.kind == REMIMU_KIND_OPEN || tk.kind == REMIMU_KIND_NCOPEN {
if !just_rewinded {
let close_id = tokens[(k as isize + tk.pair_offset as isize) as usize].mask[0] as usize;
if (tk.mode & REMIMU_MODE_LAZY) != 0
&& (tk.count_lo == 0 || q_group_accepts_zero[close_id] != 0)
{
range_min = 0;
range_max = 0;
if rewind_save(
&mut rewind_stack,
stack_size_max,
tokens,
&q_group_state,
&mut q_group_stack,
k,
i,
range_min,
range_max,
false,
)
.is_err()
{
return None;
}
k = (k as isize + tk.pair_offset as isize) as usize;
} else {
range_min = 1;
range_max = 0;
if rewind_save(
&mut rewind_stack,
stack_size_max,
tokens,
&q_group_state,
&mut q_group_stack,
k,
i,
range_min,
range_max,
false,
)
.is_err()
{
return None;
}
}
} else {
just_rewinded = false;
let orig_k = k;
if range_min != 0 {
k += range_min as usize;
if tokens[k - 1].kind == REMIMU_KIND_OR {
k += tokens[k - 1].pair_offset as usize - 1;
} else if tokens[k - 1].kind == REMIMU_KIND_OPEN
|| tokens[k - 1].kind == REMIMU_KIND_NCOPEN
{
k += tokens[k - 1].mask[15] as usize - 1;
}
if tokens[k].kind == REMIMU_KIND_END {
return None;
}
if tokens[k].kind == REMIMU_KIND_CLOSE {
let id = tokens[k].mask[0] as usize;
if tokens[k].count_lo == 0 || q_group_accepts_zero[id] != 0 {
q_group_state[id] = 0;
if (tokens[k].mode & REMIMU_MODE_LAZY) == 0 {
q_group_stack[id] = 0;
}
k += 1;
continue;
} else {
if rewind_stack.is_empty() {
return None;
}
rewind_stack.pop();
while !rewind_stack.is_empty()
&& rewind_stack[rewind_stack.len() - 1].prev == 0xFAC7
{
rewind_stack.pop();
}
if rewind_stack.is_empty() {
return None;
}
let s = rewind_stack[rewind_stack.len() - 1];
just_rewinded = true;
range_min = s.range_min;
range_max = s.range_max;
i = s.i;
k = s.k as usize;
if tokens[k].kind == REMIMU_KIND_CLOSE {
let id2 = tokens[k].mask[0] as usize;
q_group_state[id2] = s.group_state;
q_group_stack[id2] = s.prev;
}
if k == 0 {
return None;
}
k -= 1;
k += 1;
continue;
}
}
}
let k_diff = k as isize - orig_k as isize;
range_min = (k_diff + 1) as u64;
if rewind_save(
&mut rewind_stack,
stack_size_max,
tokens,
&q_group_state,
&mut q_group_stack,
(k as isize - k_diff) as usize,
i,
range_min,
range_max,
false,
)
.is_err()
{
return None;
}
}
} else if tk.kind == REMIMU_KIND_CLOSE {
if tk.count_lo == 1 && tk.count_hi == 2 {
let cap_index = q_group_cap_index[tk.mask[0] as usize];
if cap_index != u16::MAX {
if rewind_save(
&mut rewind_stack,
stack_size_max,
tokens,
&q_group_state,
&mut q_group_stack,
k,
i,
range_min,
range_max,
true,
)
.is_err()
{
return None;
}
}
} else if !just_rewinded {
let id = tk.mask[0] as usize;
let prev = q_group_stack[id];
range_max = tk.count_hi as u64;
range_max = range_max.saturating_sub(1);
range_min = if q_group_accepts_zero[id] != 0 {
0
} else {
tk.count_lo as u64
};
if q_group_state[id] + 1 < range_min as u32 {
q_group_state[id] += 1;
if rewind_save(
&mut rewind_stack,
stack_size_max,
tokens,
&q_group_state,
&mut q_group_stack,
k,
i,
range_min,
range_max,
false,
)
.is_err()
{
return None;
}
k = (k as isize + tk.pair_offset as isize) as usize;
continue;
} else if tk.count_hi != 0 && q_group_state[id] + 1 > range_max as u32 {
range_max = range_max.saturating_sub(1);
if rewind_stack.is_empty() {
return None;
}
rewind_stack.pop();
while !rewind_stack.is_empty()
&& rewind_stack[rewind_stack.len() - 1].prev == 0xFAC7
{
rewind_stack.pop();
}
if rewind_stack.is_empty() {
return None;
}
let s = rewind_stack[rewind_stack.len() - 1];
just_rewinded = true;
range_min = s.range_min;
range_max = s.range_max;
i = s.i;
k = s.k as usize;
if tokens[k].kind == REMIMU_KIND_CLOSE {
let id2 = tokens[k].mask[0] as usize;
q_group_state[id2] = s.group_state;
q_group_stack[id2] = s.prev;
}
if k == 0 {
return None;
}
k -= 1;
k += 1;
continue;
}
let mut force_zero = 0u8;
if prev != 0
&& rewind_stack
.get(prev as usize)
.map(|s| s.i as u32 > i as u32)
.unwrap_or(false)
{
let mut n = rewind_stack.len() - 1;
while n > 0
&& rewind_stack[n].k as usize
!= (k as isize + tk.pair_offset as isize) as usize
{
n -= 1;
}
if n == 0 {
return None;
}
if rewind_stack[n].i == i {
force_zero = 1;
}
}
if force_zero != 0
|| (prev != 0
&& rewind_stack
.get(prev as usize)
.map(|s| s.i as u32 == i as u32)
.unwrap_or(false))
{
q_group_accepts_zero[id] = 1;
if rewind_stack.is_empty() {
return None;
}
rewind_stack.pop();
while !rewind_stack.is_empty()
&& rewind_stack[rewind_stack.len() - 1].prev == 0xFAC7
{
rewind_stack.pop();
}
if rewind_stack.is_empty() {
return None;
}
let s = rewind_stack[rewind_stack.len() - 1];
just_rewinded = true;
range_min = s.range_min;
range_max = s.range_max;
i = s.i;
k = s.k as usize;
if tokens[k].kind == REMIMU_KIND_CLOSE {
let id2 = tokens[k].mask[0] as usize;
q_group_state[id2] = s.group_state;
q_group_stack[id2] = s.prev;
}
if k == 0 {
return None;
}
k -= 1;
} else if (tk.mode & REMIMU_MODE_LAZY) != 0 {
q_group_state[id] += 1;
if rewind_save(
&mut rewind_stack,
stack_size_max,
tokens,
&q_group_state,
&mut q_group_stack,
k,
i,
range_min,
range_max,
false,
)
.is_err()
{
return None;
}
q_group_state[id] = 0;
} else {
if (tk.mode & REMIMU_MODE_POSSESSIVE) != 0 {
let mut k2 = k;
if q_group_state[id] == 0 {
k2 = (k as isize + tk.pair_offset as isize) as usize;
}
if rewind_stack.is_empty() {
return None;
}
rewind_stack.pop();
while !rewind_stack.is_empty()
&& rewind_stack[rewind_stack.len() - 1].k as usize != k2
{
rewind_stack.pop();
}
if rewind_stack.is_empty() {
return None;
}
}
let open_id =
tokens[(k as isize + tk.pair_offset as isize) as usize].mask[0] as usize;
if q_group_state[open_id] < i as u32 {
q_group_state[id] += 1;
if rewind_save(
&mut rewind_stack,
stack_size_max,
tokens,
&q_group_state,
&mut q_group_stack,
k,
i,
range_min,
range_max,
false,
)
.is_err()
{
return None;
}
k = (k as isize + tk.pair_offset as isize) as usize;
}
}
} else {
just_rewinded = false;
let id = tk.mask[0] as usize;
if (tk.mode & REMIMU_MODE_LAZY) != 0 {
if rewind_save(
&mut rewind_stack,
stack_size_max,
tokens,
&q_group_state,
&mut q_group_stack,
k,
i,
range_min,
range_max,
true,
)
.is_err()
{
return None;
}
q_group_stack[id] = rewind_stack.len() as u32 - 1;
k = (k as isize + tk.pair_offset as isize) as usize;
} else {
if q_group_state[id] < range_min as u32 && q_group_accepts_zero[id] == 0 {
if rewind_stack.is_empty() {
return None;
}
rewind_stack.pop();
while !rewind_stack.is_empty()
&& rewind_stack[rewind_stack.len() - 1].prev == 0xFAC7
{
rewind_stack.pop();
}
if rewind_stack.is_empty() {
return None;
}
let s = rewind_stack[rewind_stack.len() - 1];
just_rewinded = true;
range_min = s.range_min;
range_max = s.range_max;
i = s.i;
k = s.k as usize;
if tokens[k].kind == REMIMU_KIND_CLOSE {
let id2 = tokens[k].mask[0] as usize;
q_group_state[id2] = s.group_state;
q_group_stack[id2] = s.prev;
}
if k == 0 {
return None;
}
k -= 1;
} else {
q_group_state[id] = 0;
let cap_index = q_group_cap_index[id];
if cap_index != u16::MAX {
if rewind_save(
&mut rewind_stack,
stack_size_max,
tokens,
&q_group_state,
&mut q_group_stack,
k,
i,
range_min,
range_max,
true,
)
.is_err()
{
return None;
}
}
}
}
}
} else if tk.kind == REMIMU_KIND_OR {
k = (k as isize + tk.pair_offset as isize) as usize;
} else if tk.kind == REMIMU_KIND_NORMAL {
if !just_rewinded {
let mut n = 0u64;
let old_i = i;
while n < tk.count_lo as u64 && text_at(i) != 0 && tk.check_mask(text_at(i)) {
i += 1;
n += 1;
}
if n < tk.count_lo as u64 {
i = old_i;
if rewind_stack.is_empty() {
return None;
}
rewind_stack.pop();
while !rewind_stack.is_empty()
&& rewind_stack[rewind_stack.len() - 1].prev == 0xFAC7
{
rewind_stack.pop();
}
if rewind_stack.is_empty() {
return None;
}
let s = rewind_stack[rewind_stack.len() - 1];
just_rewinded = true;
range_min = s.range_min;
range_max = s.range_max;
i = s.i;
k = s.k as usize;
if tokens[k].kind == REMIMU_KIND_CLOSE {
let id = tokens[k].mask[0] as usize;
q_group_state[id] = s.group_state;
q_group_stack[id] = s.prev;
}
if k == 0 {
return None;
}
k -= 1;
k += 1;
continue;
}
if (tk.mode & REMIMU_MODE_LAZY) != 0 {
range_min = n;
range_max = tk.count_hi as u64 - 1;
if rewind_save(
&mut rewind_stack,
stack_size_max,
tokens,
&q_group_state,
&mut q_group_stack,
k,
i,
range_min,
range_max,
false,
)
.is_err()
{
return None;
}
} else {
let mut lim = tk.count_hi as u64;
if lim == 0 {
lim = !lim;
}
range_min = n;
while text_at(i) != 0 && tk.check_mask(text_at(i)) && n + 1 < lim {
i += 1;
n += 1;
}
range_max = n;
if (tk.mode & REMIMU_MODE_POSSESSIVE) == 0 {
if rewind_save(
&mut rewind_stack,
stack_size_max,
tokens,
&q_group_state,
&mut q_group_stack,
k,
i,
range_min,
range_max,
false,
)
.is_err()
{
return None;
}
}
}
} else {
just_rewinded = false;
if (tk.mode & REMIMU_MODE_LAZY) != 0 {
let mut lim = range_max;
if lim == 0 {
lim = !lim;
}
if text_at(i) != 0 && tk.check_mask(text_at(i)) && range_min < lim {
i += 1;
range_min += 1;
if rewind_save(
&mut rewind_stack,
stack_size_max,
tokens,
&q_group_state,
&mut q_group_stack,
k,
i,
range_min,
range_max,
false,
)
.is_err()
{
return None;
}
} else {
if rewind_stack.is_empty() {
return None;
}
rewind_stack.pop();
while !rewind_stack.is_empty()
&& rewind_stack[rewind_stack.len() - 1].prev == 0xFAC7
{
rewind_stack.pop();
}
if rewind_stack.is_empty() {
return None;
}
let s = rewind_stack[rewind_stack.len() - 1];
just_rewinded = true;
range_min = s.range_min;
range_max = s.range_max;
i = s.i;
k = s.k as usize;
if tokens[k].kind == REMIMU_KIND_CLOSE {
let id = tokens[k].mask[0] as usize;
q_group_state[id] = s.group_state;
q_group_stack[id] = s.prev;
}
if k == 0 {
return None;
}
k -= 1;
}
} else if range_max > range_min {
i = i.saturating_sub(1);
range_max -= 1;
if rewind_save(
&mut rewind_stack,
stack_size_max,
tokens,
&q_group_state,
&mut q_group_stack,
k,
i,
range_min,
range_max,
false,
)
.is_err()
{
return None;
}
} else {
if rewind_stack.is_empty() {
return None;
}
rewind_stack.pop();
while !rewind_stack.is_empty()
&& rewind_stack[rewind_stack.len() - 1].prev == 0xFAC7
{
rewind_stack.pop();
}
if rewind_stack.is_empty() {
return None;
}
let s = rewind_stack[rewind_stack.len() - 1];
just_rewinded = true;
range_min = s.range_min;
range_max = s.range_max;
i = s.i;
k = s.k as usize;
if tokens[k].kind == REMIMU_KIND_CLOSE {
let id = tokens[k].mask[0] as usize;
q_group_state[id] = s.group_state;
q_group_stack[id] = s.prev;
}
if k == 0 {
return None;
}
k -= 1;
}
}
} else {
panic!("unimplemented token kind {}", tk.kind);
}
k += 1;
}
if caps != 0 {
for s in &rewind_stack {
let kind = tokens[s.k as usize].kind;
if kind == REMIMU_KIND_OPEN || kind == REMIMU_KIND_CLOSE {
let cap_index = q_group_cap_index[tokens[s.k as usize].mask[0] as usize];
if cap_index == u16::MAX {
continue;
}
let idx = cap_index as usize;
if idx >= cap_pos.len() || idx >= cap_span.len() {
continue;
}
if kind == REMIMU_KIND_OPEN {
cap_pos[idx] = s.i as i64;
} else if cap_pos[idx] >= 0 {
cap_span[idx] = s.i as i64 - cap_pos[idx];
}
}
}
for n in 0..caps.min(cap_span.len()).min(cap_pos.len()) {
if cap_span[n] == -1 {
cap_pos[n] = -1;
}
}
}
Some(i as usize)
}
pub fn print_regex_tokens(tokens: &[RegexToken]) {
let kind_to_str = [
"NORMAL", "OPEN", "NCOPEN", "CLOSE", "OR", "CARET", "DOLLAR", "BOUND", "NBOUND", "END",
];
let mode_to_str = ["GREEDY", "POSSESS", "LAZY"];
for token in tokens {
let mode_idx = match token.mode {
0 => 0,
1 => 1,
2 => 2,
_ => 0,
};
print!(
"{}\t{}\t",
kind_to_str[token.kind as usize], mode_to_str[mode_idx]
);
let mut c_old: i32 = -1;
if token.kind == REMIMU_KIND_NORMAL {
for c in 0..256 {
let matched = token.check_mask(c as u8);
if matched {
if c_old == -1 {
c_old = c;
}
} else if c_old != -1 {
let print_c = |ch: i32| {
if (0x20..=0x7E).contains(&ch) {
print!("{}", ch as u8 as char);
} else {
print!("\\x{:02x}", ch);
}
};
if c - 1 == c_old {
print_c(c_old);
} else if c - 2 == c_old {
print_c(c_old);
print_c(c_old + 1);
} else {
print_c(c_old);
print!("-");
print_c(c - 1);
}
c_old = -1;
}
}
}
println!(
"\t{{{},{}}}\t({})",
token.count_lo,
token.count_hi.saturating_sub(1),
token.pair_offset
);
if token.kind == REMIMU_KIND_END {
break;
}
}
}
