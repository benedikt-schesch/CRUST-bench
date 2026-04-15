use crate::trusted_utils;

use std::cell::RefCell;

thread_local! {
static CLAUSES: RefCell<Vec<(u64, Vec<i32>)>> = const { RefCell::new(Vec::new()) };
static VAR_VALUES: RefCell<Vec<i8>> = const { RefCell::new(Vec::new()) };
static ASSIGNED_UNITS: RefCell<Vec<i32>> = const { RefCell::new(Vec::new()) };
static CHECK_MODEL: RefCell<bool> = const { RefCell::new(false) };
static LENIENT: RefCell<bool> = const { RefCell::new(false) };
static ID_TO_ADD: RefCell<u64> = const { RefCell::new(1) };
static NB_LOADED_CLAUSES: RefCell<u64> = const { RefCell::new(0) };
static CLAUSE_TO_ADD: RefCell<Vec<i32>> = const { RefCell::new(Vec::new()) };
static DONE_LOADING: RefCell<bool> = const { RefCell::new(false) };
static UNSAT_PROVEN: RefCell<bool> = const { RefCell::new(false) };
static LAST_SIG: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
static MSG: RefCell<String> = const { RefCell::new(String::new()) };
}

fn set_msg(s: String) {
MSG.with(|m| *m.borrow_mut() = s);
}

fn get_clause(id: u64) -> Option<Vec<i32>> {
CLAUSES.with(|c| c.borrow().iter().find(|(k, _)| *k == id).map(|(_, v)| v.clone()))
}

pub fn reset_assignments() {
ASSIGNED_UNITS.with(|au| {
VAR_VALUES.with(|vv| {
let mut vals = vv.borrow_mut();
for v in au.borrow().iter() {
let idx = *v as usize;
if idx < vals.len() {
vals[idx] = 0;
}
}
});
au.borrow_mut().clear();
});
}

pub fn lrat_check_add_clause(id: u64, lits: &[i32], nb_lits: i32, hints: &[u64], nb_hints: i32) -> bool {
if !check_clause(id, lits, nb_lits, hints, nb_hints) {
return false;
}
lrat_check_add_axiomatic_clause(id, lits, nb_lits)
}

pub fn lrat_check_add_axiomatic_clause(id: u64, lits: &[i32], nb_lits: i32) -> bool {
let cls = clause_init(lits, nb_lits);
let inserted = CLAUSES.with(|c| {
let mut clauses = c.borrow_mut();
if clauses.iter().any(|(k, _)| *k == id) {
false
} else {
clauses.push((id, cls.clone()));
true
}
});

let mut ok = inserted;
if !ok {
let lenient = LENIENT.with(|l| *l.borrow());
if lenient {
let old = get_clause(id);
if let Some(old_cls) = old {
if clauses_equivalent(&old_cls, &cls) {
ok = true;
}
}
}
if !ok {
set_msg(format!("Insertion of clause {} unsuccessful - already present?", id));
}
} else if nb_lits == 0 {
UNSAT_PROVEN.with(|u| *u.borrow_mut() = true);
}
ok
}

pub fn check_clause(base_id: u64, lits: &[i32], nb_lits: i32, hints: &[u64], nb_hints: i32) -> bool {
ASSIGNED_UNITS.with(|au| au.borrow_mut().reserve((nb_lits + nb_hints) as usize));

for lit in lits.iter().take(nb_lits as usize) {
let var = if *lit > 0 { *lit } else { -*lit };
VAR_VALUES.with(|vv| {
let mut vals = vv.borrow_mut();
let idx = var as usize;
if idx < vals.len() {
vals[idx] = if *lit > 0 { -1 } else { 1 };
}
});
ASSIGNED_UNITS.with(|au| au.borrow_mut().push(var));
}

let mut ok = true;
for (i, hint_id) in hints.iter().take(nb_hints as usize).enumerate() {
let cls = match get_clause(*hint_id) {
Some(c) => c,
None => {
set_msg(format!("Derivation {}: hint {} not found", base_id, hint_id));
ok = false;
break;
}
};

let mut new_unit = 0;
for lit in cls.iter() {
if *lit == 0 {
break;
}
let var = if *lit > 0 { *lit } else { -*lit };
let assigned = VAR_VALUES.with(|vv| {
let vals = vv.borrow();
let idx = var as usize;
if idx < vals.len() { vals[idx] } else { 0 }
});
if assigned == 0 {
if new_unit != 0 {
set_msg(format!("Derivation {}: multiple literals unassigned", base_id));
ok = false;
break;
}
new_unit = *lit;
continue;
}

let sign = assigned > 0;
if sign == (*lit > 0) {
set_msg(format!("Derivation {}: dependency {} is satisfied", base_id, hint_id));
ok = false;
break;
}
}
if !ok {
break;
}

if new_unit == 0 {
if i + 1 < nb_hints as usize {
set_msg(format!(
"Derivation {}: empty clause produced at non-final hint {}",
base_id, hint_id
));
reset_assignments();
return false;
}
reset_assignments();
return true;
}

let var = if new_unit > 0 { new_unit } else { -new_unit };
VAR_VALUES.with(|vv| {
let mut vals = vv.borrow_mut();
let idx = var as usize;
if idx < vals.len() {
vals[idx] = if new_unit > 0 { 1 } else { -1 };
}
});
ASSIGNED_UNITS.with(|au| au.borrow_mut().push(var));
}

if ok {
set_msg(format!("Derivation {}: no empty clause was produced", base_id));
}
reset_assignments();
false
}

pub fn lrat_check_end_load(out_sig: &mut Option<Vec<u8>>) -> bool {
let nonempty = CLAUSE_TO_ADD.with(|c| !c.borrow().is_empty());
if nonempty {
set_msg("literals left in unterminated clause".to_string());
return false;
}
let sig = LAST_SIG.with(|s| s.borrow().clone());
*out_sig = Some(sig);
DONE_LOADING.with(|d| *d.borrow_mut() = true);
ID_TO_ADD.with(|id| {
NB_LOADED_CLAUSES.with(|n| *n.borrow_mut() = *id.borrow() - 1);
});
true
}

pub fn lrat_check_delete_clause(ids: &[u64], nb_ids: i32) -> bool {
for id in ids.iter().take(nb_ids as usize) {
let found = get_clause(*id);
if found.is_none() {
set_msg(format!("Clause deletion: ID {} not found", id));
return false;
}
let check_model = CHECK_MODEL.with(|c| *c.borrow());
let nb_loaded = NB_LOADED_CLAUSES.with(|n| *n.borrow());
if check_model && *id <= nb_loaded {
continue;
}
CLAUSES.with(|c| {
let mut clauses = c.borrow_mut();
if let Some(pos) = clauses.iter().position(|(k, _)| *k == *id) {
clauses.remove(pos);
}
});
}
true
}

pub fn clauses_equivalent(left_cls: &[i32], right_cls: &[i32]) -> bool {
let left: Vec<i32> = left_cls.iter().copied().take_while(|x| *x != 0).collect();
let right: Vec<i32> = right_cls.iter().copied().take_while(|x| *x != 0).collect();
if left.len() != right.len() {
return false;
}
left.iter().all(|l| right.contains(l))
}

pub fn lrat_check_validate_sat(model: &[i32], size: u64) -> bool {
let done_loading = DONE_LOADING.with(|d| *d.borrow());
if !done_loading {
set_msg("SAT validation illegal - loading formula was not concluded".to_string());
return false;
}

let check_model = CHECK_MODEL.with(|c| *c.borrow());
if !check_model {
set_msg("SAT validation illegal - not executed to explicitly support this".to_string());
return false;
}

let nb_loaded = NB_LOADED_CLAUSES.with(|n| *n.borrow());
for id in 1..=nb_loaded {
let cls = match get_clause(id) {
Some(c) => c,
None => {
set_msg(format!("SAT validation: original ID {} not found", id));
return false;
}
};

let mut satisfied = false;
for lit in cls.iter().copied().take_while(|x| *x != 0) {
let var = if lit > 0 { lit } else { -lit };
if (var as u64).saturating_sub(1) >= size {
set_msg(format!("SAT validation: model does not cover variable {}", var));
return false;
}
let model_lit = model[(var - 1) as usize];
if model_lit != var && model_lit != -var && model_lit != 0 {
set_msg(format!(
"SAT validation: unexpected literal {} in assignment of variable {}",
model_lit, var
));
return false;
}
let chosen = if model_lit == 0 { lit } else { model_lit };
if chosen == lit {
satisfied = true;
break;
}
}
if !satisfied {
set_msg(format!("SAT validation: original clause {} not satisfied", id));
return false;
}
}
true
}

pub fn lrat_check_load(lit: i32) -> bool {
if lit == 0 {
let clause = CLAUSE_TO_ADD.with(|c| c.borrow().clone());
if !lrat_check_add_axiomatic_clause(
ID_TO_ADD.with(|id| *id.borrow()),
&clause,
clause.len() as i32,
) {
return false;
}
ID_TO_ADD.with(|id| *id.borrow_mut() += 1);
CLAUSE_TO_ADD.with(|c| c.borrow_mut().clear());
return true;
}
CLAUSE_TO_ADD.with(|c| c.borrow_mut().push(lit));
true
}

pub fn lrat_check_init(nb_vars: i32, opt_check_model: bool, opt_lenient: bool) {
CLAUSES.with(|c| c.borrow_mut().clear());
CLAUSE_TO_ADD.with(|c| c.borrow_mut().clear());
VAR_VALUES.with(|v| *v.borrow_mut() = vec![0; (nb_vars + 1).max(0) as usize]);
ASSIGNED_UNITS.with(|a| a.borrow_mut().clear());
CHECK_MODEL.with(|c| *c.borrow_mut() = opt_check_model);
LENIENT.with(|l| *l.borrow_mut() = opt_lenient);
ID_TO_ADD.with(|id| *id.borrow_mut() = 1);
NB_LOADED_CLAUSES.with(|n| *n.borrow_mut() = 0);
DONE_LOADING.with(|d| *d.borrow_mut() = false);
UNSAT_PROVEN.with(|u| *u.borrow_mut() = false);
LAST_SIG.with(|s| *s.borrow_mut() = vec![0; trusted_utils::SIG_SIZE_BYTES]);
}

pub fn clause_init(data: &[i32], nb_lits: i32) -> Vec<i32> {
let mut cls = Vec::with_capacity(nb_lits as usize + 1);
cls.extend_from_slice(&data[..nb_lits as usize]);
cls.push(0);
cls
}

pub fn lrat_check_validate_unsat() -> bool {
let done_loading = DONE_LOADING.with(|d| *d.borrow());
if !done_loading {
set_msg("UNSAT validation illegal - loading formula was not concluded".to_string());
return false;
}
let unsat = UNSAT_PROVEN.with(|u| *u.borrow());
if !unsat {
set_msg("UNSAT validation unsuccessful - did not derive or import empty clause".to_string());
return false;
}
true
}
