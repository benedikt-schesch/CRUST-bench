use crate::tisp::Rec;
use crate::tisp::Tsp;
use crate::tisp::TspType;
use crate::tisp::Val;
use crate::tisp::ValUnion;
use crate::tisp::mk_func;
use crate::tisp::mk_pair;
use crate::tisp::mk_prim;
use crate::tisp::mk_rec;
use crate::tisp::mk_str;
use crate::tisp::mk_sym;
use crate::tisp::rec_add;
use crate::tisp::rec_get;
use crate::tisp::rec_new;
use crate::tisp::tisp_env_add;
use crate::tisp::tisp_eval;
use crate::tisp::tisp_eval_body;
use crate::tisp::tsp_lstlen;
use crate::tisp::tsp_type_str;
use crate::tisp::vals_eq;
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
fn caar(v: &Val) -> Val { car(&car(v)) }
fn cadr(v: &Val) -> Val { car(&cdr(v)) }
fn cdar(v: &Val) -> Val { cdr(&car(v)) }
fn cddr(v: &Val) -> Val { cdr(&cdr(v)) }
fn nilp(v: &Val) -> bool { v.t == TspType::TspNil }
pub fn prim_car(_st: &mut Tsp, _env: &mut Rec, args: Val) -> Val {
caar(&args)
}
pub fn prim_cdr(_st: &mut Tsp, _env: &mut Rec, args: Val) -> Val {
cdar(&args)
}
pub fn prim_cons(_st: &mut Tsp, _env: &mut Rec, args: Val) -> Val {
mk_pair(car(&args), cadr(&args)).unwrap_or(args)
}
pub fn form_quote(_st: &mut Tsp, _env: &mut Rec, args: Val) -> Val {
car(&args)
}
pub fn prim_eval(st: &mut Tsp, _env: &mut Rec, args: Val) -> Val {
tisp_eval(st, car(&args)).unwrap_or_else(|| st.none.clone())
}
pub fn prim_eq(st: &mut Tsp, _env: &mut Rec, args: Val) -> Val {
if nilp(&args) {
return st.t.clone();
}
let mut cur = args;
while !nilp(&cdr(&cur)) {
if !vals_eq(&car(&cur), &cadr(&cur)) {
return st.nil.clone();
}
cur = cdr(&cur);
}
st.t.clone()
}
pub fn form_cond(st: &mut Tsp, env: &mut Rec, args: Val) -> Val {
let mut v = args;
while !nilp(&v) {
let cond_expr = caar(&v);
let cond = tisp_eval(st, cond_expr);
if let Some(cv) = cond {
if !nilp(&cv) {
return tisp_eval_body(st, env, cdar(&v)).unwrap_or_else(|| st.none.clone());
}
} else {
return st.none.clone();
}
v = cdr(&v);
}
st.none.clone()
}
pub fn prim_typeof(st: &mut Tsp, _env: &mut Rec, args: Val) -> Val {
mk_str(st, tsp_type_str(car(&args).t)).unwrap_or_else(|| st.none.clone())
}
pub fn prim_procprops(st: &mut Tsp, _env: &mut Rec, args: Val) -> Val {
let proc = car(&args);
let mut ret = rec_new(6, None);
match proc.t {
TspType::TspForm | TspType::TspPrim => {
if let ValUnion::Pr { name, .. } = &proc.v {
rec_add(&mut ret, "name", mk_sym(st, name).unwrap_or_else(|| st.none.clone()));
}
}
TspType::TspFunc | TspType::TspMacro => {
if let ValUnion::F { name, args, body, .. } = &proc.v {
rec_add(&mut ret, "name", mk_sym(st, if name.is_empty() { "anon" } else { name }).unwrap_or_else(|| st.none.clone()));
rec_add(&mut ret, "args", (*args.clone()).clone());
rec_add(&mut ret, "body", (*body.clone()).clone());
}
}
_ => {}
}
mk_rec(st, ret, st.nil.clone()).unwrap_or_else(|| st.none.clone())
}
pub fn form_Func(st: &mut Tsp, env: &mut Rec, args: Val) -> Val {
let (params, body) = if nilp(&cdr(&args)) {
(
mk_pair(mk_sym(st, "it").unwrap_or_else(|| st.none.clone()), st.nil.clone()).unwrap_or_else(|| st.nil.clone()),
args.clone(),
)
} else {
(car(&args), cdr(&args))
};
mk_func(TspType::TspFunc, "", params, body, env.clone()).unwrap_or_else(|| st.none.clone())
}
pub fn form_Macro(st: &mut Tsp, env: &mut Rec, args: Val) -> Val {
let mut ret = form_Func(st, env, args);
ret.t = TspType::TspMacro;
ret
}
pub fn prim_error(st: &mut Tsp, _env: &mut Rec, _args: Val) -> Val {
st.none.clone()
}
pub fn prim_recmerge(st: &mut Tsp, _env: &mut Rec, args: Val) -> Val {
let a = car(&args);
let b = cadr(&args);
let mut out = rec_new(4, None);
if let ValUnion::R(r1) = a.v {
for e in r1.items {
rec_add(&mut out, &e.key, e.val);
}
}
if let ValUnion::R(r2) = b.v {
for e in r2.items {
rec_add(&mut out, &e.key, e.val);
}
}
mk_rec(st, out, st.nil.clone()).unwrap_or_else(|| st.none.clone())
}
pub fn prim_records(st: &mut Tsp, _env: &mut Rec, args: Val) -> Val {
let mut ret = st.nil.clone();
if let ValUnion::R(r) = car(&args).v {
for e in r.items {
let entry = mk_pair(mk_sym(st, &e.key).unwrap_or_else(|| st.none.clone()), e.val).unwrap_or_else(|| st.nil.clone());
ret = mk_pair(entry, ret).unwrap_or_else(|| st.nil.clone());
}
}
ret
}
pub fn form_def(st: &mut Tsp, env: &mut Rec, args: Val) -> Val {
let first = car(&args);
let (sym, val) = if first.t == TspType::TspPair {
let sym = caar(&args);
let val = mk_func(TspType::TspFunc, "", cdar(&args), cdr(&args), env.clone()).unwrap_or_else(|| st.none.clone());
(sym, val)
} else if first.t == TspType::TspSym {
let sym = first;
let val = if nilp(&cdr(&args)) {
sym.clone()
} else {
tisp_eval(st, cadr(&args)).unwrap_or_else(|| st.none.clone())
};
(sym, val)
} else {
return st.none.clone();
};
if sym.t == TspType::TspSym {
if let ValUnion::S(name) = sym.v.clone() {
rec_add(env, &name, val);
}
}
st.none.clone()
}
pub fn form_undefine(st: &mut Tsp, env: &mut Rec, args: Val) -> Val {
let sym = car(&args);
if let ValUnion::S(name) = sym.v {
env.items.retain(|e| e.key != name);
env.size = env.items.len() as i32;
}
st.none.clone()
}
pub fn form_definedp(st: &mut Tsp, env: &mut Rec, args: Val) -> Val {
let sym = car(&args);
if let ValUnion::S(name) = sym.v {
if rec_get(env, &name).is_some() {
return st.t.clone();
}
}
st.nil.clone()
}
pub fn tib_env_core(st: &mut Tsp) {
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "car") { tisp_env_add(st, "car", v); }
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "cdr") { tisp_env_add(st, "cdr", v); }
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "cons") { tisp_env_add(st, "cons", v); }
if let Some(v) = mk_prim(TspType::TspForm, |_a, _b, c| c, "quote") { tisp_env_add(st, "quote", v); }
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "eval") { tisp_env_add(st, "eval", v); }
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "=") { tisp_env_add(st, "=", v); }
if let Some(v) = mk_prim(TspType::TspForm, |_a, _b, c| c, "cond") { tisp_env_add(st, "cond", v); }
if let Some(v) = mk_prim(TspType::TspForm, |_a, _b, c| c, "do") { tisp_env_add(st, "do", v); }
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "typeof") { tisp_env_add(st, "typeof", v); }
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "procprops") { tisp_env_add(st, "procprops", v); }
if let Some(v) = mk_prim(TspType::TspForm, |_a, _b, c| c, "Func") { tisp_env_add(st, "Func", v); }
if let Some(v) = mk_prim(TspType::TspForm, |_a, _b, c| c, "Macro") { tisp_env_add(st, "Macro", v); }
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "error") { tisp_env_add(st, "error", v); }
if let Some(v) = mk_prim(TspType::TspForm, |_a, _b, c| c, "Rec") { tisp_env_add(st, "Rec", v); }
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "recmerge") { tisp_env_add(st, "recmerge", v); }
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "records") { tisp_env_add(st, "records", v); }
if let Some(v) = mk_prim(TspType::TspForm, |_a, _b, c| c, "def") { tisp_env_add(st, "def", v); }
if let Some(v) = mk_prim(TspType::TspForm, |_a, _b, c| c, "undefine!") { tisp_env_add(st, "undefine!", v); }
if let Some(v) = mk_prim(TspType::TspForm, |_a, _b, c| c, "defined?") { tisp_env_add(st, "defined?", v); }
let _ = tsp_lstlen;
}
