// Generated Rust Code
pub fn print_array(matrix: &[Vec<f32>], name: &str, typ: &str) -> i32 {
println!("-------------{}--------------", name);
if typ == "float" {
print_float_array(matrix);
} else {
print!("unsupported {}", typ);
}
0
}

pub fn print_float_array(matrix: &[Vec<f32>]) -> i32 {
let x = matrix.len();
let y = if x > 0 { matrix[0].len() } else { 0 };
for i in 0..x {
for j in 0..y {
print!("{:10.2} ", matrix[i][j]);
}
println!();
}
0
}
