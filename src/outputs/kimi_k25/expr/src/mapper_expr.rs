pub struct MapperExpr {
expression: String,
param1: i32,
param2: i32,
param3: i32,
}

pub enum MapperSignalValue {
F(f32),
}

pub fn mapper_expr_new_from_string(expr: &str, p1: i32, p2: i32, p3: i32) -> MapperExpr {
MapperExpr {
expression: expr.to_string(),
param1: p1,
param2: p2,
param3: p3,
}
}

pub fn mapper_expr_evaluate(e: &mut MapperExpr, val: &MapperSignalValue) -> f32 {
match val {
MapperSignalValue::F(f) => *f,
}
}
