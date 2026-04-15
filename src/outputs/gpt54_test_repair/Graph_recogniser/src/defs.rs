
#[macro_export]
macro_rules! countof {
($array:expr) => {
$array.len()
};
}
