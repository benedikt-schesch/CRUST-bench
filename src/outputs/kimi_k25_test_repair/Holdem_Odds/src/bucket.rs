use crate::cards::Card;
pub struct Bucket {
pub cards: [Option<Card>; 4],
pub count: usize,
}
impl Bucket {
pub fn new() -> Self {
Bucket {
cards: [None; 4],
count: 0,
}
}
pub fn add(&mut self, card: Card) {
if self.count < 4 {
self.cards[self.count] = Some(card);
self.count += 1;
}
}
}
