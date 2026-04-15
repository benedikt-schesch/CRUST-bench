use crate::lzma_packet::LZMAPacket;
use crate::lzma_state::LZMAState;
use crate::substring_enumerator::SubstringEnumerator;
pub struct PacketEnumerator<'a> {
pub data: &'a [u8],
pub substring_enumerator: Box<SubstringEnumerator<'a>>,
}
impl<'a> PacketEnumerator<'a> {
pub fn memory_usage(data_size: usize) -> usize {
std::mem::size_of::<Self>() + SubstringEnumerator::memory_usage(data_size)
}
pub fn new(data: &'a [u8]) -> Self {
Self {
data,
substring_enumerator: Box::new(SubstringEnumerator::new(data, 2, 273)),
}
}
pub fn for_each<F>(&self, state: &LZMAState, callback: F)
where
F: Fn(&LZMAState, LZMAPacket),
{
assert!(std::ptr::eq(self.data.as_ptr(), state.data.as_ptr()) || self.data == state.data);
callback(state, LZMAPacket::literal_packet());
if state.position > 0 {
let rep0_position = state.position - state.dists[0] as usize - 1;
if rep0_position < self.data.len()
&& state.position < self.data.len()
&& self.data[state.position] == self.data[rep0_position]
{
callback(state, LZMAPacket::short_rep_packet());
}
}
self.substring_enumerator
.for_each(state.position, |offset, length| {
let dist = (state.position - offset - 1) as u32;
callback(state, LZMAPacket::match_packet(dist, length as u32));
for i in 0..4 {
if dist == state.dists[i] {
callback(state, LZMAPacket::long_rep_packet(i as u32, length as u32));
}
}
});
}
}
