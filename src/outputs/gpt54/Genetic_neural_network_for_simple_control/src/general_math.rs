// Generated Rust Code
use rand::Rng;

pub fn createRandomFloat(min: f32, max: f32) -> f32 {
let mut rng = rand::thread_rng();
let random: f32 = rng.r#gen();
let diff = max - min;
let r = random * diff;
min + r
}
