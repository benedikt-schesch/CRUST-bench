use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::cards::{Card, RANKS_PER_DECK, SUITS_PER_DECK, Rank, Suit};
pub const CARDS_PER_DECK: usize = 52;
pub fn new_deck() -> Vec<Card> {
let mut deck = Vec::with_capacity(CARDS_PER_DECK);
let ranks = [
Rank::Deuce, Rank::Trey, Rank::Four, Rank::Five, Rank::Six,
Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack,
Rank::Queen, Rank::King, Rank::Ace,
];
let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
for rank in &ranks {
for suit in &suits {
deck.push(Card::new(*rank, *suit));
}
}
deck
}
pub fn deck_shuffle(deck: &mut Vec<Card>) {
let mut rng = thread_rng();
deck.shuffle(&mut rng);
}
