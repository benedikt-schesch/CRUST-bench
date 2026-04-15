
use crate::polynomial::Polynomial;
fn main() {
let poly = Polynomial::new(vec![1, 2, 3]);
let mut p = poly.clone();
p.set_zero();
let result = poly.add(&poly, 97).unwrap();
let expected = Polynomial::new(vec![2, 4, 6]);
assert_eq!(result, expected);
}
