use crate::cards::{Card, card_compare, RANKS_PER_DECK, Rank};
use crate::bucket::Bucket;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum HandType {
InvalidHand,
HighCard,
Pair,
TwoPair,
ThreeOfAKind,
Straight,
Wheel,
Flush,
FullHouse,
FourOfAKind,
StraightFlush,
WheelFlush,
}
fn hand_type_value(ht: HandType) -> i32 {
match ht {
HandType::InvalidHand => 0,
HandType::HighCard => 1,
HandType::Pair => 2,
HandType::TwoPair => 3,
HandType::ThreeOfAKind => 4,
HandType::Wheel => 5,
HandType::Straight => 6,
HandType::Flush => 7,
HandType::FullHouse => 8,
HandType::FourOfAKind => 9,
HandType::WheelFlush => 10,
HandType::StraightFlush => 11,
}
}
pub fn hand_compare(hand1: &[Card; 5], hand2: &[Card; 5]) -> i32 {
let type1 = hand_classify(hand1);
let type2 = hand_classify(hand2);
let diff = hand_type_value(type1) - hand_type_value(type2);
if diff != 0 {
return diff;
}
for i in 0..5 {
let cmp = card_compare(&hand1[i], &hand2[i]);
if cmp != 0 {
return cmp;
}
}
0
}
pub fn hand_sort(hand: &mut [Card; 5]) {
let mut buckets: [Bucket; RANKS_PER_DECK] = [
Bucket::new(), Bucket::new(), Bucket::new(), Bucket::new(),
Bucket::new(), Bucket::new(), Bucket::new(), Bucket::new(),
Bucket::new(), Bucket::new(), Bucket::new(), Bucket::new(),
Bucket::new(),
];
for i in 0..5 {
let bucket_index = hand[i].rank as usize - 1;
buckets[bucket_index].add(hand[i]);
}
let mut index = 0;
for count in (1..=4).rev() {
for j in (0..RANKS_PER_DECK).rev() {
if buckets[j].count == count {
for k in 0..count {
hand[index] = buckets[j].cards[k].unwrap();
index += 1;
}
}
}
}
}
pub fn hand_classify(cards: &[Card; 5]) -> HandType {
if hand_is_flush(cards) {
if hand_is_straight(cards) {
return HandType::StraightFlush;
}
if hand_is_wheel(cards) {
return HandType::WheelFlush;
}
return HandType::Flush;
} else {
if hand_is_four_of_a_kind(cards) {
return HandType::FourOfAKind;
}
if hand_is_full_house(cards) {
return HandType::FullHouse;
}
if hand_is_straight(cards) {
return HandType::Straight;
}
if hand_is_wheel(cards) {
return HandType::Wheel;
}
if hand_is_three_of_a_kind(cards) {
return HandType::ThreeOfAKind;
}
if hand_is_two_pair(cards) {
return HandType::TwoPair;
}
if hand_is_pair(cards) {
return HandType::Pair;
}
HandType::HighCard
}
}
pub fn hand_is_valid(cards: &[Card; 5]) -> bool {
for i in 0..5 {
if !cards[i].is_valid() {
return false;
}
}
true
}
fn hand_is_straight_flush(cards: &[Card; 5]) -> bool {
hand_is_straight(cards) && hand_is_flush(cards)
}
fn hand_is_wheel_flush(cards: &[Card; 5]) -> bool {
hand_is_wheel(cards) && hand_is_flush(cards)
}
fn hand_is_four_of_a_kind(cards: &[Card; 5]) -> bool {
cards[0].rank == cards[3].rank
}
fn hand_is_full_house(cards: &[Card; 5]) -> bool {
cards[0].rank == cards[2].rank && cards[3].rank == cards[4].rank
}
fn hand_is_flush(cards: &[Card; 5]) -> bool {
let suit = cards[0].suit;
for i in 1..5 {
if cards[i].suit != suit {
return false;
}
}
true
}
fn hand_is_straight(cards: &[Card; 5]) -> bool {
(cards[0].rank as i32) == (cards[1].rank as i32) + 1 &&
(cards[0].rank as i32) == (cards[4].rank as i32) + 4
}
fn hand_is_wheel(cards: &[Card; 5]) -> bool {
cards[0].rank == Rank::Ace && cards[1].rank == Rank::Five
}
fn hand_is_three_of_a_kind(cards: &[Card; 5]) -> bool {
cards[0].rank == cards[2].rank
}
fn hand_is_two_pair(cards: &[Card; 5]) -> bool {
cards[2].rank == cards[3].rank
}
fn hand_is_pair(cards: &[Card; 5]) -> bool {
cards[0].rank == cards[1].rank
}
fn hand_is_high_card(cards: &[Card; 5]) -> bool {
let _ = cards;
true
}
