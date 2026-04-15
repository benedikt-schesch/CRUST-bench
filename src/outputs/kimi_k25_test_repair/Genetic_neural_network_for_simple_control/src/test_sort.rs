use crate::sort::quickSort;
fn main() {
let mut data = vec![3.0, 1.0, 4.0, 1.0, 5.0, 9.0, 2.0, 6.0];
quickSort(&mut data);
println!("Sorted: {:?}", data);
let mut int_data = vec![3, 1, 4, 1, 5, 9, 2, 6];
quickSort(&mut int_data);
println!("Sorted ints: {:?}", int_data);
}
