use crate::dberror::RC;
use crate::record_mgr::get_attr;
use crate::tables::DataType;
use crate::tables::Record;
use crate::tables::Schema;
use crate::tables::Value;
use crate::tables::ValueUnion;
#[derive(Debug, Clone)]
pub enum ExprType {
ExprOp,
ExprConst,
ExprAttrRef,
}
#[derive(Debug, Clone)]
pub struct Expr {
pub expr_type: ExprType,
pub expr: ExprUnion,
}
#[derive(Debug, Clone)]
pub enum ExprUnion {
Cons(Value),
AttrRef(i32),
Op(Box<Operator>),
}
#[derive(Debug, Clone)]
pub struct Operator {
pub op_type: OpType,
pub args: Vec<Expr>,
}
#[derive(Debug, Clone)]
pub enum OpType {
OpBoolAnd,
OpBoolOr,
OpBoolNot,
OpCompEqual,
OpCompSmaller,
}
pub fn value_equals(left: &Value, right: &Value, result: &mut Value) -> RC {
if std::mem::discriminant(&left.dt) != std::mem::discriminant(&right.dt) {
return RC::RmCompareValueOfDifferentDatatype;
}
result.dt = DataType::DtBool;
result.v = ValueUnion::BoolV(match (&left.v, &right.v) {
(ValueUnion::IntV(a), ValueUnion::IntV(b)) => a == b,
(ValueUnion::FloatV(a), ValueUnion::FloatV(b)) => a == b,
(ValueUnion::BoolV(a), ValueUnion::BoolV(b)) => a == b,
(ValueUnion::StringV(a), ValueUnion::StringV(b)) => a == b,
_ => false,
});
RC::Ok
}
pub fn value_smaller(left: &Value, right: &Value, result: &mut Value) -> RC {
if std::mem::discriminant(&left.dt) != std::mem::discriminant(&right.dt) {
return RC::RmCompareValueOfDifferentDatatype;
}
result.dt = DataType::DtBool;
result.v = ValueUnion::BoolV(match (&left.v, &right.v) {
(ValueUnion::IntV(a), ValueUnion::IntV(b)) => a < b,
(ValueUnion::FloatV(a), ValueUnion::FloatV(b)) => a < b,
(ValueUnion::BoolV(a), ValueUnion::BoolV(b)) => (*a as i32) < (*b as i32),
(ValueUnion::StringV(a), ValueUnion::StringV(b)) => a < b,
_ => false,
});
RC::Ok
}
pub fn bool_not(input: &Value, result: &mut Value) -> RC {
if !matches!(input.dt, DataType::DtBool) {
return RC::RmBooleanExprArgIsNotBoolean;
}
result.dt = DataType::DtBool;
result.v = match input.v {
ValueUnion::BoolV(v) => ValueUnion::BoolV(!v),
_ => ValueUnion::BoolV(false),
};
RC::Ok
}
pub fn bool_and(left: &Value, right: &Value, result: &mut Value) -> RC {
if !matches!(left.dt, DataType::DtBool) || !matches!(right.dt, DataType::DtBool) {
return RC::RmBooleanExprArgIsNotBoolean;
}
result.dt = DataType::DtBool;
result.v = match (&left.v, &right.v) {
(ValueUnion::BoolV(a), ValueUnion::BoolV(b)) => ValueUnion::BoolV(*a && *b),
_ => ValueUnion::BoolV(false),
};
RC::Ok
}
pub fn bool_or(left: &Value, right: &Value, result: &mut Value) -> RC {
if !matches!(left.dt, DataType::DtBool) || !matches!(right.dt, DataType::DtBool) {
return RC::RmBooleanExprArgIsNotBoolean;
}
result.dt = DataType::DtBool;
result.v = match (&left.v, &right.v) {
(ValueUnion::BoolV(a), ValueUnion::BoolV(b)) => ValueUnion::BoolV(*a || *b),
_ => ValueUnion::BoolV(false),
};
RC::Ok
}
pub fn eval_expr(record: &Record, schema: &Schema, expr: &Expr, result: &mut Value) -> RC {
*result = Value {
dt: DataType::DtInt,
v: ValueUnion::IntV(-1),
};
match &expr.expr_type {
ExprType::ExprOp => {
if let ExprUnion::Op(op) = &expr.expr {
let mut l_in = Value {
dt: DataType::DtInt,
v: ValueUnion::IntV(-1),
};
let rc = eval_expr(record, schema, &op.args[0], &mut l_in);
if rc != RC::Ok {
return rc;
}
let two_args = !matches!(op.op_type, OpType::OpBoolNot);
let mut r_in = Value {
dt: DataType::DtInt,
v: ValueUnion::IntV(-1),
};
if two_args {
let rc2 = eval_expr(record, schema, &op.args[1], &mut r_in);
if rc2 != RC::Ok {
return rc2;
}
}
match op.op_type {
OpType::OpBoolNot => bool_not(&l_in, result),
OpType::OpBoolAnd => bool_and(&l_in, &r_in, result),
OpType::OpBoolOr => bool_or(&l_in, &r_in, result),
OpType::OpCompEqual => value_equals(&l_in, &r_in, result),
OpType::OpCompSmaller => value_smaller(&l_in, &r_in, result),
}
} else {
RC::Ok
}
}
ExprType::ExprConst => {
if let ExprUnion::Cons(v) = &expr.expr {
*result = v.clone();
}
RC::Ok
}
ExprType::ExprAttrRef => {
if let ExprUnion::AttrRef(a) = expr.expr {
get_attr(record, schema, a, result)
} else {
RC::Ok
}
}
}
}
pub fn free_expr(_expr: &mut Expr) -> RC {
RC::Ok
}
pub fn free_val(_val: &mut Value) {}
