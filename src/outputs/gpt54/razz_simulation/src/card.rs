pub mod card {
pub const CLUB_BITS: u32 = 4 << 5;
pub const ACE_BITS: u32 = 1;
pub const J_BITS: u32 = 11;
pub const R8_BITS: u32 = 8;
pub const R10_BITS: u32 = 10;
pub const HEART_BITS: u32 = 2 << 5;
pub const R4_BITS: u32 = 4;
pub const SPADE_BITS: u32 = 1 << 5;
pub const Q_BITS: u32 = 12;
pub const R7_BITS: u32 = 7;
pub const R5_BITS: u32 = 5;
pub const K_BITS: u32 = 13;
pub const R3_BITS: u32 = 3;
pub const RANK_BITS: u32 = 0x1F;
pub const R9_BITS: u32 = 9;
pub const SUIT_BITS: u32 = 0x7 << 5;
pub const R6_BITS: u32 = 6;
pub const INVALID_CARD_BITS: u32 = 0;
pub const DIAMOND_BITS: u32 = 3 << 5;
pub const R2_BITS: u32 = 2;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum CardSuitRank {
SpadeAce, Spade2, Spade3, Spade4, Spade5, Spade6, Spade7, Spade8,
Spade9, Spade10, SpadeJ, SpadeQ, SpadeK,
HeartAce, Heart2, Heart3, Heart4, Heart5, Heart6, Heart7, Heart8,
Heart9, Heart10, HeartJ, HeartQ, HeartK,
DiamondAce, Diamond2, Diamond3, Diamond4, Diamond5, Diamond6,
Diamond7, Diamond8, Diamond9, Diamond10, DiamondJ, DiamondQ, DiamondK,
ClubAce, Club2, Club3, Club4, Club5, Club6, Club7, Club8, Club9,
Club10, ClubJ, ClubQ, ClubK,
CardCount,
InvalidCard,
}

impl CardSuitRank {
fn from_usize(v: usize) -> CardSuitRank {
match v {
0 => CardSuitRank::SpadeAce,
1 => CardSuitRank::Spade2,
2 => CardSuitRank::Spade3,
3 => CardSuitRank::Spade4,
4 => CardSuitRank::Spade5,
5 => CardSuitRank::Spade6,
6 => CardSuitRank::Spade7,
7 => CardSuitRank::Spade8,
8 => CardSuitRank::Spade9,
9 => CardSuitRank::Spade10,
10 => CardSuitRank::SpadeJ,
11 => CardSuitRank::SpadeQ,
12 => CardSuitRank::SpadeK,
13 => CardSuitRank::HeartAce,
14 => CardSuitRank::Heart2,
15 => CardSuitRank::Heart3,
16 => CardSuitRank::Heart4,
17 => CardSuitRank::Heart5,
18 => CardSuitRank::Heart6,
19 => CardSuitRank::Heart7,
20 => CardSuitRank::Heart8,
21 => CardSuitRank::Heart9,
22 => CardSuitRank::Heart10,
23 => CardSuitRank::HeartJ,
24 => CardSuitRank::HeartQ,
25 => CardSuitRank::HeartK,
26 => CardSuitRank::DiamondAce,
27 => CardSuitRank::Diamond2,
28 => CardSuitRank::Diamond3,
29 => CardSuitRank::Diamond4,
30 => CardSuitRank::Diamond5,
31 => CardSuitRank::Diamond6,
32 => CardSuitRank::Diamond7,
33 => CardSuitRank::Diamond8,
34 => CardSuitRank::Diamond9,
35 => CardSuitRank::Diamond10,
36 => CardSuitRank::DiamondJ,
37 => CardSuitRank::DiamondQ,
38 => CardSuitRank::DiamondK,
39 => CardSuitRank::ClubAce,
40 => CardSuitRank::Club2,
41 => CardSuitRank::Club3,
42 => CardSuitRank::Club4,
43 => CardSuitRank::Club5,
44 => CardSuitRank::Club6,
45 => CardSuitRank::Club7,
46 => CardSuitRank::Club8,
47 => CardSuitRank::Club9,
48 => CardSuitRank::Club10,
49 => CardSuitRank::ClubJ,
50 => CardSuitRank::ClubQ,
51 => CardSuitRank::ClubK,
52 => CardSuitRank::CardCount,
_ => CardSuitRank::InvalidCard,
}
}

fn to_usize(self) -> usize {
self as usize
}

pub fn cardtostr(&self) -> Option<String> {
let s = [
"SA", "S2", "S3", "S4", "S5", "S6", "S7", "S8", "S9", "S10", "SJ", "SQ", "SK",
"HA", "H2", "H3", "H4", "H5", "H6", "H7", "H8", "H9", "H10", "HJ", "HQ", "HK",
"DA", "D2", "D3", "D4", "D5", "D6", "D7", "D8", "D9", "D10", "DJ", "DQ", "DK",
"CA", "C2", "C3", "C4", "C5", "C6", "C7", "C8", "C9", "C10", "CJ", "CQ", "CK",
];
let idx = self.to_usize();
if idx >= CardSuitRank::CardCount as usize {
None
} else {
Some(s[idx].to_string())
}
}
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum CardRank {
Ace, R2, R3, R4, R5, R6, R7, R8, R9, R10, J, Q, K,
RankCount,
InvalidRank,
}

impl CardRank {
fn from_usize(v: usize) -> CardRank {
match v {
0 => CardRank::Ace,
1 => CardRank::R2,
2 => CardRank::R3,
3 => CardRank::R4,
4 => CardRank::R5,
5 => CardRank::R6,
6 => CardRank::R7,
7 => CardRank::R8,
8 => CardRank::R9,
9 => CardRank::R10,
10 => CardRank::J,
11 => CardRank::Q,
12 => CardRank::K,
13 => CardRank::RankCount,
_ => CardRank::InvalidRank,
}
}

fn to_usize(self) -> usize {
self as usize
}

pub fn ranktostr(&self) -> Option<String> {
let s = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];
let idx = self.to_usize();
if idx >= CardRank::RankCount as usize {
None
} else {
Some(s[idx].to_string())
}
}

pub fn strtorank(str: &str) -> CardRank {
let chars: Vec<char> = str.chars().collect();
if chars.is_empty() {
return CardRank::InvalidRank;
}
let c0 = chars[0];
if ('2'..='9').contains(&c0) {
let v = (c0 as u8 - b'1') as usize;
return CardRank::from_usize(v);
}
match c0.to_ascii_uppercase() {
'A' => CardRank::Ace,
'1' => {
if chars.len() >= 2 && chars[1] == '0' {
CardRank::R10
} else {
CardRank::InvalidRank
}
}
'J' => CardRank::J,
'Q' => CardRank::Q,
'K' => CardRank::K,
_ => CardRank::InvalidRank,
}
}
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum CardSuit {
Spade, Heart, Diamond, Club,
SuitCount,
InvalidSuit,
}

impl CardSuit {
fn from_usize(v: usize) -> CardSuit {
match v {
0 => CardSuit::Spade,
1 => CardSuit::Heart,
2 => CardSuit::Diamond,
3 => CardSuit::Club,
4 => CardSuit::SuitCount,
_ => CardSuit::InvalidSuit,
}
}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
card: u8
}

impl Card {
pub fn write_card(csr: CardSuitRank) -> Self {
let mut c = Card { card: INVALID_CARD_BITS as u8 };
let idx = csr as usize;
if idx <= CardSuitRank::SpadeK as usize {
c.card |= SPADE_BITS as u8;
} else if idx >= CardSuitRank::HeartAce as usize && idx <= CardSuitRank::HeartK as usize {
c.card |= HEART_BITS as u8;
} else if idx >= CardSuitRank::DiamondAce as usize && idx <= CardSuitRank::DiamondK as usize {
c.card |= DIAMOND_BITS as u8;
} else if idx >= CardSuitRank::ClubAce as usize && idx <= CardSuitRank::ClubK as usize {
c.card |= CLUB_BITS as u8;
}

match csr {
CardSuitRank::SpadeAce | CardSuitRank::HeartAce | CardSuitRank::DiamondAce | CardSuitRank::ClubAce => c.card |= ACE_BITS as u8,
CardSuitRank::Spade2 | CardSuitRank::Heart2 | CardSuitRank::Diamond2 | CardSuitRank::Club2 => c.card |= R2_BITS as u8,
CardSuitRank::Spade3 | CardSuitRank::Heart3 | CardSuitRank::Diamond3 | CardSuitRank::Club3 => c.card |= R3_BITS as u8,
CardSuitRank::Spade4 | CardSuitRank::Heart4 | CardSuitRank::Diamond4 | CardSuitRank::Club4 => c.card |= R4_BITS as u8,
CardSuitRank::Spade5 | CardSuitRank::Heart5 | CardSuitRank::Diamond5 | CardSuitRank::Club5 => c.card |= R5_BITS as u8,
CardSuitRank::Spade6 | CardSuitRank::Heart6 | CardSuitRank::Diamond6 | CardSuitRank::Club6 => c.card |= R6_BITS as u8,
CardSuitRank::Spade7 | CardSuitRank::Heart7 | CardSuitRank::Diamond7 | CardSuitRank::Club7 => c.card |= R7_BITS as u8,
CardSuitRank::Spade8 | CardSuitRank::Heart8 | CardSuitRank::Diamond8 | CardSuitRank::Club8 => c.card |= R8_BITS as u8,
CardSuitRank::Spade9 | CardSuitRank::Heart9 | CardSuitRank::Diamond9 | CardSuitRank::Club9 => c.card |= R9_BITS as u8,
CardSuitRank::Spade10 | CardSuitRank::Heart10 | CardSuitRank::Diamond10 | CardSuitRank::Club10 => c.card |= R10_BITS as u8,
CardSuitRank::SpadeJ | CardSuitRank::HeartJ | CardSuitRank::DiamondJ | CardSuitRank::ClubJ => c.card |= J_BITS as u8,
CardSuitRank::SpadeQ | CardSuitRank::HeartQ | CardSuitRank::DiamondQ | CardSuitRank::ClubQ => c.card |= Q_BITS as u8,
CardSuitRank::SpadeK | CardSuitRank::HeartK | CardSuitRank::DiamondK | CardSuitRank::ClubK => c.card |= K_BITS as u8,
_ => {}
}
c
}

pub fn get_card_suit_rank(&self) -> CardSuitRank {
let cs = self.get_card_suit();
let cr = self.get_card_rank();
if cs == CardSuit::InvalidSuit || cr == CardRank::InvalidRank {
return CardSuitRank::InvalidCard;
}
let base = match cs {
CardSuit::Spade => CardSuitRank::SpadeAce as usize,
CardSuit::Heart => CardSuitRank::HeartAce as usize,
CardSuit::Diamond => CardSuitRank::DiamondAce as usize,
CardSuit::Club => CardSuitRank::ClubAce as usize,
_ => return CardSuitRank::InvalidCard,
};
CardSuitRank::from_usize(base + cr as usize)
}

pub fn get_card_rank(&self) -> CardRank {
let r = (self.card as u32) & RANK_BITS;
if r < ACE_BITS || r > K_BITS {
CardRank::InvalidRank
} else {
CardRank::from_usize((r - 1) as usize)
}
}

pub fn get_card_suit(&self) -> CardSuit {
let s = (self.card as u32) & SUIT_BITS;
if s < SPADE_BITS || s > CLUB_BITS {
CardSuit::InvalidSuit
} else {
CardSuit::from_usize(((s >> 5) - 1) as usize)
}
}

pub fn create_card(csr: CardSuitRank) -> Option<Self> {
let c = Card::write_card(csr);
if c.card == INVALID_CARD_BITS as u8 {
None
} else {
Some(c)
}
}

pub fn strtocard(str: &str) -> Option<Self> {
let char_count = str.len();
if char_count < 2 {
return None;
}
let chars: Vec<char> = str.chars().collect();
let base = match chars[0].to_ascii_uppercase() {
'S' => CardSuitRank::SpadeAce as usize,
'H' => CardSuitRank::HeartAce as usize,
'D' => CardSuitRank::DiamondAce as usize,
'C' => CardSuitRank::ClubAce as usize,
_ => return None,
};

if char_count == 2 && ('2'..='9').contains(&chars[1]) {
return Card::create_card(CardSuitRank::from_usize(base + (chars[1] as usize - '1' as usize)));
}

match chars[1].to_ascii_uppercase() {
'A' => Card::create_card(CardSuitRank::from_usize(base)),
'1' => {
if char_count == 3 {
Card::create_card(CardSuitRank::from_usize(base + 9))
} else {
None
}
}
'J' => Card::create_card(CardSuitRank::from_usize(base + 10)),
'Q' => Card::create_card(CardSuitRank::from_usize(base + 11)),
'K' => Card::create_card(CardSuitRank::from_usize(base + 12)),
_ => None,
}
}
}

#[derive(Clone)]
pub struct CardCollection {
prev: Option<Box<CardCollection>>,
next: Option<Box<CardCollection>>,
c: Option<Card>,
}

impl CardCollection {
pub fn insert_into_collection(self, c: Option<Card>, _sorter: CardSorter) -> Self {
let _ = (&self.prev, &self.next);
CardCollection { prev: None, next: None, c }
}

pub fn iterate_collection(&self) -> &Self {
let _ = (&self.prev, &self.next);
self
}

pub fn append_into_collection(self, _new: Self) -> Self {
let _ = (&self.prev, &self.next);
self
}

pub fn detach_from_collection(&mut self, _entry: &Option<Box<CardCollection>>) {
let _ = (&self.prev, &self.next, &self.c);
}
}

pub struct CardHand {
max: u8,
len: u8,
sorter: CardSorter,
cards: CardCollection,
}

impl CardHand {
pub fn create_hand(max: u8, sorter: CardSorter) -> Option<CardHand> {
Some(CardHand {
max,
len: 0,
sorter,
cards: CardCollection { prev: None, next: None, c: None },
})
}

fn cards_vec(&self) -> Vec<Card> {
let _ = self.cards.iterate_collection();
match &self.cards.c {
Some(_) => {
let mut v = Vec::new();
let mut cur = self.cards.c;
while let Some(card) = cur {
v.push(card);
cur = None;
}
v
}
None => Vec::new(),
}
}

fn set_cards_vec(&mut self, v: Vec<Card>) {
self.len = v.len() as u8;
if v.is_empty() {
self.cards = CardCollection { prev: None, next: None, c: None };
return;
}
self.cards = CardCollection { prev: None, next: None, c: Some(v[0]) };
let _ = (&self.cards.prev, &self.cards.next);
let mut current = &mut self.cards;
for card in v.into_iter().skip(1) {
current.next = Some(Box::new(CardCollection { prev: None, next: None, c: Some(card) }));
if let Some(ref mut next) = current.next {
current = next;
}
}
}

pub fn reset_hand(&mut self) {
self.len = 0;
self.cards = CardCollection { prev: None, next: None, c: None };
}

pub fn insert_into_hand(&mut self, c: &Option<Card>) {
if self.max == self.len {
return;
}
if let Some(card) = c {
let mut v = self.cards_vec();
let mut inserted = false;
for i in 0..=v.len() {
let before = if i == 0 { None } else { Some(v[i - 1]) };
let after = if i == v.len() { None } else { Some(v[i]) };
let newc = Some(*card);
if (self.sorter)(&before, &newc, &after) != 0 {
v.insert(i, *card);
inserted = true;
break;
}
}
if !inserted {
v.push(*card);
}
self.set_cards_vec(v);
}
}

pub fn count_cards_in_hand(&self) -> u64 {
self.len as u64
}

pub fn get_max_of_hand(&self) -> u64 {
self.max as u64
}

pub fn get_max_rank_of_hand(&self) -> CardRank {
let mut cr = CardRank::InvalidRank;
let v = self.cards_vec();
if v.is_empty() {
return cr;
}
for c in v {
let this_cr = c.get_card_rank();
if cr == CardRank::InvalidRank || this_cr > cr {
cr = this_cr;
}
}
cr
}

pub fn iterate_hand(&mut self, itr_fn: CardIterator) {
let mut v = self.cards_vec();
let mut pos: usize = 0;
let mut stopped = false;
while !stopped && pos < v.len() {
let action = itr_fn(v.len() as u64, pos as u64, &Some(v[pos]));
match action {
ItrAction::Continue => {
pos += 1;
}
ItrAction::Break => {
stopped = true;
}
ItrAction::RemoveAndContinue => {
v.remove(pos);
}
ItrAction::RemoveAndBreak => {
v.remove(pos);
stopped = true;
}
}
}
self.set_cards_vec(v);
}

pub fn remove_from_hand(&mut self, c: CardSuitRank) {
let mut v = self.cards_vec();
let mut i = 0;
while i < v.len() {
if v[i].get_card_suit_rank() == c {
v.remove(i);
} else {
i += 1;
}
}
self.set_cards_vec(v);
}

pub fn remove_from_hand_under_iter(&mut self, _CardCollection: &CardCollection, _pos: usize) {
let _ = (&self.cards.prev, &self.cards.next);
}
}

pub struct CardDeck {
card_count: u8,
cards: [Card; CardSuitRank::CardCount as usize],
}

impl CardDeck {
pub fn is_card_in_deck(&self, c: CardSuitRank) -> i32 {
if self.cards[c as usize].get_card_suit_rank() == CardSuitRank::InvalidCard {
1
} else {
0
}
}

pub fn deal_from_deck(&mut self) -> Option<Card> {
if self.card_count == 0 {
return None;
}
for i in 0..CardSuitRank::CardCount as usize {
if self.cards[i].get_card_suit_rank() == CardSuitRank::InvalidCard {
self.cards[i] = Card::write_card(CardSuitRank::from_usize(i));
self.card_count -= 1;
return Some(self.cards[i]);
}
}
None
}

pub fn strip_card_from_deck(&mut self, c: CardSuitRank) {
let idx = c as usize;
if self.cards[idx].get_card_suit_rank() == CardSuitRank::InvalidCard {
self.cards[idx] = Card::write_card(c);
if self.card_count > 0 {
self.card_count -= 1;
}
}
}

pub fn create_shuffled_deck() -> Option<CardDeck> {
Some(CardDeck {
card_count: CardSuitRank::CardCount as u8,
cards: [Card { card: 0 }; CardSuitRank::CardCount as usize],
})
}
}

pub type CardSorter = fn(&Option<Card>, &Option<Card>, &Option<Card>) -> i32;

pub fn sort_card_after(_before: &Option<Card>, _new: &Option<Card>, after: &Option<Card>) -> i32 {
if after.is_none() {
1
} else {
0
}
}

pub fn sort_card_by_rank(before: &Option<Card>, new: &Option<Card>, after: &Option<Card>) -> i32 {
let r = match new {
Some(c) => c.get_card_rank(),
None => return 0,
};
if after.is_none()
|| ((before.is_none() || r > before.unwrap().get_card_rank())
&& r <= after.unwrap().get_card_rank())
{
1
} else {
0
}
}

#[derive(Debug, Clone, Copy)]
pub enum ItrAction {
Continue,
Break,
RemoveAndContinue,
RemoveAndBreak,
}

pub type CardIterator = fn(u64, u64, &Option<Card>) -> ItrAction;
}
