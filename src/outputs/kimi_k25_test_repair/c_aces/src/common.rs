
pub fn gcd(a: i64, b: i64) -> i64 {
if b == 0 {
a.abs()
} else {
gcd(b, a % b)
}
}
pub struct Xgcd {
pub gcd: i64,
pub a: i64,
pub b: i64,
}
pub fn xgcd(a: i64, b: i64) -> Xgcd {
if b == 0 {
Xgcd { gcd: a.abs(), a: 1, b: 0 }
} else {
let result = xgcd(b, a % b);
Xgcd {
gcd: result.gcd,
a: result.b,
b: result.a - (a / b) * result.b,
}
}
}
pub fn randrange(min: u64, max: u64) -> i64 {
(min + (max - min) / 2) as i64
}
