
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LZMAPacketType {
Literal,
Match,
ShortRep,
LongRep,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LZMAPacket {
pub packet_type: LZMAPacketType,
pub length: usize,
pub distance: u32,
}
impl LZMAPacket {
pub fn literal() -> Self {
Self {
packet_type: LZMAPacketType::Literal,
length: 1,
distance: 0,
}
}
pub fn match_packet(length: usize, distance: u32) -> Self {
Self {
packet_type: LZMAPacketType::Match,
length,
distance,
}
}
pub fn short_rep_packet() -> Self {
Self {
packet_type: LZMAPacketType::ShortRep,
length: 1,
distance: 0,
}
}
}
