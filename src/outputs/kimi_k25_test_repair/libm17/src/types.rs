
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
Stream,
Packet,
Reserved,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreambleType {
Standard,
Short,
}
#[derive(Debug, Clone, Copy)]
pub struct LSF {
pub data: [u8; 30],
}
pub const SYM_PER_FRA: usize = 192;
pub const SYM_PER_PLD: usize = 184;
pub const SYM_PER_SWD: usize = 8;
