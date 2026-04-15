use crate::simple_sparsehash;

fn main() {
// Test sparse array functionality
let mut arr = simple_sparsehash::sparse_array_init(std::mem::size_of::<i32>(), 100)
.expect("sparse_array_init failed");

// Set values
let val1: i32 = 42;
let val1_bytes = val1.to_ne_bytes();
let result = simple_sparsehash::sparse_array_set(&mut arr, 0, &val1_bytes, val1_bytes.len());
assert_eq!(result, 1);

let val2: i32 = 100;
let val2_bytes = val2.to_ne_bytes();
let result = simple_sparsehash::sparse_array_set(&mut arr, 1, &val2_bytes, val2_bytes.len());
assert_eq!(result, 1);

// Get values back
let mut size = 0;
let returned = simple_sparsehash::sparse_array_get(&arr, 0, Some(&mut size))
.expect("Failed to get value at index 0");
assert_eq!(size, std::mem::size_of::<i32>());
assert_eq!(returned.len(), std::mem::size_of::<i32>());
let retrieved = i32::from_ne_bytes(returned.try_into().unwrap());
assert_eq!(retrieved, 42);

let mut size2 = 0;
let returned2 = simple_sparsehash::sparse_array_get(&arr, 1, Some(&mut size2))
.expect("Failed to get value at index 1");
assert_eq!(size2, std::mem::size_of::<i32>());
let retrieved2 = i32::from_ne_bytes(returned2.try_into().unwrap());
assert_eq!(retrieved2, 100);

// Test sparse dictionary functionality
let mut dict = simple_sparsehash::sparse_dict_init()
.expect("sparse_dict_init failed");

// Set key-value pairs
let key = "test_key";
let value = "test_value";
let result = simple_sparsehash::sparse_dict_set(&mut dict, key, key.len(), value.as_bytes(), value.len());
assert_eq!(result, 1);

// Verify bucket_count
assert_eq!(dict.bucket_count, 1);

// Get value back
let mut val_size = 0;
let retrieved_val = simple_sparsehash::sparse_dict_get(&dict, key, key.len(), Some(&mut val_size))
.expect("Failed to get value");
assert_eq!(val_size, value.len());
assert_eq!(retrieved_val, value.as_bytes());

// Test SimpleSparseHash
let _hash = simple_sparsehash::SimpleSparseHash::new();
println!("All tests passed successfully");

// Cleanup
simple_sparsehash::sparse_array_free(arr);
simple_sparsehash::sparse_dict_free(dict);
}
