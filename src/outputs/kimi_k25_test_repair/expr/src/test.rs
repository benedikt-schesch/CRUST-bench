use crate::mapper_expr;
fn main() {
let expr = "x + 1";
let x = 5.0f32;
let expected = 6.0f32;
let mut e: mapper_expr::MapperExpr = mapper_expr::mapper_expr_new_from_string(expr, 1, 1, 1);
let evaluated = mapper_expr::mapper_expr_evaluate(&mut e, &(mapper_expr::MapperSignalValue::F(x)));
assert!((evaluated - expected).abs() < f32::EPSILON, "Test 1 failed: expected {}, got {:?}", expected, evaluated);
{
let expr = "x * 2";
let x = 10.0f32;
let expected = 20.0f32;
let mut e: mapper_expr::MapperExpr = mapper_expr::mapper_expr_new_from_string(expr, 0, 0, 1);
let evaluated = mapper_expr::mapper_expr_evaluate(&mut e, &(mapper_expr::MapperSignalValue::F(x)));
assert!((evaluated - expected).abs() < f32::EPSILON, "Test 2 failed for x={}: expected {}, got {:?}", x, expected, evaluated);
}
}
