use razz_simulation::card::*;
static SEED3_DEALING_ORDER: [CardSuitRank; 52] = [
CardSuitRank::Heart9, CardSuitRank::SpadeAce, CardSuitRank::Heart10, CardSuitRank::Club2,
CardSuitRank::Diamond6, CardSuitRank::HeartQ, CardSuitRank::Diamond2, CardSuitRank::Diamond9,
CardSuitRank::Heart7, CardSuitRank::Club10, CardSuitRank::Club4, CardSuitRank::Diamond5,
CardSuitRank::ClubK, CardSuitRank::Spade2, CardSuitRank::HeartK, CardSuitRank::Heart4,
CardSuitRank::Diamond10, CardSuitRank::ClubQ, CardSuitRank::Spade5, CardSuitRank::SpadeK,
CardSuitRank::Heart6, CardSuitRank::DiamondJ, CardSuitRank::Spade9, CardSuitRank::ClubAce,
CardSuitRank::Club5, CardSuitRank::Diamond8, CardSuitRank::Club9, CardSuitRank::Heart2,
CardSuitRank::SpadeJ, CardSuitRank::Club7, CardSuitRank::Diamond4, CardSuitRank::DiamondK,
CardSuitRank::DiamondQ, CardSuitRank::Spade8, CardSuitRank::Spade6, CardSuitRank::Spade7,
CardSuitRank::SpadeQ, CardSuitRank::Heart8, CardSuitRank::HeartJ, CardSuitRank::Diamond3,
CardSuitRank::Diamond7, CardSuitRank::Spade3, CardSuitRank::Heart3, CardSuitRank::ClubJ,
CardSuitRank::Club6, CardSuitRank::HeartAce, CardSuitRank::Club3, CardSuitRank::DiamondAce,
CardSuitRank::Heart5, CardSuitRank::Spade4, CardSuitRank::Spade10, CardSuitRank::Club8,
];
fn test_sort_card_by_rank_1(_len: u64, _pos: u64, c: &Option<Card>) -> ItrAction {
if let Some(card) = c {
let rank = card.get_card_suit_rank();
if rank == CardSuitRank::SpadeAce || rank == CardSuitRank::Diamond2 || rank == CardSuitRank::Club2 ||
rank == CardSuitRank::Diamond6 || rank == CardSuitRank::Heart9 || rank == CardSuitRank::Heart10 ||
rank == CardSuitRank::HeartQ {
return ItrAction::Continue;
}
}
ItrAction::Continue
}
fn test_sort_card_by_rank_2(_len: u64, _pos: u64, c: &Option<Card>) -> ItrAction {
if let Some(card) = c {
let rank = card.get_card_suit_rank();
if rank == CardSuitRank::SpadeAce || rank == CardSuitRank::Diamond2 || rank == CardSuitRank::Club2 ||
rank == CardSuitRank::Heart9 || rank == CardSuitRank::Heart10 || rank == CardSuitRank::HeartQ {
return ItrAction::Continue;
}
}
ItrAction::Continue
}
fn test_sort_card_by_rank_3(_len: u64, _pos: u64, c: &Option<Card>) -> ItrAction {
if let Some(card) = c {
let rank = card.get_card_suit_rank();
if rank == CardSuitRank::SpadeAce || rank == CardSuitRank::Diamond2 || rank == CardSuitRank::Club2 ||
rank == CardSuitRank::Heart9 || rank == CardSuitRank::Heart10 {
return ItrAction::Continue;
}
}
ItrAction::Continue
}
fn test_sort_card_by_rank_4(_len: u64, _pos: u64, c: &Option<Card>) -> ItrAction {
if let Some(card) = c {
let rank = card.get_card_suit_rank();
if rank == CardSuitRank::Heart9 || rank == CardSuitRank::Heart10 {
return ItrAction::Continue;
}
}
ItrAction::Continue
}
fn test_empty_hand(_len: u64, _pos: u64, _c: &Option<Card>) -> ItrAction {
ItrAction::Continue
}
fn test_card_operations() {
let mut c: Option<Card>;
let mut d: Option<CardDeck>;
let mut h: Option<CardHand>;
assert!(CardSuitRank::SpadeAce < CardSuitRank::SpadeK);
assert!(CardSuitRank::HeartK < CardSuitRank::ClubK);
assert!(CardRank::Ace < CardRank::R3);
c = Card::create_card(CardSuitRank::SpadeAce);
assert!(c.is_some());
assert_eq!(c.as_ref().unwrap().get_card_suit_rank(), CardSuitRank::SpadeAce);
assert_eq!(c.as_ref().unwrap().get_card_rank(), CardRank::Ace);
assert_eq!(c.as_ref().unwrap().get_card_suit(), CardSuit::Spade);
c = Card::create_card(CardSuitRank::CardCount);
assert!(c.is_none());
c = Card::create_card(CardSuitRank::Club8);
assert!(c.is_some());
assert_eq!(c.as_ref().unwrap().get_card_suit_rank(), CardSuitRank::Club8);
assert_eq!(c.as_ref().unwrap().get_card_rank(), CardRank::R8);
assert_eq!(c.as_ref().unwrap().get_card_suit(), CardSuit::Club);
c = Card::strtocard("S8");
assert!(c.is_some());
assert_eq!(c.as_ref().unwrap().get_card_suit_rank(), CardSuitRank::Spade8);
assert_eq!(c.as_ref().unwrap().get_card_rank(), CardRank::R8);
assert_eq!(c.as_ref().unwrap().get_card_suit(), CardSuit::Spade);
c = Card::strtocard("dk");
assert!(c.is_some());
assert_eq!(c.as_ref().unwrap().get_card_suit_rank(), CardSuitRank::DiamondK);
assert_eq!(c.as_ref().unwrap().get_card_rank(), CardRank::K);
assert_eq!(c.as_ref().unwrap().get_card_suit(), CardSuit::Diamond);
c = Card::strtocard("Ca");
assert!(c.is_some());
assert_eq!(c.as_ref().unwrap().get_card_suit_rank(), CardSuitRank::ClubAce);
assert_eq!(c.as_ref().unwrap().get_card_rank(), CardRank::Ace);
assert_eq!(c.as_ref().unwrap().get_card_suit(), CardSuit::Club);
c = Card::strtocard("hJ");
assert!(c.is_some());
assert_eq!(c.as_ref().unwrap().get_card_suit_rank(), CardSuitRank::HeartJ);
assert_eq!(c.as_ref().unwrap().get_card_rank(), CardRank::J);
assert_eq!(c.as_ref().unwrap().get_card_suit(), CardSuit::Heart);
c = Card::strtocard("SQ");
assert!(c.is_some());
assert_eq!(c.as_ref().unwrap().get_card_suit_rank(), CardSuitRank::SpadeQ);
assert_eq!(c.as_ref().unwrap().get_card_rank(), CardRank::Q);
assert_eq!(c.as_ref().unwrap().get_card_suit(), CardSuit::Spade);
c = Card::strtocard("SS");
assert!(c.is_none());
c = Card::strtocard("S0");
assert!(c.is_none());
c = Card::strtocard("S1");
assert!(c.is_none());
c = Card::strtocard("a2");
assert!(c.is_none());
assert_eq!(CardRank::strtorank("ace"), CardRank::Ace);
assert_eq!(CardRank::strtorank("8"), CardRank::R8);
assert_eq!(CardRank::strtorank("K"), CardRank::K);
assert_eq!(CardRank::strtorank("10"), CardRank::R10);
assert_eq!(CardRank::strtorank("1"), CardRank::InvalidRank);
assert_eq!(CardSuitRank::Spade8.cardtostr().unwrap(), "S8");
assert_eq!(CardSuitRank::Club10.cardtostr().unwrap(), "C10");
assert_eq!(CardSuitRank::SpadeAce.cardtostr().unwrap(), "SA");
assert_eq!(CardSuitRank::ClubK.cardtostr().unwrap(), "CK");
assert!(CardSuitRank::CardCount.cardtostr().is_none());
assert_eq!(CardRank::R8.ranktostr().unwrap(), "8");
assert_eq!(CardRank::R10.ranktostr().unwrap(), "10");
assert_eq!(CardRank::Ace.ranktostr().unwrap(), "A");
assert_eq!(CardRank::K.ranktostr().unwrap(), "K");
assert!(CardRank::InvalidRank.ranktostr().is_none());
d = CardDeck::create_shuffled_deck();
assert!(d.is_some());
assert!(d.as_ref().unwrap().count_card_in_deck(CardSuitRank::HeartK) != 0);
d.as_mut().unwrap().strip_card_from_deck(CardSuitRank::HeartK);
assert!(d.as_ref().unwrap().count_card_in_deck(CardSuitRank::HeartK) == 0);
d.as_mut().unwrap().strip_card_from_deck(CardSuitRank::Heart9);
assert!(d.as_ref().unwrap().count_card_in_deck(CardSuitRank::Heart9) == 0);
d = CardDeck::create_shuffled_deck();
h = CardHand::create_hand(7, sort_card_by_rank);
assert!(h.is_some());
assert_eq!(h.as_ref().unwrap().get_max_rank_of_hand(), CardRank::InvalidRank);
h.as_mut().unwrap().add_card(CardSuitRank::Heart9);
assert_eq!(h.as_ref().unwrap().get_max_rank_of_hand(), CardRank::R9);
h.as_mut().unwrap().add_card(CardSuitRank::Heart10);
assert_eq!(h.as_ref().unwrap().get_max_rank_of_hand(), CardRank::R10);
h.as_mut().unwrap().add_card(CardSuitRank::HeartQ);
assert_eq!(h.as_ref().unwrap().get_max_rank_of_hand(), CardRank::Q);
h.as_mut().unwrap().remove_from_hand(CardSuitRank::Diamond6);
assert_eq!(h.as_ref().unwrap().get_max_rank_of_hand(), CardRank::Q);
h.as_mut().unwrap().remove_from_hand(CardSuitRank::HeartQ);
assert_eq!(h.as_ref().unwrap().get_max_rank_of_hand(), CardRank::R10);
h.as_mut().unwrap().remove_from_hand(CardSuitRank::Heart10);
h.as_mut().unwrap().remove_from_hand(CardSuitRank::Heart9);
assert_eq!(h.as_ref().unwrap().get_max_rank_of_hand(), CardRank::InvalidRank);
d = CardDeck::create_shuffled_deck();
h = CardHand::create_hand(1, sort_card_by_rank);
h.as_mut().unwrap().add_card(CardSuitRank::Heart9);
assert_eq!(h.as_ref().unwrap().get_max_rank_of_hand(), CardRank::R9);
h.as_mut().unwrap().remove_from_hand(CardSuitRank::HeartQ); 
h.as_mut().unwrap().remove_from_hand(CardSuitRank::HeartQ);
d = CardDeck::create_shuffled_deck();
h = CardHand::create_hand(3, sort_card_by_rank);
h.as_mut().unwrap().add_card(CardSuitRank::Heart9);
assert_eq!(h.as_ref().unwrap().get_max_rank_of_hand(), CardRank::R9);
h.as_mut().unwrap().add_card(CardSuitRank::SpadeAce);
assert_eq!(h.as_ref().unwrap().get_max_rank_of_hand(), CardRank::R9);
h.as_mut().unwrap().add_card(CardSuitRank::Heart10);
assert_eq!(h.as_ref().unwrap().get_max_rank_of_hand(), CardRank::R10);
let mut card_count = [0u64; CardSuitRank::CardCount as usize];
let end = CardSuitRank::CardCount as usize * 1000;
for _ in 0..1000 {
d = CardDeck::create_shuffled_deck();
if let Some(ref mut deck) = d {
for _ in 0..52 {
if let Some(card) = deck.deal_card() {
card_count[card as usize] += 1;
}
}
}
}
for i in 0..CardSuitRank::CardCount as usize {
assert_eq!(card_count[i], 1000);
}
}
fn main() {
test_card_operations();
println!("All tests passed!");
}
