pub const RANKS_PER_DECK: usize = 13;
pub const SUITS_PER_DECK: usize = 4;
pub const RANK_CHARS: &str = "23456789TJQKA";
pub const SUIT_CHARS: &str = "cdhs";

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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
if s.chars().count() != 2 {
Card {
rank: Rank::InvalidRank,
suit: Suit::InvalidSuit,
}
} else {
let mut it = s.chars();
let r = it.next().unwrap();
let su = it.next().unwrap();
Card::from_chars(r, su)
}
}

pub fn from_chars(rank: char, suit: char) -> Card {
let r = char_to_rank(rank);
let s = char_to_suit(suit);
Card::new(r, s)
}

pub fn is_valid(&self) -> bool {
self.rank >= Rank::Deuce
&& self.rank <= Rank::Ace
&& matches!(self.suit, Suit::Club | Suit::Diamond | Suit::Heart | Suit::Spade)
}

pub fn to_string(&self) -> String {
if !self.is_valid() {
"--".to_string()
} else {
let mut s = String::with_capacity(2);
s.push(self.rank.to_char());
s.push(self.suit.to_char());
s
}
}
}

pub fn card_swap(card1: &mut Card, card2: &mut Card) {
std::mem::swap(card1, card2);
}

impl Rank {
pub fn from_char(c: char) -> Rank {
char_to_rank(c)
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
char_to_suit(c)
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
for (i, ch) in chars.chars().enumerate() {
if ch == c {
return Some(i + 1);
}
}
None
}

pub fn char_to_rank(c: char) -> Rank {
match index_of(c, RANK_CHARS) {
Some(1) => Rank::Deuce,
Some(2) => Rank::Trey,
Some(3) => Rank::Four,
Some(4) => Rank::Five,
Some(5) => Rank::Six,
Some(6) => Rank::Seven,
Some(7) => Rank::Eight,
Some(8) => Rank::Nine,
Some(9) => Rank::Ten,
Some(10) => Rank::Jack,
Some(11) => Rank::Queen,
Some(12) => Rank::King,
Some(13) => Rank::Ace,
_ => Rank::InvalidRank,
}
}

pub fn char_to_suit(c: char) -> Suit {
match index_of(c, SUIT_CHARS) {
Some(1) => Suit::Club,
Some(2) => Suit::Diamond,
Some(3) => Suit::Heart,
Some(4) => Suit::Spade,
_ => Suit::InvalidSuit,
}
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
