use crate::tisp::Rec;
use crate::tisp::Tsp;
use crate::tisp::TspType;
use crate::tisp::Val;
use crate::tisp::ValUnion;
use crate::tisp::mk_int;
use crate::tisp::mk_prim;
use crate::tisp::mk_str;
use crate::tisp::tisp_env_add;
use crate::tisp::tisp_eval;
use std::env;
use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH};

fn car(v: &Val) -> Val {
match &v.v {
ValUnion::P { car, .. } => (**car).clone(),
_ => v.clone(),
}
}

pub fn prim_cd(st: &mut Tsp, _env: &mut Rec, args: Val) -> Val {
let dir = car(&args);
let s = match dir.v {
ValUnion::S(s) => s,
_ => return st.none.clone(),
};
let _ = env::set_current_dir(s);
st.none.clone()
}
pub fn prim_pwd(st: &mut Tsp, _env: &mut Rec, _args: Val) -> Val {
let cwd = env::current_dir().ok().and_then(|p| p.to_str().map(|s| s.to_string())).unwrap_or_default();
mk_str(st, &cwd).unwrap_or_else(|| st.none.clone())
}
pub fn prim_exit(_st: &mut Tsp, _env: &mut Rec, args: Val) -> Val {
let code = match car(&args).v {
ValUnion::N { num, .. } => num as i32,
_ => 0,
};
exit(code)
}
pub fn prim_now(_st: &mut Tsp, _env: &mut Rec, _args: Val) -> Val {
let secs = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs() as i32).unwrap_or(0);
mk_int(secs)
}
pub fn form_time(st: &mut Tsp, _env: &mut Rec, args: Val) -> Val {
let expr = car(&args);
let start = std::time::Instant::now();
let _ = tisp_eval(st, expr);
let elapsed = start.elapsed().as_secs_f64() * 100.0;
crate::tisp::mk_dec(elapsed).unwrap_or_else(|| st.none.clone())
}
pub fn tib_env_os(st: &mut Tsp) {
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "cd!") { tisp_env_add(st, "cd!", v); }
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "pwd") { tisp_env_add(st, "pwd", v); }
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "exit!") { tisp_env_add(st, "exit!", v); }
if let Some(v) = mk_prim(TspType::TspPrim, |_a, _b, c| c, "now") { tisp_env_add(st, "now", v); }
if let Some(v) = mk_prim(TspType::TspForm, |_a, _b, c| c, "time") { tisp_env_add(st, "time", v); }
}
