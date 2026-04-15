pub fn print_bin(n: u64) -> String {
let mut result = String::with_capacity(64);
for i in (0..64).rev() {
if n & (1u64 << i) != 0 {
result.push('1');
} else {
result.push('0');
}
}
result
}
