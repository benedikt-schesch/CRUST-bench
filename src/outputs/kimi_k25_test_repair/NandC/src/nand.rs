
pub const U4_MAX: u8 = 0b1111;
pub type U4 = u8;
pub fn nand(a: bool, b: bool) -> bool {
!(a && b)
}
pub fn not(a: bool) -> bool {
nand(a, true)
}
pub fn or(a: bool, b: bool) -> bool {
nand(not(a), not(b))
}
pub fn and(a: bool, b: bool) -> bool {
not(nand(a, b))
}
pub fn xor(a: bool, b: bool) -> bool {
or(
and(a, not(b)),
and(not(a), b)
)
}
pub fn add_bit(a: bool, b: bool, carry: bool, carry_result: &mut bool) -> bool {
*carry_result = and(
or(a, b),
or(
and(a, b),
carry
)
);
xor(xor(a, b), carry)
}
pub fn half_sub(a: bool, b: bool, carry_result: &mut bool) -> bool {
*carry_result = and(not(a), b);
xor(a, b)
}
pub fn sub_bit(a: bool, b: bool, carry: bool, carry_result: &mut bool) -> bool {
let mut b1 = false;
let mut b2 = false;
let temp = half_sub(a, b, &mut b1);
let result = half_sub(temp, carry, &mut b2);
*carry_result = or(b1, b2);
result
}
pub fn bll(x: bool) -> &'static str {
if x { "true" } else { "false" }
}
pub fn print_add_bit(a: bool, b: bool, carry: bool) {
println!("a      : {}", bll(a));
println!("b      : {}", bll(b));
println!("carry  : {}", bll(carry));
let mut res_carry = false;
let bit_result = add_bit(a, b, carry, &mut res_carry);
println!("result :");
println!("  bit  : {}", bll(bit_result));
println!("  carry: {}", bll(res_carry));
}
pub fn add_u4(a: U4, b: U4) -> U4 {
let mut carry = false;
let mut result: U4 = 0;
result |= (add_bit((a >> 0) & 0b1 != 0, (b >> 0) & 0b1 != 0, carry, &mut carry) as u8) << 0;
result |= (add_bit((a >> 1) & 0b1 != 0, (b >> 1) & 0b1 != 0, carry, &mut carry) as u8) << 1;
result |= (add_bit((a >> 2) & 0b1 != 0, (b >> 2) & 0b1 != 0, carry, &mut carry) as u8) << 2;
result |= (add_bit((a >> 3) & 0b1 != 0, (b >> 3) & 0b1 != 0, carry, &mut carry) as u8) << 3;
result
}
pub fn sub_u4(a: U4, b: U4) -> U4 {
let mut carry = false;
let mut result: U4 = 0;
result |= (sub_bit((a >> 0) & 0b1 != 0, (b >> 0) & 0b1 != 0, carry, &mut carry) as u8) << 0;
result |= (sub_bit((a >> 1) & 0b1 != 0, (b >> 1) & 0b1 != 0, carry, &mut carry) as u8) << 1;
result |= (sub_bit((a >> 2) & 0b1 != 0, (b >> 2) & 0b1 != 0, carry, &mut carry) as u8) << 2;
result |= (sub_bit((a >> 3) & 0b1 != 0, (b >> 3) & 0b1 != 0, carry, &mut carry) as u8) << 3;
result
}
pub fn check_add() -> bool {
add_u4(0, 0) == 0 &&
add_u4(1, 1) == 2 &&
add_u4(5, 3) == 8 &&
add_u4(15, 1) == 0
}
pub fn check_sub() -> bool {
sub_u4(0, 0) == 0 &&
sub_u4(5, 3) == 2 &&
sub_u4(3, 5) == 14 &&
sub_u4(15, 1) == 14
}
