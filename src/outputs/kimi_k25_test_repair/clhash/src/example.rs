use clhash::clhash::{clhash, get_random_key_for_clhash};
fn main() {
let key = get_random_key_for_clhash();
let data = b"Hello, CLHash world!";
let hash_value = clhash(&key, data);
println!("Data: {:?}", std::str::from_utf8(data).unwrap());
println!("CLHash value: {:016x}", hash_value);
}
