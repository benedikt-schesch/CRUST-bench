
pub fn exclusive_or(a: bool, b: bool) -> bool {
(a || b) && !(a && b)
}
pub fn roundn(val: f64, n: u32) -> f64 {
assert!(n > 0);
let x = 10_f64.powi(n as i32);
(val * x).round() / x
}
pub fn custom_assert(condition: i32) {
if condition == 0 {
println!("Assertion failed.");
print_call_stack();
panic!("Assertion failed");
}
}
pub fn print_call_stack() {
println!("----------------------------- STACK TRACE ------------------------------");
let bt = std::backtrace::Backtrace::capture();
println!("{bt}");
println!("------------------------------------------------------------------------");
}
