use std::env;
use std::process;
use rand::thread_rng;
use crate::cards::{Card, card_equal};
use crate::deck::{new_deck, deck_shuffle};
use crate::hands::{hand_sort, hand_compare};
const ITERATIONS: usize = 2_000_000;
fn usage(program_name: &str) {
eprintln!("Usage: {} <c1> <c2> <c3> <c4>", program_name);
process::exit(1);
}
fn main() {
let args: Vec<String> = env::args().collect();
if args.len() != 5 {
usage(&args[0]);
}
let mut hole_cards: [Card; 4] = [Card::new(crate::cards::Rank::InvalidRank, crate::cards::Suit::InvalidSuit); 4];
for i in 0..4 {
hole_cards[i] = Card::from_string(&args[i + 1]);
}
let mut deck = new_deck();
deck.retain(|c| !hole_cards.iter().any(|h| card_equal(c, h)));
let mut results: [usize; 3] = [0, 0, 0];
for _ in 0..ITERATIONS {
deck_shuffle(&mut deck);
let mut hand1: [Card; 5] = [
hole_cards[0],
hole_cards[1],
deck[0],
deck[1],
deck[2],
];
let mut hand2: [Card; 5] = [
hole_cards[2],
hole_cards[3],
deck[0],
deck[1],
deck[2],
];
hand_sort(&mut hand1);
hand_sort(&mut hand2);
let cmp = hand_compare(&hand1, &hand2);
if cmp > 0 {
results[0] += 1;
} else if cmp == 0 {
results[1] += 1;
} else {
results[2] += 1;
}
}
let total = ITERATIONS as f64;
println!("WIN: {:.2}\tTIE: {:.2}\tLOSS: {:.2}",
results[0] as f64 / total,
results[1] as f64 / total,
results[2] as f64 / total);
}
