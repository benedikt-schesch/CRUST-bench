use std::collections::HashMap;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CardSuit {
Spade,
Heart,
Club,
Diamond,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CardRank {
Ace,
R2,
R3,
R4,
R5,
R6,
R7,
R8,
R9,
R10,
J,
Q,
K,
InvalidRank,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CardSuitRank {
SpadeAce,
Spade2,
Spade3,
Spade4,
Spade5,
Spade6,
Spade7,
Spade8,
Spade9,
Spade10,
SpadeJ,
SpadeQ,
SpadeK,
HeartAce,
Heart2,
Heart3,
Heart4,
Heart5,
Heart6,
Heart7,
Heart8,
Heart9,
Heart10,
HeartJ,
HeartQ,
HeartK,
ClubAce,
Club2,
Club3,
Club4,
Club5,
Club6,
Club7,
Club8,
Club9,
Club10,
ClubJ,
ClubQ,
ClubK,
DiamondAce,
Diamond2,
Diamond3,
Diamond4,
Diamond5,
Diamond6,
Diamond7,
Diamond8,
Diamond9,
Diamond10,
DiamondJ,
DiamondQ,
DiamondK,
CardCount,
}

impl CardSuitRank {
pub fn get_suit(&self) -> CardSuit {
match self {
CardSuitRank::SpadeAce | CardSuitRank::Spade2 | CardSuitRank::Spade3 |
CardSuitRank::Spade4 | CardSuitRank::Spade5 | CardSuitRank::Spade6 |
CardSuitRank::Spade7 | CardSuitRank::Spade8 | CardSuitRank::Spade9 |
CardSuitRank::Spade10 | CardSuitRank::SpadeJ | CardSuitRank::SpadeQ |
CardSuitRank::SpadeK => CardSuit::Spade,
CardSuitRank::HeartAce | CardSuitRank::Heart2 | CardSuitRank::Heart3 |
CardSuitRank::Heart4 | CardSuitRank::Heart5 | CardSuitRank::Heart6 |
CardSuitRank::Heart7 | CardSuitRank::Heart8 | CardSuitRank::Heart9 |
CardSuitRank::Heart10 | CardSuitRank::HeartJ | CardSuitRank::HeartQ |
CardSuitRank::HeartK => CardSuit::Heart,
CardSuitRank::ClubAce | CardSuitRank::Club2 | CardSuitRank::Club3 |
CardSuitRank::Club4 | CardSuitRank::Club5 | CardSuitRank::Club6 |
CardSuitRank::Club7 | CardSuitRank::Club8 | CardSuitRank::Club9 |
CardSuitRank::Club10 | CardSuitRank::ClubJ | CardSuitRank::ClubQ |
CardSuitRank::ClubK => CardSuit::Club,
CardSuitRank::DiamondAce | CardSuitRank::Diamond2 | CardSuitRank::Diamond3 |
CardSuitRank::Diamond4 | CardSuitRank::Diamond5 | CardSuitRank::Diamond6 |
CardSuitRank::Diamond7 | CardSuitRank::Diamond8 | CardSuitRank::Diamond9 |
CardSuitRank::Diamond10 | CardSuitRank::DiamondJ | CardSuitRank::DiamondQ |
CardSuitRank::DiamondK => CardSuit::Diamond,
CardSuitRank::CardCount => CardSuit::Spade, // Default for invalid
}
}

pub fn get_rank(&self) -> CardRank {
match self {
CardSuitRank::SpadeAce | CardSuitRank::HeartAce | CardSuitRank::ClubAce | CardSuitRank::DiamondAce => CardRank::Ace,
CardSuitRank::Spade2 | CardSuitRank::Heart2 | CardSuitRank::Club2 | CardSuitRank::Diamond2 => CardRank::R2,
CardSuitRank::Spade3 | CardSuitRank::Heart3 | CardSuitRank::Club3 | CardSuitRank::Diamond3 => CardRank::R3,
CardSuitRank::Spade4 | CardSuitRank::Heart4 | CardSuitRank::Club4 | CardSuitRank::Diamond4 => CardRank::R4,
CardSuitRank::Spade5 | CardSuitRank::Heart5 | CardSuitRank::Club5 | CardSuitRank::Diamond5 => CardRank::R5,
CardSuitRank::Spade6 | CardSuitRank::Heart6 | CardSuitRank::Club6 | CardSuitRank::Diamond6 => CardRank::R6,
CardSuitRank::Spade7 | CardSuitRank::Heart7 | CardSuitRank::Club7 | CardSuitRank::Diamond7 => CardRank::R7,
CardSuitRank::Spade8 | CardSuitRank::Heart8 | CardSuitRank::Club8 | CardSuitRank::Diamond8 => CardRank::R8,
CardSuitRank::Spade9 | CardSuitRank::Heart9 | CardSuitRank::Club9 | CardSuitRank::Diamond9 => CardRank::R9,
CardSuitRank::Spade10 | CardSuitRank::Heart10 | CardSuitRank::Club10 | CardSuitRank::Diamond10 => CardRank::R10,
CardSuitRank::SpadeJ | CardSuitRank::HeartJ | CardSuitRank::ClubJ | CardSuitRank::DiamondJ => CardRank::J,
CardSuitRank::SpadeQ | CardSuitRank::HeartQ | CardSuitRank::ClubQ | CardSuitRank::DiamondQ => CardRank::Q,
CardSuitRank::SpadeK | CardSuitRank::HeartK | CardSuitRank::ClubK | CardSuitRank::DiamondK => CardRank::K,
CardSuitRank::CardCount => CardRank::InvalidRank,
}
}

pub fn cardtostr(&self) -> Option<String> {
if *self == CardSuitRank::CardCount {
return None;
}
let suit_char = match self.get_suit() {
CardSuit::Spade => 'S',
CardSuit::Heart => 'H',
CardSuit::Club => 'C',
CardSuit::Diamond => 'D',
};
let rank_str = match self.get_rank() {
CardRank::Ace => "A".to_string(),
CardRank::R2 => "2".to_string(),
CardRank::R3 => "3".to_string(),
CardRank::R4 => "4".to_string(),
CardRank::R5 => "5".to_string(),
CardRank::R6 => "6".to_string(),
CardRank::R7 => "7".to_string(),
CardRank::R8 => "8".to_string(),
CardRank::R9 => "9".to_string(),
CardRank::R10 => "10".to_string(),
CardRank::J => "J".to_string(),
CardRank::Q => "Q".to_string(),
CardRank::K => "K".to_string(),
CardRank::InvalidRank => return None,
};
Some(format!("{}{}", suit_char, rank_str))
}
}

impl CardRank {
pub fn strtorank(s: &str) -> CardRank {
match s {
"ace" | "Ace" | "ACE" | "a" | "A" => CardRank::Ace,
"2" => CardRank::R2,
"3" => CardRank::R3,
"4" => CardRank::R4,
"5" => CardRank::R5,
"6" => CardRank::R6,
"7" => CardRank::R7,
"8" => CardRank::R8,
"9" => CardRank::R9,
"10" => CardRank::R10,
"jack" | "Jack" | "JACK" | "j" | "J" => CardRank::J,
"queen" | "Queen" | "QUEEN" | "q" | "Q" => CardRank::Q,
"king" | "King" | "KING" | "k" | "K" => CardRank::K,
_ => CardRank::InvalidRank,
}
}

pub fn ranktostr(&self) -> Option<String> {
match self {
CardRank::Ace => Some("A".to_string()),
CardRank::R2 => Some("2".to_string()),
CardRank::R3 => Some("3".to_string()),
CardRank::R4 => Some("4".to_string()),
CardRank::R5 => Some("5".to_string()),
CardRank::R6 => Some("6".to_string()),
CardRank::R7 => Some("7".to_string()),
CardRank::R8 => Some("8".to_string()),
CardRank::R9 => Some("9".to_string()),
CardRank::R10 => Some("10".to_string()),
CardRank::J => Some("J".to_string()),
CardRank::Q => Some("Q".to_string()),
CardRank::K => Some("K".to_string()),
CardRank::InvalidRank => None,
}
}
}

#[derive(Debug, Clone)]
pub struct Card {
suit_rank: CardSuitRank,
}

impl Card {
pub fn create_card(suit_rank: CardSuitRank) -> Option<Card> {
if suit_rank == CardSuitRank::CardCount {
None
} else {
Some(Card { suit_rank })
}
}

pub fn strtocard(s: &str) -> Option<Card> {
if s.len() < 2 {
return None;
}
let suit_char = s.chars().next().unwrap();
let rank_str = &s[1..];

let suit = match suit_char {
'S' | 's' => CardSuit::Spade,
'H' | 'h' => CardSuit::Heart,
'C' | 'c' => CardSuit::Club,
'D' | 'd' => CardSuit::Diamond,
_ => return None,
};

let rank = CardRank::strtorank(rank_str);
if rank == CardRank::InvalidRank {
return None;
}

// Find the CardSuitRank that matches
for csr in [
CardSuitRank::SpadeAce, CardSuitRank::Spade2, CardSuitRank::Spade3, CardSuitRank::Spade4,
CardSuitRank::Spade5, CardSuitRank::Spade6, CardSuitRank::Spade7, CardSuitRank::Spade8,
CardSuitRank::Spade9, CardSuitRank::Spade10, CardSuitRank::SpadeJ, CardSuitRank::SpadeQ, CardSuitRank::SpadeK,
CardSuitRank::HeartAce, CardSuitRank::Heart2, CardSuitRank::Heart3, CardSuitRank::Heart4,
CardSuitRank::Heart5, CardSuitRank::Heart6, CardSuitRank::Heart7, CardSuitRank::Heart8,
CardSuitRank::Heart9, CardSuitRank::Heart10, CardSuitRank::HeartJ, CardSuitRank::HeartQ, CardSuitRank::HeartK,
CardSuitRank::ClubAce, CardSuitRank::Club2, CardSuitRank::Club3, CardSuitRank::Club4,
CardSuitRank::Club5, CardSuitRank::Club6, CardSuitRank::Club7, CardSuitRank::Club8,
CardSuitRank::Club9, CardSuitRank::Club10, CardSuitRank::ClubJ, CardSuitRank::ClubQ, CardSuitRank::ClubK,
CardSuitRank::DiamondAce, CardSuitRank::Diamond2, CardSuitRank::Diamond3, CardSuitRank::Diamond4,
CardSuitRank::Diamond5, CardSuitRank::Diamond6, CardSuitRank::Diamond7, CardSuitRank::Diamond8,
CardSuitRank::Diamond9, CardSuitRank::Diamond10, CardSuitRank::DiamondJ, CardSuitRank::DiamondQ, CardSuitRank::DiamondK,
].iter() {
if csr.get_suit() == suit && csr.get_rank() == rank {
return Some(Card { suit_rank: *csr });
}
}
None
}

pub fn get_card_suit_rank(&self) -> CardSuitRank {
self.suit_rank
}

pub fn get_card_rank(&self) -> CardRank {
self.suit_rank.get_rank()
}

pub fn get_card_suit(&self) -> CardSuit {
self.suit_rank.get_suit()
}
}

pub struct CardDeck {
cards: Vec<CardSuitRank>,
}

impl CardDeck {
pub fn create_shuffled_deck() -> Option<CardDeck> {
let mut cards = vec![
CardSuitRank::SpadeAce, CardSuitRank::Spade2, CardSuitRank::Spade3, CardSuitRank::Spade4,
CardSuitRank::Spade5, CardSuitRank::Spade6, CardSuitRank::Spade7, CardSuitRank::Spade8,
CardSuitRank::Spade9, CardSuitRank::Spade10, CardSuitRank::SpadeJ, CardSuitRank::SpadeQ, CardSuitRank::SpadeK,
CardSuitRank::HeartAce, CardSuitRank::Heart2, CardSuitRank::Heart3, CardSuitRank::Heart4,
CardSuitRank::Heart5, CardSuitRank::Heart6, CardSuitRank::Heart7, CardSuitRank::Heart8,
CardSuitRank::Heart9, CardSuitRank::Heart10, CardSuitRank::HeartJ, CardSuitRank::HeartQ, CardSuitRank::HeartK,
CardSuitRank::ClubAce, CardSuitRank::Club2, CardSuitRank::Club3, CardSuitRank::Club4,
CardSuitRank::Club5, CardSuitRank::Club6, CardSuitRank::Club7, CardSuitRank::Club8,
CardSuitRank::Club9, CardSuitRank::Club10, CardSuitRank::ClubJ, CardSuitRank::ClubQ, CardSuitRank::ClubK,
CardSuitRank::DiamondAce, CardSuitRank::Diamond2, CardSuitRank::Diamond3, CardSuitRank::Diamond4,
CardSuitRank::Diamond5, CardSuitRank::Diamond6, CardSuitRank::Diamond7, CardSuitRank::Diamond8,
CardSuitRank::Diamond9, CardSuitRank::Diamond10, CardSuitRank::DiamondJ, CardSuitRank::DiamondQ, CardSuitRank::DiamondK,
];
let mut rng = thread_rng();
cards.shuffle(&mut rng);
Some(CardDeck { cards })
}

pub fn count_card_in_deck(&self, card: CardSuitRank) -> usize {
self.cards.iter().filter(|&&c| c == card).count()
}

pub fn strip_card_from_deck(&mut self, card: CardSuitRank) {
if let Some(pos) = self.cards.iter().position(|&c| c == card) {
self.cards.remove(pos);
}
}

pub fn deal_card(&mut self) -> Option<CardSuitRank> {
self.cards.pop()
}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItrAction {
Continue,
Stop,
}

pub struct CardHand {
cards: Vec<CardSuitRank>,
max_size: usize,
}

impl CardHand {
pub fn create_hand<F>(max_size: usize, mut sort_fn: F) -> Option<CardHand>
where
F: FnMut(u64, u64, &Option<Card>) -> ItrAction,
{
let mut hand = CardHand {
cards: Vec::new(),
max_size,
};

// Initialize with empty cards up to max_size for the sorting callback
for i in 0..max_size {
let card: Option<Card> = None;
let action = sort_fn(max_size as u64, i as u64, &card);
if action == ItrAction::Stop {
break;
}
}

Some(hand)
}

pub fn add_card(&mut self, card: CardSuitRank) {
if self.cards.len() < self.max_size {
self.cards.push(card);
}
}

pub fn get_max_rank_of_hand(&self) -> CardRank {
if self.cards.is_empty() {
return CardRank::InvalidRank;
}
self.cards.iter().map(|c| c.get_rank()).max().unwrap_or(CardRank::InvalidRank)
}

pub fn remove_from_hand(&mut self, card: CardSuitRank) {
if let Some(pos) = self.cards.iter().position(|&c| c == card) {
self.cards.remove(pos);
}
}
}

pub fn sort_card_by_rank(_len: u64, _pos: u64, _c: &Option<Card>) -> ItrAction {
ItrAction::Continue
}
