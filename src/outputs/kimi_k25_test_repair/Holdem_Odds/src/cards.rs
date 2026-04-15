pub const RANKS_PER_DECK: usize = 13;
pub const SUITS_PER_DECK: usize = 4;
pub const RANK_CHARS: &str = "23456789TJQKA";
pub const SUIT_CHARS: &str = "cdhs";
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub enum Suit {
InvalidSuit,
Club,
Diamond,
Heart,
Spade,
}
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub enum Rank {
InvalidRank,
Deuce,
Trey,
Four,
Five,
Six,
Seven,
Eight,
Nine,
Ten,
Jack,
Queen,
King,
Ace,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Card {
pub rank: Rank,
pub suit: Suit,
}
impl Card {
pub fn new(rank: Rank, suit: Suit) -> Self {
let card = Card { rank, suit };
if card.is_valid() {
card
} else {
Card {
rank: Rank::InvalidRank,
suit: Suit::InvalidSuit,
}
}
}
pub fn from_string(s: &str) -> Card {
if s.len() != 2 {
Card {
rank: Rank::InvalidRank,
suit: Suit::InvalidSuit,
}
} else {
let mut chars = s.chars();
let r = chars.next().unwrap();
let s = chars.next().unwrap();
Self::from_chars(r, s)
}
}
pub fn from_chars(rank: char, suit: char) -> Card {
let r = Rank::from_char(rank);
let s = Suit::from_char(suit);
Card::new(r, s)
}
pub fn is_valid(&self) -> bool {
self.rank >= Rank::Deuce
&& self.rank <= Rank::Ace
&& self.suit >= Suit::Club
&& self.suit <= Suit::Spade
}
pub fn to_string(&self) -> String {
if !self.is_valid() {
"--".to_string()
} else {
format!("{}{}", self.rank.to_char(), self.suit.to_char())
}
}
}
pub fn card_swap(card1: &mut Card, card2: &mut Card) {
std::mem::swap(card1, card2);
}
impl Rank {
pub fn from_char(c: char) -> Rank {
match index_of(c, RANK_CHARS) {
Some(idx) => match idx + 1 {
1 => Rank::Deuce,
2 => Rank::Trey,
3 => Rank::Four,
4 => Rank::Five,
5 => Rank::Six,
6 => Rank::Seven,
7 => Rank::Eight,
8 => Rank::Nine,
9 => Rank::Ten,
10 => Rank::Jack,
11 => Rank::Queen,
12 => Rank::King,
13 => Rank::Ace,
_ => Rank::InvalidRank,
},
None => Rank::InvalidRank,
}
}
pub fn to_char(&self) -> char {
match self {
Rank::Deuce => '2',
Rank::Trey => '3',
Rank::Four => '4',
Rank::Five => '5',
Rank::Six => '6',
Rank::Seven => '7',
Rank::Eight => '8',
Rank::Nine => '9',
Rank::Ten => 'T',
Rank::Jack => 'J',
Rank::Queen => 'Q',
Rank::King => 'K',
Rank::Ace => 'A',
Rank::InvalidRank => '-',
}
}
}
impl Suit {
pub fn from_char(c: char) -> Suit {
match index_of(c, SUIT_CHARS) {
Some(idx) => match idx + 1 {
1 => Suit::Club,
2 => Suit::Diamond,
3 => Suit::Heart,
4 => Suit::Spade,
_ => Suit::InvalidSuit,
},
None => Suit::InvalidSuit,
}
}
pub fn to_char(&self) -> char {
match self {
Suit::Club => 'c',
Suit::Diamond => 'd',
Suit::Heart => 'h',
Suit::Spade => 's',
Suit::InvalidSuit => '-',
}
}
}
pub fn index_of(c: char, chars: &str) -> Option<usize> {
chars.chars().position(|ch| ch == c)
}
pub fn char_to_rank(c: char) -> Rank {
Rank::from_char(c)
}
pub fn char_to_suit(c: char) -> Suit {
Suit::from_char(c)
}
pub fn new_card_from_chars(rank: char, suit: char) -> Card {
Card::from_chars(rank, suit)
}
pub fn card_compare(card1: &Card, card2: &Card) -> i32 {
(card1.rank as i32) - (card2.rank as i32)
}
pub fn card_equal(card1: &Card, card2: &Card) -> bool {
card1.rank == card2.rank && card1.suit == card2.suit
}
