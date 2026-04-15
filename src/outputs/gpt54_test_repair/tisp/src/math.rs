use crate::tisp::Rec;
use crate::tisp::TSP_NUM;
use crate::tisp::Tsp;
use crate::tisp::TspType;
use crate::tisp::Val;
use crate::tisp::ValUnion;
use crate::tisp::mk_dec;
use crate::tisp::mk_int;
use crate::tisp::mk_list;
use crate::tisp::mk_prim;
use crate::tisp::mk_rat;
use crate::tisp::mk_sym;
use crate::tisp::tisp_env_add;
use crate::tisp::tsp_lstlen;
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
fn num(v: &Val) -> f64 {
match &v.v {
ValUnion::N { num, .. } => *num,
_ => 0.0,
}
}
fn den(v: &Val) -> f64 {
match &v.v {
ValUnion::N { den, .. } => *den,
_ => 1.0,
}
}
pub fn create_int(num: f64, _den: f64) -> Val {
mk_int(num as i32)
}
pub fn create_dec(num: f64, _den: f64) -> Val {
mk_dec(num).unwrap_or_else(|| mk_int(0))
}
pub fn create_rat(num: f64, den: f64) -> Val {
mk_rat(num as i32, den as i32).unwrap_or_else(|| mk_int(0))
}
pub fn mk_num(a: TspType, b: TspType, force: i32) -> fn(f64, f64) -> Val {
if force == 1 {
return create_rat;
}
if force == 2 {
return create_dec;
}
if a == TspType::TspDec || b == TspType::TspDec {
return create_dec;
}
if a == TspType::TspRatio || b == TspType::TspRatio {
return create_rat;
}
create_int
}
pub fn prim_add(_st: &mut Tsp, _vars: &mut Rec, args: Val) -> Val {
let a = car(&args);
let b = cadr(&args);
if (a.t as u32) & TSP_NUM == 0 || (b.t as u32) & TSP_NUM == 0 {
return mk_int(0);
}
if a.t == TspType::TspDec || b.t == TspType::TspDec {
return mk_dec((num(&a) / den(&a)) + (num(&b) / den(&b))).unwrap_or_else(|| mk_int(0));
}
mk_num(a.t, b.t, 0)(
num(&a) * den(&b) + den(&a) * num(&b),
den(&a) * den(&b),
)
}
pub fn prim_sub(_st: &mut Tsp, _vars: &mut Rec, args: Val) -> Val {
let len = tsp_lstlen(&args);
let mut a = car(&args);
let b = if len == 1 {
let old = a.clone();
a = mk_int(0);
old
} else {
cadr(&args)
};
if a.t == TspType::TspDec || b.t == TspType::TspDec {
return mk_dec((num(&a) / den(&a)) - (num(&b) / den(&b))).unwrap_or_else(|| mk_int(0));
}
mk_num(a.t, b.t, 0)(
num(&a) * den(&b) - den(&a) * num(&b),
den(&a) * den(&b),
)
}
pub fn prim_mul(_st: &mut Tsp, _vars: &mut Rec, args: Val) -> Val {
let a = car(&args);
let b = cadr(&args);
if a.t == TspType::TspDec || b.t == TspType::TspDec {
return mk_dec((num(&a) / den(&a)) * (num(&b) / den(&b))).unwrap_or_else(|| mk_int(0));
}
mk_num(a.t, b.t, 0)(num(&a) * num(&b), den(&a) * den(&b))
}
pub fn prim_div(_st: &mut Tsp, _vars: &mut Rec, args: Val) -> Val {
let len = tsp_lstlen(&args);
let mut a = car(&args);
let b = if len == 1 {
let old = a.clone();
a = mk_int(1);
old
} else {
cadr(&args)
};
if a.t == TspType::TspDec || b.t == TspType::TspDec {
return mk_dec((num(&a) / den(&a)) / (num(&b) / den(&b))).unwrap_or_else(|| mk_int(0));
}
mk_num(a.t, b.t, 1)(num(&a) * den(&b), den(&a) * num(&b))
}
pub fn prim_mod(_st: &mut Tsp, _vars: &mut Rec, args: Val) -> Val {
let a = car(&args);
let b = cadr(&args);
let bv = num(&b) as i32;
if bv == 0 {
return mk_int(0);
}
mk_int((num(&a) as i32) % bv.abs())
}
pub fn prim_pow(st: &mut Tsp, _vars: &mut Rec, args: Val) -> Val {
let b = car(&args);
let p = cadr(&args);
let bnum = num(&b).powf(num(&p) / den(&p));
let bden = den(&b).powf(num(&p) / den(&p));
if (bnum == bnum as i32 as f64 && bden == bden as i32 as f64)
|| b.t == TspType::TspDec
|| p.t == TspType::TspDec
{
return mk_num(b.t, p.t, 0)(bnum, bden);
}
let sym_pow = mk_sym(st, "^").unwrap_or_else(|| mk_int(0));
mk_list(st, 3, vec![sym_pow, b, p]).unwrap_or_else(|| mk_int(0))
}
pub fn prim_denominator(_st: &mut Tsp, _env: &mut Rec, args: Val) -> Val {
mk_int(den(&car(&args)) as i32)
}
pub fn tib_env_math(st: &mut Tsp) {
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "Int") {
tisp_env_add(st, "Int", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "Dec") {
tisp_env_add(st, "Dec", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "floor") {
tisp_env_add(st, "floor", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "ceil") {
tisp_env_add(st, "ceil", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "round") {
tisp_env_add(st, "round", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "numerator") {
tisp_env_add(st, "numerator", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "denominator") {
tisp_env_add(st, "denominator", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "+") {
tisp_env_add(st, "+", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "-") {
tisp_env_add(st, "-", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "*") {
tisp_env_add(st, "*", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "/") {
tisp_env_add(st, "/", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "mod") {
tisp_env_add(st, "mod", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "^") {
tisp_env_add(st, "^", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "<") {
tisp_env_add(st, "<", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, ">") {
tisp_env_add(st, ">", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "<=") {
tisp_env_add(st, "<=", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, ">=") {
tisp_env_add(st, ">=", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "sin") {
tisp_env_add(st, "sin", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "cos") {
tisp_env_add(st, "cos", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "tan") {
tisp_env_add(st, "tan", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "sinh") {
tisp_env_add(st, "sinh", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "cosh") {
tisp_env_add(st, "cosh", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "tanh") {
tisp_env_add(st, "tanh", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "arcsin") {
tisp_env_add(st, "arcsin", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "arccos") {
tisp_env_add(st, "arccos", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "arctan") {
tisp_env_add(st, "arctan", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "arcsinh") {
tisp_env_add(st, "arcsinh", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "arccosh") {
tisp_env_add(st, "arccosh", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "arctanh") {
tisp_env_add(st, "arctanh", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "exp") {
tisp_env_add(st, "exp", v);
}
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "log") {
tisp_env_add(st, "log", v);
}
}
