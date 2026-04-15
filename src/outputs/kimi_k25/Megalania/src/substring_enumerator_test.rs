//! Tests for substring enumerator.

use crate::substring_enumerator::SubstringEnumerator;

fn main() {
let data = b"hello world";
let enumerator = SubstringEnumerator::new(data.to_vec(), 2, 273);

// Test with_data constructor
let enumerator2 = SubstringEnumerator::with_data(data.to_vec());

let data2 = b"hello world";
let enumerator3 = SubstringEnumerator::new(data2.to_vec(), 2, 3);

let data3 = b"test dat";
let enumerator4 = SubstringEnumerator::new(data3.to_vec(), 2, 273);

// Test methods
let _ = enumerator.data();
let _ = enumerator.find_next_match(0);
}
