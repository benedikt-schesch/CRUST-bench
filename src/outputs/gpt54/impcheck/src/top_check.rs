use crate::confirm::confirm_result;
use crate::lrat_check::{
lrat_check_add_axiomatic_clause, lrat_check_add_clause, lrat_check_delete_clause,
lrat_check_end_load, lrat_check_init, lrat_check_load, lrat_check_validate_sat,
lrat_check_validate_unsat,
};
use crate::secret::SECRET_KEY;
use crate::siphash::SipHash;
use crate::trusted_utils::{trusted_utils_equal_signatures, SIG_SIZE_BYTES};

use std::cell::RefCell;

thread_local! {
static FORMULA_SIGNATURE: RefCell<[u8; SIG_SIZE_BYTES]> = const { RefCell::new([0; SIG_SIZE_BYTES]) };
static VALID: RefCell<bool> = const { RefCell::new(true) };
}

pub fn top_check_init(nb_vars: i32, check_model: bool, lenient: bool) {
let _ = SipHash::siphash_init(&SECRET_KEY);
lrat_check_init(nb_vars, check_model, lenient);
VALID.with(|v| *v.borrow_mut() = true);
}

pub fn top_check_commit_formula_sig(f_sig: &[u8]) {
FORMULA_SIGNATURE.with(|f| {
let mut s = f.borrow_mut();
s.copy_from_slice(&f_sig[..SIG_SIZE_BYTES]);
});
}

pub fn top_check_validate_sat(model: &[i32], size: u64, out_signature_or_null: &mut Option<Vec<u8>>) -> bool {
let ok = lrat_check_validate_sat(model, size);
VALID.with(|v| *v.borrow_mut() &= ok);
if !top_check_valid() {
return false;
}
if out_signature_or_null.is_some() {
let mut sig = vec![0u8; SIG_SIZE_BYTES];
FORMULA_SIGNATURE.with(|f| confirm_result(&f.borrow()[..], 10, &mut sig));
*out_signature_or_null = Some(sig);
}
true
}

pub fn top_check_delete(ids: &[u64], nb_ids: i32) -> bool {
lrat_check_delete_clause(ids, nb_ids)
}

pub fn top_check_end_load() -> bool {
let mut sig_from_chk = None;
let mut valid = lrat_check_end_load(&mut sig_from_chk);
if !valid {
VALID.with(|v| *v.borrow_mut() = false);
return false;
}
let got = sig_from_chk.unwrap_or_else(|| vec![0u8; SIG_SIZE_BYTES]);
valid = FORMULA_SIGNATURE.with(|f| trusted_utils_equal_signatures(&got, &f.borrow()[..]));
VALID.with(|v| *v.borrow_mut() &= valid);
valid
}

pub fn top_check_import(id: u64, literals: &[i32], nb_literals: i32, signature_data: &[u8]) -> bool {
let mut computed_sig = vec![0u8; SIG_SIZE_BYTES];
compute_clause_signature(id, literals, nb_literals, &mut computed_sig);
if !trusted_utils_equal_signatures(signature_data, &computed_sig) {
VALID.with(|v| *v.borrow_mut() = false);
return false;
}
let ok = lrat_check_add_axiomatic_clause(id, literals, nb_literals);
VALID.with(|v| *v.borrow_mut() &= ok);
ok
}

pub fn top_check_valid() -> bool {
VALID.with(|v| *v.borrow())
}

pub fn top_check_load(lit: i32) {
let ok = lrat_check_load(lit);
VALID.with(|v| *v.borrow_mut() &= ok);
}

pub fn compute_clause_signature(id: u64, lits: &[i32], nb_lits: i32, out: &mut [u8]) {
let mut sip = SipHash::siphash_init(&SECRET_KEY);
sip.siphash_reset();
sip.siphash_update(&id.to_le_bytes(), std::mem::size_of::<u64>() as u64);
for lit in lits.iter().take(nb_lits as usize) {
sip.siphash_update(&lit.to_le_bytes(), std::mem::size_of::<i32>() as u64);
}
FORMULA_SIGNATURE.with(|f| sip.siphash_update(&f.borrow()[..], SIG_SIZE_BYTES as u64));
let hash_out = sip.siphash_digest();
out[..SIG_SIZE_BYTES].copy_from_slice(&hash_out[..SIG_SIZE_BYTES]);
}

pub fn top_check_validate_unsat(out_signature_or_null: &mut Option<Vec<u8>>) -> bool {
let ok = lrat_check_validate_unsat();
VALID.with(|v| *v.borrow_mut() &= ok);
if !top_check_valid() {
return false;
}
if out_signature_or_null.is_some() {
let mut sig = vec![0u8; SIG_SIZE_BYTES];
FORMULA_SIGNATURE.with(|f| confirm_result(&f.borrow()[..], 20, &mut sig));
*out_signature_or_null = Some(sig);
}
true
}

pub fn top_check_produce(
id: u64,
literals: &[i32],
nb_literals: i32,
hints: &[u64],
nb_hints: i32,
out_sig_or_null: &mut Option<Vec<u8>>,
) -> bool {
let ok = lrat_check_add_clause(id, literals, nb_literals, hints, nb_hints);
VALID.with(|v| *v.borrow_mut() &= ok);
if !top_check_valid() {
return false;
}
if out_sig_or_null.is_some() {
let mut sig = vec![0u8; SIG_SIZE_BYTES];
compute_clause_signature(id, literals, nb_literals, &mut sig);
*out_sig_or_null = Some(sig);
}
true
}
