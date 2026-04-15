use std::ops::{Deref, DerefMut};
pub struct Utf8String {
data: Vec<u8>,
}
impl Utf8String {
pub fn new() -> Self {
Self {
data: Vec::new(),
}
}
}
impl AsRef<[u8]> for Utf8String {
fn as_ref(&self) -> &[u8] {
&self.data
}
}
impl AsMut<[u8]> for Utf8String {
fn as_mut(&mut self) -> &mut [u8] {
&mut self.data
}
}
impl Deref for Utf8String {
type Target = [u8];
fn deref(&self) -> &[u8] {
&self.data
}
}
impl DerefMut for Utf8String {
fn deref_mut(&mut self) -> &mut [u8] {
&mut self.data
}
}
