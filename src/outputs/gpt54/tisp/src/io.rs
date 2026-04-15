use crate::tisp::Rec;
use crate::tisp::Tsp;
use crate::tisp::TspType;
use crate::tisp::Val;
use crate::tisp::ValUnion;
use crate::tisp::mk_pair;
use crate::tisp::mk_prim;
use crate::tisp::mk_str;
use crate::tisp::mk_sym;
use crate::tisp::tisp_env_add;
use crate::tisp::tisp_eval_body;
use crate::tisp::tisp_read_line;
use std::fs;
use std::io::{Read, Write};

fn car(v: &Val) -> Val {
match &v.v {
ValUnion::P { car, .. } => (**car).clone(),
_ => v.clone(),
}
}
fn cdr(v: &Val) -> Val {
match &v.v {
ValUnion::P { cdr, .. } => (**cdr).clone(),
_ => v.clone(),
}
}
fn cadr(v: &Val) -> Val {
car(&cdr(v))
}
fn cddr(v: &Val) -> Val {
cdr(&cdr(v))
}
fn nilp(v: &Val) -> bool {
v.t == TspType::TspNil
}

pub fn count_parens(s: &str, len: i32) -> i32 {
let mut pcount = 0;
let mut bcount = 0;
let mut ccount = 0;
for ch in s.chars().take(len as usize) {
match ch {
'(' => pcount += 1,
'[' => bcount += 1,
'{' => ccount += 1,
')' => pcount -= 1,
']' => bcount -= 1,
'}' => ccount -= 1,
_ => {}
}
}
if pcount != 0 {
pcount
} else if bcount != 0 {
bcount
} else {
ccount
}
}
pub fn read_file(fname: &str) -> String {
if fname.is_empty() {
let mut s = String::new();
let _ = std::io::stdin().read_to_string(&mut s);
s
} else {
fs::read_to_string(fname).unwrap_or_default()
}
}
pub fn prim_write(st: &mut Tsp, _env: &mut Rec, args: Val) -> Val {
let target = car(&args);
let append = !nilp(&cadr(&args));
let mut rest = cddr(&args);
let mut buf = String::new();
while !nilp(&rest) && rest.t == TspType::TspPair {
let v = car(&rest);
match v.v {
ValUnion::S(s) => buf.push_str(&s),
ValUnion::N { num, den } => {
if v.t == TspType::TspInt {
buf.push_str(&(num as i32).to_string());
} else if v.t == TspType::TspRatio {
buf.push_str(&format!("{}/{}", num as i32, den as i32));
} else {
buf.push_str(&num.to_string());
}
}
_ => {}
}
rest = cdr(&rest);
}
match target.v {
ValUnion::S(s) if target.t == TspType::TspSym && s == "stdout" => {
let _ = std::io::stdout().write_all(buf.as_bytes());
}
ValUnion::S(s) if target.t == TspType::TspSym && s == "stderr" => {
let _ = std::io::stderr().write_all(buf.as_bytes());
}
ValUnion::S(s) => {
if append {
let _ = fs::OpenOptions::new()
.create(true)
.append(true)
.open(s)
.and_then(|mut f| f.write_all(buf.as_bytes()));
} else {
let _ = fs::write(s, buf);
}
}
_ => {}
}
st.none.clone()
}
pub fn prim_read(st: &mut Tsp, _env: &mut Rec, args: Val) -> Val {
let fname = if crate::tisp::tsp_lstlen(&args) == 1 {
match car(&args).v {
ValUnion::S(s) => s,
_ => String::new(),
}
} else {
String::new()
};
let file = read_file(&fname);
if file.is_empty() {
st.nil.clone()
} else {
mk_str(st, &file).unwrap_or_else(|| st.nil.clone())
}
}
pub fn prim_parse(st: &mut Tsp, _env: &mut Rec, args: Val) -> Val {
let expr = car(&args);
if nilp(&expr) {
return mk_sym(st, "quit").unwrap_or_else(|| st.none.clone());
}
let old_file = st.file.clone();
let old_filec = st.filec;
let src = match expr.v {
ValUnion::S(s) => s,
_ => String::new(),
};
st.file = src;
st.filec = 0;
let mut ret = mk_pair(
mk_sym(st, "do").unwrap_or_else(|| st.none.clone()),
st.nil.clone(),
)
.unwrap_or_else(|| st.nil.clone());
let mut list = Vec::new();
while st.filec < st.file.len() {
if let Some(v) = tisp_read_line(st, 0) {
list.push(v);
} else {
break;
}
}
let mut built = st.nil.clone();
for v in list.into_iter().rev() {
built = mk_pair(v, built).unwrap_or_else(|| st.nil.clone());
}
ret = mk_pair(car(&ret), built).unwrap_or_else(|| st.nil.clone());
st.file = old_file;
st.filec = old_filec;
if cdr(&ret).t == TspType::TspPair && nilp(&cddr(&ret)) {
cadr(&ret)
} else {
ret
}
}
pub fn prim_load(st: &mut Tsp, env: &mut Rec, args: Val) -> Val {
let tib = car(&args);
let name = match tib.v {
ValUnion::S(s) => s,
_ => String::new(),
};
let paths = ["/usr/local/lib/tisp/pkgs/", "/usr/lib/tisp/pkgs/", "./"];
for p in paths {
let path = format!("{}{}.tsp", p, name);
if fs::metadata(&path).is_ok() {
let file = read_file(&path);
let parsed_sym = mk_sym(st, &file).unwrap_or_else(|| st.none.clone());
let parsed_args =
mk_pair(parsed_sym, st.nil.clone()).unwrap_or_else(|| st.nil.clone());
let body = prim_parse(st, env, parsed_args);
let _ = tisp_eval_body(st, env, body);
return st.none.clone();
}
}
st.none.clone()
}
pub fn tib_env_io(st: &mut Tsp) {
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "write") {
tisp_env_add(st, "write", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "read") {
tisp_env_add(st, "read", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "parse") {
tisp_env_add(st, "parse", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "load") {
tisp_env_add(st, "load", v);
}
}
