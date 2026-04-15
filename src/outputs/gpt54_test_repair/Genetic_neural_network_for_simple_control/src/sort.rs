pub fn quickSort(fit:&mut Vec<f32>, input:&mut Vec<i32>, length:i32){
let len = length as usize;
if len <= 1 {
return;
}
let index = len / 2;
let pivot = input[index];
let pivot_fit = fit[pivot as usize];
let mut array_more: Vec<i32> = Vec::new();
let mut array_less: Vec<i32> = Vec::new();
for i in 0..len {
if i != index {
if fit[input[i] as usize] >= pivot_fit {
array_more.push(input[i]);
} else {
array_less.push(input[i]);
}
}
}
if array_more.len() > 1 {
let more_len = array_more.len() as i32;
quickSort(fit, &mut array_more, more_len);
}
if array_less.len() > 1 {
let less_len = array_less.len() as i32;
quickSort(fit, &mut array_less, less_len);
}
let mut index_global = 0usize;
for v in array_less {
input[index_global] = v;
index_global += 1;
}
input[index_global] = pivot;
index_global += 1;
for v in array_more {
input[index_global] = v;
index_global += 1;
}
}
