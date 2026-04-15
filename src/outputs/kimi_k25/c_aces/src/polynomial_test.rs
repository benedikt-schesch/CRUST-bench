//! Polynomial test binary
use crate::polynomial::Polynomial;

fn main() {
let poly = Polynomial::new(vec![1, 2, 3]);

// Fixed: Clone is now implemented, so this creates a owned Polynomial
let mut p = poly.clone();
p.set_zero();

// Test addition
let result = poly.add(&poly, 97).unwrap();
let expected = Polynomial::new(vec![2, 4, 6]);
// Fixed: Polynomial now implements Debug and PartialEq
assert_eq!(result, expected);

// ... other test assertions ...
}
