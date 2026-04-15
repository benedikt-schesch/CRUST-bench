use std::env;
use std::process;
use rand::thread_rng;
use crate::cards::{Card, new_card_from_chars};
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
let _rng = thread_rng();
let mut cards = [Card::from_string("--"); 4];
for i in 0..4 {
let s = &args[i + 1];
let chars: Vec<char> = s.chars().collect();
if chars.len() != 2 {
usage(&args[0]);
}
cards[i] = new_card_from_chars(chars[0], chars[1]);
}
let full_deck = new_deck();
let mut deck: Vec<Card> = full_deck
.into_iter()
.filter(|c| !cards.iter().any(|r| r == c))
.collect();
let mut hand1 = [Card::from_string("--"); 5];
let mut hand2 = [Card::from_string("--"); 5];
let mut results = [0usize, 0usize, 0usize];
for _ in 0..ITERATIONS {
deck_shuffle(&mut deck);
hand1[0] = cards[0];
hand1[1] = cards[1];
hand2[0] = cards[2];
hand2[1] = cards[3];
for j in 0..3 {
hand1[2 + j] = deck[j];
hand2[2 + j] = deck[j];
}
hand_sort(&mut hand1);
hand_sort(&mut hand2);
let c = hand_compare(&hand1, &hand2);
if c > 0 {
results[0] += 1;
} else if c == 0 {
results[1] += 1;
} else {
results[2] += 1;
}
}
println!(
"WIN: {:.2}\tTIE: {:.2}\tLOSS: {:.2}",
results[0] as f64 / ITERATIONS as f64,
results[1] as f64 / ITERATIONS as f64,
results[2] as f64 / ITERATIONS as f64
);
}
