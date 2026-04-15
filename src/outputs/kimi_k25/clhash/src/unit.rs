use clhash::clhash::{clhash, get_random_key_for_clhash};

fn main() {
// Test that clhash is deterministic
let key = get_random_key_for_clhash();
let data = b"test data for clhash";

let hash1 = clhash(&key, data);
let hash2 = clhash(&key, data);

assert_eq!(hash1, hash2, "CLHash should be deterministic for same key and data");

// Test that different data produces different hashes (with high probability)
let data2 = b"test data for clhash!";
let hash3 = clhash(&key, data2);

assert_ne!(hash1, hash3, "Different data should produce different hashes");

println!("All unit tests passed!");
println!("Hash of '{:?}': {:016x}", std::str::from_utf8(data).unwrap(), hash1);
}
