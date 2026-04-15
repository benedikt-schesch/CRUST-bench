use crate::tisp::Prim;
use crate::tisp::Rec;
use crate::tisp::Tsp;
use crate::tisp::TspType;
use crate::tisp::Val;
use crate::tisp::ValUnion;
use crate::tisp::mk_int;
use crate::tisp::mk_prim;
use crate::tisp::mk_str;
use crate::tisp::mk_sym;
use crate::tisp::tisp_env_add;
use crate::tisp::tisp_eval_list;
pub type MkFn = fn(&mut Tsp, &str) -> Val;
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
pub fn val_string(st: &mut Tsp, args: Val, mk_fn: MkFn) -> Val {
let mut ret = String::new();
let mut cur = args;
while cur.t == TspType::TspPair {
let v = car(&cur);
match v.t {
TspType::TspNone => {}
TspType::TspNil => ret.push_str("Nil"),
TspType::TspInt => {
if let ValUnion::N { num, .. } = v.v {
ret.push_str(&(num as i32).to_string());
}
}
TspType::TspDec => {
if let ValUnion::N { num, .. } = v.v {
ret.push_str(&format!("{:.15}", num).trim_end_matches('0').trim_end_matches('.').to_string());
}
}
TspType::TspRatio => {
if let ValUnion::N { num, den } = v.v {
ret.push_str(&format!("{}/{}", num as i32, den as i32));
}
}
TspType::TspStr | TspType::TspSym => {
if let ValUnion::S(s) = v.v {
ret.push_str(&s);
}
}
_ => {}
}
cur = cdr(&cur);
}
mk_fn(st, &ret)
}
pub fn prim_Str(st: &mut Tsp, _env: &mut Rec, args: Val) -> Val {
val_string(st, args, |st, s| mk_str(st, s).unwrap_or_else(|| st.none.clone()))
}
pub fn prim_Sym(st: &mut Tsp, _env: &mut Rec, args: Val) -> Val {
val_string(st, args, |st, s| mk_sym(st, s).unwrap_or_else(|| st.none.clone()))
}
pub fn prim_strlen(_st: &mut Tsp, _env: &mut Rec, args: Val) -> Val {
let v = car(&args);
if let ValUnion::S(s) = v.v {
mk_int(s.len() as i32)
} else {
mk_int(0)
}
}
pub fn form_strformat(st: &mut Tsp, env: &mut Rec, args: Val) -> Val {
let v = car(&args);
if let ValUnion::S(s) = v.v {
let _ = tisp_eval_list(st, env, st.nil.clone());
mk_str(st, &s).unwrap_or_else(|| st.none.clone())
} else {
st.none.clone()
}
}
pub fn tib_env_string(st: &mut Tsp) {
let _x: Option<Prim> = None;
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "Sym") { tisp_env_add(st, "Sym", v); }
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "Str") { tisp_env_add(st, "Str", v); }
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "strlen") { tisp_env_add(st, "strlen", v); }
if let Some(v) = mk_prim(TspType::TspForm, |_a, _b, c| c, "strformat") { tisp_env_add(st, "strformat", v); }
}
