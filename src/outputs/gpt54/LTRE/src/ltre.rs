// Generated Rust Code
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SymSet {
bits: [u8; 256 / 8],
}

impl SymSet {
pub fn empty() -> Self {
Self { bits: [0; 32] }
}

pub fn full() -> Self {
Self { bits: [0xff; 32] }
}

pub fn contains(&self, c: u8) -> bool {
self.bits[(c as usize) / 8] & (1 << ((c as usize) % 8)) != 0
}

pub fn insert(&mut self, c: u8) {
self.bits[(c as usize) / 8] |= 1 << ((c as usize) % 8);
}

pub fn invert(&mut self) {
for b in &mut self.bits {
*b = !*b;
}
}

pub fn union_with(&mut self, other: &SymSet) {
for i in 0..self.bits.len() {
self.bits[i] |= other.bits[i];
}
}

pub fn intersect_with(&mut self, other: &SymSet) {
for i in 0..self.bits.len() {
self.bits[i] &= other.bits[i];
}
}

pub fn is_empty(&self) -> bool {
self.bits.iter().all(|&b| b == 0)
}
}

const METACHARS: &str = "\\.-^$*+?{}[]<>()|&~";

fn is_print(c: u8) -> bool {
(0x20..=0x7e).contains(&c)
}

fn is_digit(c: u8) -> bool {
c.is_ascii_digit()
}

fn is_upper(c: u8) -> bool {
c.is_ascii_uppercase()
}

fn is_lower(c: u8) -> bool {
c.is_ascii_lowercase()
}

fn is_alpha(c: u8) -> bool {
c.is_ascii_alphabetic()
}

fn is_alnum(c: u8) -> bool {
c.is_ascii_alphanumeric()
}

fn is_space(c: u8) -> bool {
matches!(c, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

fn to_lower(c: u8) -> u8 {
c.to_ascii_lowercase()
}

fn to_upper(c: u8) -> u8 {
c.to_ascii_uppercase()
}

pub fn symset_fmt(set: &SymSet) -> String {
let mut buf = String::new();
let mut nbuf = String::new();
let mut nsym = 0i32;
let mut nnsym = 0i32;

nbuf.push('^');
buf.push('[');
nbuf.push('[');

let mut chr = 0usize;
while chr < 256 {
let included = set.contains(chr as u8);
if included {
nsym += 1;
} else {
nnsym += 1;
}

let p = if included { &mut buf } else { &mut nbuf };
let ch = chr as u8;
let is_metachar = ch != 0 && METACHARS.as_bytes().contains(&ch);

if !is_print(ch) && !is_metachar {
p.push_str(&format!("\\x{:02x}", ch));
} else {
if is_metachar {
p.push('\\');
}
p.push(ch as char);
}

let start = chr;
while chr < 255 && set.contains(chr as u8) == set.contains((chr + 1) as u8) {
chr += 1;
}
if chr - start >= 2 {
p.push('-');
if included {
nsym -= 1;
} else {
nnsym -= 1;
}
}
if chr - start >= 1 {
let included2 = set.contains(chr as u8);
if included2 {
nsym += 1;
} else {
nnsym += 1;
}
let p2 = if included2 { &mut buf } else { &mut nbuf };
let ch2 = chr as u8;
let is_metachar2 = ch2 != 0 && METACHARS.as_bytes().contains(&ch2);

if !is_print(ch2) && !is_metachar2 {
p2.push_str(&format!("\\x{:02x}", ch2));
} else {
if is_metachar2 {
p2.push('\\');
}
p2.push(ch2 as char);
}
}

chr += 1;
}

buf.push(']');
nbuf.push(']');

if nnsym == 0 {
"<>".to_string()
} else if nsym == 1 {
buf[1..buf.len() - 1].to_string()
} else if nnsym == 1 {
let inner = &nbuf[2..nbuf.len() - 1];
format!("^{}", inner)
} else if buf.len() < nbuf.len() {
buf
} else {
nbuf
}
}

#[derive(Clone, Debug)]
pub struct NState {
pub label: SymSet,
pub target: Option<usize>,
pub epsilon0: Option<usize>,
pub epsilon1: Option<usize>,
}

impl NState {
pub fn new() -> Self {
Self {
label: SymSet::empty(),
target: None,
epsilon0: None,
epsilon1: None,
}
}
}

#[derive(Clone, Debug)]
pub struct Nfa {
pub states: Vec<NState>,
pub initial: usize,
pub final_: usize,
pub complemented: bool,
}

impl Nfa {
pub fn new_single() -> Self {
Self {
states: vec![NState::new()],
initial: 0,
final_: 0,
complemented: false,
}
}

pub fn len(&self) -> usize {
self.states.len()
}
}

pub fn nfa_free(_nfa: Nfa) {}
pub fn dfa_free(_dfa: Dfa) {}

pub fn nfa_clone(orig: &Nfa) -> Nfa {
orig.clone()
}

pub fn nfa_concat(nfa1: &mut Nfa, nfa2: Nfa) {
if nfa1.initial == nfa1.final_ {
*nfa1 = nfa2;
return;
}
if nfa2.initial == nfa2.final_ {
return;
}

let off = nfa1.states.len() - 1;
let mut rhs_states = nfa2.states;
let rhs_initial = rhs_states.remove(0);

nfa1.states[nfa1.final_] = rhs_initial;
for st in &mut nfa1.states[nfa1.final_..=nfa1.final_] {
shift_option(&mut st.target, off);
shift_option(&mut st.epsilon0, off);
shift_option(&mut st.epsilon1, off);
}

for st in &mut rhs_states {
shift_option(&mut st.target, off);
shift_option(&mut st.epsilon0, off);
shift_option(&mut st.epsilon1, off);
}

nfa1.states.extend(rhs_states);
nfa1.final_ = nfa2.final_ + off;
}

pub fn nfa_pad_initial(nfa: &mut Nfa) {
let old_initial = nfa.initial;
let mut new_states = Vec::with_capacity(nfa.states.len() + 1);
let mut st = NState::new();
st.epsilon0 = Some(old_initial + 1);
new_states.push(st);
let mut old = nfa.states.clone();
for s in &mut old {
shift_option(&mut s.target, 1);
shift_option(&mut s.epsilon0, 1);
shift_option(&mut s.epsilon1, 1);
}
new_states.extend(old);
nfa.states = new_states;
nfa.initial = 0;
nfa.final_ += 1;
}

pub fn nfa_pad_final(nfa: &mut Nfa) {
let new_final = nfa.states.len();
nfa.states.push(NState::new());
nfa.states[nfa.final_].epsilon0 = Some(new_final);
nfa.final_ = new_final;
}

pub fn nfa_uncomplement(nfa: &mut Nfa) -> Result<(), String> {
if !nfa.complemented {
return Ok(());
}
let dfa = ltre_compile(nfa.clone());
let unc = ltre_uncompile(&dfa);
*nfa = unc;
Ok(())
}

pub fn nfa_dump(nfa: &Nfa) {
println!("graph LR");
println!("  I( ) --> {}", nfa.initial);
println!("  {} --> F( )", nfa.final_);
for (id, st) in nfa.states.iter().enumerate() {
if let Some(e) = st.epsilon0 {
println!("  {} --> {}", id, e);
}
if let Some(e) = st.epsilon1 {
println!("  {} --> {}", id, e);
}
if st.label.is_empty() {
continue;
}
if let Some(t) = st.target {
println!("  {} --{}--> {}", id, symset_fmt(&st.label), t);
}
}
}

#[derive(Clone)]
pub struct DState {
pub transitions: [usize; 256],
pub accepting: bool,
pub terminating: bool,
pub bitset: Vec<u8>,
}

impl std::fmt::Debug for DState {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
f.debug_struct("DState")
.field("accepting", &self.accepting)
.field("terminating", &self.terminating)
.field("bitset", &self.bitset)
.finish()
}
}

#[derive(Clone, Debug)]
pub struct Dfa {
pub states: Vec<DState>,
pub initial: usize,
}

impl Dfa {
pub fn new() -> Self {
Self {
states: Vec::new(),
initial: 0,
}
}

pub fn len(&self) -> usize {
self.states.len()
}
}

pub fn dfa_serialize(dfa: &Dfa) -> Vec<u8> {
let dfa_size = dfa.states.len() as i32;
let mut buf = Vec::new();
leb128_put(&mut buf, dfa_size);
for st in &dfa.states {
buf.push(((st.accepting as u8) << 1) | (st.terminating as u8));
let mut chr = 0usize;
while chr < 256 {
let start = chr;
while chr < 255 && st.transitions[chr] == st.transitions[chr + 1] {
chr += 1;
}
buf.push((chr - start) as u8);
leb128_put(&mut buf, st.transitions[chr] as i32);
chr += 1;
}
}
buf
}

pub fn dfa_deserialize(buf: &[u8]) -> Result<(Dfa, usize), String> {
let mut p = 0usize;
let dfa_size = leb128_get(buf, &mut p)? as usize;
let mut states = Vec::with_capacity(dfa_size);
for _ in 0..dfa_size {
states.push(DState {
transitions: [0; 256],
accepting: false,
terminating: false,
bitset: Vec::new(),
});
}
for id in 0..dfa_size {
if p >= buf.len() {
return Err("buffer too short".to_string());
}
let b = buf[p];
p += 1;
states[id].accepting = ((b >> 1) & 1) != 0;
states[id].terminating = (b & 1) != 0;
let mut chr = 0usize;
while chr < 256 {
if p >= buf.len() {
return Err("buffer too short".to_string());
}
let len = buf[p] as usize;
p += 1;
let target = leb128_get(buf, &mut p)? as usize;
for _ in 0..=len {
if chr >= 256 {
return Err("invalid serialized DFA".to_string());
}
states[id].transitions[chr] = target;
chr += 1;
}
}
}
Ok((Dfa { states, initial: 0 }, p))
}

pub fn dfa_dump(dfa: &Dfa) {
println!("graph LR");
println!("  I( ) --> {}", dfa.initial);
for (i, ds1) in dfa.states.iter().enumerate() {
if ds1.accepting {
println!("  {} --> F( )", i);
}
for (j, _) in dfa.states.iter().enumerate() {
let mut transitions = SymSet::empty();
for chr in 0..256 {
if ds1.transitions[chr] == j {
transitions.insert(chr as u8);
}
}
if transitions.is_empty() {
continue;
}
println!("  {} --{}--> {}", i, symset_fmt(&transitions), j);
}
}
}

fn leb128_put(buf: &mut Vec<u8>, mut n: i32) {
while (n >> 7) != 0 {
buf.push(((n & 0x7f) as u8) | 0x80);
n >>= 7;
}
buf.push(n as u8);
}

fn leb128_get(buf: &[u8], p: &mut usize) -> Result<i32, String> {
let mut n = 0i32;
let mut c = 0i32;
loop {
if *p >= buf.len() {
return Err("buffer too short".to_string());
}
let b = buf[*p];
*p += 1;
n |= ((b & 0x7f) as i32) << (c * 7);
c += 1;
if b & 0x80 == 0 {
break;
}
}
Ok(n)
}

pub fn ltre_parse(regex: &str) -> Result<Nfa, String> {
let mut ctx = ParseContext::new(regex);
let nfa = parse_regex(&mut ctx)?;
if !ctx.is_eof() {
return Err("expected end of input".to_string());
}
Ok(nfa)
}

pub fn ltre_fixed_string(s: &str) -> Nfa {
let mut nfa = Nfa::new_single();
for &b in s.as_bytes() {
let initial = nfa.final_;
let next = nfa.states.len();
nfa.states.push(NState::new());
nfa.states[initial].target = Some(next);
nfa.states[initial].label.insert(b);
nfa.final_ = next;
}
nfa
}

pub fn ltre_partial(nfa: &mut Nfa) -> Result<(), String> {
nfa_uncomplement(nfa)?;
nfa_pad_initial(nfa);
nfa_pad_final(nfa);
let i = nfa.initial;
let f = nfa.final_;
nfa.states[i].target = Some(i);
nfa.states[f].target = Some(f);
nfa.states[i].label = SymSet::full();
nfa.states[f].label = SymSet::full();
Ok(())
}

pub fn ltre_ignorecase(nfa: &mut Nfa) -> Result<(), String> {
nfa_uncomplement(nfa)?;
for st in &mut nfa.states {
let mut extra = SymSet::empty();
for chr in 0..=255u8 {
if st.label.contains(chr) {
extra.insert(to_lower(chr));
extra.insert(to_upper(chr));
}
}
st.label.union_with(&extra);
}
Ok(())
}

pub fn ltre_complement(nfa: &mut Nfa) {
nfa.complemented = !nfa.complemented;
}

pub fn ltre_compile(nfa: Nfa) -> Dfa {
let nfa_size = nfa.len();
let bitset_size = (nfa_size + 7) / 8;
let mut states: Vec<DState> = Vec::new();

let start_bs = epsilon_closure_vec(&nfa, nfa.initial, nfa_size);
states.push(DState {
transitions: [usize::MAX; 256],
accepting: bitset_test(&start_bs, nfa.final_) ^ nfa.complemented,
terminating: false,
bitset: start_bs,
});

let mut idx = 0usize;
while idx < states.len() {
for chr in 0..256usize {
let next_bs = step_powerset(&nfa, &states[idx].bitset, chr as u8);
let mut found = None;
for (j, st) in states.iter().enumerate() {
if st.bitset == next_bs {
found = Some(j);
break;
}
}
let target = if let Some(j) = found {
j
} else {
let accepting = bitset_test(&next_bs, nfa.final_) ^ nfa.complemented;
states.push(DState {
transitions: [usize::MAX; 256],
accepting,
terminating: false,
bitset: next_bs.clone(),
});
states.len() - 1
};
states[idx].transitions[chr] = target;
}
idx += 1;
}

let mut dfa = Dfa { states, initial: 0 };
dfa_minimize(&mut dfa, nfa.complemented);

for i in 0..dfa.states.len() {
let mut terminating = true;
for chr in 0..256 {
if dfa.states[i].transitions[chr] != i {
terminating = false;
break;
}
}
dfa.states[i].terminating = terminating;
}

let _ = bitset_size;
let _ = find_or_create_dead;
dfa
}

fn find_or_create_dead(states: &mut Vec<DState>) -> usize {
for (i, st) in states.iter().enumerate() {
if st.transitions.iter().all(|&t| t == i) && !st.accepting {
return i;
}
}
let idx = states.len();
states.push(DState {
transitions: [idx; 256],
accepting: false,
terminating: true,
bitset: Vec::new(),
});
idx
}

fn step_powerset(nfa: &Nfa, bitset: &[u8], chr: u8) -> Vec<u8> {
let nfa_size = nfa.len();
let mut out = vec![0u8; (nfa_size + 7) / 8];
for id in all_bitset_indices(bitset) {
if id >= nfa_size {
continue;
}
let st = &nfa.states[id];
if st.label.contains(chr) {
if let Some(t) = st.target {
epsilon_closure_into(nfa, t, &mut out);
}
}
}
out
}

fn epsilon_closure_vec(nfa: &Nfa, start: usize, nfa_size: usize) -> Vec<u8> {
let mut out = vec![0u8; (nfa_size + 7) / 8];
epsilon_closure_into(nfa, start, &mut out);
out
}

fn epsilon_closure_into(nfa: &Nfa, st_id: usize, bitset: &mut [u8]) {
if bitset_test(bitset, st_id) {
return;
}
bitset_set(bitset, st_id);
if let Some(e) = nfa.states[st_id].epsilon0 {
epsilon_closure_into(nfa, e, bitset);
}
if let Some(e) = nfa.states[st_id].epsilon1 {
epsilon_closure_into(nfa, e, bitset);
}
}

fn all_bitset_indices(bitset: &[u8]) -> Vec<usize> {
let mut indices = Vec::new();
for (byte_index, byte) in bitset.iter().copied().enumerate() {
if byte == 0 {
continue;
}
for bit in 0..8usize {
if (byte & (1u8 << bit)) != 0 {
indices.push(byte_index * 8 + bit);
}
}
}
indices
}

fn dfa_minimize(dfa: &mut Dfa, _complemented: bool) {
if dfa.states.is_empty() {
return;
}
let n = dfa.states.len();
let mut dis = vec![vec![false; n]; n];

for i in 0..n {
for j in (i + 1)..n {
if dfa.states[i].accepting != dfa.states[j].accepting {
dis[i][j] = true;
dis[j][i] = true;
}
}
}

let mut changed = true;
while changed {
changed = false;
for i in 0..n {
for j in (i + 1)..n {
if dis[i][j] {
continue;
}
for chr in 0..256 {
let t1 = dfa.states[i].transitions[chr];
let t2 = dfa.states[j].transitions[chr];
if t1 != t2 && dis[t1][t2] {
dis[i][j] = true;
dis[j][i] = true;
changed = true;
break;
}
}
}
}
}

let mut rep: Vec<usize> = (0..n).collect();
for i in 0..n {
for j in 0..i {
if !dis[i][j] {
rep[i] = rep[j];
break;
}
}
}

let mut map_new = vec![usize::MAX; n];
let mut new_states = Vec::<DState>::new();
for i in 0..n {
if rep[i] == i {
map_new[i] = new_states.len();
new_states.push(dfa.states[i].clone());
}
}
for i in 0..n {
if map_new[i] == usize::MAX {
map_new[i] = map_new[rep[i]];
}
}

for st in &mut new_states {
for chr in 0..256 {
st.transitions[chr] = map_new[st.transitions[chr]];
}
}

dfa.initial = map_new[dfa.initial];
dfa.states = new_states;
}

fn bitset_test(bs: &[u8], idx: usize) -> bool {
idx / 8 < bs.len() && (bs[idx / 8] & (1 << (idx % 8))) != 0
}

fn bitset_set(bs: &mut [u8], idx: usize) {
bs[idx / 8] |= 1 << (idx % 8);
}

pub fn ltre_matches(dfa: &Dfa, input: &[u8]) -> bool {
let mut s = dfa.initial;
let mut i = 0usize;
while !dfa.states[s].terminating && i < input.len() && input[i] != 0 {
s = dfa.states[s].transitions[input[i] as usize];
i += 1;
}
dfa.states[s].accepting
}

pub fn ltre_matches_lazy(dfap: &mut Option<Dfa>, nfa: &Nfa, input: &[u8]) -> bool {
if dfap.is_none() {
*dfap = Some(ltre_compile(nfa.clone()));
}
ltre_matches(dfap.as_ref().unwrap(), input)
}

pub fn ltre_uncompile(dfa: &Dfa) -> Nfa {
let dfa_size = dfa.len();
let mut nfa = Nfa {
states: vec![NState::new(), NState::new()],
initial: 0,
final_: 1,
complemented: false,
};

let mut nstates = Vec::with_capacity(dfa_size);
for _ in 0..dfa_size {
let idx = nfa.states.len();
nfa.states.push(NState::new());
nstates.push(idx);
}

nfa.states[nfa.initial].epsilon1 = Some(nstates[dfa.initial]);

for (id, ds) in dfa.states.iter().enumerate() {
if ds.accepting {
nfa.states[nstates[id]].epsilon1 = Some(nfa.final_);
}
}

for (id1, ds1) in dfa.states.iter().enumerate() {
let mut free = None::<usize>;
for (id2, _) in dfa.states.iter().enumerate() {
let mut transitions = SymSet::empty();
for chr in 0..256 {
if ds1.transitions[chr] == id2 {
transitions.insert(chr as u8);
}
}
if transitions.is_empty() {
continue;
}

let src = if free.is_none() {
let x = nstates[id1];
free = Some(x);
x
} else {
let x = nfa.states.len();
nfa.states.push(NState::new());
let fr = free.unwrap();
if nfa.states[fr].epsilon1.is_none() {
nfa.states[fr].epsilon1 = Some(x);
} else {
nfa.states[fr].epsilon0 = Some(x);
free = Some(x);
}
x
};

nfa.states[src].target = Some(nstates[id2]);
nfa.states[src].label = transitions;
}
}

nfa
}

pub fn ltre_decompile(dfa: &Dfa) -> String {
if dfa.states.is_empty() {
return "[]".to_string();
}

let n = dfa.states.len();
let s = n;
let f = n + 1;
let size = n + 2;
let mut arrows = vec![vec![None::<String>; size]; size];

arrows[s][dfa.initial] = Some(String::new());
for i in 0..n {
if dfa.states[i].accepting {
arrows[i][f] = Some(String::new());
}
for j in 0..n {
let mut transitions = SymSet::empty();
for chr in 0..256 {
if dfa.states[i].transitions[chr] == j {
transitions.insert(chr as u8);
}
}
if !transitions.is_empty() {
arrows[i][j] = Some(symset_fmt(&transitions));
}
}
}

let mut eliminate: Vec<usize> = (0..n).collect();
while let Some(k) = eliminate.pop() {
for i in 0..size {
if i == k {
continue;
}
for j in 0..size {
if j == k {
continue;
}
let rin = arrows[i][k].clone();
let rout = arrows[k][j].clone();
if rin.is_none() || rout.is_none() {
continue;
}
let rin = rin.unwrap();
let rout = rout.unwrap();
let rself = arrows[k][k].clone().unwrap_or_default();

let mid = if rself.is_empty() {
String::new()
} else {
format!("({})*", rself)
};

let mut bypass = String::new();
bypass.push_str(&rin);
bypass.push_str(&mid);
bypass.push_str(&rout);

match arrows[i][j].clone() {
None => arrows[i][j] = Some(bypass),
Some(existing) => {
if existing.is_empty() {
arrows[i][j] = Some(format!("({})?", bypass));
} else {
arrows[i][j] = Some(format!("{}|{}", existing, bypass));
}
}
}
}
}
for i in 0..size {
arrows[i][k] = None;
arrows[k][i] = None;
}
}

arrows[s][f].clone().unwrap_or_else(|| "[]".to_string())
}

struct ParseContext<'a> {
chars: &'a [u8],
pos: usize,
}

impl<'a> ParseContext<'a> {
fn new(s: &'a str) -> Self {
Self {
chars: s.as_bytes(),
pos: 0,
}
}

fn peek(&self) -> Option<u8> {
self.chars.get(self.pos).copied()
}

fn next(&mut self) -> Option<u8> {
let ch = self.peek()?;
self.pos += 1;
Some(ch)
}

fn is_eof(&self) -> bool {
self.pos >= self.chars.len()
}

fn expect_char(&mut self) -> Result<u8, String> {
self.next().ok_or_else(|| "expected symbol".to_string())
}
}

fn parse_regex(ctx: &mut ParseContext) -> Result<Nfa, String> {
let mut re = parse_term(ctx)?;
while matches!(ctx.peek(), Some(b'|') | Some(b'&')) {
let intersect = ctx.next() == Some(b'&');
let mut alt = parse_term(ctx)?;
re.complemented ^= intersect;
alt.complemented ^= intersect;
nfa_uncomplement(&mut re)?;
nfa_uncomplement(&mut alt)?;

nfa_pad_initial(&mut re);
nfa_pad_final(&mut alt);

let re_initial = re.initial;
let re_final = re.final_;
let alt_initial = alt.initial;
let alt_final = alt.final_;

let offset = re.states.len();
let mut alt_states = alt.states.clone();
for s in &mut alt_states {
shift_option(&mut s.target, offset);
shift_option(&mut s.epsilon0, offset);
shift_option(&mut s.epsilon1, offset);
}
re.states.extend(alt_states);
let alt_initial2 = alt_initial + offset;
let alt_final2 = alt_final + offset;

re.states[re_initial].epsilon1 = Some(alt_initial2);
re.states[re_final].epsilon0 = Some(alt_final2);
re.final_ = alt_final2;
re.complemented ^= intersect;
}
Ok(re)
}

fn parse_term(ctx: &mut ParseContext) -> Result<Nfa, String> {
let mut complement = false;
if ctx.peek() == Some(b'~') {
ctx.next();
complement = true;
}

let mut term = Nfa::new_single();
while !matches!(ctx.peek(), Some(b')') | Some(b'|') | Some(b'&') | None) {
let mut factor = parse_factor(ctx)?;
nfa_uncomplement(&mut factor)?;
nfa_concat(&mut term, factor);
}
if complement {
term.complemented = true;
}
Ok(term)
}

fn parse_factor(ctx: &mut ParseContext) -> Result<Nfa, String> {
let mut atom = parse_atom(ctx)?;
match ctx.peek() {
Some(b'*') => {
ctx.next();
nfa_uncomplement(&mut atom)?;
let init = atom.initial;
let fin = atom.final_;
atom.states[fin].epsilon1 = Some(init);
nfa_pad_initial(&mut atom);
nfa_pad_final(&mut atom);
let i = atom.initial;
let f = atom.final_;
atom.states[i].epsilon1 = Some(f);
Ok(atom)
}
Some(b'+') => {
ctx.next();
nfa_uncomplement(&mut atom)?;
let init = atom.initial;
let fin = atom.final_;
atom.states[fin].epsilon1 = Some(init);
nfa_pad_initial(&mut atom);
nfa_pad_final(&mut atom);
Ok(atom)
}
Some(b'?') => {
ctx.next();
nfa_uncomplement(&mut atom)?;
let i = atom.initial;
let f = atom.final_;
if atom.states[i].epsilon1.is_some() {
nfa_pad_initial(&mut atom);
}
let i2 = atom.initial;
atom.states[i2].epsilon1 = Some(f + if i2 == 0 && f == atom.final_ { 0 } else { 0 });
atom.states[i2].epsilon1 = Some(atom.final_);
Ok(atom)
}
Some(b'{') => {
let save = ctx.pos;
ctx.next();
nfa_uncomplement(&mut atom)?;

let min = match parse_natural(ctx) {
Ok(v) => v,
Err(_) => 0,
};
let mut max = min;
let mut max_unbounded = false;

if ctx.peek() == Some(b',') {
ctx.next();
match parse_natural(ctx) {
Ok(v) => max = v,
Err(_) => max_unbounded = true,
}
}
if ctx.peek() != Some(b'}') {
return Err("expected '}'".to_string());
}
ctx.next();

if min > max && !max_unbounded {
ctx.pos = save;
return Err("misbounded quantifier".to_string());
}

let mut atoms = Nfa::new_single();
let mut i = 0u32;
loop {
if max_unbounded {
if i > min {
break;
}
} else if i >= max {
break;
}

let mut clone = nfa_clone(&atom);
if i >= min {
if max_unbounded {
let init = clone.initial;
let fin = clone.final_;
clone.states[fin].epsilon1 = Some(init);
nfa_pad_initial(&mut clone);
nfa_pad_final(&mut clone);
}
let ini = clone.initial;
let fin = clone.final_;
clone.states[ini].epsilon1 = Some(fin);
}
nfa_concat(&mut atoms, clone);
if i == u32::MAX {
break;
}
i += 1;
}

Ok(atoms)
}
_ => Ok(atom),
}
}

fn parse_atom(ctx: &mut ParseContext) -> Result<Nfa, String> {
if ctx.peek() == Some(b'(') {
ctx.next();
let sub = parse_regex(ctx)?;
if ctx.peek() != Some(b')') {
return Err("expected ')'".to_string());
}
ctx.next();
return Ok(sub);
}

let mut chars = Nfa {
states: vec![NState::new(), NState::new()],
initial: 0,
final_: 1,
complemented: false,
};
chars.states[0].target = Some(1);
chars.states[0].label = parse_symset(ctx)?;
Ok(chars)
}

fn parse_symset(ctx: &mut ParseContext) -> Result<SymSet, String> {
let mut complement = false;
if ctx.peek() == Some(b'^') {
ctx.next();
complement = true;
}

let save = ctx.pos;

let shorthand = match ctx.peek() {
Some(b'\\') => {
ctx.next();
match ctx.next() {
Some(b'd') => Some(digits_set()),
Some(b'D') => Some(not_digits_set()),
Some(b's') => Some(spaces_set()),
Some(b'S') => Some(not_spaces_set()),
Some(b'w') => Some(wordchar_set()),
Some(b'W') => Some(not_wordchar_set()),
Some(other) => {
ctx.pos -= 2;
let _ = other;
None
}
None => {
ctx.pos = save;
None
}
}
}
Some(b'.') => {
ctx.next();
let mut s = SymSet::full();
let mut nl = SymSet::empty();
nl.insert(b'\n');
for i in 0..32 {
s.bits[i] &= !nl.bits[i];
}
Some(s)
}
_ => None,
};

let mut out = if let Some(s) = shorthand {
s
} else {
ctx.pos = save;
if ctx.peek() == Some(b'[') {
ctx.next();
let mut s = SymSet::empty();
while ctx.peek() != Some(b']') {
if ctx.is_eof() {
return Err("expected ']'".to_string());
}
let sub = parse_symset(ctx)?;
s.union_with(&sub);
}
ctx.next();
s
} else if ctx.peek() == Some(b'<') {
ctx.next();
let mut s = SymSet::full();
while ctx.peek() != Some(b'>') {
if ctx.is_eof() {
return Err("expected '>'".to_string());
}
let sub = parse_symset(ctx)?;
s.intersect_with(&sub);
}
ctx.next();
s
} else {
let begin = if ctx.peek() == Some(b'\\') {
ctx.next();
match ctx.next() {
Some(c) if METACHARS.as_bytes().contains(&c) => c,
Some(b'a') => 0x07,
Some(b'b') => 0x08,
Some(b'f') => 0x0c,
Some(b'n') => b'\n',
Some(b'r') => b'\r',
Some(b't') => b'\t',
Some(b'v') => 0x0b,
Some(b'x') => {
let h1 = ctx.next().ok_or_else(|| "expected hex digit".to_string())?;
let h2 = ctx.next().ok_or_else(|| "expected hex digit".to_string())?;
let hv = |c: u8| -> Option<u8> {
match c {
b'0'..=b'9' => Some(c - b'0'),
b'a'..=b'f' => Some(c - b'a' + 10),
b'A'..=b'F' => Some(c - b'A' + 10),
_ => None,
}
};
match (hv(h1), hv(h2)) {
(Some(a), Some(b)) => (a << 4) | b,
_ => return Err("expected hex digit".to_string()),
}
}
Some(_) => return Err("unknown escape".to_string()),
None => return Err("unknown escape".to_string()),
}
} else {
let c = ctx.expect_char()?;
if METACHARS.as_bytes().contains(&c) {
return Err("unexpected metacharacter".to_string());
}
if !is_print(c) {
return Err("unexpected nonprintable character".to_string());
}
c
};

let mut end = begin;
if ctx.peek() == Some(b'-') {
ctx.next();
end = if ctx.peek() == Some(b'\\') {
ctx.next();
match ctx.next() {
Some(c) if METACHARS.as_bytes().contains(&c) => c,
Some(b'a') => 0x07,
Some(b'b') => 0x08,
Some(b'f') => 0x0c,
Some(b'n') => b'\n',
Some(b'r') => b'\r',
Some(b't') => b'\t',
Some(b'v') => 0x0b,
Some(b'x') => {
let h1 = ctx.next().ok_or_else(|| "expected hex digit".to_string())?;
let h2 = ctx.next().ok_or_else(|| "expected hex digit".to_string())?;
let hv = |c: u8| -> Option<u8> {
match c {
b'0'..=b'9' => Some(c - b'0'),
b'a'..=b'f' => Some(c - b'a' + 10),
b'A'..=b'F' => Some(c - b'A' + 10),
_ => None,
}
};
match (hv(h1), hv(h2)) {
(Some(a), Some(b)) => (a << 4) | b,
_ => return Err("expected hex digit".to_string()),
}
}
Some(_) => return Err("unknown escape".to_string()),
None => return Err("unknown escape".to_string()),
}
} else {
let c = ctx.expect_char()?;
if METACHARS.as_bytes().contains(&c) {
return Err("unexpected metacharacter".to_string());
}
if !is_print(c) {
return Err("unexpected nonprintable character".to_string());
}
c
};
}

let mut s = SymSet::empty();
let mut x = begin;
loop {
s.insert(x);
if x == end {
break;
}
x = x.wrapping_add(1);
}
s
}
};

if complement {
out.invert();
}
Ok(out)
}

fn union_inplace(a: &mut SymSet, b: &SymSet) {
a.union_with(b);
}

fn intersect_inplace(a: &mut SymSet, b: &SymSet) {
a.intersect_with(b);
}

fn digits_set() -> SymSet {
let mut s = SymSet::empty();
for c in b'0'..=b'9' {
s.insert(c);
}
s
}

fn not_digits_set() -> SymSet {
let mut s = digits_set();
s.invert();
s
}

fn spaces_set() -> SymSet {
let mut s = SymSet::empty();
for c in 0u8..=255 {
if is_space(c) {
s.insert(c);
}
}
s
}

fn not_spaces_set() -> SymSet {
let mut s = spaces_set();
s.invert();
s
}

fn wordchar_set() -> SymSet {
let mut s = SymSet::empty();
for c in 0u8..=255 {
if c == b'_' || is_alnum(c) {
s.insert(c);
}
}
s
}

fn not_wordchar_set() -> SymSet {
let mut s = wordchar_set();
s.invert();
s
}

fn parse_natural(ctx: &mut ParseContext) -> Result<u32, String> {
if !matches!(ctx.peek(), Some(c) if is_digit(c)) {
return Err("expected natural number".to_string());
}
let mut n: u32 = 0;
while let Some(c) = ctx.peek() {
if !is_digit(c) {
break;
}
let digit = (c - b'0') as u32;
if n > u32::MAX / 10 || n * 10 > u32::MAX - digit {
return Err("natural number overflow".to_string());
}
n = n * 10 + digit;
ctx.next();
}
Ok(n)
}

fn shift_option(opt: &mut Option<usize>, offset: usize) {
if let Some(v) = *opt {
*opt = Some(v + offset);
}
}

#[allow(dead_code)]
fn _keep_helpers_used() {
let _ = is_upper as fn(u8) -> bool;
let _ = is_lower as fn(u8) -> bool;
let _ = is_alpha as fn(u8) -> bool;
let _ = union_inplace as fn(&mut SymSet, &SymSet);
let _ = intersect_inplace as fn(&mut SymSet, &SymSet);
}
