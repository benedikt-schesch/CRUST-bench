#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameType {
Lsf,
Stream,
Packet,
Bert,
Eot,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreambleType {
Lsf,
Stream,
Packet,
Bert,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LSF {
pub dst: [u8; 6],
pub src: [u8; 6],
pub type_field: [u8; 2],
pub meta: [u8; 14],
pub crc: [u8; 2],
}
pub const SYM_PER_FRA: usize = 192;
pub const SYM_PER_PLD: usize = 184;
pub const SYM_PER_SWD: usize = 8;
pub const CHAR_MAP: &str = " ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-/.";
