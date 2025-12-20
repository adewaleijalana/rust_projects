#![allow(unused)]
use crate::card_deck::deck::Deck;

mod card_deck;

fn main() {
    let mut deck = Deck::new();

    println!("deck before shuffle: {:#?}", deck);

    deck.shuffle();

    println!("deck after shuffle: {:#?}", deck)
}
