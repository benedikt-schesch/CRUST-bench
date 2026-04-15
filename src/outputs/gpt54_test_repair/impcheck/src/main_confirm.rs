use crate::confirm::confirm_result;
use crate::trusted_utils::{trusted_utils_equal_signatures, trusted_utils_log_err, trusted_utils_str_to_sig, SIG_SIZE_BYTES};
pub fn error() -> i32 {
println!("s NOT VERIFIED");
1
}
pub fn main(_argc: i32, argv: Vec<String>) -> i32 {
let mut formula_input = String::new();
let mut result_sig = String::new();
let mut resultint_str = String::new();
for arg in argv.iter() {
if let Some(v) = arg.strip_prefix("-formula-input=") {
formula_input = v.to_string();
}
if let Some(v) = arg.strip_prefix("-result-sig=") {
result_sig = v.to_string();
}
if let Some(v) = arg.strip_prefix("-result=") {
resultint_str = v.to_string();
}
}
let _ = formula_input;
let result: i32 = resultint_str.parse().unwrap_or(0);
if result != 10 && result != 20 {
trusted_utils_log_err("Result code missing or invalid");
return error();
}
if result_sig.len() != 2 * SIG_SIZE_BYTES {
trusted_utils_log_err("Result signature missing or malformed");
return error();
}
let sig_formula = vec![0u8; SIG_SIZE_BYTES];
let mut sig_res_reported = [0u8; SIG_SIZE_BYTES];
if !trusted_utils_str_to_sig(&result_sig, &mut sig_res_reported) {
trusted_utils_log_err("Invalid signature string");
return error();
}
let mut sig_res_computed = [0u8; SIG_SIZE_BYTES];
confirm_result(&sig_formula, result as u8, &mut sig_res_computed);
if !trusted_utils_equal_signatures(&sig_res_computed, &sig_res_reported) {
trusted_utils_log_err("Signature does not match!");
return error();
}
if result == 10 {
println!("s VERIFIED SATISFIABLE");
}
if result == 20 {
println!("s VERIFIED UNSATISFIABLE");
}
0
}
