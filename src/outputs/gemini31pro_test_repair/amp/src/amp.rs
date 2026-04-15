
pub const AMP_VERSION: u32 = 1;
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Amp {
pub version: u32,
}
pub fn amp_encode<T>(_args: &T) -> Vec<u8> {
Vec::new()
}
