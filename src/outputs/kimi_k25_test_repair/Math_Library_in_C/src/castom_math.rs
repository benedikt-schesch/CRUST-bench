pub const S21_NAN: f64 = f64::NAN;
pub const EPS_6: f64 = 1e-06;
pub const S21_M_E: f64 = 2.71828182845904523536028747135266250;
pub const S21_INFINITY: f64 = f64::INFINITY;
pub const EPS_10: f64 = 1e-10;
pub const S21_M_PI: f64 = 3.14159265358979323846264338327950288;

pub fn castom_floor(x: f64) -> f64 {
if x >= i64::MAX as f64 || x <= i64::MIN as f64 || x.is_nan() {
return x;
}
let truncation = x as i64 as f64;
if truncation > x {
truncation - 1.0
} else {
truncation
}
}

pub fn castom_tan(x: f64) -> f64 {
if x == 0.0 {
0.0
} else if (S21_M_PI / 6.0 - x).abs() < EPS_10 {
1.0 / castom_sqrt(3.0)
} else if (S21_M_PI / 4.0 - x).abs() < EPS_10 {
1.0
} else if (S21_M_PI / 3.0 - x).abs() < EPS_10 {
castom_sqrt(3.0)
} else if (S21_M_PI / 2.0 - x).abs() < EPS_10 {
S21_INFINITY
} else if (S21_M_PI - x).abs() < EPS_10 {
0.0
} else if (3.0 * S21_M_PI / 2.0 - x).abs() < EPS_10 {
S21_INFINITY
} else if (2.0 * S21_M_PI - x).abs() < EPS_10 {
0.0
} else {
castom_sin(x) / castom_cos(x)
}
}

pub fn castom_atan(x: f64) -> f64 {
castom_asin(x / castom_sqrt(1.0 + x * x))
}

pub fn castom_ceil(x: f64) -> f64 {
if x >= i64::MAX as f64 || x <= i64::MIN as f64 || x.is_nan() {
return x;
}
let truncation = x as i64 as f64;
if truncation < x {
truncation + 1.0
} else {
truncation
}
}

pub fn castom_sqrt(x: f64) -> f64 {
if x.is_nan() || x < 0.0 {
return f64::NAN;
}
if x == 0.0 {
return 0.0;
}
let mut sqrt = x / 2.0;
let mut temp = 0.0;
while (sqrt - temp).abs() > EPS_10 {
temp = sqrt;
sqrt = (x / temp + temp) / 2.0;
}
sqrt
}

pub fn castom_trunc(x: f64) -> f64 {
if x > 0.0 {
castom_floor(x)
} else {
castom_ceil(x)
}
}

pub fn castom_exp(x: f64) -> f64 {
if x.is_nan() {
return f64::NAN;
}
if x.is_infinite() {
return if x > 0.0 { f64::INFINITY } else { 0.0 };
}

// Range reduction: exp(x) = 2^k * exp(r) where r = x - k*ln(2)
// This ensures |r| <= 0.5*ln(2) ≈ 0.3466 for better numerical stability
const LN_2: f64 = 0.693147180559945309417232121458176568;
let k = (x / LN_2).round() as i32;
// Use mul_add for higher precision in the range reduction: x - k*LN_2
let r = (-(k as f64)).mul_add(LN_2, x);

// Compute exp(r) using Taylor series for the reduced range
let mut sum = 1.0;
let mut term = 1.0;
let mut n = 1;

// Use more iterations to ensure precision for large k values
while castom_fabs(term) > f64::EPSILON && n < 50 {
term *= r / n as f64;
sum += term;
n += 1;
}

// Scale back by 2^k
sum * (2.0_f64).powi(k)
}

pub fn castom_factorial(n: u64) -> u64 {
if n == 0 {
return 1;
}
let mut result = 1u64;
for i in 1..=n {
result = result.wrapping_mul(i);
}
result
}

pub fn castom_asin(x: f64) -> f64 {
if x > 1.0 || x < -1.0 {
return f64::NAN;
}
if x == 1.0 {
return S21_M_PI / 2.0;
}
if x == -1.0 {
return -S21_M_PI / 2.0;
}
let mut term = x;
let mut sum = term;
let x_sq = x * x;
let mut k = 1i32;
while castom_fabs(term) > EPS_10 {
term *= x_sq * k as f64 / (k as f64 + 1.0);
sum += term / (k as f64 + 2.0);
k += 2;
}
sum
}

pub fn castom_fmod(x: f64, y: f64) -> f64 {
x - castom_trunc(x / y) * y
}

pub fn castom_acos(x: f64) -> f64 {
S21_M_PI / 2.0 - castom_asin(x)
}

pub fn castom_pow(base: f64, exp: f64) -> f64 {
if base == 0.0 && exp != 0.0 {
return 0.0;
}
if exp < 0.0 {
return castom_pow(1.0 / base, -exp);
}
if base < 0.0 && exp != exp.trunc() {
return f64::NAN;
}
if exp == 0.0 {
return 1.0;
}

let mut flag = 1.0;
if base < 0.0 && (exp as i64) % 2 != 0 {
flag = -1.0;
}

let abs_base = base.abs();
let int_exp = exp as i64;
let frac_exp = exp - int_exp as f64;

// Calculate integer part using powi for better accuracy when possible
let int_result = if int_exp >= 0 && int_exp <= i32::MAX as i64 {
abs_base.powi(int_exp as i32)
} else {
// Fallback to exponentiation by squaring for very large exponents
let mut result = 1.0;
let mut b = abs_base;
let mut e = int_exp;
while e > 0 {
if e % 2 == 1 {
result *= b;
}
b *= b;
e /= 2;
}
result
};

// Calculate fractional part: exp(frac_exp * log(abs_base))
let frac_result = if frac_exp == 0.0 {
1.0
} else {
castom_exp(frac_exp * castom_log(abs_base))
};

int_result * frac_result * flag
}

pub fn castom_abs(x: i32) -> i32 {
if x > 0 {
x
} else {
-x
}
}

pub fn castom_log(x: f64) -> f64 {
let mut a: u32 = 0;
let mut b: f64 = 0.0;
if x > 0.0 {
let mut c: f64;
let mut f: f64;
let mut e: f64;
let mut d: u32;
if x < 1.0 {
c = 1.0 / x;
} else {
c = x;
}
loop {
c /= S21_M_E;
if c <= 1.0 {
break;
}
a += 1;
}
c = 1.0 / (c * S21_M_E - 1.0);
c = c + c + 1.0;
f = c * c;
b = 0.0;
d = 1;
c /= 2.0;
loop {
e = b;
b += 1.0 / (d as f64 * c);
if b == e {
break;
}
d += 2;
c *= f;
}
} else {
if x == 0.0 {
b = f64::INFINITY;
} else {
b = f64::NAN;
}
}
if x < 1.0 {
-(a as f64 + b)
} else {
a as f64 + b
}
}

pub fn castom_sin(x: f64) -> f64 {
let x = castom_fmod(x, 2.0 * S21_M_PI);
let mut sum = 0.0;
for i in 0..=20 {
let mut fa = 1.0;
let mut pow_val = 1.0;
let limit = 2 * i + 1;
for j in 1..=limit {
fa *= j as f64;
pow_val *= x;
}
sum += ((if i % 2 == 1 { -1.0 } else { 1.0 }) / fa) * pow_val;
}
sum
}

pub fn castom_cos(x: f64) -> f64 {
let x = castom_fmod(x, 2.0 * S21_M_PI);
let mut t_s = 0.0;
let mut last = 1.0;
let mut k = 1;
while castom_fabs(last) > EPS_10 {
t_s += last;
last *= -x * x / ((2.0 * k as f64 - 1.0) * (2.0 * k as f64));
k += 1;
}
t_s
}

pub fn castom_fabs(x: f64) -> f64 {
if x > 0.0 {
x
} else {
-x
}
}
