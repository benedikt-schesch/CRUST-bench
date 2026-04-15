use clhash::clhash::{clhash, get_random_key_for_clhash};

fn main() {
// Generate a random key
let key = get_random_key_for_clhash();

// Sample data to hash
let data = b"Hello, CLHash world!";

// Compute hash
let hash_value = clhash(&key, data);

println!("Data: {:?}", std::str::from_utf8(data).unwrap());
println!("CLHash value: {:016x}", hash_value);
}
