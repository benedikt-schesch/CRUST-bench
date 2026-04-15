// Generated Rust Code
#[macro_export]
#[cfg(debug_assertions)]
macro_rules! check {
($x:expr) => {
assert!($x);
};
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! check {
($x:expr) => {
()
};
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debugin {
($stm:stmt) => {
$stm
};
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debugin {
($stm:stmt) => {
()
};
}
